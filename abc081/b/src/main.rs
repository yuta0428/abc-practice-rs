use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: u32,
        mut a: [u32; n]
    }

    let mut s = 0;
    while a.iter().all(|x| x & 1 == 0) {
        a = a.iter().map(|x| x / 2).collect_vec();
        s += 1;
    }

    println!("{s}");
}