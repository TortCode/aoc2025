use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tile {
    x: u64,
    y: u64,
}

fn area(u: &Tile, v: &Tile) -> u64 {
    let min_x = u.x.min(v.x);
    let max_x = u.x.max(v.x);
    let min_y = u.y.min(v.y);
    let max_y = u.y.max(v.y);
    (max_x - min_x + 1) * (max_y - min_y + 1)
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d9p1.txt")?;
    let reader = BufReader::new(file);
    let mut tiles: Vec<Tile> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts = line
            .split(',')
            .map(|p| p.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        tiles.push(Tile {
            x: parts[0],
            y: parts[1],
        });
    }

    let mut max_area = 0u64;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            max_area = max_area.max(area(&tiles[i], &tiles[j]));
        }
    }

    println!("{}", max_area);

    Ok(())
}
