extern crate cansi;
extern crate colored;

use cansi::*;
use colored::*;
use std::io::Write;

fn print_bytes(bytes: &[u8]) {
	for ch in String::from_utf8_lossy(bytes).chars() {
		print!("{} ", ch);
	}
	println!("",);
	for byte in bytes {
		print!("{} ", byte);
	}
	println!("",);
}

#[test]
fn test_styling() {
	let test_string: &[u8] = b"test";
	let v = &mut Vec::new();

	write!(v, "{}", "test".black()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Black,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"black()"
	);
	v.clear();
	write!(v, "{}", "test".red()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Red,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"red()"
	);
	v.clear();
	write!(v, "{}", "test".green()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Green,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"green()"
	);
	v.clear();
	write!(v, "{}", "test".yellow()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Yellow,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"yellow()"
	);
	v.clear();
	write!(v, "{}", "test".blue()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Blue,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"blue()"
	);
	v.clear();
	write!(v, "{}", "test".magenta()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Magenta,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"magenta()"
	);
	v.clear();
	write!(v, "{}", "test".purple()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Magenta,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"purple()"
	);
	v.clear();
	write!(v, "{}", "test".cyan()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::Cyan,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"cyan()"
	);
	v.clear();
	write!(v, "{}", "test".white()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"white()"
	);
	v.clear();
	write!(v, "{}", "test".bright_black()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightBlack,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_black()"
	);
	v.clear();
	write!(v, "{}", "test".bright_red()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightRed,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_red()"
	);
	v.clear();
	write!(v, "{}", "test".bright_green()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightGreen,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_green()"
	);
	v.clear();
	write!(v, "{}", "test".bright_yellow()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightYellow,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_yellow()"
	);
	v.clear();
	write!(v, "{}", "test".bright_blue()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightBlue,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_blue()"
	);
	v.clear();
	write!(v, "{}", "test".bright_magenta()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightMagenta,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_magenta()"
	);
	v.clear();
	write!(v, "{}", "test".bright_purple()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightMagenta,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_purple()"
	);
	v.clear();
	write!(v, "{}", "test".bright_cyan()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightCyan,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_cyan()"
	);
	v.clear();
	write!(v, "{}", "test".bright_white()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::BrightWhite,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bright_white()"
	);
	v.clear();
	write!(v, "{}", "test".on_black()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_black()"
	);
	v.clear();
	write!(v, "{}", "test".on_red()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Red,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_red()"
	);
	v.clear();
	write!(v, "{}", "test".on_green()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Green,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_green()"
	);
	v.clear();
	write!(v, "{}", "test".on_yellow()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Yellow,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_yellow()"
	);
	v.clear();
	write!(v, "{}", "test".on_blue()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Blue,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_blue()"
	);
	v.clear();
	write!(v, "{}", "test".on_magenta()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Magenta,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_magenta()"
	);
	v.clear();
	write!(v, "{}", "test".on_purple()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Magenta,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_purple()"
	);
	v.clear();
	write!(v, "{}", "test".on_cyan()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Cyan,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_cyan()"
	);
	v.clear();
	write!(v, "{}", "test".on_white()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::White,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_white()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_black()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightBlack,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_black()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_red()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightRed,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_red()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_green()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightGreen,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_green()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_yellow()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightYellow,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_yellow()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_blue()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightBlue,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_blue()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_magenta()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightMagenta,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_magenta()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_purple()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightMagenta,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_purple()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_cyan()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightCyan,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_cyan()"
	);
	v.clear();
	write!(v, "{}", "test".on_bright_white()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::BrightWhite,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"on_bright_white()"
	);
	v.clear();
	write!(v, "{}", "test".clear()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"clear()"
	);
	v.clear();
	write!(v, "{}", "test".normal()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"normal()"
	);
	v.clear();
	write!(v, "{}", "test".bold()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Bold,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"bold()"
	);
	v.clear();
	write!(v, "{}", "test".dimmed()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Faint,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"dimmed()"
	);
	v.clear();
	write!(v, "{}", "test".italic()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: true,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"italic()"
	);
	v.clear();
	write!(v, "{}", "test".underline()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: true,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"underline()"
	);
	v.clear();
	write!(v, "{}", "test".blink()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: true,
			reversed: false,
			hidden: false,
			strikethrough: false
		},
		"blink()"
	);
	v.clear();
	write!(v, "{}", "test".reverse()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: true,
			hidden: false,
			strikethrough: false
		},
		"reverse()"
	);
	v.clear();
	write!(v, "{}", "test".reversed()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: true,
			hidden: false,
			strikethrough: false
		},
		"reversed()"
	);
	v.clear();
	write!(v, "{}", "test".hidden()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: true,
			strikethrough: false
		},
		"hidden()"
	);
	v.clear();
	write!(v, "{}", "test".strikethrough()).unwrap();
	print_bytes(&v);
	assert_eq!(
		categorise_text(&v)[0],
		CategorisedSlice {
			text: test_string,
			fg_colour: Color::White,
			bg_colour: Color::Black,
			intensity: Intensity::Normal,
			italic: false,
			underline: false,
			blink: false,
			reversed: false,
			hidden: false,
			strikethrough: true
		},
		"strikethrough()"
	);
	v.clear();
}
