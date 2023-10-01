use proconio::input;
use itertools::Itertools;

/* Answer https://atcoder.jp/contests/abc321/submissions/45834352
fn main() {
    input! {
        n: usize,
        x: i32,
        mut a: [i32; n-1],
    }
    a.sort();
    let mut s = 0;
    for i in 1..n-2 {s+=a[i];}
    let mut t = x-s;
    if t > a[n-2] {t = -1;}
    else if t <= a[0] {t = 0;}
    println!("{}", t);
}
 */

fn main() {
    input! {
        n: usize,
        x: i32,
        a: [i32; n-1]
    }

    let aa = a.into_iter()
        .sorted()
        .collect_vec();

    let sum: i32 = aa
        .get(1..(n-2) as usize).unwrap()
        .iter()
        .sum();
   
    let mut y = x - sum;

    if y > aa[n-2] {
        y = -1;
    }
    else if y <= aa[0] {
        y = 0;
    }  
    println!("{y}");

}
