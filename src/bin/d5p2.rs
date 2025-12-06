use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Copy, Clone)]
struct FreshRange {
    start: u64,
    end: u64,
}

impl FreshRange {
    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }

    fn merge(&self, other: &FreshRange) -> Option<FreshRange> {
        if other.start > 1 + self.end || self.start > 1 + other.end {
            None
        } else {
            Some(FreshRange {
                start: self.start.min(other.start),
                end: self.end.max(other.end),
            })
        }
    }
}

fn merge_ranges(ranges: &mut Vec<FreshRange>) -> Vec<FreshRange> {
    ranges.sort_by_key(|range| range.start);
    let mut merged = Vec::new();
    let mut current: Option<FreshRange> = None;
    for range in ranges.iter() {
        if let Some(curr) = current {
            if let Some(merger) = curr.merge(range) {
                current = Some(merger);
            } else {
                merged.push(curr);
                current = Some(*range);
            }
        } else {
            current = Some(*range);
        }
    }
    if let Some(curr) = current {
        merged.push(curr);
    }
    merged
}

fn main() -> std::io::Result<()> {
    let mut ranges = Vec::<FreshRange>::new();

    let file = File::open("input/d5p2.txt")?;
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
        ranges.push(FreshRange { start, end });
    }

    let merged = merge_ranges(&mut ranges);
    let mut fresh_count = 0u64;
    for range in merged.iter() {
        fresh_count += range.len() as u64;
    }
    println!("{}", fresh_count);
    Ok(())
}