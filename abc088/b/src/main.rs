use proconio::input;
use itertools::Itertools;

fn main() {
    input! {
        n: u32,
        a: [u32; n]
    }

    let mut alice = 0; 
    let mut bob = 0; 
    for (i, aa) in a.into_iter().sorted().rev().enumerate() {
        if i & 1 == 0 {
            alice += aa;
        }
        else {
            bob += aa;
        }
    }

    println!("{}", alice - bob);
}