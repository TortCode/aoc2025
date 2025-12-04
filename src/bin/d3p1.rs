use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("input/d3p1.txt")?;
    let reader = std::io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        let line = line?;
        let bank = line.trim();
        let mut max_joltage = 0;
        for i in 1..bank.len() {
            for j in i+1..bank.len() {
                let first_digit = bank.chars().nth(i).unwrap().to_digit(10).unwrap();
                let second_digit = bank.chars().nth(j).unwrap().to_digit(10).unwrap();
                let joltage = first_digit * 10 + second_digit;
                max_joltage = max_joltage.max(joltage);
            }
        }
        total += max_joltage;
    }

    println!("Total: {}", total);
    Ok(())
}