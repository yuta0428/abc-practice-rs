use proconio::input;
// use itertools::Itertools;

/* Answer: https://atcoder.jp/contests/abc318/submissions/45229061
use proconio::input;
fn main() {
  input!{n:i32,m:i32,p:i32}
  println!("{}",(n-m+p)/p)
}
 */

fn main() {
    input! {
        n: i32,
        m: i32,
        p: i32,
    }
    let a: i32 = n - m;
    if a < 0 {
        println!("0");
    }
    else {
        println!("{}", a / p + 1);
    }
}