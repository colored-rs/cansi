use super::*;
use colored::Colorize;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

#[test]
fn cover_other_parameters() {
    // colored doesn't always test all match arms, so i test here

    // no escape sequences
    let text = "test";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 0,
            end: 4,
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

    // empty sequences
    let text = "\x1b[;mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 4,
            end: 8,
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

    // empty text - doesn't add it
    let text = "\x1b[;mtest\x1b[;m\x1b[;m";
    assert_eq!(categorise_text(&text[..]).len(), 1);
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 4,
            end: 8,
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

    // 22
    let text = "\x1b[1;22mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 23
    let text = "\x1b[3;23mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 24
    let text = "\x1b[4;24mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 25
    let text = "\x1b[5;25mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 27
    let text = "\x1b[7;27mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 28
    let text = "\x1b[8;28mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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

    // 29
    let text = "\x1b[9;29mtest";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
            start: 7,
            end: 11,
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
}

#[test]
fn split_on_new_line_tests() {
    fn fn_as_str(s: &str) -> (&str, Option<&str>) {
        let (first, remainder) = split_on_new_line(s);
        (&s[..first], remainder.map(|i| &s[i..]))
    }

    // no remainder
    let (first, remainder) = fn_as_str("Hello worlds");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, None);

    let (first, remainder) = fn_as_str("Hello worlds\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some(""));

    let (first, remainder) = fn_as_str("Hello worlds\r\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some(""));

    // some remainder
    let (first, remainder) = fn_as_str("Hello worlds\none two three");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one two three"));

    let (first, remainder) = fn_as_str("Hello worlds\r\none two three");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one two three"));

    let (first, remainder) = fn_as_str("Hello worlds\r\none\ntwo\nthree\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one\ntwo\nthree\n"));

    // no first
    let (first, remainder) = fn_as_str("\r\nHello worlds\none two three");
    assert_eq!(first, "");
    assert_eq!(remainder, Some("Hello worlds\none two three"));

    let (first, remainder) = fn_as_str("\nHello worlds\r\none two three");
    assert_eq!(first, "");
    assert_eq!(remainder, Some("Hello worlds\r\none two three"));

    let (first, remainder) = fn_as_str("\r\n");
    assert_eq!(first, "");
    assert_eq!(remainder, Some(""));
}

#[test]
fn clone_style_test() {
    use colored::*;
    let s = "hello".green();
    let c = categorise_text(&s);
    let d = c[0].clone_style("why", 0, 0);

    assert_eq!(d.text, "why");

    let e = d.clone_style("hello", 0, 5);

    assert_eq!(c[0], e);
}

#[test]
fn line_iter_test() {
    let mut green = CategorisedSlice::default_style("", 0, 0);
    let mut red = CategorisedSlice::default_style("", 0, 0);
    green.fg_colour = Color::Green;
    red.fg_colour = Color::Red;

    let cat = categorise_text("hello, world");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello, world", 0, 12)])
    );
    assert_eq!(iter.next(), None);

    let cat = categorise_text("hello, world\nhow are you");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello, world", 0, 12)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you", 13, 24)])
    );
    assert_eq!(iter.next(), None);

    let s = format!("{}{}\nhow are you", "hello, ".green(), "world".red());
    let cat = categorise_text(&s);
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![
            green.clone_style("hello, ", 5, 12),
            red.clone_style("world", 21, 26)
        ])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you", 31, 42)])
    );
    assert_eq!(&s[5..12], "hello, ");
    assert_eq!(&s[21..26], "world");
    assert_eq!(&s[31..42], "how are you");
    assert_eq!(iter.next(), None);

    let s = format!(
        "{}{}\nhow are you\r\ntoday",
        "hello, ".green(),
        "world".red()
    );
    let cat = categorise_text(&s);
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![
            green.clone_style("hello, ", 5, 12),
            red.clone_style("world", 21, 26)
        ])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you", 31, 42)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("today", 44, 49)])
    );
    assert_eq!(iter.next(), None);
    assert_eq!(&s[5..12], "hello, ");
    assert_eq!(&s[21..26], "world");
    assert_eq!(&s[31..42], "how are you");
    assert_eq!(&s[44..49], "today");

    let cat = categorise_text("\n\n\n\n");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 0, 0)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 1, 1)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 2, 2)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 3, 3)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 4, 4)])
    );
    assert_eq!(iter.next(), None);

    let cat = categorise_text("\r\n\r\n\r\n\r\n");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 0, 0)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 2, 2)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 4, 4)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 6, 6)])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("", 8, 8)])
    );
    assert_eq!(iter.next(), None);
}

#[test]
fn line_iter_newline_starts_with_esc() {
    let mut green = CategorisedSlice::default_style("", 0, 0);
    green.fg_colour = Color::Green;

    let s = format!("hello\n{}", "world".green());
    let cat = categorise_text(&s);
    let mut iter = line_iter(&cat);

    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello", 0, 5)])
    );
    assert_eq!(iter.next(), Some(vec![green.clone_style("world", 11, 16)]));
    assert_eq!(&s[11..16], "world");
}

#[test]
fn line_iter_bugs() {
    let bug_str = "\u{1b}[36mpapyrus\u{1b}[0m=> 5+6\n\u{1b}[36mpapyrus\u{1b}[0m \u{1b}[92m[out0]\u{1b}[0m: 11                                            \n\u{1b}[36mpapyrus\u{1b}[0m=>
                              \n                                                                                \n                                                                                \n
                     \n                                                                                \n                                                                                \n
            \n                                                                                \n                                                                                \n
   \n                                                                                \n                                                                                \n                                                                                \n
                                                                            \n                                                                                \n                                                                                \n
                                                                   \n                                                                                \n                                                                                \n
                                                          \n                                                                                \n                                                                                \n";

    let cat = categorise_text(bug_str);
    let mut iter = line_iter(&cat);

    let mut cyan = CategorisedSlice::default_style("", 0, 0);
    cyan.fg_colour = Color::Cyan;

    assert_eq!(
        iter.next(),
        Some(vec![
            cyan.clone_style("papyrus", 5, 12),
            CategorisedSlice::default_style("=> 5+6", 16, 22)
        ])
    );

    assert_eq!(&bug_str[5..12], "papyrus");
    assert_eq!(&bug_str[16..22], "=> 5+6");
}
