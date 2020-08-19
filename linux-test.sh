set -e	# stop on failure
cargo +stable fmt -- --check
cargo clippy
cargo test
echo "PASSED"
