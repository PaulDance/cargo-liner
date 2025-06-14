#![expect(
    unused,
    reason = "This is testing-only code, so is allowed to be saved \"just in case\"."
)]

use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::{env, io, iter};

use cargo_test_support::registry::{
    HttpServer, Package, RegistryBuilder, Request, Response, TestRegistry,
};
use cargo_test_support::{TestEnvCommandExt, compare};
use semver::Version;
use snapbox::cmd::Command;

/// List of example packages, their respective versions and if they are locally
/// installed, including self.
pub const FIXTURE_PACKAGES: [(&str, &str, bool); 3] = [
    ("abc", "0.0.1", false),
    ("def", "0.0.2", true),
    ("cargo-liner", "0.0.3", false),
];

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
            #[expect(
                clippy::allow_attributes,
                reason = "Unable to make `expect` work here."
            )]
            #[allow(clippy::print_stderr, reason = "Testing-only module.")]
            let (res_len, pkg_res) = dl_path
                .join(req_pkg)
                .read_dir()
                .inspect_err(|err| {
                    eprintln!(
                        "Error when reading directory `{req_pkg}` under `{}`: {err:#?}",
                        dl_path.display(),
                    );
                })
                // Sub-directories have versions as names: take the max.
                .map(|mut dir_itr| {
                    dir_itr
                        .map(|subdir| {
                            subdir
                                .unwrap()
                                .file_name()
                                .into_string()
                                .unwrap()
                                .parse::<Version>()
                                .unwrap()
                        })
                        .max()
                        .unwrap()
                })
                .map_or((0, String::new()), |pkg_ver| {
                    (
                        1,
                        format!(
                            r#"{{
                                "name": "{req_pkg}",
                                "description": "whatever",
                                "newest_version": "{pkg_ver}",
                                "max_version": "{pkg_ver}"
                            }}"#,
                        ),
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
                        "crates": [{pkg_res}],
                        "meta": {{
                            "next_page": "?q={}&page=2",
                            "prev_page": null,
                            "total": {res_len}
                        }}
                    }}"#,
                    req.url.query().unwrap(),
                )
                .as_bytes()
                .to_vec(),
            }
        })
        .build()
}

/// Applies Cargo's testing environment to the current process.
///
/// # Safety
///
/// Should not be used in parallel of other test threads. This function is not
/// marked as unsafe in order to avoid chore in testing-only code, but should
/// still be considered as such.
pub fn set_env() {
    struct CurrentEnv;
    impl TestEnvCommandExt for CurrentEnv {
        fn current_dir<S: AsRef<Path>>(self, path: S) -> Self {
            env::set_current_dir(path).unwrap();
            self
        }
        fn env<S: AsRef<OsStr>>(self, key: &str, value: S) -> Self {
            // SAFETY: upheld by caller.
            unsafe { env::set_var(key, value) };
            self
        }
        fn env_remove(self, key: &str) -> Self {
            // SAFETY: upheld by caller.
            unsafe { env::remove_var(key) };
            self
        }
    }
    CurrentEnv {}.test_env();
}

/// Returns the filesystem path pointing to the user configuration.
pub fn user_config_path() -> PathBuf {
    cargo_test_support::paths::home().join(".cargo/liner.toml")
}

/// Reads the user configuration file to a string.
#[must_use]
pub fn read_user_config() -> String {
    fs::read_to_string(user_config_path()).unwrap()
}

/// Writes the given lines of content to the `$CARGO_HOME/liner.toml` user
/// configuration.
pub fn write_user_config(content_lines: &[&str]) {
    let _ = fs::create_dir(cargo_test_support::paths::cargo_home())
        .inspect_err(|err| assert_eq!(err.kind(), io::ErrorKind::AlreadyExists));
    fs::write(user_config_path(), content_lines.join("\n")).unwrap();
}

/// Runs [`write_user_config`] with an example configuration excluding self.
pub fn fixture_write_user_config() {
    let cfg_pkg_lines = FIXTURE_PACKAGES
        .into_iter()
        .filter(|(pkg, _, _)| *pkg != clap::crate_name!())
        .map(|(pkg, _, _)| format!("{pkg} = '*'"))
        .collect::<Vec<_>>();
    let cfg_lines = iter::once("[packages]")
        .chain(cfg_pkg_lines.iter().map(String::as_str))
        .collect::<Vec<_>>();
    write_user_config(&cfg_lines);
}

/// Asserts the user configuration does not exist.
pub fn assert_user_config_absent() {
    assert!(!user_config_path().exists());
}

