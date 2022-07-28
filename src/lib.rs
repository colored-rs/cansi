//! [![Build Status](https://travis-ci.com/kurtlawrence/cansi.svg?branch=master)](https://travis-ci.com/kurtlawrence/cansi)
//! [![Latest Version](https://img.shields.io/crates/v/cansi.svg)](https://crates.io/crates/cansi)
//! [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cansi)
//! [![codecov](https://codecov.io/gh/kurtlawrence/cansi/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cansi)
//!
//! # **C**atergorise **ANSI** - ANSI escape code parser and categoriser
//!
//! See the [rs docs.](https://docs.rs/cansi/)
//! Look at progress and contribute on [github.](https://github.com/kurtlawrence/cansi)
//!
//! `cansi` will parse text with ANSI escape sequences in it and return a deconstructed
//! text with metadata around the colouring and styling. `cansi` is only concerned
//! with `CSI` sequences, particuarly the `SGR` parameters. `cansi` will not construct
//! escaped text, there are crates such as [`colored`](https://crates.io/crates/colored)
//! that do a great job of colouring and styling text.
//!
//! # Example usage
//!
//! > This example was done using the `colored` crate to help with constructing the escaped
//! > text string. It will work with other tools that inject escape sequences into text strings (given they follow [ANSI specification](https://en.wikipedia.org/wiki/ANSI_escape_code)).
//!
//! ```rust
//! # use cansi::*;
//! # use colored::Colorize;
//! # use std::io::Write;
//! # colored::control::set_override(true);
//!
//! let v = &mut Vec::new();
//! write!(
//!   v,
//!   "Hello, {}{}{}{}{}{}",
//!   "w".white().on_red(),
//!   "o".cyan().on_green(),
//!   "r".magenta().on_yellow(),
//!   "l".blue().on_white(),
//!   "d".yellow().on_bright_cyan(),
//!   "!".bright_red().on_bright_yellow(),
//! )
//! .unwrap();
//!
//! let text = String::from_utf8_lossy(&v);
//! let result = categorise_text(&text); // cansi function
//!
//! assert_eq!(result.len(), 7); // there should be seven differently styled components
//!
//! assert_eq!("Hello, world!", &construct_text_no_codes(&result));
//!
//! // 'Hello, ' is just defaults
//! assert_eq!(
//!   result[0],
//!   CategorisedSlice {
//!     text: "Hello, ",
//!     start: 0,
//!     end: 7,
//!     fg_colour: Color::White,
//!     bg_colour: Color::Black,
//!     intensity: Intensity::Normal,
//!     italic: false,
//!     underline: false,
//!     blink: false,
//!     reversed: false,
//!     hidden: false,
//!     strikethrough: false
//!   }
//! );
//!
//! // 'w' is coloured differently
//! assert_eq!(
//!   result[1],
//!   CategorisedSlice {
//!     text: "w",
//!     start: 15,
//!     end: 16,
//!     fg_colour: Color::White,
//!     bg_colour: Color::Red,
//!     intensity: Intensity::Normal,
//!     italic: false,
//!     underline: false,
//!     blink: false,
//!     reversed: false,
//!     hidden: false,
//!     strikethrough: false
//!   }
//! );
//! ```
//!
//! ## Targeting no_std
//! This crate can use `alloc` in place of the standard library for no_std targets.
//! The standard library is enabled by default, so disabling default features and enabling the
//! `alloc` feature is required to use the crate this way.
//!
//! ```toml
//! [dependencies]
//! cansi = { version = "2.1.0", default-features = false, features = ["alloc"] }
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::string::String;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

mod categorise;
mod parsing;

#[cfg(test)]
mod tests;

pub use categorise::categorise_text;
pub use parsing::{parse, Match};

/// Type definition of the collection of `CategorisedSlice`s.
pub type CategorisedSlices<'text> = Vec<CategorisedSlice<'text>>;

