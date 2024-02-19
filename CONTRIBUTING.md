# Contributing

Contribution to this repository is of course welcome, encouraged even. When
contributing to this repository, please first discuss the change you wish to
make via issue, email, or any other method with the owners of this repository
before making a change.

Please note we have a [code of conduct](./CODE_OF_CONDUCT.md), please follow it
in all your interactions with the project.


## Development environment

The current development environment is relatively standard.

* The used toolchain and target is stable Rust for x86_64 GNU/Linux, as checked
  in CI at least.
* Integration tests are mostly based on [Trycmd] for the README check and
  [Snapbox] with [`cargo-test-support`] for the rest. In particular, most
  outputs are exported in separate files living in the [`tests/fixtures/`]
  directory. They both have ways to automatically update their respective
  output files: see their docs or the comments included in this project.
* The `cargo-test-*` dev-only Git dependencies currently make Cargo emit a
  *lot* of false-positive warnings that cannot be disabled in any way. This
  does not affect the build itself, however. A workaround is underway in Cargo.
* The Rustfmt configuration used in CI uses unstable features, so a nightly
  toolchain will be required specifically for this.
* A `Justfile` is provided in order to make a bunch of useful shortcuts
  available. It is to be used with [Just].

[Trycmd]: https://crates.io/crates/trycmd
[Snapbox]: https://crates.io/crates/snapbox
[`cargo-test-support`]: https://github.com/rust-lang/cargo/tree/master/crates/cargo-test-support
[Just]: https://github.com/casey/just


## Pull Request Process

1. Please GPG-sign your Git commits.
2. Add as much tests as possible. Unit tests are directly in modules.
   Integration tests are in the `tests` directory. Add feature tests when
   adding new functionalities. Add regression tests when fixing bugs.
3. Update the `README.md` with details of changes to the interface, this
   includes new environment variables, exposed CLI options, useful file
   locations and configuration options.
4. Increase the version numbers in any examples files and the `README.md` to
   the new version that this Pull Request would represent. The versioning
   scheme we use is [SemVer](http://semver.org/).
5. Ensure any install or build dependencies are removed before the end of the
   layer when doing a Docker build, if applicable.
6. You may merge the Pull Request in once you have the sign-off at least one
   other developer, or if you do not have permission to do that, you may
   request the second reviewer to merge it for you.


## Issue Reports

* Be sure to make the best use of predefined issue templates.
* `--verbose` is definitely on. We consider better to add as much details you
  judge relevant to the issue, even if they reveal to be useless after all.
  More is just more.
* Respect a contributor's decision: if it's a wontfix, then it's a wontfix.
