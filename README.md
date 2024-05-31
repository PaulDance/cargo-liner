<div align="center">
    <img
        src="https://cdn.dribbble.com/users/959848/screenshots/4159478/media/e21863defd968fff7f27690f36a2ccef.gif"
        alt="Cargo ship on its way"
        width="60%"
    />
    <br/>
    <h1>Cargo Liner</h1>
    <h3>Configuration-oriented wrapper around <code>cargo install</code></h3>
    <a href="https://github.com/PaulDance/cargo-liner/releases/latest">
        <img
            alt="GitHub release (latest by date including pre-releases)"
            src="https://img.shields.io/github/v/release/PaulDance/cargo-liner?style=for-the-badge&include_prereleases"
        />
    </a>
    <a href="https://github.com/PaulDance/cargo-liner/releases/latest">
        <img
            alt="GitHub (Pre-)Release Date"
            src="https://img.shields.io/github/release-date-pre/PaulDance/cargo-liner?style=for-the-badge&display_date=published_at"
        />
    </a>
    <a href="https://crates.io/crates/cargo-liner">
        <img
            alt="Crates.io version"
            src="https://img.shields.io/crates/v/cargo-liner?style=for-the-badge"
        />
    </a>
    <br/>
    <a href="https://github.com/PaulDance/cargo-liner/actions">
        <img
            alt="GitHub Workflow Status"
            src="https://img.shields.io/github/actions/workflow/status/PaulDance/cargo-liner/ci.yml?style=for-the-badge&branch=master"
        />
    </a>
    <a href="https://crates.io/crates/cargo-liner#user-content-bottom">
        <img
            alt="Crates.io downloads"
            src="https://img.shields.io/crates/d/cargo-liner?style=for-the-badge"
        />
    </a>
    <br/>
    <a href="https://github.com/PaulDance/cargo-liner/pulse">
        <img
            alt="Maintainance status: active"
            src="https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg?style=for-the-badge"
        />
    </a>
    <a href="https://github.com/PaulDance/cargo-liner/blob/master/LICENSE.txt">
        <img
            alt="License"
            src="https://img.shields.io/github/license/PaulDance/cargo-liner?style=for-the-badge"
        />
    </a>
</div>