/// Constructs a string of the categorised text without the ANSI escape characters.
///
/// # Example
/// ```rust
/// use cansi::*;
/// let categorised = categorise_text("\x1b[30mH\x1b[31me\x1b[32ml\x1b[33ml\x1b[34mo");
/// assert_eq!("Hello", &construct_text_no_codes(&categorised));
/// ```
pub fn construct_text_no_codes(categorised_slices: &CategorisedSlices) -> String {
    let slices = categorised_slices;
    let mut s = String::with_capacity(
        categorised_slices
            .iter()
            .map(|x| x.text.len())
            .sum::<usize>(),
    );
    for sl in slices {
        s.push_str(sl.text);
    }

    s
}

/// Construct an iterator over each new line (`\n` or `\r\n`) and returns the categorised slices within those.
/// `CategorisedSlice`s that include a new line are split with the same style.
///
/// # Example
/// ```rust
/// # use colored::Colorize;
/// # use cansi::*;
/// # colored::control::set_override(true);
///
/// let s = format!("{}{}\nhow are you\r\ntoday", "hello, ".green(), "world".red());
/// let cat = categorise_text(&s);
/// let mut iter = line_iter(&cat);
///
/// let first = iter.next().unwrap();
/// assert_eq!(first[0].text, "hello, ");
/// assert_eq!(first[0].fg_colour, Color::Green);
///
/// assert_eq!(first[1].text, "world");
/// assert_eq!(first[1].fg_colour, Color::Red);
///
/// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "how are you");
/// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "today");
/// assert_eq!(iter.next(), None);
/// ```
pub fn line_iter<'text, 'iter>(
    categorised_slices: &'iter CategorisedSlices<'text>,
) -> CategorisedLineIterator<'text, 'iter> {
    CategorisedLineIterator {
        slices: categorised_slices,
        idx: 0,
        prev: None,
    }
}

/// An iterator structure for `CategorisedSlices`, iterating over each new line (`\n` or `\r\n`) and returns the categorised slices within those.
/// `CategorisedSlice`s that include a new line are split with the same style.
///
/// # Example
/// ```rust
/// # use colored::Colorize;
/// # colored::control::set_override(true);
/// # use cansi::*;
///
/// let s = format!("{}{}\nhow are you\r\ntoday", "hello, ".green(), "world".red());
/// let cat = categorise_text(&s);
/// let mut iter = line_iter(&cat);
///
/// let first = iter.next().unwrap();
/// assert_eq!(first[0].text, "hello, ");
/// assert_eq!(first[0].fg_colour, Color::Green);
///
/// assert_eq!(first[1].text, "world");
/// assert_eq!(first[1].fg_colour, Color::Red);
///
/// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "how are you");
/// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "today");
/// assert_eq!(iter.next(), None);
/// ```
pub struct CategorisedLineIterator<'text, 'iter> {
    slices: &'iter CategorisedSlices<'text>,
    idx: usize,
    prev: Option<CategorisedSlice<'text>>,
}

/// The item type of `CategorisedLineIterator`.
///
/// # Note
/// > The type alias is the same as `CategorisedSlices`, so functions such as `construct_text_no_codes` will work.
pub type CategorisedLine<'text> = Vec<CategorisedSlice<'text>>;

impl<'text, 'iter> Iterator for CategorisedLineIterator<'text, 'iter> {
    type Item = CategorisedLine<'text>;
    fn next(&mut self) -> Option<Self::Item> {
        let mut v = Vec::new();

        if let Some(prev) = &self.prev {
            // need to test splitting this, might be more new lines in remainder
            let (first, remainder) = split_on_new_line(prev.text);

            // push first slice on -- only if not empty
            // if first.len() == 0 it is because there is a sequence of new lines
            v.push(prev.clone_style(&prev.text[..first], prev.start, prev.start + first));

            if let Some(remainder) = remainder {
                // there is a remainder, which means that a new line was hit
                self.prev = Some(prev.clone_style(
                    &prev.text[remainder..],
                    prev.start + remainder,
                    prev.end,
                ));
                return Some(v); // exit early
            }

            self.prev = None; // consumed prev
        }

        while let Some(slice) = self.slices.get(self.idx) {
            self.idx += 1; // increment to next slice, always happens as well split this slice.

            let (first, remainder) = split_on_new_line(slice.text);

            // push first slice on -- only if not empty
            if first > 0 || v.len() == 0 {
                v.push(slice.clone_style(&slice.text[..first], slice.start, slice.start + first));
            }

            if let Some(remainder) = remainder {
                // there is a remainder, which means that a new line was hit
                if !slice.text[remainder..].is_empty() {
                    // not just a trailing new line.
                    self.prev = Some(slice.clone_style(
                        &slice.text[remainder..],
                        slice.start + remainder,
                        slice.end,
                    ));
                }
                break; // exit looping
            }
        }

        if v.is_empty() && self.idx >= self.slices.len() {
            None // stop iterating if no slices were met and the index is above the slices len
        } else {
            Some(v)
        }
    }
}

