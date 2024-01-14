use std::io::{BufRead, BufReader, BufWriter, stdin, stdout, Write};

fn main() {
    let stdin = stdin();
    let mut reader = BufReader::new(stdin.lock());

    let mut buffer = String::new();
    reader.read_line(&mut buffer).unwrap();
    let cnt: usize = buffer.trim().parse().unwrap();

    let mut arr: Vec<i32> = Vec::with_capacity(cnt);

    for _ in 0..cnt{
        buffer.clear();
        reader.read_line(&mut buffer).unwrap();
        arr.push(buffer.trim().parse().unwrap());
    }

    arr.sort_unstable();

    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for x in arr.iter(){
        writeln!(writer, "{}", x).unwrap();
    }
}
