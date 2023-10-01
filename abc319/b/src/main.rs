use proconio::input;

fn main() {
    input! {
        n: u32,
    }

    let jv = (1..10)
        .filter(|j| n % j == 0)
        .collect::<Vec<u32>>();

    for i in 0..(n + 1) {
        let s = jv
            .clone()
            .into_iter()
            .filter(|j| i % (n / j) == 0)
            .min();
        match s {
            Some(s) => print!("{s}"),
            _ => print!("-")
        }
    }
    println!();
}