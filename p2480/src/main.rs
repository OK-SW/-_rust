use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let d1: u32 = v.get(0).clone().unwrap().parse().expect(" ");
    let d2: u32 = v.get(1).clone().unwrap().parse().expect(" ");
    let d3: u32 = v.get(2).clone().unwrap().parse().expect(" ");

    if d1 == d2 && d2 == d3 {
        println!("{}", 10000 + d1 * 1000);
    } else if d1 == d2 {
        print!("{}", 1000 + d1 * 100);
    } else if d2 == d3 {
        print!("{}", 1000 + d2 * 100);
    } else if d1 == d3 {
        print!("{}", 1000 + d3 * 100);
    } else {
        if d1 > d2 {
            if d1 > d3 {
                println!("{}", d1 * 100)
            } else {
                println!("{}", d3 * 100)
            }
        } else {
            if d2 > d3 {
                println!("{}", d2 * 100)
            } else {
                println!("{}", d3 * 100)
            }
        }
    }
}
