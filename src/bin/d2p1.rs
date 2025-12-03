use std::fs;

fn main() -> std::io::Result<()> {

    let input = fs::read_to_string("input/d2p1.txt")?;

    let mut sum = 0u64;
    for range in input.trim().split(",") {
        let (lo, hi) = range.split_once('-').expect("invalid range");
        let lo = lo.parse::<u64>().expect("invalid number");
        let hi = hi.parse::<u64>().expect("invalid number");

        for n in lo..=hi {
            let s = n.to_string();
            if s.len() % 2 != 0 {
                continue;
            }
            let (left, right) = s.split_at(s.len() / 2);
            if left == right {
                sum += n;
            }
        }
    }

    println!("{sum}");
    Ok(())
}