/// Splits on the first instance of `\r\n` or `\n` bytes.
/// Returns the _exclusive_ end of the first componenet, and the _inclusive_ start of the remaining items if there is a split.
/// Can return an empty remainder slice (if terminated with a new line). Can return empty first slice (say `"\nHello"`);
fn split_on_new_line(txt: &str) -> (usize, Option<usize>) {
    let cr = txt.find('\r');
    let nl = txt.find('\n');

    match (cr, nl) {
        (None, None) => (txt.len(), None),
        (Some(_), None) => (txt.len(), None), // special case, no new line but cr
        (None, Some(nl)) => (nl, Some(nl + 1)),
        (Some(cr), Some(nl)) => {
            if nl.saturating_sub(1) == cr {
                (cr, Some(nl + 1))
            } else {
                (nl, Some(nl + 1))
            }
        }
    }
}

/// Data structure that holds information about colouring and styling of a text slice.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CategorisedSlice<'text> {
    /// The text slice.
    pub text: &'text str,
    /// _Inclusive_ starting byte position.
    pub start: usize,
    /// _Exclusive_ ending byte position.
    pub end: usize,

    /// The foreground (or text) colour.
    pub fg_colour: Color,
    /// The background colour.
    pub bg_colour: Color,

    /// The emphasis state (bold, faint, normal).
    pub intensity: Intensity,

    /// Italicised.
    pub italic: bool,
    /// Underlined.
    pub underline: bool,

    /// Slow blink text.
    pub blink: bool,
    /// Inverted colours. See [https://en.wikipedia.org/wiki/Reverse_video](https://en.wikipedia.org/wiki/Reverse_video).
    pub reversed: bool,
    /// Invisible text.
    pub hidden: bool,
    /// Struck-through.
    pub strikethrough: bool,
}

impl<'text> CategorisedSlice<'text> {
    const fn clone_style(&self, text: &'text str, start: usize, end: usize) -> Self {
        let mut c = *self;
        c.text = text;
        c.start = start;
        c.end = end;
        c
    }

    #[cfg(test)]
    fn default_style(text: &'text str, start: usize, end: usize) -> Self {
        v3::CategorisedSlice::with_sgr(SGR::default(), text, start, end).into()
    }
}

/// Populates with defaults.
impl<'a> From<v3::CategorisedSlice<'a>> for CategorisedSlice<'a> {
    fn from(x: v3::CategorisedSlice<'a>) -> Self {
        let v3::CategorisedSlice {
            text,
            start,
            end,
            fg,
            bg,
            intensity,
            italic,
            underline,
            blink,
            reversed,
            hidden,
            strikethrough,
        } = x;

        Self {
            text,
            start,
            end,
            fg_colour: fg.unwrap_or(Color::White),
            bg_colour: bg.unwrap_or(Color::Black),
            intensity: intensity.unwrap_or(Intensity::Normal),
            italic: italic.unwrap_or_default(),
            underline: underline.unwrap_or_default(),
            blink: blink.unwrap_or_default(),
            reversed: reversed.unwrap_or_default(),
            hidden: hidden.unwrap_or_default(),
            strikethrough: strikethrough.unwrap_or_default(),
        }
    }
}

/// The formatting components `SGR (Select Graphic Rendition)`.
/// [spec](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
#[derive(Clone, Copy, Default)]
struct SGR {
    fg: Option<Color>,
    bg: Option<Color>,
    intensity: Option<Intensity>,
    italic: Option<bool>,
    underline: Option<bool>,
    blink: Option<bool>,
    reversed: Option<bool>,
    hidden: Option<bool>,
    strikethrough: Option<bool>,
}

