use std::cmp::Ordering;
use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let cnt: usize = buffer.trim().parse().clone().unwrap();

    let mut arr:Vec<String> = Vec::with_capacity(cnt);

    for _ in 0..cnt{
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        match arr.contains(&buffer.trim().to_string()) {
            true => continue,
            false => arr.push(buffer.trim().to_string())
        }
    }

    arr.sort_unstable_by(|a, b| match a.len().cmp(&b.len()){
        Ordering::Equal => a.cmp(&b),
        _ => a.len().cmp(&b.len()),
    });


    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for x in arr.iter() {
        writeln!(writer, "{}", x).unwrap();
    }
}