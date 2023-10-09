use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        n: u32,
        y: u32,
    }

    for i in 0..n+1 {
        for j in 0..(n-i+1) {
            let k = n-i-j;
            if k*10000+j*5000+i*1000 == y {
                println!("{} {} {}", k, j, i);
                return;
            }
        }
    }
    println!("-1 -1 -1");
}