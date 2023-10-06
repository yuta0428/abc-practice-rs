
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc319/tasks/abc319_c
fi
cargo build && oj t -c "target/debug/main"
    