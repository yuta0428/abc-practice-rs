
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc049/tasks/arc065_a
fi
cargo build && oj t -c "target/debug/main"
    