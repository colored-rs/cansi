use regex::{Matches, Regex};

lazy_static! {
    /// A `Regex` that matches ANSI escape codes.
    ///
    /// ```rust
    /// assert_eq!(
    ///     cansi::ANSI_RGX.replace_all("Hello, \x1b[31;4mworld\x1b[0m!", ""),
    ///     "Hello, world!",
    /// );
    pub static ref ANSI_RGX: Regex = Regex::new(ANSI_RE).unwrap();
}

/// Parses ANSI escape codes from the given text, returning an `Iterator<Item = Match>`.
///
/// ```rust
/// let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
/// let parsed: Vec<_> = cansi::parse(ansi_text)
///     .map(|m| (m.start(), m.end()))
///     .collect();
/// assert_eq!(
///     parsed,
///     vec![(7, 14), (19, 23)],
/// );
/// ```
pub fn parse(text: &str) -> Matches {
    ANSI_RGX.find_iter(text)
}

const ANSI_RE: &str = r"[\x1b\x9b]\[[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-ORZcf-nqry=><]";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ansi_text = "Hello, \x1b[31;4mworld\x1b[0m!";
        let parsed: Vec<_> = parse(ansi_text).map(|m| (m.start(), m.end())).collect();
        assert_eq!(&parsed, &[(7, 14), (19, 23)],);
    }
}
