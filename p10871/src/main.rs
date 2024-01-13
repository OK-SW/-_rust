use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect(" ");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let x: u32 = v.get(1).unwrap().parse().expect(" ");

    let mut input = String::new();
    stdin().read_line(&mut input).expect(" ");
    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    for vv in v {
        match x.cmp(match &vv.trim().parse(){ Ok(t) => t, Err(_) => continue, }){
            Ordering::Greater => print!("{vv} "),
            _ => continue,
        }
    }
}
