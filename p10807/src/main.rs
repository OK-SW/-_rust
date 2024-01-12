use std::io::stdin;
use std::collections::HashMap;

fn main() {
    let mut input = String::new();

    stdin().read_line(&mut input).unwrap();

    let mut inp = String::new();

    stdin().read_line(&mut inp).unwrap();

    let v: Vec<&str> = inp.split_ascii_whitespace().collect();

    let mut tofind = String::new();

    stdin().read_line(&mut tofind).unwrap();

    tofind = tofind.trim().to_string();

    let mut check = HashMap::new();

    for vv in v {
        let count = check.entry(vv.to_string()).or_insert(0);
        *count += 1;
    }

    match check.get(&tofind) {
        Some(t) => println!("{t}"),
        None => println!("0"),
    }
}
