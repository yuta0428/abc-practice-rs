use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    
    if (a * b) & 1 == 0 {
        println!("Even");
    }
    else {
        println!("Odd");
    }
}