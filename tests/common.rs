#![allow(unused)]
#![allow(clippy::missing_panics_doc)]

use std::collections::HashMap;
use std::env;
use std::fs::{self, File};
use std::io::Write;

use cargo_test_support::{
    compare,
    registry::{HttpServer, RegistryBuilder, Request, Response, TestRegistry},
    TestEnv,
};
use snapbox::cmd::Command;

/// Invoke `cargo-liner liner` with the test environment.
#[must_use]
pub fn cargo_liner() -> Command {
    Command::new(snapbox::cmd::cargo_bin(clap::crate_name!()))
        .with_assert(compare::assert_ui())
        .test_env()
        .arg("liner")
        .arg("--color=never")
}

/// Initializes the registry without a token, with an HTTP index and API, and
/// with a custom API responder that mimics `cargo search`'s endpoint.
///
/// Returns a test registry handle that must be saved in order for the test
/// server to be kept alive.
#[must_use]
pub fn init_registry() -> TestRegistry {
    // Must be fetched ahead of time, otherwise the underlying global state is
    // uninitialized for some reason, very probably because the request handler
    // below is ran in another thread and the global state is thread-local as
    // each thread normally corresponds to a different test ran in parallel of
    // others.
    let test_root = cargo_test_support::paths::root();
    RegistryBuilder::new()
        .no_configure_token()
        .http_api()
        .http_index()
        .add_responder("/api/v1/crates", move |req: &Request, _srv: &HttpServer| {
            let req_params = req
                .url
                .query_pairs()
                .map(|(k, v)| (k.into_owned(), v.into_owned()))
                .collect::<HashMap<_, _>>();
            // Search mandatory parameter: string query.
            let req_pkg = req_params.get("q").unwrap();
            let dl_path = test_root.join("dl");

            // Use the `dl` directory instead of the `registry` one in order to
            // easily test whether a packaged has previously been published or
            // not: test if a sub-directory of the package's name exists.
            let pkg_res = dl_path
                .join(req_pkg)
                .read_dir()
                .map_or(String::new(), |mut dir| {
                    let pkg_ver = dir
                        .next()
                        .unwrap()
                        .unwrap()
                        .file_name()
                        .into_string()
                        .unwrap();
                    format!(
                        r#"{{
                            "name": "{req_pkg}",
                            "description": "whatever",
                            "newest_version": "{pkg_ver}",
                            "max_version": "{pkg_ver}"
                        }}"#,
                    )
                });

            // Return only the information actually required by the `cargo
            // search` command. Might need to be extended in the future if more
            // gets required.
            Response {
                code: 200,
                headers: Vec::new(),
                body: format!(
                    r#"{{
                        "crates": [{}],
                        "meta": {{
                            "next_page": "?q={}&page=2",
                            "prev_page": null,
                            "total": 1
                        }}
                    }}"#,
                    pkg_res,
                    req.url.query().unwrap(),
                )
                .as_bytes()
                .to_vec(),
            }
        })
        .build()
}

/// Sets various environment variables to mimic Cargo's testing framework.
pub fn set_env() {
    let tmp_home = cargo_test_support::paths::home();
    env::set_var("HOME", tmp_home.to_str().unwrap());
    env::set_var("CARGO_HOME", tmp_home.join(".cargo").to_str().unwrap());
    env::set_var(
        "__CARGO_TEST_ROOT",
        cargo_test_support::paths::global_root().to_str().unwrap(),
    );
    env::set_var("CARGO_INCREMENTAL", "0");
}

/// Fakes the result of a `cargo install` run for the given package name and
/// version.
///
/// Creates the `$CARGO_HOME/bin` directory if it does not exist; adds an empty
/// file of the package's name in it; adds the package's name and version to
/// the `$CARGO_HOME/.crates.toml` file, creating it if it does not exist.
pub fn fake_install(pkg: &str, ver: &str) {
    let tmp_home = cargo_test_support::paths::home();
    let tmp_cargo_home = tmp_home.join(".cargo");
    let tmp_cargo_home_bin = tmp_cargo_home.join("bin");
    let tmp_cargo_home_crates = tmp_cargo_home.join(".crates.toml");

    // bin directory things.
    fs::create_dir_all(tmp_cargo_home_bin.clone()).unwrap();
    File::options()
        .write(true)
        .create_new(true)
        .open(tmp_cargo_home_bin.join(pkg))
        .unwrap();

    // Initialize .crates.toml if not exist.
    if let Ok(mut crates_toml) = File::options()
        .write(true)
        .create_new(true)
        .open(tmp_cargo_home_crates.clone())
    {
        writeln!(&mut crates_toml, "[v1]").unwrap();
    }

    // Append new package to it.
    writeln!(
        &mut File::options()
            .append(true)
            .open(tmp_cargo_home_crates)
            .unwrap(),
        "\"{pkg} {ver} (registry+https://github.com/rust-lang/crates.io-index)\" = [\"{pkg}\"]",
    )
    .unwrap();
}

/// Runs [`fake_install`] for each package name and version pair yielded by the
/// given iterator.
pub fn fake_install_all<'p, 'v>(pkg_vers: impl IntoIterator<Item = (&'p str, &'v str)>) {
    for (pkg, ver) in pkg_vers {
        fake_install(pkg, ver);
    }
}
