# Publishing Procedure

1. `cargo my-readme` - updates the readmes on the `lib.rs` and `main.rs`
2. `cargo update` - updates the compatible versions
3. `cargo outdated` - check outdated versions, update to latest major if possible
4. `cargo test`
5. Increment version number
6. `cargo publish`