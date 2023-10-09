use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        n: usize,
        txy: [(i32, i32, i32); n]
    }
    
    let (mut ct, mut cx, mut cy) = (0,0,0);

    for i in 0..n {
        let (t1, x1, y1) = txy[i];
        let t = (t1 - ct).abs();
        let x = (x1 - cx).abs();
        let y = (y1 - cy).abs();

        let tt: i32 = t - x - y;
        if tt < 0 || tt & 1 == 1 {
            println!("No");
            return;
        }
        (ct, cx, cy) = (t1, x1, y1);
    } 

    println!("Yes");
}