
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc088/tasks/abc088_b
fi
cargo build && oj t -c "target/debug/main"
    