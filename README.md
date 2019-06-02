# cansi

[![Build Status](https://travis-ci.com/kurtlawrence/cansi.svg?branch=master)](https://travis-ci.com/kurtlawrence/cansi)
[![Latest Version](https://img.shields.io/crates/v/cansi.svg)](https://crates.io/crates/cansi)
[![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cansi)
[![codecov](https://codecov.io/gh/kurtlawrence/cansi/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cansi)

## **C**atergorise **ANSI** - ANSI escape code parser and categoriser

See the [rs docs.](https://docs.rs/cansi/)
Look at progress and contribute on [github.](https://github.com/kurtlawrence/cansi)

`cansi` will parse text with ANSI escape sequences in it and return a deconstructed text with metadata around the colouring and styling. `cansi` is only concerned with `CSI` sequences, particuarly the `SGR` parameters. `cansi` will not construct escaped text, there are crates such as [`colored`](https://crates.io/crates/colored) that do a great job of colouring and styling text.

## Example usage

> This example was done using the `colored` crate to help with constructing the escaped text string. It will work with other tools that inject escape sequences into text strings (given they follow [ANSI specification](https://en.wikipedia.org/wiki/ANSI_escape_code)).

```rust
extern crate cansi;
extern crate colored;

use cansi::*;
use colored::Colorize;
use std::io::Write;

let v = &mut Vec::new();
write!(
  v,
  "Hello, {}{}{}{}{}{}",
  "w".white().on_red(),
  "o".cyan().on_green(),
  "r".magenta().on_yellow(),
  "l".blue().on_white(),
  "d".yellow().on_bright_cyan(),
  "!".bright_red().on_bright_yellow(),
)
.unwrap();

let text = String::from_utf8_lossy(&v);
let result = categorise_text(&text); // cansi function

assert_eq!(result.len(), 7); // there should be seven differently styled components

assert_eq!("Hello, world!", &construct_text_no_codes(&result));

// 'Hello, ' is just defaults
assert_eq!(
  result[0],
  CategorisedSlice {
    text: "Hello, ",
    start: 0,
    end: 7,
    fg_colour: Color::White,
    bg_colour: Color::Black,
    intensity: Intensity::Normal,
    italic: false,
    underline: false,
    blink: false,
    reversed: false,
    hidden: false,
    strikethrough: false
  }
);

// 'w' is coloured differently
assert_eq!(
  result[1],
  CategorisedSlice {
    text: "w",
    start: 15,
    end: 16,
    fg_colour: Color::White,
    bg_colour: Color::Red,
    intensity: Intensity::Normal,
    italic: false,
    underline: false,
    blink: false,
    reversed: false,
    hidden: false,
    strikethrough: false
  }
);
```