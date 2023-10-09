use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        a: u32,
        b: u32,
        c: u32,
        x: u32,
    }

    let mut s = 0;
    for aa in 0..a+1 {
        for bb in 0..b+1 {
            for cc in 0..c+1 {
                if aa * 500 + bb * 100 + cc * 50 == x {
                    s += 1;
                }
            }
        }
    }
    
    println!("{}", s);
}