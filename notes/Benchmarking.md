# Benchmarking

2019-05-14

## Summary

| fn                        | bench            | before   | after    | result      |
| ------------------------- | ---------------- | -------- | -------- | ----------- |
| `categorise_text`         | `&'static str`   | ~1.0 μs  | ~368 ns  | ✔️ improved |
| `categorise_text`         | simple str       | ~1.3 μs  | ~338 ns  | ✔️ improved |
| `categorise_text`         | long no colour   | ~430 ns  | ~2.4 μs  | ❌ regressed |
| `categorise_text`         | long complex     | ~10.2 μs | ~3.4 μs  | ✔️ improved |
| `categorise_text`         | long complex x 4 | ~37.5 μs | ~15.5 μs | ✔️ improved |
| `construct_text_no_codes` | long complex     | ~3.2 μs  | ~190 ns  | ✔️ improved |

Major speed improvements were made in all important use cases. The single regression was made when no codes exist on a string. Overall the improvements are worthwile.

## Set up

The first target was the `categorise_text` function. Five benches were set up, one which categorises a `&'static str`, one a simple string with `.bright_red()`, the other two are longer strings, one with colour and the other without. A final complex long bench, with four times the amount of colours.

The `construct_text_no_codes` function was second, with a single benchmark turning a largish categorised text into a `String`.

The results of the initial benching:

| fn                        | bench            | time     |
| ------------------------- | ---------------- | -------- |
| `categorise_text`         | `&'static str`   | ~1.0 μs  |
| `categorise_text`         | simple str       | ~1.3 μs  |
| `categorise_text`         | long no colour   | ~430 ns  |
| `categorise_text`         | long complex     | ~10.2 μs |
| `categorise_text`         | long complex x 4 | ~37.5 μs |
| `construct_text_no_codes` | long complex     | ~3.2 μs  |

The simple cases are obviously fast, and the _complex x 4_ case highlights how linear the categorising is. Reconstructing the text does take time, which flags a need for improvement.

## Rolling my own `parse` function

The first change was to roll my own `parse` version of `parse_ansi::parse_bytes`. This removes having to work in byte slices and rather work in `&str`. It does constitute a breaking change but it is for the better. The results of the benchmarks show some improvements. There is one major regression, where no colour is being applied. The regression is acceptable as this crate is meant to be used when colur **_is_** being applied, and the performance is still within acceptable numbers.

| fn                        | bench            | time     |
| ------------------------- | ---------------- | -------- |
| `categorise_text`         | `&'static str`   | ~512 ns  |
| `categorise_text`         | simple str       | ~575 ns  |
| `categorise_text`         | long no colour   | ~2.5 μs  |
| `categorise_text`         | long complex     | ~7.7 μs  |
| `categorise_text`         | long complex x 4 | ~28.6 μs |
| `construct_text_no_codes` | long complex     | ~3.3 μs  |

## Refactoring styling code

The next target was a refactor of the styling loop. Styling is cumulative, which means it must be looped through. The old code was byte orientated, splitting on `b';'`. As styling could comprise of 1-3 decimal digits, a small vector was maintained which then was read from when the separator was reached.

```rust
// before
// m is Match.

// ...

let mut escape_seq = m.text.as_bytes().iter().skip(2); // skip the first two (would be ESC *)
sgr = SGR::default();
let mut seq = Vec::new();
while let Some(byte) = escape_seq.next() {
    if byte == &b';' || (byte >= &b'\x40' && byte <= &b'\x7e') {
        // signals the end of a sequence, need to process what was transferred
        // if seq is empty, this is treated as a default flag
        if seq.len() == 0 {
            sgr = SGR::default();
        } else {
            // .. apply style function
        }
        seq.clear();
    } else {
        seq.push(*byte); // not a signal to process so just push onto seq
    }
}
```

Since there is no need to work in bytes anymore, the `str.split()` function was employed. The apply style function could also just be matched on a string slice rather than byte slice. It also made the code much more readable!

```rust
// improved code
// m is Match.

// the slice we want to process is skipped of first two bytes (ESC[) and last byte (terminating byte)
let slice = &m.text[2..(m.text.len()-1)];

let styles = slice.split(';');

let mut sgr = SGR::default();

for style in styles {
    // .. apply style function
}

// sgr can be linked to text.
```

The benchmark results showed improvements for all `categorise_text` functions with colours in them. This is as far as I believe `categorise_text` will get.

| fn                        | bench            | time     |
| ------------------------- | ---------------- | -------- |
| `categorise_text`         | `&'static str`   | ~395 ns  |
| `categorise_text`         | simple str       | ~410 ns  |
| `categorise_text`         | long no colour   | ~2.7 μs  |
| `categorise_text`         | long complex     | ~5.0 μs  |
| `categorise_text`         | long complex x 4 | ~15.3 μs |
| `construct_text_no_codes` | long complex     | ~3.6 μs  |

## Refactoring `construct_text_no_codes` function

The original `construct_text_no_codes` function was quite ineffiecient:

```rust
pub fn construct_text_no_codes(categorised_slices: &CategorisedSlices) -> String {
    String::from_utf8_lossy(
        &categorised_slices
            .iter()
            .flat_map(|r| r.text.as_bytes())
            .map(|x| *x)
            .collect::<Vec<_>>()[..]
    )
    .into_owned()
}
```

**Yikes**. Dereferencing bytes, collecting into single byte array, making this into `String`. A bit of a mess.

The first implementation change was simple, remove the scariness.

```rust
pub fn construct_text_no_codes(categorised_slices: &CategorisedSlices) -> String {
    categorised_slices.iter().map(|x| x.text).collect()
}
```

A single line of code. Very clean and simple. Benchmarking this gave the result of ~1.0 μs. It still seemed like too much time for something that should be taking nanoseconds. A more heavy handed approach was taken, allocating a `String` with a known size and pushing each slice onto it.

```rust
pub fn construct_text_no_codes(categorised_slices: &CategorisedSlices) -> String {
    let slices = categorised_slices;
    let mut s = String::with_capacity(categorised_slices.iter().map(|x| x.text.len()).sum::<usize>());
    for sl in slices {
        s.push_str(sl.text);
    }

    s
}
```

The benchmark result was ~196 ns, which seemed more likely. So depsite the neatness of iterators, they do have farily significant performance impacts.

## Thoughts

Overall the benchmarking exercise highlighted the need to maintain and spend time benchmarking functions. Code that was written to make a process _work_ may need an eye cast over it to make it _efficient_. Although this crate does not need high performance, dependents might and it was good to see improvements in the refactoring.