/// Asserts the user configuration file's contents are exactly equal to the
/// given string.
pub fn assert_user_config_eq(test_str: &str) {
    assert_eq!(read_user_config(), test_str);
}

/// Asserts the user configuration file's contents are exactly equal to the
/// given one's.
pub fn assert_user_config_eq_path(test_path: impl AsRef<Path>) {
    assert_user_config_eq(&fs::read_to_string(test_path).unwrap());
}

/// Fakes the result of a `cargo install` run for the given package name and
/// version.
///
/// Creates the `$CARGO_HOME/bin` directory if it does not exist; adds an empty
/// file of the package's name in it; adds the package's name and version to
/// the `$CARGO_HOME/.crates.toml` file, creating it if it does not exist.
pub fn fake_install(pkg: &str, ver: &str, locally_installed: bool) {
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
        "\"{pkg} {ver} ({source})\" = [\"{pkg}\"]",
        source = if locally_installed {
            "path+file:///a/b/c"
        } else {
            "registry+https://github.com/rust-lang/crates.io-index"
        }
    )
    .unwrap();
}

/// Runs [`fake_install`] and [`assert_installed`] with the current crate as
/// the package and a default version.
pub fn fake_install_self() {
    fake_install(clap::crate_name!(), "0.0.0", false);
    assert_installed_self();
}

/// Runs [`fake_install`] for each package name and version pair yielded by the
/// given iterator.
pub fn fake_install_all<'p, 'v>(pkg_vers: impl IntoIterator<Item = (&'p str, &'v str, bool)>) {
    for (pkg, ver, locally) in pkg_vers {
        fake_install(pkg, ver, locally);
    }
}

/// Runs [`fake_install_all`] on some example packages, including self.
pub fn fixture_fake_install() {
    fake_install_all(FIXTURE_PACKAGES);
}

/// Asserts that the given package is installed in the testing environment.
pub fn assert_installed(pkg: &'static str) {
    cargo_test_support::install::assert_has_installed_exe(
        cargo_test_support::paths::cargo_home(),
        pkg,
    );
}

/// Asserts that the current package is installed.
pub fn assert_installed_self() {
    assert_installed(clap::crate_name!());
}

/// Runs [`assert_installed`] on all the packages yielded by the given iterator.
pub fn assert_installed_all(pkgs: impl IntoIterator<Item = &'static str>) {
    for pkg in pkgs {
        assert_installed(pkg);
    }
}

/// Runs [`assert_installed_all`] on some example packages, self included.
pub fn fixture_assert_installed() {
    assert_installed_all(FIXTURE_PACKAGES.into_iter().map(|(pkg, _, _)| pkg));
}

/// Asserts that the given package is not installed in the testing environment.
pub fn assert_not_installed(pkg: &'static str) {
    cargo_test_support::install::assert_has_not_installed_exe(
        cargo_test_support::paths::cargo_home(),
        pkg,
    );
}

/// Runs [`assert_not_installed`] on all the packages yielded by the given
/// iterator.
pub fn assert_not_installed_all(pkgs: impl IntoIterator<Item = &'static str>) {
    for pkg in pkgs {
        assert_not_installed(pkg);
    }
}

/// Publishes the given package name and version to the local fake registry
/// with minimal contents.
pub fn fake_publish(pkg: &str, ver: &str) {
    Package::new(pkg, ver)
        .file("src/main.rs", "fn main() {}")
        .publish();
}

/// Runs [`fake_publish`] for each package name and version pair yielded by the
/// given iterator.
pub fn fake_publish_all<'p, 'v>(pkg_vers: impl IntoIterator<Item = (&'p str, &'v str)>) {
    for (pkg, ver) in pkg_vers {
        fake_publish(pkg, ver);
    }
}

/// Runs [`fake_publish_all`] on some example packages, self included.
pub fn fixture_fake_publish() {
    fake_publish_all(FIXTURE_PACKAGES.into_iter().map(|(pkg, ver, _)| (pkg, ver)));
}

/// Runs [`fake_publish_all`] on some example packages, self included, but with
/// other packages' version bumped by one patch.
pub fn fixture_fake_publish_newer_others() {
    let pkgs = FIXTURE_PACKAGES
        .into_iter()
        .map(|(pkg, ver, locally)| {
            (
                pkg,
                if pkg == clap::crate_name!() {
                    ver.to_owned()
                } else {
                    let mut ver = Version::parse(ver).unwrap();
                    ver.patch += 1;
                    ver.to_string()
                },
                locally,
            )
        })
        .collect::<Vec<_>>();
    fake_publish_all(pkgs.iter().map(|(pkg, ver, _)| (*pkg, ver.as_str())));
}
