#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

/// A match.
#[derive(Debug, PartialEq, Eq)]
pub struct Match<'t> {
    /// First byte index.
    pub start: usize,
    /// Last byte index + 1.
    pub end: usize,
    /// The text slice (ie `text[start..end]`).
    /// Note that the range is `(start..end]`.
    pub text: &'t str,
}

// ESC is 0x1b
const CSI: &str = "\x1b[";

#[inline(always)]
fn terminated_byte(byte: u8) -> bool {
    (0x40..=0x7e).contains(&byte)
}

/// Parses ANSI escape codes from the given text, returning a vector of `Match`.
///
/// ```rust
/// let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = cansi::parse(ansi_text)
///     .into_iter()
///     .map(|m| (m.start, m.end))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse(text: &str) -> Vec<Match> {
    let mut v = Vec::with_capacity(8);
    let csi_len = CSI.len();

    let mut s = text;
    let mut start = 0;
    let mut end = start + csi_len;

    while end <= text.len() {
        if s.starts_with(CSI) {
            // start of a CSI seq
            while end < text.len() && !terminated_byte(text.as_bytes()[end]) {
                end += 1;
            }

            let end = end + 1;

            if end > text.len() {
                break;
            }

            v.push(Match {
                start,
                end,
                text: &text[start..end],
            });

            start = end;
        } else {
            start += s.chars().next().expect("non-empty-str").len_utf8();
        }

        s = &text[start..];
        end = start + csi_len;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
        let parsed = parse(ansi_text);
        assert_eq!(
            parsed,
            vec![
                Match {
                    start: 7,
                    end: 14,
                    text: "\x1b[31;4m"
                },
                Match {
                    start: 19,
                    end: 23,
                    text: "\x1b[0m"
                }
            ]
        );
    }

    #[test]
    fn parse_string_with_different_chars() {
        let t = "👋, \x1b[31;4m🌍\x1b[0m!";
        let parsed = parse(t);
        assert_eq!(
            parsed,
            vec![
                Match {
                    start: 6,
                    end: 13,
                    text: "\x1b[31;4m"
                },
                Match {
                    start: 17,
                    end: 21,
                    text: "\x1b[0m"
                }
            ]
        );
    }

    #[test]
    fn malformed_escape() {
        let x = parse("oops\x1b[\n");

        assert_eq!(x, vec![]);
    }
}
