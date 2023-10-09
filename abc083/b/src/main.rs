use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        n: u32,
        a: u32,
        b: u32,
    }

    let mut s = 0;
    for nn in 1..n+1 {
        let sum: u32 = nn.to_string().chars()
                .map(|x| x.to_digit(10).unwrap())
                .sum();

        if a <= sum && sum <= b {
            s += nn;
        }
    }
    
    println!("{}", s);
}