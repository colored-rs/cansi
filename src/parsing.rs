#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

/// A match.
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
    byte >= 0x40 && byte <= 0x7e
}

/// Parses ANSI escape codes from the given text, returning a vector of `Match`.
///
/// ```rust
/// let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = cansi::parse(ansi_text)
///     .into_iter()
/// 	.map(|m| (m.start, m.end))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse(text: &str) -> Vec<Match> {
    let mut v = Vec::with_capacity(8);

    let mut start = 0;
    let mut end = start + 2;

    while end <= text.len() {
        if &text[start..end] == CSI {
            // start of a CSI seq
            let mut end_1 = end + 1;
            while end < text.len() && !terminated_byte(text[end..end_1].as_bytes()[0]) {
                end = end_1;
                end_1 = end_1 + 1;
            }

            v.push(Match {
                start: start,
                end: end_1, // we include the an extra such that text is (start, end]
                text: &text[start..end_1],
            });

            start = end;
        } else {
            start = start + 1;
        }

        end = start + 2;
    }

    v
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
        let parsed: Vec<_> = parse(ansi_text)
            .into_iter()
            .map(|m| (m.start, m.end))
            .collect();
        assert_eq!(&parsed, &[(7, 14), (19, 23)],);
    }
}
