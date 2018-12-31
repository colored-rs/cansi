# cansi

[![Build Status](https://travis-ci.com/kurtlawrence/cansi.svg?branch=master)](https://travis-ci.com/kurtlawrence/cansi) [![Latest Version](https://img.shields.io/crates/v/cansi.svg)](https://crates.io/crates/cansi) [![Rust Documentation](https://img.shields.io/badge/api-rustdoc-blue.svg)](https://docs.rs/cansi) [![codecov](https://codecov.io/gh/kurtlawrence/cansi/branch/master/graph/badge.svg)](https://codecov.io/gh/kurtlawrence/cansi)

## **C**atergorise **ANSI** - ANSI escape code parser and categoriser

See the [rs docs.](https://docs.rs/cansi/)
Look at progress and contribute on [github.](https://github.com/kurtlawrence/cansi)

`cansi` will parse text with ANSI escape sequences in it and return a deconstructed text with metadata around the colouring and styling. `cansi` is only concerned with `CSI` sequences, particuarly the `SGR` parameters. `cansi` will not constructed escaped text, there are crates such as [colored](https://crates.io/crates/colored) that do a great job of colouring and styling text.

## Example usage

```rust
extern crate cansi;
extern crate colored;

use cansi::*;
use colored::*;
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

let result = categorise_text(&v); // cansi function

assert_eq!(result.len(), 7); // there should be seven differently styled components

assert_eq!(
    b"Hello, world!",
    &result
      .iter()
      .flat_map(|r| r.text)
      .map(|x| *x)
      .collect::<Vec<_>>()[..]
  );

// 'Hello, ' is just defaults
assert_eq!(
  result[0],
  CategorisedSlice {
    text: b"Hello, ",
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
    text: b"w",
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

> **Note**
>
> The example was done using the `colored` crate to help with constructing the escaped text string. It will work with other tools that inject escape sequences into text strings.