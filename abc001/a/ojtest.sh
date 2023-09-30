
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc001/tasks/abc001_1
fi
cargo build && oj t -c "target/debug/main"
    