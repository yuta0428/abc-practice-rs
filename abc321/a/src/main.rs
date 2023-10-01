use proconio::input;
use itertools::Itertools;

/* Answer: https://atcoder.jp/contests/abc321/submissions/46121461
use itertools::Itertools;
use proconio::input;

fn main() {
    input! { n: String }
    if n.chars().tuple_windows().all(|(x, y)| x > y) {
        println!("Yes")
    } else {
        println!("No")
    }
}
 */

fn main() {
    input! {
        n: i32,
    }

    let origin = n.to_string().chars()
        .map(|c| c.to_string().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    let x = origin.iter().unique().sorted().rev();

    if itertools::equal(&origin, x) { println!("Yes") }
    else { println!("No") }
}