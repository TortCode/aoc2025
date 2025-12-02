use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/d1p1.txt")?;
    let br = BufReader::new(file);

    const MOD: i32 = 100;
    let mut zero_count: u32 = 0;
    let mut dial_pos: i32 = 50;

    for line in br.lines() {
        let line = line?;
        let shift = line[1..].parse::<i32>().expect("Could not parse number");
        match line.as_bytes()[0] as char {
            'L' => {
                dial_pos -= shift;
                dial_pos %= MOD;
                if dial_pos < 0 {
                    dial_pos += MOD;
                }
            }
            'R' => {
                dial_pos += shift;
                dial_pos %= MOD;
            }
            _ => {}
        }
        if dial_pos == 0 {
            zero_count += 1;
        }
    }
    println!("{zero_count}");
    Ok(())
}