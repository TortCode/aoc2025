use std::fs::File;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let file = File::open("input/d4p1.txt")?;
    let br = std::io::BufReader::new(file);
    let mut grid: Vec<String> = Vec::new();

    for line in br.lines() {
        let line = line?;
        grid.push(line);
    }

    let rows = grid.len();
    let cols = grid[0].len();

    let mut adj_count = vec![vec![0u8; cols]; rows];
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '@' {
                for di in -1..=1 {
                    for dj in -1..=1 {
                        if di == 0 && dj == 0 {
                            continue;
                        }
                        let ni = i as i32 + di;
                        let nj = j as i32 + dj;
                        if ni < 0 || ni >= rows as i32 || nj < 0 || nj >= cols as i32 {
                            continue;
                        }
                        adj_count[ni as usize][nj as usize] += 1;
                    }
                }
            }
        }
    }

    let mut accessable = 0u32;
    for (i, line) in grid.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            if ch == '@' && adj_count[i][j] < 4 {
                accessable += 1;
            }
        }
    }
    println!("{}", accessable);

    Ok(())
}
