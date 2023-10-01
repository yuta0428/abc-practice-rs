
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc319/tasks/abc319_b
fi
cargo build && oj t -c "target/debug/main"
    