
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc087/tasks/abc087_b
fi
cargo build && oj t -c "target/debug/main"
    