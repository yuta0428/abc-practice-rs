use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        d: [u32; n]
    }

    let s = d.into_iter().sorted().unique().count();
    
    println!("{}", s);
}