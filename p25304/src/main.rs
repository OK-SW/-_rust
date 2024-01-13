use std::io::stdin;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect(" ");
    let mut x: i32 = input.trim().parse().expect(" ");

    let mut n = String::new();
    stdin().read_line(&mut n).expect(" ");
    let n: u32 = n.trim().parse().expect(" ");

    for i in 0..n{
        let mut input = String::new();
        stdin().read_line(&mut input).expect(" ");
        let v: Vec<&str> = input.split_ascii_whitespace().collect();

        let a: i32 = v.get(0).unwrap().parse().expect(" ");
        let b: i32 = v.get(1).unwrap().parse().expect("");

        x -= a * b;
    }
    if x == 0 {
        println!("Yes")
    } else {
        println!("No")
    }
}
