use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("input/d7p1.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let grid = lines.iter()
        .map(|l| l.chars().map(|c| c == '^').collect::<Vec<bool>>())
        .collect::<Vec<Vec<bool>>>();
    let rows = grid.len();
    let cols = grid[0].len();

    let mut beam = vec![false; cols];
    beam[lines[0].find('S').unwrap()] = true;

    let mut splits = 0u64;
    for i in (0..rows).step_by(2) {
        let beam_indices = beam.iter()
            .enumerate()
            .filter_map(|(j, &s)| if s { Some(j) } else {None})
            .collect::<Vec<usize>>();
        for j in beam_indices {
            if grid[i][j] {
                splits += 1;
                beam[j] = false;
                if j > 0 {
                    beam[j-1] = true;
                }
                if j + 1 < cols {
                    beam[j+1] = true;
                }
            }
        }
    }
    println!("{splits}");
    Ok(())
}
