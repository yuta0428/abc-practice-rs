
if [ ! -e ./test ]; then
    oj d https://atcoder.jp/contests/abc081/tasks/abc081_a
fi
cargo build && oj t -c "target/debug/main"
    