
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc001/tasks/abc001_4
fi
cargo build && oj t -c "target/debug/main"
    