use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        s: i32,
    }
    
    let mut a: i32 = 0;
    for i in 0..3 {
        if s >> i & 1 == 1 {
            a += 1;
        }
    }
    

    println!("{a}");
}