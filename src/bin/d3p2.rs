use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("input/d3p2.txt")?;
    let reader = std::io::BufReader::new(file);
    let mut total = 0u64;

    for line in reader.lines() {
        let line = line?;
        let bank = line.trim().as_bytes();
        let mut joltage = 0u64;
        let mut best_idx = 0;
        for i in (0..12).rev() {
            for j in best_idx..bank.len() - i {
                if bank[j] > bank[best_idx] {
                    best_idx = j;
                }
            }
            joltage = joltage * 10 + (bank[best_idx] - b'0') as u64;
            best_idx += 1;
        }
        total += joltage;
    }

    println!("{}", total);
    Ok(())
}
