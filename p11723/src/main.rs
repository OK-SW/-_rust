use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};
use std::collections::HashSet;

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();
    let mut x;
    let mut y: i32;

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    let mut set: HashSet<i32> = HashSet::with_capacity(20);

    for _ in 0..n{
        buffer.clear();

        reader.read_line(&mut buffer).unwrap();

        let mut vv = buffer.split_ascii_whitespace();

        x = vv.next().unwrap();

        match x {
            "add" => {
                y = vv.next().unwrap().parse().unwrap();
                set.insert(y);
            },
            "remove" => {
                y = vv.next().unwrap().parse().unwrap();
                set.remove(&y);
            },
            "check" => {
                y = vv.next().unwrap().parse().unwrap();
                if set.contains(&y){
                    writeln!(writer, "{}", 1).unwrap();
                } else {
                    writeln!(writer, "{}", 0).unwrap();
                }
            }
            "toggle" => {
                y = vv.next().unwrap().parse().unwrap();
                if set.contains(&y){
                    set.remove(&y);
                } else {
                    set.insert(y);
                }
            }
            "all" => {
                for i in 1..21 {
                    if !set.contains(&i){
                        set.insert(i);
                    }
                }
            }
            "empty" => {
                set.clear();
            }
            _ => continue
        }
    }
}
