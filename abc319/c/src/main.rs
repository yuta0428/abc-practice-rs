use std::collections::HashSet;

use proconio::input;
use itertools::Itertools;

/* Answer: https://atcoder.jp/contests/abc319/submissions/45440550
use itertools::Itertools;
use proconio::input;

const LINES: [[usize; 3]; 8] = [
    [0, 1, 2],
    [3, 4, 5],
    [6, 7, 8],
    [0, 3, 6],
    [1, 4, 7],
    [2, 5, 8],
    [0, 4, 8],
    [2, 4, 6],
];

fn main() {
    input! {
        cc: [usize; 9],
    }

    let is_ok = |orders: &[usize]| {
        for line in LINES {
            if orders
                .iter()
                .filter(|&&order| line.contains(&order))
                .map(|&order| cc[order])
                .take(2)
                .all_equal()
            {
                return false;
            }
        }

        true
    };

    let cnt_ok = (0..9)
        .permutations(9)
        .filter(|orders| is_ok(orders))
        .count();
    println!("{}", cnt_ok as f64 / 362880.0);
}

 */

// 解けなかった。縦横斜めのラインの中で順に選択した際に最初2つに重複があるかをみればよかっただけ。
fn main() {
    input! {
        c: [[u32; 3]; 3],
    }

    let total: u32 = (1..10).product(); // 9!

    let i_vec = (0..9).permutations(6).collect_vec(); // 9P6

    let vec: Vec<&u32> = c.iter().flatten().collect();
    let value = i_vec.clone().into_iter()
        .filter(|iv| {
            let mut a: [u32;9] =[0; 9];
            for i in iv {
                a[*i as usize] = *vec[*i as usize];
            }
            let matrix: Vec<Vec<u32>> = a.chunks(3).map(|chunk| chunk.to_vec()).collect();
            for row in matrix.iter() {
                if filterdup(row) {
                    return true;
                }
            }
            for col in 0..3 {
                let vec = matrix.iter().map(|row| row[col]).collect_vec();
                if filterdup(&vec) {
                    return true;
                }
            }
            {
                let vec = (0..3).map(|i| matrix[i][i]).collect_vec();
                if filterdup(&vec) {
                    return true;
                }
            }
            {
                let vec = (0..3).map(|i| matrix[2-i][i]).collect_vec();
                if filterdup(&vec) {
                    return true;
                }
            }
            return false;
        })
        .collect_vec()
        .len();

    println!("value: {value}, total: {total}");
    println!("{}", value as f32 / total as f32);
    
}

fn filterdup(vec: &Vec<u32>) -> bool {
    let mut seen = HashSet::new();
    if !vec.iter().filter(|v| **v != 0).all(|item| seen.insert(item)) {
        return true;
    }
    return false;
}