use proconio::input;

fn main() {
    input! {
        n: u32,
        s: [String; n],
    }

    let mut min = [false; 24 * 60 + 2];
    for ss  in s.iter() {
        let vv = ss.split("-")
            .map(|c| c.parse::<u32>().unwrap())
            .map(|i| {
                let hh = i / 100 * 60;
                let mm = i % 100;
                hh + mm
            })
            .collect::<Vec<u32>>();

        let a = vv[0] / 5 * 5;
        let b = (vv[1] + 4) / 5 * 5;
        for i in a..(b + 1) {
            min[i as usize] = true;
        }
    }

    let mut flag = false;
    let mut ii = 0;
    for (i, h) in min.iter().enumerate(){
        if !flag && *h {
            ii = i;
        }
        if flag && !h {
            println!("{:02}{:02}-{:02}{:02}", ii / 60, ii % 60, (i-1) / 60, (i-1) % 60);
        }
        flag = *h;
    }
}