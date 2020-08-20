set -e	# stop on failure
cargo +stable fmt -- --check
cargo clippy -- -D warnings
cargo test
echo "PASSED"
