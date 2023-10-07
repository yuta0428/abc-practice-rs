use proconio::input;
use itertools::Itertools;

/* Answer: https://atcoder.jp/contests/abc318/submissions/45143690
use proconio::*;

fn main() {
    input!{ n: usize, d: usize, p: u64, mut fs: [u64; n] };

    let mut ans = 0;
    fs.sort_by(|a, b| b.cmp(&a));
    for chunk in fs.chunks(d) {
        let normal = chunk.iter().sum::<u64>();
        ans += normal.min(p);
    }
    println!("{ans}");
}
*/
fn main() {
    input! {
        n: u64,
        d: u64,
        p: u64,
        f: [u64; n],
    }

    let mut s: u64 = 0;
    // 金額降順ソート
    let ff = f.into_iter().sorted().rev().collect_vec();
    for dd in ff.chunks (d as usize) {
        let sum: u64 = dd.iter().sum1().unwrap();
        if sum > p {
            s += p;
        }
        else {
            s += sum;
        }
    }
    println!("{}", s);
}