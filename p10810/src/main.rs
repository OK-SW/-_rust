use std::collections::HashMap;
use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let n: u32 = v.get(0).unwrap().parse().expect(" ");
    let m: u32 = v.get(1).unwrap().parse().expect(" ");

    let mut map = HashMap::new();

    for i in 1..(n+1) {
        map.insert(i, 0);
    }

    for _o in 0..m {
        let mut input = String::new();

        stdin().read_line(&mut input).expect("");

        let v: Vec<&str> = input.split_ascii_whitespace().collect();

        let i: u32 = v.get(0).unwrap().parse().expect("");
        let j: u32 = v.get(1).unwrap().parse().expect("");
        let k: u32 = v.get(2).unwrap().parse().expect("");

        for u in i..j+1{
            map.insert(u, k);
        }
    }

    for i in 0..n+1{
        match map.get(&(i + 1)) {
            Some(t) => print!("{t} "),
            None => continue,
        }
    }
}
