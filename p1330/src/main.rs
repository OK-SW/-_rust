use std::cmp::Ordering;
use std::io::{Read, stdin};

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect("Failed");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let a = v.get(0).clone();
    let aa: i32 = a.unwrap().parse().expect("11");

    let b = v.get(1).clone();
    let bb: i32 = b.unwrap().parse().expect("22");

    match aa.cmp(&bb) {
        Ordering::Less => println!("<"),
        Ordering::Equal => println!("=="),
        Ordering::Greater => println!(">"),
    }
}
