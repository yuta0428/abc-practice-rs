
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc321/tasks/abc321_b
fi
cargo build && oj t -c "target/debug/main"
    