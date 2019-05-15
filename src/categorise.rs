use super::*;

const SEPARATOR: char = ';';

/// Parses the text and returns each formatted slice in order.
/// The ANSI escape codes are not included in the text slices.
///
/// Each different text slice is returned in order such that the text without the escape characters can be reconstructed.
/// There is a helper function (`construct_text_no_codes`) on `CategorisedSlices` for this.
pub fn categorise_text(text: &str) -> CategorisedSlices {
    let matches = parse(text);

    let mut sgr = SGR::default();

    let mut lo = 0;

    // will always less than or equal to matches + 1 in length, see tests
    let mut slices: Vec<CategorisedSlice> = Vec::with_capacity(matches.len() + 1);

    for m in matches {
        // add in the text before CSI with the previous SGR format
        let hi = m.start;
        if hi != lo {
            slices.push(CategorisedSlice::with_sgr(sgr, &text[lo..hi]));
        }

        sgr = handle_seq(&m);

        lo = m.end;

        // let mut escape_seq = m.text.as_bytes().iter().skip(2); // skip the first two (would be ESC *)
        // sgr = SGR::default();
        // let mut seq = Vec::new();
        // while let Some(byte) = escape_seq.next() {
        //     if byte == &SEPARATOR || (byte >= &b'\x40' && byte <= &b'\x7e') {
        //         // signals the end of a sequence, need to process what was transferred
        //         // if seq is empty, this is treated as a default flag
        //         if seq.len() == 0 {
        //             sgr = SGR::default();
        //         } else {
        //             // this map is a bit weird but i didn't want to have to convert to characters just
        //             // to make this mapping more simple. so the seq is in bytes and has to map back to utf8
        //             // 0-9 characters (effectively 48-57 in decimal notation)
        //             match &seq[..] {
        //                 &[48] => sgr = SGR::default(),                         // 0
        //                 &[49] => sgr.intensity = Intensity::Bold,              // 1
        //                 &[50] => sgr.intensity = Intensity::Faint,             // 2
        //                 &[51] => sgr.italic = true,                            // 3
        //                 &[52] => sgr.underline = true,                         // 4
        //                 &[53] => sgr.blink = true,                             // 5
        //                 &[55] => sgr.reversed = true,                          // 7
        //                 &[56] => sgr.hidden = true,                            // 8
        //                 &[57] => sgr.strikethrough = true,                     // 9
        //                 &[50, 50] => sgr.intensity = Intensity::Normal,        // 22
        //                 &[50, 51] => sgr.italic = false,                       // 23
        //                 &[50, 52] => sgr.underline = false,                    // 24
        //                 &[50, 53] => sgr.blink = false,                        // 25
        //                 &[50, 55] => sgr.reversed = false,                     // 27
        //                 &[50, 56] => sgr.hidden = false,                       // 28
        //                 &[50, 57] => sgr.strikethrough = false,                // 29
        //                 &[51, 48] => sgr.fg_colour = Color::Black,             // 30
        //                 &[51, 49] => sgr.fg_colour = Color::Red,               // 31
        //                 &[51, 50] => sgr.fg_colour = Color::Green,             // 32
        //                 &[51, 51] => sgr.fg_colour = Color::Yellow,            // 33
        //                 &[51, 52] => sgr.fg_colour = Color::Blue,              // 34
        //                 &[51, 53] => sgr.fg_colour = Color::Magenta,           // 35
        //                 &[51, 54] => sgr.fg_colour = Color::Cyan,              // 36
        //                 &[51, 55] => sgr.fg_colour = Color::White,             // 37
        //                 &[52, 48] => sgr.bg_colour = Color::Black,             // 40
        //                 &[52, 49] => sgr.bg_colour = Color::Red,               // 41
        //                 &[52, 50] => sgr.bg_colour = Color::Green,             // 42
        //                 &[52, 51] => sgr.bg_colour = Color::Yellow,            // 43
        //                 &[52, 52] => sgr.bg_colour = Color::Blue,              // 44
        //                 &[52, 53] => sgr.bg_colour = Color::Magenta,           // 45
        //                 &[52, 54] => sgr.bg_colour = Color::Cyan,              // 46
        //                 &[52, 55] => sgr.bg_colour = Color::White,             // 47
        //                 &[57, 48] => sgr.fg_colour = Color::BrightBlack,       // 90
        //                 &[57, 49] => sgr.fg_colour = Color::BrightRed,         // 91
        //                 &[57, 50] => sgr.fg_colour = Color::BrightGreen,       // 92
        //                 &[57, 51] => sgr.fg_colour = Color::BrightYellow,      // 93
        //                 &[57, 52] => sgr.fg_colour = Color::BrightBlue,        // 94
        //                 &[57, 53] => sgr.fg_colour = Color::BrightMagenta,     // 95
        //                 &[57, 54] => sgr.fg_colour = Color::BrightCyan,        // 96
        //                 &[57, 55] => sgr.fg_colour = Color::BrightWhite,       // 97
        //                 &[49, 48, 48] => sgr.bg_colour = Color::BrightBlack,   // 100
        //                 &[49, 48, 49] => sgr.bg_colour = Color::BrightRed,     // 101
        //                 &[49, 48, 50] => sgr.bg_colour = Color::BrightGreen,   // 102
        //                 &[49, 48, 51] => sgr.bg_colour = Color::BrightYellow,  // 103
        //                 &[49, 48, 52] => sgr.bg_colour = Color::BrightBlue,    // 104
        //                 &[49, 48, 53] => sgr.bg_colour = Color::BrightMagenta, // 105
        //                 &[49, 48, 54] => sgr.bg_colour = Color::BrightCyan,    // 106
        //                 &[49, 48, 55] => sgr.bg_colour = Color::BrightWhite,   // 107
        //                 _ => (),
        //             }
        //         }
        //         seq.clear();
        //     } else {
        //         seq.push(*byte); // not a signal to process so just push onto seq
        //     }
        // }
    }

    if lo != text.len() {
        slices.push(CategorisedSlice::with_sgr(sgr, &text[lo..text.len()]));
    }

    slices
}

