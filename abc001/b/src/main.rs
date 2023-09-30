use proconio::input;

fn main() {
    input! {
        m: u32,
    }

    let vv = match m {
        i if i < 100 => 0,
        i if i <= 5000 => i * 10 / 1000,
        i if i <= 30000 => i / 1000 + 50,
        i if i <= 70000 => (i / 1000 - 30) / 5 + 80,
        _ => 89
    };
    println!("{vv:02}");
}