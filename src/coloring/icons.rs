//! Collection of adequate characters to be used as display icons.

/// When nothing to display or needs to be done: already up-to-date.
pub(super) const NONE: char = 'ø';
/// When the element could not be determined, for example the new version
/// of a package if `skip-check` is used.
pub(super) const UNKNOWN: char = '?';
/// When something needs to be performed: installation or update of a
/// package.
pub(super) const TODO: char = '🛈';
/// When something was successfully added: new installation of a package.
pub(super) const NEW: char = '+';
/// When something failed.
pub(super) const ERR: char = '✘';
/// When things went right: already up-to-date or successful update.
pub(super) const OK: char = '✔';
