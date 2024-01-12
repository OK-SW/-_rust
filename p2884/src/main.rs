use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let hour: u8 = v.get(0).clone().unwrap().parse().expect(" ");
    let min: u8 = v.get(1).clone().unwrap().parse().expect(" ");

    if hour > 0 {
        if min > 44 {
            println!("{hour} {}", min - 45)
        } else {
            println!("{} {}", hour - 1, 60 - (45 - min))
        }
    } else {
        if min > 44 {
            println!("{} {}", hour, min - 45)
        } else {
            println!("{} {}", 23 , 60 - (45 - min));
        }
    }
}