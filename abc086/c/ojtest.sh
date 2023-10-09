
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc086/tasks/arc089_a
fi
cargo build && oj t -c "target/debug/main"
    