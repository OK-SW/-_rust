use std::io::stdin;

fn main() {
    let mut sum:f32 = 0.0;
    let mut total:f32 = 0.0;
    for _i in 0..20 {
        let mut input = String::new();
        stdin().read_line(&mut input).expect("");
        let v: Vec<&str> = input.split_ascii_whitespace().collect();

        let credit: f32 = v.get(1).unwrap().parse().expect("");
        let grade: String = v.get(2).clone().unwrap().to_string();
        
        let convert = match grade.as_str() {
            "A+" => 4.5,
            "A0" => 4.0,
            "B+" => 3.5,
            "B0" => 3.0,
            "C+" => 2.5,
            "C0" => 2.0,
            "D+" => 1.5,
            "D0" => 1.0,
            "F" => 0.0,
            _ => {
                continue
            }
        };

        sum += credit * convert;
        total += credit;
    }

    if total == 0.0 {
        println!("0.000000")
    } else {
        println!("{}", sum / total);
    }
}