/// The emphasis (bold, faint) states.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Intensity {
    /// Normal intensity (no emphasis).
    Normal,
    /// Bold.
    Bold,
    /// Faint.
    Faint,
}

/// The 8 standard colors.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

/// Update API for version 3.0 of the crate.
///
/// To start using v3, import the items with `cansi::v3::*`. This way, moving to version 3.0 will
/// only require a change in import code.
/// Note that version 3.0 will remove the deprecated version 2.0 items.
pub mod v3 {
    use super::split_on_new_line;
    use crate::{Color, Intensity, SGR};

    pub use super::categorise::categorise_text_v3 as categorise_text;

    /// Data structure that holds information about colouring and styling of a text slice.
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct CategorisedSlice<'text> {
        /// The text slice.
        pub text: &'text str,
        /// _Inclusive_ starting byte position.
        pub start: usize,
        /// _Exclusive_ ending byte position.
        pub end: usize,

        /// The foreground (or text) colour.
        pub fg: Option<Color>,
        /// The background colour.
        pub bg: Option<Color>,

        /// The emphasis state (bold, faint, normal).
        pub intensity: Option<Intensity>,

        /// Italicised.
        pub italic: Option<bool>,
        /// Underlined.
        pub underline: Option<bool>,

        /// Slow blink text.
        pub blink: Option<bool>,
        /// Inverted colours. See [https://en.wikipedia.org/wiki/Reverse_video](https://en.wikipedia.org/wiki/Reverse_video).
        pub reversed: Option<bool>,
        /// Invisible text.
        pub hidden: Option<bool>,
        /// Struck-through.
        pub strikethrough: Option<bool>,
    }

    impl<'text> CategorisedSlice<'text> {
        pub(crate) const fn with_sgr(sgr: SGR, text: &'text str, start: usize, end: usize) -> Self {
            let SGR {
                fg,
                bg,
                intensity,
                italic,
                underline,
                blink,
                reversed,
                hidden,
                strikethrough,
            } = sgr;

            Self {
                text,
                start,
                end,
                fg,
                bg,
                intensity,
                italic,
                underline,
                blink,
                reversed,
                hidden,
                strikethrough,
            }
        }

        const fn clone_style(&self, text: &'text str, start: usize, end: usize) -> Self {
            let mut c = *self;
            c.text = text;
            c.start = start;
            c.end = end;
            c
        }

        #[cfg(test)]
        fn default_style(text: &'text str, start: usize, end: usize) -> Self {
            Self::with_sgr(SGR::default(), text, start, end)
        }
    }

    /// Type definition of the collection of `CategorisedSlice`s.
    pub type CategorisedSlices<'text> = Vec<CategorisedSlice<'text>>;

    /// The item type of `CategorisedLineIterator`.
    ///
    /// # Note
    /// > The type alias is the same as `CategorisedSlices`, so functions such as `construct_text_no_codes` will work.
    pub type CategorisedLine<'text> = Vec<CategorisedSlice<'text>>;

    /// Construct an iterator over each new line (`\n` or `\r\n`) and returns the categorised slices within those.
    /// `CategorisedSlice`s that include a new line are split with the same style.
    ///
    /// # Example
    /// ```rust
    /// # use colored::Colorize;
    /// # use cansi::*;
    /// # colored::control::set_override(true);
    ///
    /// let s = format!("{}{}\nhow are you\r\ntoday", "hello, ".green(), "world".red());
    /// let cat = categorise_text(&s);
    /// let mut iter = line_iter(&cat);
    ///
    /// let first = iter.next().unwrap();
    /// assert_eq!(first[0].text, "hello, ");
    /// assert_eq!(first[0].fg_colour, Color::Green);
    ///
    /// assert_eq!(first[1].text, "world");
    /// assert_eq!(first[1].fg_colour, Color::Red);
    ///
    /// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "how are you");
    /// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "today");
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn line_iter<'text, 'iter>(
        categorised_slices: &'iter CategorisedSlices<'text>,
    ) -> CategorisedLineIterator<'text, 'iter> {
        CategorisedLineIterator {
            slices: categorised_slices,
            idx: 0,
            prev: None,
        }
    }

    /// An iterator structure for `CategorisedSlices`, iterating over each new line (`\n` or `\r\n`) and returns the categorised slices within those.
    /// `CategorisedSlice`s that include a new line are split with the same style.
    ///
    /// # Example
    /// ```rust
    /// # use colored::Colorize;
    /// # colored::control::set_override(true);
    /// # use cansi::*;
    ///
    /// let s = format!("{}{}\nhow are you\r\ntoday", "hello, ".green(), "world".red());
    /// let cat = categorise_text(&s);
    /// let mut iter = line_iter(&cat);
    ///
    /// let first = iter.next().unwrap();
    /// assert_eq!(first[0].text, "hello, ");
    /// assert_eq!(first[0].fg_colour, Color::Green);
    ///
    /// assert_eq!(first[1].text, "world");
    /// assert_eq!(first[1].fg_colour, Color::Red);
    ///
    /// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "how are you");
    /// assert_eq!(&construct_text_no_codes(&iter.next().unwrap()), "today");
    /// assert_eq!(iter.next(), None);
    /// ```
    pub struct CategorisedLineIterator<'text, 'iter> {
        slices: &'iter CategorisedSlices<'text>,
        idx: usize,
        prev: Option<CategorisedSlice<'text>>,
    }

    impl<'text, 'iter> Iterator for CategorisedLineIterator<'text, 'iter> {
        type Item = CategorisedLine<'text>;
        fn next(&mut self) -> Option<Self::Item> {
            let mut v = Vec::new();

            if let Some(prev) = &self.prev {
                // need to test splitting this, might be more new lines in remainder
                let (first, remainder) = split_on_new_line(prev.text);

                // push first slice on -- only if not empty
                // if first.len() == 0 it is because there is a sequence of new lines
                v.push(prev.clone_style(&prev.text[..first], prev.start, prev.start + first));

                if let Some(remainder) = remainder {
                    // there is a remainder, which means that a new line was hit
                    self.prev = Some(prev.clone_style(
                        &prev.text[remainder..],
                        prev.start + remainder,
                        prev.end,
                    ));
                    return Some(v); // exit early
                }

                self.prev = None; // consumed prev
            }

            while let Some(slice) = self.slices.get(self.idx) {
                self.idx += 1; // increment to next slice, always happens as well split this slice.

                let (first, remainder) = split_on_new_line(slice.text);

                // push first slice on -- only if not empty
                if first > 0 || v.len() == 0 {
                    v.push(slice.clone_style(
                        &slice.text[..first],
                        slice.start,
                        slice.start + first,
                    ));
                }

                if let Some(remainder) = remainder {
                    // there is a remainder, which means that a new line was hit
                    if !slice.text[remainder..].is_empty() {
                        // not just a trailing new line.
                        self.prev = Some(slice.clone_style(
                            &slice.text[remainder..],
                            slice.start + remainder,
                            slice.end,
                        ));
                    }
                    break; // exit looping
                }
            }

            if v.is_empty() && self.idx >= self.slices.len() {
                None // stop iterating if no slices were met and the index is above the slices len
            } else {
                Some(v)
            }
        }
    }

    /// Constructs a string of the categorised text without the ANSI escape characters.
    ///
    /// # Example
    /// ```rust
    /// use cansi::*;
    /// let categorised = categorise_text("\x1b[30mH\x1b[31me\x1b[32ml\x1b[33ml\x1b[34mo");
    /// assert_eq!("Hello", &construct_text_no_codes(&categorised));
    /// ```
    pub fn construct_text_no_codes(categorised_slices: &CategorisedSlices) -> String {
        let slices = categorised_slices;
        let mut s = String::with_capacity(
            categorised_slices
                .iter()
                .map(|x| x.text.len())
                .sum::<usize>(),
        );
        for sl in slices {
            s.push_str(sl.text);
        }

        s
    }
}