/// Produce an `SGR` from a styling sequence.
///
/// Requires `Match` as we can assume skipping of certain bytes.
fn handle_seq(m: &Match) -> SGR {
    // the slice we want to process is skipped of first two bytes (ESC[) and last byte (terminating byte)
    let slice = &m.text[2..(m.text.len() - 1)];

    let styles = slice.split(SEPARATOR);

    let mut sgr = SGR::default();

    for style in styles {
        adjust_sgr(&mut sgr, style);
    }

    sgr
}

/// Apply the style seq to the SGR. Maps decimal numbers according to
/// spec at https://en.wikipedia.org/wiki/ANSI_escape_code#Escape_sequences.
fn adjust_sgr(sgr: &mut SGR, seq: &str) {
    match seq {
        "0" => *sgr = SGR::default(),                  // 0
        "1" => sgr.intensity = Intensity::Bold,        // 1
        "2" => sgr.intensity = Intensity::Faint,       // 2
        "3" => sgr.italic = true,                      // 3
        "4" => sgr.underline = true,                   // 4
        "5" => sgr.blink = true,                       // 5
        "7" => sgr.reversed = true,                    // 7
        "8" => sgr.hidden = true,                      // 8
        "9" => sgr.strikethrough = true,               // 9
        "22" => sgr.intensity = Intensity::Normal,     // 22
        "23" => sgr.italic = false,                    // 23
        "24" => sgr.underline = false,                 // 24
        "25" => sgr.blink = false,                     // 25
        "27" => sgr.reversed = false,                  // 27
        "28" => sgr.hidden = false,                    // 28
        "29" => sgr.strikethrough = false,             // 29
        "30" => sgr.fg_colour = Color::Black,          // 30
        "31" => sgr.fg_colour = Color::Red,            // 31
        "32" => sgr.fg_colour = Color::Green,          // 32
        "33" => sgr.fg_colour = Color::Yellow,         // 33
        "34" => sgr.fg_colour = Color::Blue,           // 34
        "35" => sgr.fg_colour = Color::Magenta,        // 35
        "36" => sgr.fg_colour = Color::Cyan,           // 36
        "37" => sgr.fg_colour = Color::White,          // 37
        "40" => sgr.bg_colour = Color::Black,          // 40
        "41" => sgr.bg_colour = Color::Red,            // 41
        "42" => sgr.bg_colour = Color::Green,          // 42
        "43" => sgr.bg_colour = Color::Yellow,         // 43
        "44" => sgr.bg_colour = Color::Blue,           // 44
        "45" => sgr.bg_colour = Color::Magenta,        // 45
        "46" => sgr.bg_colour = Color::Cyan,           // 46
        "47" => sgr.bg_colour = Color::White,          // 47
        "90" => sgr.fg_colour = Color::BrightBlack,    // 90
        "91" => sgr.fg_colour = Color::BrightRed,      // 91
        "92" => sgr.fg_colour = Color::BrightGreen,    // 92
        "93" => sgr.fg_colour = Color::BrightYellow,   // 93
        "94" => sgr.fg_colour = Color::BrightBlue,     // 94
        "95" => sgr.fg_colour = Color::BrightMagenta,  // 95
        "96" => sgr.fg_colour = Color::BrightCyan,     // 96
        "97" => sgr.fg_colour = Color::BrightWhite,    // 97
        "100" => sgr.bg_colour = Color::BrightBlack,   // 100
        "101" => sgr.bg_colour = Color::BrightRed,     // 101
        "102" => sgr.bg_colour = Color::BrightGreen,   // 102
        "103" => sgr.bg_colour = Color::BrightYellow,  // 103
        "104" => sgr.bg_colour = Color::BrightBlue,    // 104
        "105" => sgr.bg_colour = Color::BrightMagenta, // 105
        "106" => sgr.bg_colour = Color::BrightCyan,    // 106
        "107" => sgr.bg_colour = Color::BrightWhite,   // 107
        _ => (),
    }
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
