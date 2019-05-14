use super::*;
use colored::Colorize;

#[test]
fn cover_other_parameters() {
    // colored doesn't always test all match arms, so i test here

    // no escape sequences
    let text = "test";
    assert_eq!(
        categorise_text(&text[..])[0],
        CategorisedSlice {
            text: "test",
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
    // no remainder
    let (first, remainder) = split_on_new_line("Hello worlds");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, None);

    let (first, remainder) = split_on_new_line("Hello worlds\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some(""));

    let (first, remainder) = split_on_new_line("Hello worlds\r\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some(""));

    // some remainder
    let (first, remainder) = split_on_new_line("Hello worlds\none two three");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one two three"));

    let (first, remainder) = split_on_new_line("Hello worlds\r\none two three");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one two three"));

    let (first, remainder) = split_on_new_line("Hello worlds\r\none\ntwo\nthree\n");
    assert_eq!(first, "Hello worlds");
    assert_eq!(remainder, Some("one\ntwo\nthree\n"));

    // no first
    let (first, remainder) = split_on_new_line("\r\nHello worlds\none two three");
    assert_eq!(first, "");
    assert_eq!(remainder, Some("Hello worlds\none two three"));

    let (first, remainder) = split_on_new_line("\nHello worlds\r\none two three");
    assert_eq!(first, "");
    assert_eq!(remainder, Some("Hello worlds\r\none two three"));

    let (first, remainder) = split_on_new_line("\r\n");
    assert_eq!(first, "");
    assert_eq!(remainder, Some(""));
}

#[test]
fn clone_style_test() {
    use colored::*;
    let s = "hello".green();
    let c = categorise_text(&s);
    let d = c[0].clone_style("why");

    assert_eq!(d.text, "why");

    let e = d.clone_style("hello");

    assert_eq!(c[0], e);
}

#[test]
fn line_iter_test() {
    let mut green = CategorisedSlice::default_style("");
    let mut red = CategorisedSlice::default_style("");
    green.fg_colour = Color::Green;
    red.fg_colour = Color::Red;

    let cat = categorise_text("hello, world");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello, world")])
    );
    assert_eq!(iter.next(), None);

    let cat = categorise_text("hello, world\nhow are you");
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello, world")])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you")])
    );
    assert_eq!(iter.next(), None);

    let s = format!("{}{}\nhow are you", "hello, ".green(), "world".red());
    let cat = categorise_text(&s);
    let mut iter = line_iter(&cat);
    assert_eq!(
        iter.next(),
        Some(vec![green.clone_style("hello, "), red.clone_style("world")])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you")])
    );
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
        Some(vec![green.clone_style("hello, "), red.clone_style("world")])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("how are you")])
    );
    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("today")])
    );
    assert_eq!(iter.next(), None);

    let cat = categorise_text("\n\n\n\n");
    let mut iter = line_iter(&cat);
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), None);

    let cat = categorise_text("\r\n\r\n\r\n\r\n");
    let mut iter = line_iter(&cat);
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), Some(vec![CategorisedSlice::default_style("")]));
    assert_eq!(iter.next(), None);
}

#[test]
fn line_iter_newline_starts_with_esc() {
    let mut green = CategorisedSlice::default_style("");
    green.fg_colour = Color::Green;

    let s = format!("hello\n{}", "world".green());
    let cat = categorise_text(&s);
    let mut iter = line_iter(&cat);

    assert_eq!(
        iter.next(),
        Some(vec![CategorisedSlice::default_style("hello")])
    );
    assert_eq!(iter.next(), Some(vec![green.clone_style("world")]));
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

    let mut cyan = CategorisedSlice::default_style("");
    cyan.fg_colour = Color::Cyan;

    assert_eq!(
        iter.next(),
        Some(vec![
            cyan.clone_style("papyrus"),
            CategorisedSlice::default_style("=> 5+6")
        ])
    );
}
