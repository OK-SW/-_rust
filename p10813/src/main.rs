use std::io::stdin;
use std::mem::swap;

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).expect("");
    let v:Vec<&str> = input.split_ascii_whitespace().collect();

    let n: usize = v.get(0).unwrap().parse().expect("");
    let m: u32 = v.get(1).unwrap().parse().expect("");

    let mut arr = vec![0; n];

    for i in 0..arr.len(){
        arr[i] = i + 1;
    }

    for k in 0..m{
        let mut input = String::new();
        stdin().read_line(&mut input).expect("");
        let v: Vec<&str> = input.split_ascii_whitespace().collect();

        let i:usize = v.get(0).unwrap().parse().expect("");
        let j:usize = v.get(1).unwrap().parse().expect("");

        let mut x = arr[i-1].clone();
        let mut y = arr[j-1].clone();

        swap(&mut arr[i-1], &mut y);
        swap(&mut arr[j-1], &mut x);
    }

    for k in arr {
        print!("{k} ");
    }
}
