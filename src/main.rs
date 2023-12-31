use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::process::Command;
use proconio::input;
// use itertools::Itertools;

use proconio::marker::Chars;
use string_template::Template;

fn main() {
    setup();
}

fn setup() {
    println!("Please Contest Number>> ");
    input! {
        n: u32, // コンテスト番号 001
    }
    println!("Please Problem Tasks>> ");
    input! {
        c: Chars, // 問題 a~f
    }
    let dir_path = format!("./abc{:03}", n);

    // make directories
    exec(&format!("mkdir -p {dir_path}"));

    let _dir_abcd = if n < 126 { "abcd" } else { "abcdef" };
    for (i, dir) in _dir_abcd.chars().enumerate() {
        if !c.contains(&dir) {continue}

        // make directories
        exec(&format!("rm -r {dir_path}/{dir}"));
        let cmd1 = format!("cargo generate --name {} --path ./template", dir);
        exec(&cmd1);

        // execute oj d
        let atcoder_url = if n < 20 {
            format!("https://atcoder.jp/contests/abc{:03}/tasks/abc{:03}_{}", n, n, i + 1)
        } else {
            format!("https://atcoder.jp/contests/abc{:03}/tasks/abc{:03}_{}", n, n, dir)
        };
        add_ojtest(&dir, &atcoder_url);
        add_ojsubmit(&dir, &atcoder_url);

        // mv
        let cmd3 = format!("mv ./{} {}", dir, dir_path);
        exec(&cmd3);
    }
}

fn exec(cmd: &str) {
    println!("{}", cmd);
    Command::new("sh")
        .arg("-c")
        .arg(&cmd)
        .output()
        .expect("Failed to execute command");
}

fn write_file(file_path: &str, contents: &str, args: &HashMap<&str, &str>) {
    let template = Template::new(contents);
    let result = template.render(&args);

    let mut file = File::create(file_path).unwrap();
    file.write_all(result.as_bytes()).unwrap();
}

fn add_ojtest(dir: &char, atcoder_url: &str) {
    let file_path = format!("{dir}/ojtest.sh");
    let contents = r#"
if [ ! -e ./test ]; then
    oj d {{atcoder_url}}
fi
cargo build && oj t -c "target/debug/main"
    "#;
    let mut args = HashMap::new();
    args.insert("atcoder_url", atcoder_url);
    write_file(&file_path, &contents, &args)
}

fn add_ojsubmit(dir: &char, atcoder_url: &str) {
    let file_path = format!("{dir}/ojsubmit.sh");
    let contents = r#"
oj s {{atcoder_url}} src/main.rs
    "#;
    let mut args = HashMap::new();
    args.insert("atcoder_url", atcoder_url);
    write_file(&file_path, &contents, &args)
}
