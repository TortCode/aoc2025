use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Machine {
    indicator: u32,
    buttons: Vec<u32>,
}

impl Machine {
    fn new(indicator: u32, buttons: Vec<u32>) -> Self {
        Machine { indicator, buttons }
    }

    fn min_presses(&self) -> u8 {
        if self.indicator == 0 {
            return 0;
        }
        for i in 1..=self.buttons.len() {
            for combo in self.buttons.iter().combinations(i) {
                if combo
                    .iter()
                    .map(|x| (*x).clone())
                    .reduce(|x, y| x ^ y)
                    .unwrap()
                    == self.indicator
                {
                    return i as u8;
                }
            }
        }
        u8::MAX
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d10p1.txt")?;
    let reader = BufReader::new(file);
    let mut machines: Vec<Machine> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts = line.split(' ').collect::<Vec<_>>();
        let raw_indicator = &parts[0][1..parts[0].len() - 1];
        let mut indicator = 0u32;
        for c in raw_indicator.chars().rev() {
            indicator <<= 1;
            if c == '#' {
                indicator |= 1;
            }
        }

        let buttons = parts[1..parts.len() - 1].iter()
            .map(|s| {
                let raw_button = s[1..s.len() - 1]
                    .split(',')
                    .map(|t| t.parse::<u8>().unwrap())
                    .collect::<Vec<_>>();
                let mut button = 0u32;
                for n in raw_button {
                    button |= 1 << n;
                }
                button
            })
            .collect::<Vec<_>>();
        machines.push(Machine::new(indicator, buttons));
    }
    let total_presses: u32 = machines.iter().map(|m| m.min_presses() as u32).sum();
    println!("{}", total_presses);
    Ok(())
}