[car·go·lin·er](https://www.dictionary.com/browse/cargo-liner), _noun_:

> 1. a cargo ship that sails regularly between designated ports according to a
>    published schedule.


## Summary

Cargo Liner is a tool to help one who has packages currently installed or to be
installed through the official `cargo install` command to install and maintain
them up-to-date by editing a small and stable configuration file located at
`$CARGO_HOME/liner.toml`.

Goals:
 * Simple and intuitive API.
 * Stable configuration file: avoid editing it automatically.
 * Actually use `cargo install`, `cargo search` or `cargo config get` and not
   much else.

Non-goals:
 * Super-duper stability guarantees.
 * Re-implementing half of Cargo for small functionalities.
 * Being """pretty""" _above all else_.
 * Handling the synchronization of the configuration file between various hosts.


## Rationale

`cargo install` works very well to download, compile and install a binary
package. However, it does not offer means to update currently installed
programs without having to specify them manually one by one on the CLI. That
becomes quickly bothersome when having to maintain several packages up-to-date,
especially if it needs to be done on multiple workstations.

Some projects, such as [cargo-update] or [cargo-updater], exist in order to
solve this issue. Their strategy is to exploit the `$CARGO_HOME/.crates.toml`
and `$CARGO_HOME/.crates2.json` files that Cargo generates and maintains in
order to track which packages are installed, their exact version, where they
were downloaded from and which programs they have installed. This strategy is
quite effective, so if you are looking for exactly that, then check them out.

However, some problems are still not solved like this: when configuring a new
workstation, there is still the need to specify each package manually at least
once; when adding a new package on one already-configured workstation, there is
still the need to install it manually on all others. These tools lack *sharing*
and *synchronization*.

The current project therefore inspires itself from tools such as [zplug] for
Zsh and [vim-plug] for Vim by taking orders from a central configuration file.
The tool then simply runs `cargo search` for all packages listed in that file
in order to retrieve their latest versions available and then `cargo install`
for those that do indeed need an install or update using the results from the
search. That enables one to install and maintain all packages up-to-date, but
also to keep all of one's workstations synchronized by sharing the file between
them in some way, using Git for example.

[cargo-update]: https://github.com/nabijaczleweli/cargo-update
[cargo-updater]: https://github.com/pombadev/cargo-updater
[zplug]: https://github.com/zplug/zplug
[vim-plug]: https://github.com/junegunn/vim-plug


## Installation

 * Run: `cargo install cargo-liner`.
 * Create the configuration file to be located at: `$CARGO_HOME/liner.toml`.
   * See the reference documentation about [Cargo Home] if you have trouble
     locating the directory.
 * If you are using a different Cargo installation root than `$CARGO_HOME`,
   please make sure it is properly configured in the `$CARGO_INSTALL_ROOT`
   environment variable or the `install.root` key of the
   `$CARGO_HOME/config.toml` file so that the current tool may be able to
   detect that on itself. See the [`cargo install`] documentation for more
   details about this.
 * Populate the file with packages you wish to be installed, for example:

   ```toml
   [packages]
   cargo-expand = "*"
   cargo-tarpaulin = "~0.22"
   nu = "=0.71.0"
   ripgrep = { version = "13.0.0", all-features = true }
   sqlx-cli = {
       version = "0.6.2",
       default-features = false,
       features = ["native-tls", "postgres"],
   }
   ```

   or use `cargo liner import` to do it automatically for you, see below for
   more detailed explanations.

[Cargo Home]: https://doc.rust-lang.org/cargo/guide/cargo-home.html
[`cargo install`]: https://doc.rust-lang.org/cargo/commands/cargo-install.html


## Usage
### CLI

A few commands are available:

```console
$ cargo help liner
Cargo subcommand to install and update binary packages listed in
configuration.

Usage: cargo liner [OPTIONS] [COMMAND]

Commands:
  ship         The default command if omitted: install and update
                   configured packages
  import       Import the `$CARGO_HOME/.crates.toml` Cargo-edited
                   save file as a new Liner configuration file
  completions  Generate an auto-completion script for the given
                   shell
  help         Print this message or the help of the given
                   subcommand(s)

Options:
  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so
          each time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, DEBUG and above messages of
          only this crate are logged and error backtraces are shown
          (`RUST_BACKTRACE=1`). When used twice, DEBUG and above
          messages of all crates are logged, `-v` is given to Cargo
          calls (details ran commands), and error backtraces are
          fully shown (`RUST_BACKTRACE=full`). When used three times
          or more, TRACE and above messages of all crates are logged,
          `-vv` is given to Cargo calls (includes build output) and
          error backtraces are fully shown (`RUST_BACKTRACE=full`).
          This takes precedence over the environment.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each
          time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, WARN and above messages of only
          this crate are logged. When used twice, ERROR messages of
          all crates are logged. When used three times or more, no
          message will be logged, including Cargo's by passing `-q`
          to it and error reports are silenced. This takes precedence
          over the environment.

      --color <WHEN>
          Control the coloring of the logging output.
          
          This enables one to manually specify when should the logs
          and error reports be colored or not, for example if the
          automatic detection is either not wished or not functional.
          The value is also passed onto calls to Cargo.
          
          [default: auto]
          [possible values: auto, always, never]

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version

```


#### Default command

When the subcommand is omitted, it will use the `ship` subcommand with default
options. See its specific documentation for more details.

Simply run `cargo liner` in order to:
 * Read packages from the configuration file.
 * Detect currently-installed packages from Cargo's installation.
 * Check the latest available version for each of them.
 * Install or update the ones that need to, respecting the version requirements.
 * Self-update.

Example output if `bat` and `cargo-expand` are required:

```console
$ cargo liner
 INFO  cargo_liner::cargo > Fetching latest package versions...
...
 INFO  cargo_liner        > Results:
┌──────────────┬─────────────┬─────────────┬────────┐
│ Name         │ Old version │ New version │ Status │
├──────────────┼─────────────┼─────────────┼────────┤
│ bat          │ ø           │ 0.24.0      │ 🛈      │
│ cargo-expand │ 1.0.78      │ 1.0.79      │ 🛈      │
│ cargo-liner  │ 0.0.0       │ ø           │ ✔      │
└──────────────┴─────────────┴─────────────┴────────┘
 INFO  cargo_liner::cargo > Installing `bat`...
    Updating [..] index
 Downloading crates ...
  Downloaded bat v0.24.0 (registry [..])
  Installing bat v0.24.0
    Updating [..] index
...
   Compiling bat v0.24.0
    Finished `release` profile [optimized] target(s) in [..]s
  Installing [..]/.cargo/bin/bat
   Installed package `bat v0.24.0` (executable `bat`)
...
 INFO  cargo_liner::cargo > Updating `cargo-expand`...
    Updating [..] index
 Downloading crates ...
  Downloaded cargo-expand v1.0.79 (registry [..])
  Installing cargo-expand v1.0.79
    Updating [..] index
...
   Compiling cargo-expand v1.0.79
    Finished `release` profile [optimized] target(s) in [..]s
   Replacing [..]/.cargo/bin/cargo-expand
    Replaced package `cargo-expand v1.0.78` with `cargo-expand v1.0.79` (executable `cargo-expand`)
...
 INFO  cargo_liner        > Installation report:
┌──────────────┬─────────────┬─────────────┬────────┐
│ Name         │ Old version │ New version │ Status │
├──────────────┼─────────────┼─────────────┼────────┤
│ bat          │ ø           │ 0.24.0      │ +      │
│ cargo-expand │ 1.0.78      │ 1.0.79      │ ✔      │
└──────────────┴─────────────┴─────────────┴────────┘
 INFO  cargo_liner        > Done.

```

in which the first table displays each configured package's currently-installed
and most recent versions, along with the action that will be performed; the
second table shows an ending report displaying each touched package's previous
and new versions, along with the result of the call to `cargo install`.

Status icons are optionally colored in the output and stand for the following:
 * `ø`: when nothing to display or needs to be done: already up-to-date.
 * `🛈`: when something needs to be performed: installation or update of a
   package.
 * `+`: when something was successfully added: new installation of a package.
 * `✘`: when something failed.
 * `✔`: when things went right: already up-to-date or successful update.


#### `ship` subcommand

The main command: do the installing and updating of packages.

```console
$ cargo liner help ship
The default command if omitted: install and update configured
packages.

Self-updating is enabled by default.

Usage: cargo liner ship [OPTIONS]

Options:
  -n, --no-self
          Disable self-updating.
          
          Cannot be used in conjunction with `--only-self`. Default:
          `false`, i.e. self-update.

  -s, --only-self
          Only self-update and do not install or update any other
          package.
          
          Cannot be used in conjunction with `--no-self`. Default:
          `false`, i.e. install or update other packages as well.

  -c, --skip-check
          Skip the summary version check and directly call `cargo
          install` on each configured package.
          
          The version check is relatively quick and enables skipping
          calls to `cargo install` when no update is required, which
          saves quite a bit of time. However, if you wish, this
          option is still available in order not to run the check:
          doing so will probably take more time in the end most of
          the time, except if you have a very small amount of
          packages configured (e.g. one or two) or if all or almost
          all packages are not already installed.
          
          It can also be used as a workaround in case a certain
          operation fails in your particular environment, for
          example: reading from `.crates.toml` under the
          `$CARGO_HOME` or `$CARGO_INSTALL_ROOT` directory or making
          requests to the registry. These operations will thus be
          entirely skipped.

  -k, --keep-going
          Disable the default fail-fast execution of `cargo
          install`s.
          
          By default, whenever a call to `cargo install` fails for
          any reason, the overall operation is stopped as soon as
          possible. In some cases, such as packages simply failing to
          compile, this is a bit too restrictive as it prevents
          installing the following packages. The option it therefore
          provided in order to make the installation keep on going by
          continuing to call `cargo install` on each configured
          package, even if some previous one failed. However, in case
          any of the packages fails to install and the option is
          used, an error will still be reported at the end,
          containing an indication of all the packages that failed to
          install.

  -f, --force
          Force overwriting existing crates or binaries.
          
          Passes the option flag onto each call of `cargo install`.
          It will, for example, redownload, recompile and reinstall
          every configured package when used in conjunction with
          `--skip-check`.

  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so
          each time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, DEBUG and above messages of
          only this crate are logged and error backtraces are shown
          (`RUST_BACKTRACE=1`). When used twice, DEBUG and above
          messages of all crates are logged, `-v` is given to Cargo
          calls (details ran commands), and error backtraces are
          fully shown (`RUST_BACKTRACE=full`). When used three times
          or more, TRACE and above messages of all crates are logged,
          `-vv` is given to Cargo calls (includes build output) and
          error backtraces are fully shown (`RUST_BACKTRACE=full`).
          This takes precedence over the environment.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each
          time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, WARN and above messages of only
          this crate are logged. When used twice, ERROR messages of
          all crates are logged. When used three times or more, no
          message will be logged, including Cargo's by passing `-q`
          to it and error reports are silenced. This takes precedence
          over the environment.

      --color <WHEN>
          Control the coloring of the logging output.
          
          This enables one to manually specify when should the logs
          and error reports be colored or not, for example if the
          automatic detection is either not wished or not functional.
          The value is also passed onto calls to Cargo.
          
          [default: auto]
          [possible values: auto, always, never]

  -h, --help
          Print help (see a summary with '-h')

```

Simply run `cargo liner ship` in order to:
 * Read packages from the configuration file.
 * Read currently installed packages from the Cargo-managed `.crates.toml` file
   under the `$CARGO_INSTALL_ROOT` directory if `cargo config get` is able to
   retrieve its value from either the environment variable or the `install.root`
   configuration item in `$CARGO_HOME/config.toml`, or fall back to searching
   the file under the default `$CARGO_HOME` directory if the first attempt
   fails for any reason, the simple absence of the setting being one of them.
   See the [`cargo install`] documentation for more details about this.
   Whenever the first attempt fails, it is logged as a `DEBUG` message before
   attempting the default, so use `-vv` to investiguate if your configuration
   seems not to be taken into account.
 * Check the latest available version for each of them using `cargo search`.
 * Run `cargo install` for each that needs an install or update, respecting
   the version requirements.
 * Self-update only if `--no-self` is not given.

[`cargo install`](https://doc.rust-lang.org/cargo/commands/cargo-install.html)


#### `import` subcommand

This command is meant to be used upon installing the tool and using it for the
first time: it populates the configuration file with currently-installed
packages.

```console
$ cargo liner help import
Import the `$CARGO_HOME/.crates.toml` Cargo-edited save file as a new
Liner configuration file.

Star versions are used by default. The version transformation options
are mutually exclusive.

Usage: cargo liner import [OPTIONS]

Options:
  -e, --exact
          Import package versions as "exact versions", i.e. prepended
          with an equal operator.
          
          Cannot be used in conjunction with either `--compatible` or
          `--patch`. Default: `false`, i.e. use a star requirement.

  -c, --compatible
          Import package versions as "compatible versions", i.e.
          prepended with a caret operator.
          
          Cannot be used in conjunction with either `--exact` or
          `--patch`. Default: `false`, i.e. use a star requirement.

  -p, --patch
          Import package versions as "patch versions", i.e. prepended
          with a tilde operator.
          
          Cannot be used in conjunction with either `--exact` or
          `--compatible`. Default: `false`, i.e. use a star
          requirement.

  -f, --force
          Overwrite the current configuration file if it already
          exists.
          
          Default: `false`, i.e. return an error in case the file
          already exists.

  -s, --keep-self
          Also import this `cargo-liner` package into the
          configuration, for example in order to specify a certain
          version requirement later on.
          
          Default: `false`, i.e. exclude the current package from the
          list of packages to install or update in the resulting
          configuration file. Note however that the `ship` command
          will still self-update by default.

  -l, --keep-local
          Also import all locally-installed packages into the
          configuration. This means packages installed via `cargo
          install --path <path>` will be present in the
          configuration.
          
          Default: `false`, i.e. exclude all packages installed via
          `cargo install --path <path>` from the list of packages to
          install or update in the resulting configuration file.

  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so
          each time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, DEBUG and above messages of
          only this crate are logged and error backtraces are shown
          (`RUST_BACKTRACE=1`). When used twice, DEBUG and above
          messages of all crates are logged, `-v` is given to Cargo
          calls (details ran commands), and error backtraces are
          fully shown (`RUST_BACKTRACE=full`). When used three times
          or more, TRACE and above messages of all crates are logged,
          `-vv` is given to Cargo calls (includes build output) and
          error backtraces are fully shown (`RUST_BACKTRACE=full`).
          This takes precedence over the environment.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each
          time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, WARN and above messages of only
          this crate are logged. When used twice, ERROR messages of
          all crates are logged. When used three times or more, no
          message will be logged, including Cargo's by passing `-q`
          to it and error reports are silenced. This takes precedence
          over the environment.

      --color <WHEN>
          Control the coloring of the logging output.
          
          This enables one to manually specify when should the logs
          and error reports be colored or not, for example if the
          automatic detection is either not wished or not functional.
          The value is also passed onto calls to Cargo.
          
          [default: auto]
          [possible values: auto, always, never]

  -h, --help
          Print help (see a summary with '-h')

```

For example, if you had previously installed:
 * `bat@0.22.1`
 * `cargo-make@0.36.3`
 * `cargo-outdated@0.11.1`

Then running `cargo liner import` will result in the following configuration
file, if it does not already exist:

```toml
[packages]
bat = "*"
cargo-make = "*"
cargo-outdated = "*"
```

The command will by default exclude from the destination file:
 * `cargo-liner`: would duplicate the dedicated self-updating; the
   `-s/--keep-self` option flag is there to disable this exclusion.
 * all locally-installed packages, i.e. the ones installed through `cargo
   install --path=...`: avoids polluting the destination file with packages
   that cannot be updated anyway since they have no guarantee of existing in
   the registry used; the `-l/--keep-local` option flag is there to disable
   this exclusion.

The command will by default import the packages with star version requirements.
The `--exact`, `--compatible` and `--patch` options are provided in order to
customize how the currently-installed versions are imported into version
requirements: `--exact` will prepend them with `=`, `--compatible` with `^`,
and `--patch` with `~`.

For example, using the previous three packages already installed, running
`cargo liner import --patch` would give:

```toml
[packages]
bat = "~0.22.1"
cargo-make = "~0.36.3"
cargo-outdated = "~0.11.1"
```

The file can of course be edited manually afterwards, as intended.


#### `completions` subcommand

This enables obtaining CLI auto-completion in a shell for the current project:

```console
$ cargo liner help completions
Generate an auto-completion script for the given shell.

The script is generated for `cargo-liner`, but with arguments rooted
on `cargo-liner liner`, thus making auto-completing work when typing
`cargo liner`. The generated script is emitted to standard output.

Usage: cargo liner completions [OPTIONS] <SHELL>

Arguments:
  <SHELL>
          The shell flavor to use when generating the completions
          
          [possible values: bash, elvish, fish, powershell, zsh]

Options:
  -v, --verbose...
          Be more verbose. Use multiple times to be more and more so
          each time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, DEBUG and above messages of
          only this crate are logged and error backtraces are shown
          (`RUST_BACKTRACE=1`). When used twice, DEBUG and above
          messages of all crates are logged, `-v` is given to Cargo
          calls (details ran commands), and error backtraces are
          fully shown (`RUST_BACKTRACE=full`). When used three times
          or more, TRACE and above messages of all crates are logged,
          `-vv` is given to Cargo calls (includes build output) and
          error backtraces are fully shown (`RUST_BACKTRACE=full`).
          This takes precedence over the environment.

  -q, --quiet...
          Be quieter. Use multiple times to be more and more so each
          time.
          
          When omitted, INFO and above messages of only this crate
          are logged. When used once, WARN and above messages of only
          this crate are logged. When used twice, ERROR messages of
          all crates are logged. When used three times or more, no
          message will be logged, including Cargo's by passing `-q`
          to it and error reports are silenced. This takes precedence
          over the environment.

      --color <WHEN>
          Control the coloring of the logging output.
          
          This enables one to manually specify when should the logs
          and error reports be colored or not, for example if the
          automatic detection is either not wished or not functional.
          The value is also passed onto calls to Cargo.
          
          [default: auto]
          [possible values: auto, always, never]

  -h, --help
          Print help (see a summary with '-h')

```

Its result can either be saved to a file configured to be `source`d (or
equivalent) by your shell, or generated and `eval`uated (or equivalent) at each
startup. For example with Zsh, adding:

```zsh
[[ -f ~/.cargo/bin/cargo-liner ]] && eval "$(cargo liner -qqq completions zsh)"
```

to one's `~/.zshrc` will enable the completions in every new shell, but only if
Cargo Liner is indeed installed (that part may need to be adapted to your
particular environment).


### Configuration

The file must be located at `$CARGO_HOME/liner.toml` and contain a
properly-formed TOML document respecting the following format:

```toml
[packages]
package-name-1 = "version-req-1"
package-name-2 = "version-req-2"
package-name-3 = {
    version = "version-req-3",
    all-features = boolean,
    default-features = boolean,
    features = ["feature-1", "feature-2"],
    extra-arguments = ["--arg1", "--arg2"],
    environment = { ENV1 = "abc", ENV2 = "def" },
}
#...
```

where:
 * `packages` (mandatory): map of package name to package details instructing
   which and how packages should be installed or updated.
 * `version` (mandatory): version requirement string to use when installing or
   updating the associated package; this is the detailed field set when only
   using the simple configuration style.
 * `all-features` (optional): boolean that, when set to `true`, enables the
   `--all-features` flag of `cargo install`.
 * `default-features` (optional): boolean that, when set to `false`, enables
   the `--no-default-features` flag of `cargo install`.
 * `features` (optional): list of strings instructing which of the associated
   crate's Cargo features should be enabled when building it.
 * `index` (optional): string specifying the registry index to install from.
 * `registry` (optional): string specifying the registry to use.
 * `extra-arguments` (optional): list of strings given as additional arguments
   to `cargo install` for the associated package and located between the last
   one given by Cargo Liner and the following `--` seperating options from
   fixed arguments. This can be used in order to successfully manage a package
   using a Cargo Liner version that does not yet implement the desired option.
 * `environment` (optional): map of string to strings specifying which and how
   environment variables should be set for the spawned `cargo install` process.

with the following constraints, mostly enforced by Cargo, but also by TOML:
 * `package-name-*` must be a valid [package name], i.e. match
   `[a-zA-Z][a-zA-Z0-9_-]*` or something like that.
 * `version-req-*` must be a valid [SemVer] requirement, [Cargo style]. In
   particular, the catch-all wildcard `*` can be used to require the latest
   version available.
 * `feature-*` must be the name of a [Cargo feature] defined by the package
   being installed, which has constraints similar to a package name; in
   particular, it shouldn't contain a comma.
 * `--arg*` must be the name of a [`cargo install` CLI argument].
 * `ENV*` should be the name of a [`cargo install` environment variable].
 * `boolean` is a [TOML boolean], either `true` or `false`.

[package name]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-name-field
[SemVer]: https://semver.org/
[Cargo style]: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
[Cargo feature]: https://doc.rust-lang.org/cargo/reference/features.html
[`cargo install` CLI argument]: https://doc.rust-lang.org/cargo/commands/cargo-install.html
[`cargo install` environment variable]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[TOML boolean]: https://toml.io/en/v1.0.0#boolean


## Contributing

See the [contributing guidelines]. Please also take note of the [code of
conduct].

[contributing guidelines]: ./CONTRIBUTING.md
[code of conduct]: ./CODE_OF_CONDUCT.md


## Copyright notice

[Sergej Tucakov] for the animation used as this project's logo.

[Sergej Tucakov]: https://dribbble.com/sergejtucakov

<a id="bottom"></a>
