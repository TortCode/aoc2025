use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("input/d6p2.txt")?;
    let br = BufReader::new(file);
    let lines = br.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let ops = lines[lines.len() - 1]
        .match_indices(&['+', '*'])
        .collect::<Vec<_>>();

    let mut total = 0u64;
    for i in 0..ops.len() {
        let start = ops[i].0;
        let end = if i + 1 < ops.len() {
            ops[i + 1].0 - 1
        } else {
            lines[0].len() - 1
        };
        let op = ops[i].1.chars().nth(0).unwrap();
        let mut temp = match op {
            '+' => 0u64,
            '*' => 1u64,
            _ => unreachable!(),
        };
        for j in start..=end {
            let mut part = 0u64;
            for k in 0..lines.len() - 1 {
                let c = lines[k].chars().nth(j).unwrap();
                if c != ' ' {
                    let val = c.to_digit(10).unwrap() as u64;
                    part = part * 10 + val;
                }
            }
            if part != 0 {
                match op {
                    '+' => temp += part,
                    '*' => temp *= part,
                    _ => unreachable!(),
                }
            }
        }
        total += temp;
    }
    println!("{total}");
    Ok(())
}
