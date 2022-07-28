use super::*;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

const SEPARATOR: char = ';';

/// Parses the text and returns each formatted slice in order.
/// The ANSI escape codes are not included in the text slices.
///
/// Each different text slice is returned in order such that the text without the escape characters can be reconstructed.
/// There is a helper function (`construct_text_no_codes`) on `CategorisedSlices` for this.
#[deprecated = "please use v3::categorise_text to move to API v3.0. \
                this function will be removed with v3.0 of cansi"]
#[allow(deprecated)]
pub fn categorise_text(text: &str) -> CategorisedSlices {
    categorise_text_v3(text)
        .into_iter()
        .map(Into::into)
        .collect()
}

/// Parses the text and returns each formatted slice in order.
/// The ANSI escape codes are not included in the text slices.
///
/// Each different text slice is returned in order such that the text without the escape characters can be reconstructed.
/// There is a helper function (`construct_text_no_codes`) on `CategorisedSlices` for this.
pub fn categorise_text_v3(text: &str) -> v3::CategorisedSlices {
    let matches = parse(text);

    let mut sgr = SGR::default();

    let mut lo = 0;

    // will always less than or equal to matches + 1 in length, see tests
    let mut slices: Vec<v3::CategorisedSlice> = Vec::with_capacity(matches.len() + 1);

    for m in matches {
        // add in the text before CSI with the previous SGR format
        if m.start != lo {
            slices.push(v3::CategorisedSlice::with_sgr(
                sgr,
                &text[lo..m.start],
                lo,
                m.start,
            ));
        }

        sgr = handle_seq(&m);

        lo = m.end;
    }

    if lo != text.len() {
        slices.push(v3::CategorisedSlice::with_sgr(
            sgr,
            &text[lo..text.len()],
            lo,
            text.len(),
        ));
    }

    slices
}

/// Produce an `SGR` from a styling sequence.
///
/// Requires `Match` as we can assume skipping of certain bytes.
fn handle_seq(m: &Match) -> SGR {
    // the slice we want to process is skipped of first two bytes (ESC[) and last byte (terminating byte)
    let slice = &m.text[2..(m.text.len() - 1)];
    slice
        .split(SEPARATOR)
        .fold(SGR::default(), |x, s| adjust_sgr(x, s))
}

/// Apply the style seq to the SGR. Maps decimal numbers according to
/// spec at https://en.wikipedia.org/wiki/ANSI_escape_code#Escape_sequences.
fn adjust_sgr(mut sgr: SGR, seq: &str) -> SGR {
    match seq {
        "0" => return SGR::default(),                    // 0
        "1" => sgr.intensity = Some(Intensity::Bold),    // 1
        "2" => sgr.intensity = Some(Intensity::Faint),   // 2
        "3" => sgr.italic = Some(true),                  // 3
        "4" => sgr.underline = Some(true),               // 4
        "5" => sgr.blink = Some(true),                   // 5
        "7" => sgr.reversed = Some(true),                // 7
        "8" => sgr.hidden = Some(true),                  // 8
        "9" => sgr.strikethrough = Some(true),           // 9
        "22" => sgr.intensity = Some(Intensity::Normal), // 22
        "23" => sgr.italic = Some(false),                // 23
        "24" => sgr.underline = Some(false),             // 24
        "25" => sgr.blink = Some(false),                 // 25
        "27" => sgr.reversed = Some(false),              // 27
        "28" => sgr.hidden = Some(false),                // 28
        "29" => sgr.strikethrough = Some(false),         // 29
        "30" => sgr.fg = Some(Color::Black),             // 30
        "31" => sgr.fg = Some(Color::Red),               // 31
        "32" => sgr.fg = Some(Color::Green),             // 32
        "33" => sgr.fg = Some(Color::Yellow),            // 33
        "34" => sgr.fg = Some(Color::Blue),              // 34
        "35" => sgr.fg = Some(Color::Magenta),           // 35
        "36" => sgr.fg = Some(Color::Cyan),              // 36
        "37" => sgr.fg = Some(Color::White),             // 37
        "40" => sgr.bg = Some(Color::Black),             // 40
        "41" => sgr.bg = Some(Color::Red),               // 41
        "42" => sgr.bg = Some(Color::Green),             // 42
        "43" => sgr.bg = Some(Color::Yellow),            // 43
        "44" => sgr.bg = Some(Color::Blue),              // 44
        "45" => sgr.bg = Some(Color::Magenta),           // 45
        "46" => sgr.bg = Some(Color::Cyan),              // 46
        "47" => sgr.bg = Some(Color::White),             // 47
        "90" => sgr.fg = Some(Color::BrightBlack),       // 90
        "91" => sgr.fg = Some(Color::BrightRed),         // 91
        "92" => sgr.fg = Some(Color::BrightGreen),       // 92
        "93" => sgr.fg = Some(Color::BrightYellow),      // 93
        "94" => sgr.fg = Some(Color::BrightBlue),        // 94
        "95" => sgr.fg = Some(Color::BrightMagenta),     // 95
        "96" => sgr.fg = Some(Color::BrightCyan),        // 96
        "97" => sgr.fg = Some(Color::BrightWhite),       // 97
        "100" => sgr.bg = Some(Color::BrightBlack),      // 100
        "101" => sgr.bg = Some(Color::BrightRed),        // 101
        "102" => sgr.bg = Some(Color::BrightGreen),      // 102
        "103" => sgr.bg = Some(Color::BrightYellow),     // 103
        "104" => sgr.bg = Some(Color::BrightBlue),       // 104
        "105" => sgr.bg = Some(Color::BrightMagenta),    // 105
        "106" => sgr.bg = Some(Color::BrightCyan),       // 106
        "107" => sgr.bg = Some(Color::BrightWhite),      // 107
        _ => (),
    }

    sgr
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;

    #[test]
    fn cat_and_matches_len() {
        let txt = "hello";
        let matches = parse(&txt);
        let cat = categorise_text(&txt);
        assert!(matches.len() + 1 >= cat.len());

        let txt = "hello".bright_green();
        let matches = parse(&txt);
        let cat = categorise_text(&txt);
        assert!(matches.len() + 1 >= cat.len());

        let txt = format!("{}{}{}", "hello".bright_green(), "world".red(), "whatever");
        let matches = parse(&txt);
        let cat = categorise_text(&txt);
        assert!(matches.len() + 1 >= cat.len());
    }
}
