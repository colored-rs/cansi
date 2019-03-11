set -e	# stop on failure

git pull  # maybe git reset --hard master if Cargo.lock playing up
cargo test
cargo tarpaulin -v -l --out Html
mv tarpaulin-report.html /mnt/c/users/kurt/desktop/tarpaulin-report.html

wdir="$(pwd)" # get working directory
cd /mnt/c/users/kurt/desktop # switch to windows desktop
./tarpaulin-html-converter.exe # run the converter
cd "$wdir" # switch back to old dir
echo "report converted!"