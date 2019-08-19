//! # cansi
//!
//! [![Build Status](https://travis-ci.com/kurtlawrence/cansi.svg?branch=master)](https://travis-ci.com/kurtlawrence/cansi)
//! [![Latest Version](https://img.shields.io/crates/v/cansi.svg)](https://crates.io/crates/cansi)
//! [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cansi)
//! [![codecov](https://codecov.io/gh/kurtlawrence/cansi/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cansi)
//!
//! ## **C**atergorise **ANSI** - ANSI escape code parser and categoriser
//!
//! See the [rs docs.](https://docs.rs/cansi/)
//! Look at progress and contribute on [github.](https://github.com/kurtlawrence/cansi)
//!
//! `cansi` will parse text with ANSI escape sequences in it and return a deconstructed text with metadata around the colouring and styling. `cansi` is only concerned with `CSI` sequences, particuarly the `SGR` parameters. `cansi` will not construct escaped text, there are crates such as [`colored`](https://crates.io/crates/colored) that do a great job of colouring and styling text.
//!
//! ## Example usage
//!
//! > This example was done using the `colored` crate to help with constructing the escaped text string. It will work with other tools that inject escape sequences into text strings (given they follow [ANSI specification](https://en.wikipedia.org/wiki/ANSI_escape_code)).
//!
//! ```rust
//! extern crate cansi;
//! extern crate colored;
//!
//! use cansi::*;
//! use colored::Colorize;
//! use std::io::Write;
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


#![cfg_attr(not(feature = "std"), no_std)]
#![warn(missing_docs)]

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::string::String;

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
/// use colored::Colorize;
/// use cansi::*;
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
/// use colored::Colorize;
/// use cansi::*;
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
            v.push(prev.clone_style(first));

            if let Some(remainder) = remainder {
                // there is a remainder, which means that a new line was hit
                self.prev = Some(prev.clone_style(remainder));
                return Some(v); // exit early
            }

            self.prev = None; // consumed prev
        }

        while let Some(slice) = self.slices.get(self.idx) {
            self.idx += 1; // increment to next slice, always happens as well split this slice.

            let (first, remainder) = split_on_new_line(slice.text);

            // push first slice on -- only if not empty
            if first.len() > 0 || v.len() == 0 {
                v.push(slice.clone_style(first));
            }

            if let Some(remainder) = remainder {
                // there is a remainder, which means that a new line was hit
                if remainder.len() > 0 {
                    // not just a trailing new line.
                    self.prev = Some(slice.clone_style(remainder));
                }
                break; // exit looping
            }
        }

        if v.len() == 0 && self.idx >= self.slices.len() {
            None // stop iterating if no slices were met and the index is above the slices len
        } else {
            Some(v)
        }
    }
}

/// Splits on the first instance of `\r\n` or `\n` bytes.
/// Returns the first split slice, and the remainder slice if there is a split and items afterwards.
/// Can return an empty remainder slice (if terminated with a new line). Can return empty first slice (say `"\nHello"`);
fn split_on_new_line(txt: &str) -> (&str, Option<&str>) {
    let mut split = txt.splitn(2, '\n'); // split on new line byte

    let first = split.next().expect("should be one I guess?"); // get the first return

    let first = first.trim_matches('\r');

    (first, split.next())
}

/// Data structure that holds information about colouring and styling of a text slice.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CategorisedSlice<'text> {
    /// The text slice.
    pub text: &'text str,
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
    const fn with_sgr(sgr: SGR, txt: &'text str) -> Self {
        let SGR {
            fg_colour,
            bg_colour,
            intensity,
            italic,
            underline,
            blink,
            reversed,
            hidden,
            strikethrough,
        } = sgr;

        Self {
            text: txt,
            fg_colour: fg_colour,
            bg_colour: bg_colour,
            intensity: intensity,
            italic: italic,
            underline: underline,
            blink: blink,
            reversed: reversed,
            hidden: hidden,
            strikethrough: strikethrough,
        }
    }

    const fn clone_style(&self, txt: &'text str) -> Self {
        let mut c = *self;
        c.text = txt;
        c
    }

    #[cfg(test)]
    const fn default_style(txt: &'text str) -> Self {
        Self::with_sgr(SGR::default(), txt)
    }
}

/// The formatting components `SGR (Select Graphic Rendition)`.
/// [spec](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
#[derive(Clone, Copy)]
struct SGR {
    fg_colour: Color,
    bg_colour: Color,
    intensity: Intensity,
    italic: bool,
    underline: bool,
    blink: bool,
    reversed: bool,
    hidden: bool,
    strikethrough: bool,
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

impl SGR {
    const fn default() -> Self {
        SGR {
            fg_colour: Color::White,
            bg_colour: Color::Black,
            intensity: Intensity::Normal,
            italic: false,
            underline: false,
            blink: false,
            reversed: false,
            hidden: false,
            strikethrough: false,
        }
    }
}
