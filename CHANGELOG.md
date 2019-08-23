# Change Log

## v2.1.0

- Added `start` and `end` byte positions to `CategorisedSlice`.

## v2.0.0

- Moved `colored` crate out of dependencies and into dev-dependency.
- `CategorisedSlice` has `text` field which is a `&str`.
- `CategorisedSlice` internally uses `const fn`.
- Introduced my own `parse` function, which returns `Vec<Match>`
- Benchmarked and refactored functions to improve performance. [see results](https://github.com/kurtlawrence/cansi/blob/master/notes/Benchmarking.md)

## v1.1.0

- Added `line_iter()` function -- ability to iterate over `CategorisedSlices` as a line iterator.

> v1.1.1
>
> - fixed bug in iterating lines not starting a new line if followed by escape codes

## v1.0.0

- stable release