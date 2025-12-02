use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/d1p2.txt")?;
    let br = BufReader::new(file);

    const MOD: i32 = 100;
    let mut zero_count: u32 = 0;
    let mut dial_pos: i32 = 50;

    for line in br.lines() {
        let line = line?;
        let shift = line[1..].parse::<u32>().expect("Could not parse number") as i32;
        let rem_shift = shift % MOD;

        let movement = match line.as_bytes()[0] as char {
            'L' => -rem_shift,
            'R' => rem_shift,
            _ => panic!("Invalid direction character"),
        };
        let new_dial_pos = (dial_pos + movement + MOD) % MOD;

        zero_count += (shift / MOD) as u32;
        if dial_pos != 0 {
            if new_dial_pos == 0 {
                zero_count += 1;
            } else if movement > 0 && new_dial_pos < dial_pos {
                zero_count += 1;
            } else if movement < 0 && new_dial_pos > dial_pos {
                zero_count += 1;
            }
        }
        dial_pos = new_dial_pos;
    }
    println!("{zero_count}");
    Ok(())
}
