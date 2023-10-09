
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc083/tasks/abc083_b
fi
cargo build && oj t -c "target/debug/main"
    