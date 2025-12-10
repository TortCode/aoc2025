use std::fs::File;
use std::io::{BufRead, BufReader};

struct Point(u64, u64, u64);

impl Point {
    fn dist(&self, other: &Point) -> u64 {
        let dx = self.0 as i64 - other.0 as i64;
        let dy = self.1 as i64 - other.1 as i64;
        let dz = self.2 as i64 - other.2 as i64;
        (dx * dx + dy * dy + dz * dz) as u64
    }
}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn same_set(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if self.size[x] < self.size[y] {
            (x, y) = (y, x);
        }
        self.size[x] += self.size[y];
        self.parent[y] = x;
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d8p2.txt")?;
    let br = BufReader::new(file);


    let points = br.lines()
        .map(|l| {
            let l = l.unwrap();
            let nums = l.split(',')
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            Point(nums[0], nums[1], nums[2])
        })
        .collect::<Vec<Point>>();

    let mut pairs = Vec::<(u64, usize, usize)>::new();
    for i in 0..points.len() {
        for j in i+1..points.len() {
            pairs.push((points[i].dist(&points[j]), i, j));
        }
    }
    pairs.sort_by_key(|p| p.0);

    let mut nsets = points.len();
    let mut uf = UnionFind::new(points.len());
    for &(_, i, j) in pairs.iter() {
        if !uf.same_set(i, j) {
            nsets -= 1;
            uf.union(i, j);
            if nsets == 1 {
                println!("{}", points[i].0 * points[j].0);
                break;
            }
        }
    }

    Ok(())
}