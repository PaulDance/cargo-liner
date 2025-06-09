//! Infrastructure to handle output colors.

use std::fmt::Display;
use std::io::IsTerminal;

use clap::ColorChoice;
use color_eyre::owo_colors::OwoColorize;

mod icons;

/// Assembles both an output stream's color capacity and a color preference in
/// order to conditionally emit colorized content.
pub struct Colorizer {
    is_terminal: bool,
    color_choice: ColorChoice,
}

impl Colorizer {
    /// Builds a new colorizer.
    ///
    ///  * `out`: the descriptor that will be used in order to write the output
    ///    of [`Self::colorize_with`].
    ///  * `color_choice`: color preference to apply.
    pub fn new(out: &impl IsTerminal, color_choice: ColorChoice) -> Self {
        Self {
            is_terminal: out.is_terminal(),
            color_choice,
        }
    }

    /// Returns `input` or `color_fn(input)` depending on the current color
    /// preference and whether a terminal is used.
    fn colorize_with<'i, 'o, I, F, O>(&self, input: &'i I, color_fn: F) -> Box<dyn Display + 'o>
    where
        'i: 'o,
        I: Display + ?Sized,
        O: Display + 'o,
        F: Fn(&'i I) -> O,
    {
        match self.color_choice {
            ColorChoice::Never => Box::new(input),
            ColorChoice::Always => Box::new(color_fn(input)),
            ColorChoice::Auto => {
                if self.is_terminal {
                    Box::new(color_fn(input))
                } else {
                    Box::new(input)
                }
            }
        }
    }

    /// Returns the colorized version of [`icons::NONE`].
    #[expect(
        clippy::unused_self,
        reason = "So refactors may be easier by keeping an API identical to the other icons."
    )]
    pub fn none_icon(&self) -> impl Display {
        icons::NONE
    }

    /// Returns the colorized version of [`icons::UNKNOWN`].
    pub fn unknown_icon(&self) -> impl Display {
        self.colorize_with(&icons::UNKNOWN, |icon| icon.bold().yellow().to_string())
    }

    /// Returns the colorized version of [`icons::TODO`].
    pub fn todo_icon(&self) -> impl Display {
        self.colorize_with(&icons::TODO, |icon| icon.bold().blue().to_string())
    }

    /// Returns the colorized version of [`icons::NEW`].
    pub fn new_icon(&self) -> impl Display {
        self.colorize_with(&icons::NEW, |icon| icon.bold().green().to_string())
    }

    /// Returns the colorized version of [`icons::ERR`].
    pub fn err_icon(&self) -> impl Display {
        self.colorize_with(&icons::ERR, char::red)
    }

    /// Returns the colorized version of [`icons::OK`].
    pub fn ok_icon(&self) -> impl Display {
        self.colorize_with(&icons::OK, char::green)
    }
}
