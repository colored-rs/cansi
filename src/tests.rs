use super::*;

#[test]
fn cover_other_parameters() {
	// colored doesn't always test all match arms, so i test here

	// no escape sequences
	let text = b"test";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[;mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[;mtest\x1b[;m\x1b[;m";
	assert_eq!(categorise_text(&text[..]).len(), 1);
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[1;22mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[3;23mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[4;24mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[5;25mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[7;27mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[8;28mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
	let text = b"\x1b[9;29mtest";
	assert_eq!(
		categorise_text(&text[..])[0],
		CategorisedSlice {
			text: b"test",
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
