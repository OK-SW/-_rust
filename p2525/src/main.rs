use std::io::stdin;

fn main(){
    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let v: Vec<&str> = input.split_ascii_whitespace().collect();

    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let time: u32 = input.trim().parse().expect(" ");

    let hour: u32 = v.get(0).clone().unwrap().parse().expect(" ");
    let min: u32 = v.get(1).clone().unwrap().parse().expect(" ");

    let pmin:u32 = min + time % 60;
    let phour: u32 = hour + time / 60 + pmin / 60;
    println!("{} {}", phour % 24, pmin % 60);
}
