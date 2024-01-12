use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect(" ");

    let grade: i32 = input.trim().parse().expect(" ");

    if grade > 89 {
        println!("A")
    } else if grade > 79 {
        println!("B")
    } else if grade > 69 {
        println!("C")
    } else if grade > 59 {
        println!("D")
    } else {
        println!("F")
    }
}
