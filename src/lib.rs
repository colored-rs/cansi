//! # cansi
//! 
//! [![Build Status](https://travis-ci.com/kurtlawrence/cansi.svg?branch=master)](https://travis-ci.com/kurtlawrence/cansi) [![Latest Version](https://img.shields.io/crates/v/cansi.svg)](https://crates.io/crates/cansi) [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cansi) [![codecov](https://codecov.io/gh/kurtlawrence/cansi/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cansi)
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
//! use colored::*;
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
//!     text_as_bytes: b"Hello, ",
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
//!     text_as_bytes: b"w",
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
extern crate colored;
extern crate parse_ansi;

#[cfg(test)]
mod tests;

/// Re-export of [colored::Color](https://docs.rs/colored/1.6.1/colored/enum.Color.html).
pub use self::colored::Color;

const SEPARATOR: u8 = b';';

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
	String::from_utf8_lossy(
		&categorised_slices
			.iter()
			.flat_map(|r| r.text_as_bytes)
			.map(|x| *x)
			.collect::<Vec<_>>()[..],
	)
	.into_owned()
}

/// Parses the text and returns each formatted slice in order.
/// The ANSI escape codes are not included in the text slices.
///
/// Each different text slice is returned in order such that the text without the escape characters can be reconstructed.
/// There is a helper function (`construct_text_no_codes`) on `CategorisedSlices` for this.
pub fn categorise_text(text: &str) -> CategorisedSlices {
	let mut sgr = SGR::default();
	let mut lo = 0;
	let mut slices: Vec<CategorisedSlice> = Vec::new();
	let text = text.as_bytes();
	for m in parse_ansi::parse_bytes(text) {
		// add in the text before CSI with the previous SGR format
		let hi = m.start();
		if hi != lo {
			slices.push(CategorisedSlice {
				text_as_bytes: &text[lo..hi],
				fg_colour: sgr.fg_colour,
				bg_colour: sgr.bg_colour,
				intensity: sgr.intensity.clone(),
				italic: sgr.italic,
				underline: sgr.underline,
				blink: sgr.blink,
				reversed: sgr.reversed,
				hidden: sgr.hidden,
				strikethrough: sgr.strikethrough,
			});
		}

		lo = m.end();
		let mut escape_seq = m.as_bytes().iter().skip(2); // skip the first two (would be ESC *)
		sgr = SGR::default();
		let mut seq = Vec::new();
		// spec at https://en.wikipedia.org/wiki/ANSI_escape_code#Escape_sequences
		while let Some(byte) = escape_seq.next() {
			if byte == &SEPARATOR || (byte >= &b'\x40' && byte <= &b'\x7e') {
				// signals the end of a sequence, need to process what was transferred
				// if seq is empty, this is treated as a default flag
				if seq.len() == 0 {
					sgr = SGR::default();
				} else {
					// this map is a bit weird but i didn't want to have to convert to characters just
					// to make this mapping more simple. so the seq is in bytes and has to map back to utf8
					// 0-9 characters (effectively 48-57 in decimal notation)
					match &seq[..] {
						&[48] => sgr = SGR::default(),                         // 0
						&[49] => sgr.intensity = Intensity::Bold,              // 1
						&[50] => sgr.intensity = Intensity::Faint,             // 2
						&[51] => sgr.italic = true,                            // 3
						&[52] => sgr.underline = true,                         // 4
						&[53] => sgr.blink = true,                             // 5
						&[55] => sgr.reversed = true,                          // 7
						&[56] => sgr.hidden = true,                            // 8
						&[57] => sgr.strikethrough = true,                     // 9
						&[50, 50] => sgr.intensity = Intensity::Normal,        // 22
						&[50, 51] => sgr.italic = false,                       // 23
						&[50, 52] => sgr.underline = false,                    // 24
						&[50, 53] => sgr.blink = false,                        // 25
						&[50, 55] => sgr.reversed = false,                     // 27
						&[50, 56] => sgr.hidden = false,                       // 28
						&[50, 57] => sgr.strikethrough = false,                // 29
						&[51, 48] => sgr.fg_colour = Color::Black,             // 30
						&[51, 49] => sgr.fg_colour = Color::Red,               // 31
						&[51, 50] => sgr.fg_colour = Color::Green,             // 32
						&[51, 51] => sgr.fg_colour = Color::Yellow,            // 33
						&[51, 52] => sgr.fg_colour = Color::Blue,              // 34
						&[51, 53] => sgr.fg_colour = Color::Magenta,           // 35
						&[51, 54] => sgr.fg_colour = Color::Cyan,              // 36
						&[51, 55] => sgr.fg_colour = Color::White,             // 37
						&[52, 48] => sgr.bg_colour = Color::Black,             // 40
						&[52, 49] => sgr.bg_colour = Color::Red,               // 41
						&[52, 50] => sgr.bg_colour = Color::Green,             // 42
						&[52, 51] => sgr.bg_colour = Color::Yellow,            // 43
						&[52, 52] => sgr.bg_colour = Color::Blue,              // 44
						&[52, 53] => sgr.bg_colour = Color::Magenta,           // 45
						&[52, 54] => sgr.bg_colour = Color::Cyan,              // 46
						&[52, 55] => sgr.bg_colour = Color::White,             // 47
						&[57, 48] => sgr.fg_colour = Color::BrightBlack,       // 90
						&[57, 49] => sgr.fg_colour = Color::BrightRed,         // 91
						&[57, 50] => sgr.fg_colour = Color::BrightGreen,       // 92
						&[57, 51] => sgr.fg_colour = Color::BrightYellow,      // 93
						&[57, 52] => sgr.fg_colour = Color::BrightBlue,        // 94
						&[57, 53] => sgr.fg_colour = Color::BrightMagenta,     // 95
						&[57, 54] => sgr.fg_colour = Color::BrightCyan,        // 96
						&[57, 55] => sgr.fg_colour = Color::BrightWhite,       // 97
						&[49, 48, 48] => sgr.bg_colour = Color::BrightBlack,   // 100
						&[49, 48, 49] => sgr.bg_colour = Color::BrightRed,     // 101
						&[49, 48, 50] => sgr.bg_colour = Color::BrightGreen,   // 102
						&[49, 48, 51] => sgr.bg_colour = Color::BrightYellow,  // 103
						&[49, 48, 52] => sgr.bg_colour = Color::BrightBlue,    // 104
						&[49, 48, 53] => sgr.bg_colour = Color::BrightMagenta, // 105
						&[49, 48, 54] => sgr.bg_colour = Color::BrightCyan,    // 106
						&[49, 48, 55] => sgr.bg_colour = Color::BrightWhite,   // 107
						_ => (),
					}
				}
				seq.clear();
			} else {
				seq.push(*byte); // not a signal to process so just push onto seq
			}
		}
	}

	if lo != text.len() {
		slices.push(CategorisedSlice {
			text_as_bytes: &text[lo..text.len()],
			fg_colour: sgr.fg_colour,
			bg_colour: sgr.bg_colour,
			intensity: sgr.intensity.clone(),
			italic: sgr.italic,
			underline: sgr.underline,
			blink: sgr.blink,
			reversed: sgr.reversed,
			hidden: sgr.hidden,
			strikethrough: sgr.strikethrough,
		});
	}

	slices
}

