use proconio::input;
// use itertools::Itertools;

fn main() {
    input! {
        s: String,
    }

    let n = match s.as_str() {
        "tourist" => 3858,
        "ksun48" => 3679,
        "Benq" => 3658,
        "Um_nik" => 3648,
        "apiad" => 3638,
        "Stonefeang" => 3630,
        "ecnerwala" => 3613,
        "mnbvmar" => 3555,
        "newbiedmy" => 3516,
        "semiexp" => 3481,
        _ => 0
    };
    
    println!("{}", n);
}