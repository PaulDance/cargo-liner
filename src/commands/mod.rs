//! Implementations of the various CLI commands.

use tabled::settings::Style;
use tabled::{Table, Tabled};

pub mod completions;
pub mod import;
pub mod jettison;
pub mod ship;

/// Builds a [`Table`] from the given iterator with a default style.
fn styled_table(iter: impl IntoIterator<Item = impl Tabled>) -> Table {
    let mut table = Table::new(iter);
    table.with(Style::sharp());
    table
}
