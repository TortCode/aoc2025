use std::fs::File;
use std::io::{BufRead, BufReader};
use good_lp::{default_solver, variable, variables, Expression, Solution, SolverModel};

struct Machine {
    buttons: Vec<Vec<u32>>,
    joltages: Vec<u32>,
}

impl Machine {
    fn min_presses(&mut self) -> u32 {
        let mut vars = variables!();
        let max_joltage = *self.joltages.iter().max().unwrap();
        let mut p = (0..self.buttons.len())
            .map(|_| vars.add(
                variable()
                    .integer()
                    .clamp(0, max_joltage)
            ))
            .collect::<Vec<_>>();
        let objective = p.iter().sum::<Expression>();
        let mut model = vars.minimise(&objective).using(default_solver);
        for (i, &joltage) in self.joltages.iter().enumerate() {
            let result = self.buttons.iter().enumerate()
                .filter_map(|(j, button)| {
                    if button.contains(&(i as u32)) {
                        Some(p[j])
                    } else {
                        None
                    }
                })
                .sum::<Expression>();
            model.add_constraint(result.eq(joltage));
        }

        let solution = model.solve().unwrap();
        solution.eval(&objective) as u32
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("input/d10p2.txt")?;
    let reader = BufReader::new(file);
    let mut machines: Vec<Machine> = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts = line.split(' ').collect::<Vec<_>>();
        let buttons = parts[1..parts.len() - 1].iter()
            .map(|s|
                s.trim_start_matches('(').trim_end_matches(')')
                    .split(',')
                    .map(|t| t.parse::<u32>().unwrap())
                    .collect::<Vec<_>>()
            )
            .collect::<Vec<_>>();
        let joltages = parts[parts.len() - 1].trim_start_matches('{').trim_end_matches('}')
            .split(',')
            .map(|t| t.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        machines.push(Machine { buttons, joltages });
    }
    let L = machines.len();
    let total_presses: u32 = machines.iter_mut().enumerate().map(|(i, m)| {
        println!("{i}/{}", L);
        m.min_presses() as u32
    }).sum();
    println!("{}", total_presses);
    Ok(())
}
