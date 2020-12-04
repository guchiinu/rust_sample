use std::fs;

fn main() {
    read_file();
}

fn read_file() {
    let text = fs::read_to_string("resource/sample.txt").unwrap();

    println!("{}", text);
    println!("{}", text.chars().rev().collect::<String>());
}