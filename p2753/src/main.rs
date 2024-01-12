use std::io::stdin;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).expect(" ");

    let year: u32 = input.trim().parse().expect(" ");

    if ((year % 4 == 0) && (year % 100 != 0)) || year % 400 == 0 {
        println!("1")
    } else {
        print!("0");
    }
}
