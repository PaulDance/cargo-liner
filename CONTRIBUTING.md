# Contributing

Contribution to this repository is of course welcome, encouraged even. When
contributing to this repository, please first discuss the change you wish to
make via issue, email, or any other method with the owners of this repository
before making a change.

Please note we have a [code of conduct](./CODE_OF_CONDUCT.md), please follow it
in all your interactions with the project.


## Development environment

The current development environment is relatively standard.

 * The toolchain used in CI is stable Rust. The supported targets are x86_64
   GNU/Linux, x86_64 Windows and ARM64 macOS.
 * Integration tests are mostly based on [Trycmd] for the README check and
   [Snapbox] with [`cargo-test-support`] for the rest. In particular, most
   outputs are exported in separate files living in the [`tests/fixtures/`]
   directory. They both have ways to automatically update their respective
   output files: see their docs or the comments included in this project.
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
 2. Add context to errors and don't simply bubble them up with `?`. Do so using
    logging with the [`log`] macros and error wraps with [`color-eyre`]'s traits.
 3. Add as much tests as possible. Unit tests are directly in modules.
    Integration tests are in the `tests` directory. Add feature tests when
    adding new functionalities. Add regression tests when fixing bugs.
 4. Update the `README.md` with details of changes to the interface. This
    includes new environment variables, exposed CLI options, useful file
    locations and configuration options.
 5. After a validating review, a maintainer is charge of merging the PR.

Things that are not necessary:
 * Updating the changelog. The project is still small enough for this to be
   done all at once by a maintainer during each release.
 * Bumping the project's version. Only a single publication branch is maintained
   at a given moment, so this can be decided during a release as well.

Optional pedantic aspects:
 * [Conventional commits] is used most of the time. This usually works well
   with the next point.
 * Micro-committing is generally preferred, as long as it does not impair a
   PR's readability. For example, when adding a functionnality that requires
   touching various aspects, a first commit can be recorded for the addition to
   the CLI, then one for the new CLI tests, then one for the addition to the
   configuration, then one for the new config tests, then one for refactors
   necessary in order to accommodate for the functionality, then one for the
   functionality's core, then one for validation tests, etc...
 * During PR iterations, it is preferred to keep the commits still useful for
   post-merge readings by modifying the commits directly concerned by a review
   comment using a rebase, rather than adding new commits on top. However, if
   making the changes in already-existing commits would not make much sense,
   then adding new commits should of course be better. If a rebase is necessary
   to update a branch when applying some recommendations, then in order to make
   the diffs readable, it is interesting to push twice: once for the first
   "pure" rebase that does not change anything beyond solving conflicts, and
   the second for the desired changes. Although the project only uses free CI
   for now, the first push's workflow run can be cancelled directly.

[Conventional commits]: https://www.conventionalcommits.org/en/v1.0.0/
[`log`]: https://docs.rs/log/latest/log/
[`color-eyre`]: https://docs.rs/color-eyre/latest/color_eyre/


## Issue Reports

 * Be sure to make the best use of predefined issue templates.
 * `--verbose` is definitely on. We consider better to add as much details you
   judge relevant to the issue, even if they reveal to be useless after all.
   More is just more.
 * Respect a contributor's decision: if it's a wontfix, then it's a wontfix.
