use std::io::stdin;

fn main() {
    let mut x = String::new();
    stdin().read_line(&mut x).expect(" ");
    let x: i32 = x.trim().parse().expect(" ");

    let mut y = String::new();
    stdin().read_line(&mut y).expect(" ");
    let y: i32 = y.trim().parse().expect(" ");

    if x > 0 {
        if y > 0 {
            println!("1");
        } else {
            println!("4");
        }
    } else {
        if y > 0{
            println!("2")
        } else {
            println!("3");
        }
    }
}
