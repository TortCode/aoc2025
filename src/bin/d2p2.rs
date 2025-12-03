use std::fs;

fn main() -> std::io::Result<()> {

    let input = fs::read_to_string("input/d2p2.txt")?;

    let mut sum = 0u64;
    for range in input.trim().split(",") {
        let (lo, hi) = range.trim().split_once('-').expect("invalid range");
        let lo = lo.parse::<u64>().expect(&("invalid number ".to_string() + lo));
        let hi = hi.parse::<u64>().expect(&("invalid number ".to_string() + hi));

        for n in lo..=hi {
            if is_invalid(&n.to_string()) {
                sum += n;
            }
        }
    }

    println!("{sum}");
    Ok(())
}

fn is_invalid(s: &str) -> bool {
    if s.len() <= 1 {
        return false;
    }
    for n in 1..=s.len()/2 {
        if s.len() % n != 0 {
            continue;
        }
        let pat = &s[..n];
        let mut bad = true;
        for i in (0..s.len()).step_by(n) {
            if pat != &s[i..i + n] {
                bad = false;
                break;
            }
        }
        if bad {
            return true;
        }
    }
    false
}
