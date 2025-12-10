use std::fs::File;
use std::io::{BufRead, BufReader};

struct Tile {
    x: u64,
    y: u64,
}

struct Rectangle {
    top: u64,
    bottom: u64,
    left: u64,
    right: u64,
}

impl Rectangle {
    fn new(u: &Tile, v: &Tile) -> Self {
        let top = u.y.min(v.y);
        let bottom = u.y.max(v.y);
        let left = u.x.min(v.x);
        let right = u.x.max(v.x);
        Rectangle {
            top,
            bottom,
            left,
            right,
        }
    }

    fn overlap_interior(&self, rect: &Rectangle) -> bool {
        if rect.top >= self.bottom || rect.bottom <= self.top {
            false
        } else if rect.left >= self.right || rect.right <= self.left {
            false
        } else {
            true
        }
    }

    fn area(&self) -> u64 {
        (self.right - self.left + 1) * (self.bottom - self.top + 1)
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d9p2.txt")?;
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

    let mut borders: Vec<Rectangle> = Vec::new();

    for i in 0..tiles.len() {
        let u = &tiles[i];
        let v = &tiles[(i+1) % tiles.len()];
        borders.push(Rectangle::new(u, v));
    }

    let mut max_area = 0u64;

    for i in 0..tiles.len() {
        for j in (i + 1)..tiles.len() {
            let rect = Rectangle::new(&tiles[i], &tiles[j]);
            if borders.iter().any(|b| rect.overlap_interior(b)) {
                continue;
            }
            max_area = max_area.max(rect.area());
        }
    }

    println!("{}", max_area);

    Ok(())
}
