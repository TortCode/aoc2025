use std::fs::File;
use std::io::{BufRead, BufReader};

struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    fn contains(&self, id: u64) -> bool {
        id >= self.start && id < self.end
    }
}


struct DB {
    ranges: Vec<FreshRange>
}

impl DB {
    fn new() -> Self {
        DB {
            ranges: Vec::new()
        }
    }

    fn add_range(&mut self, range: FreshRange) {
        self.ranges.push(range);
    }

    fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|r| r.contains(id))
    }
}

fn main() -> std::io::Result<()> {
    let mut db = DB::new();

    let file = File::open("input/d5p1.txt")?;
    let br = BufReader::new(file);
    let mut lines_iter = br.lines().map(|l| l.unwrap());
    while let Some(line) = lines_iter.next() {
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        let (raw_start, raw_end) = line.split_once("-").unwrap();
        let start = raw_start.parse::<u64>().expect(&("bad start:".to_string() + raw_start));
        let end = raw_end.parse::<u64>().expect(&("bad end:".to_string() + raw_end));
        db.add_range(FreshRange { start, end });
    }

    let mut fresh_count = 0u32;
    while let Some(line) = lines_iter.next() {
        let id = line.trim().parse::<u64>().unwrap();
        if db.is_fresh(id) {
            fresh_count += 1;
        }
    }
    println!("{}", fresh_count);
    Ok(())
}