/// Data structure that holds information about colouring and styling of a text slice.
#[derive(Debug, PartialEq)]
pub struct CategorisedSlice<'text> {
	/// The text slice as a byte array.
	///
	/// # Note
	/// Once the crate [`parse-ansi`](https://crates.io/crates/parse-ansi) moves to [`regex`](https://crates.io/crates/regex) crate `1.1.0` it will be possible to return a string slice (`&str`).
	pub text_as_bytes: &'text [u8],
	/// The foreground (or text) colour.
	pub fg_colour: Color,
	/// The background colour.
	pub bg_colour: Color,
	/// The emphasis state (bold, faint, normal).
	pub intensity: Intensity,
	pub italic: bool,
	pub underline: bool,
	/// Slow blink text.
	pub blink: bool,
	/// Inverted colours. See [https://en.wikipedia.org/wiki/Reverse_video](https://en.wikipedia.org/wiki/Reverse_video).
	pub reversed: bool,
	/// Invisible text.
	pub hidden: bool,
	pub strikethrough: bool,
}

/// The formatting components `SGR (Select Graphic Rendition)`.
/// [spec](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters)
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
#[derive(Debug, Clone, PartialEq)]
pub enum Intensity {
	Normal,
	Bold,
	Faint,
}

impl Default for SGR {
	fn default() -> Self {
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
