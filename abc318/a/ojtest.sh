
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc318/tasks/abc318_a
fi
cargo build && oj t -c "target/debug/main"
    