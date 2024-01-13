use std::io::stdin;

fn main() {
    let mut arr = [false; 30];

    for _i in 0..28 {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("");
        let index:usize = input.trim().parse().expect("");

        arr[index-1] = true;
    }

    for i in 0..30 {
        if !arr[i] {
            println!("{}", i + 1)
        }
    }
}
