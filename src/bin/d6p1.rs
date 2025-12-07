use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

enum Op {
    Add,
    Multiply,
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d6p1.txt")?;
    let br = BufReader::new(file);
    let lines = br.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let number_matrix = lines[..lines.len() - 1].iter().map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<u64>().expect(&("bad number:".to_string() + n)))
            .collect::<Vec<u64>>()
    }).collect::<Vec<Vec<u64>>>();
    let ops = lines[lines.len() - 1]
        .split_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Multiply,
            _ => panic!("bad op: {}", s),
        })
        .collect::<Vec<Op>>();

    let mut total = 0u64;
    for i in 0..ops.len() {
        let mut temp = match ops[i] {
            Op::Add => 0u64,
            Op::Multiply => 1u64,
        };
        for j in 0..number_matrix.len() {
            match ops[i] {
                Op::Add => {
                    temp += number_matrix[j][i];
                }
                Op::Multiply => {
                    temp *= number_matrix[j][i];
                }
            }
        }
        total += temp;
    }
    println!("{total}");
    Ok(())
}
