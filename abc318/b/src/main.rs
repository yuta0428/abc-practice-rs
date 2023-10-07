use itertools::Itertools;
use proconio::input;
// use itertools::Itertools;

/* Answer: https://atcoder.jp/contests/abc318/submissions/45134454
#![allow(dead_code)]
fn main() {
    proconio::input! {
        n: usize,
        s: [(usize, usize, usize, usize); n]
    }
    let mut r = [0; 10000];
    for (a, b, c, d) in s.into_iter() {
        for i in a..b {
            for j in c..d {
                r[i * 100 + j] = 1;
            }
        }
    }
    println!("{}", r.into_iter().sum::<i64>())
}
 */

fn main() {
    input! {
        n: u32,
        e: [[u32; 4]; n],
    }
    
    let mut ans = [[0; 101]; 101];
    for ee in e {
        let (x1,x2, y1, y2) = (ee[0], ee[1], ee[2], ee[3]);
        for y in y1..y2{
            for x in x1..x2{
                ans[y as usize][x as usize] = 1;
            } 
        }
    }

    let s = ans.iter()
        .flatten()
        .filter(|b| **b == 1)
        .collect_vec()
        .len();

    println!("{}", s);

}