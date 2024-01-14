use std::cmp::Ordering;
use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let n: usize = buffer.trim().parse().unwrap();

    let mut v: Vec<Coordinate> = Vec::with_capacity(n);

    for _ in 0..n {
        buffer.clear();

        reader.read_line(&mut buffer).unwrap();

        let mut iter = buffer.split_ascii_whitespace();

        v.push(
            build_coor(
                iter.next().unwrap(),
                iter.next().unwrap()
            )
        );
    }

    v.sort_by(|a, b| match a.x.cmp(&b.x){
        Ordering::Equal => a.y.cmp(&b.y),
        _ => a.x.cmp(&b.x),
    });

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for vv in v.iter() {
        writeln!(writer, "{} {}", vv.x, vv.y).unwrap()
    }
}

struct Coordinate {
    x: i32,
    y: i32,
}

fn build_coor(x: &str, y: &str) -> Coordinate {
    Coordinate{
        x: x.parse().expect(" "),
        y: y.parse().expect(" ")
    }
}