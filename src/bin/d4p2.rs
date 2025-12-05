use std::collections::VecDeque;
use std::io::{BufRead, Read};

struct Grid {
    cells: Vec<Vec<bool>>,
    adj_count: Vec<Vec<u8>>,
}

impl Grid {
    fn new(data: String) -> Self {
        let mut grid = Grid {
            cells: Vec::new(),
            adj_count: Vec::new(),
        };
        for line in data.split('\n') {
            let row = line.chars().map(|c| c == '@').collect::<Vec<bool>>();
            grid.adj_count.push(vec![0u8; row.len()]);
            grid.cells.push(row);
        }
        for (i, row) in grid.cells.iter().enumerate() {
            for (j, paper) in row.iter().enumerate() {
                if *paper {
                    for (ni, nj) in grid.neighbors(i, j) {
                        grid.adj_count[ni][nj] += 1;
                    }
                }
            }
        }
        grid
    }

    fn rows(&self) -> usize {
        self.cells.len()
    }

    fn cols(&self) -> usize {
        self.cells[0].len()
    }

    fn in_range(&self, i: i32, j: i32) -> bool {
        i >= 0 && i < self.rows() as i32 && j >= 0 && j < self.cols() as i32
    }

    fn neighbors(&self, i: usize, j: usize) -> Vec<(usize, usize)> {
        let mut pos = Vec::new();
        for di in -1..=1 {
            for dj in -1..=1 {
                if di == 0 && dj == 0 {
                    continue;
                }
                let ni = i as i32 + di;
                let nj = j as i32 + dj;
                if self.in_range(ni, nj) && self.cells[ni as usize][nj as usize] {
                    pos.push((ni as usize, nj as usize));
                }
            }
        }
        pos
    }

    fn count_accessible(&mut self) -> u32 {
        let mut accessable = 0u32;

        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for (i, row) in self.cells.iter().enumerate() {
            for (j, paper) in row.iter().enumerate() {
                if *paper && self.adj_count[i][j] < 4 {
                    queue.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = queue.pop_front() {
            accessable += 1;
            self.cells[i][j] = false;
            for (ni, nj) in self.neighbors(i, j) {
                if self.adj_count[ni][nj] >= 4 {
                    self.adj_count[ni][nj] -= 1;
                    if self.adj_count[ni][nj] < 4 {
                        queue.push_back((ni, nj));
                    }
                }
            }
        }

        accessable
    }
}

fn main() -> std::io::Result<()> {
    let data = std::fs::read_to_string("input/d4p2.txt")?;
    let mut grid = Grid::new(data);
    println!("{}", grid.count_accessible());
    Ok(())
}
