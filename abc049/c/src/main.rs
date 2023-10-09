use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        s: String,
    }

    let words = vec!["dreamer", "eraser", "dream", "erase"];
    let mut ss = s;
    let mut t = String::new();
    loop {
        let w = words.iter().find(|x| ss.ends_with(*x));
        if w.is_none(){
            break;
        }
        t = w.unwrap().to_string() + t.as_str();
        ss = ss[..ss.len() - w.unwrap().len()].to_string();
    }

    if ss.is_empty() {
        println!("YES");
    }    
    else {
        println!("NO");
    }
}