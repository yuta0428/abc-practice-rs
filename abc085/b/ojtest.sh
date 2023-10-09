
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc085/tasks/abc085_b
fi
cargo build && oj t -c "target/debug/main"
    