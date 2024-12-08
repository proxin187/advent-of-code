use std::fs;


#[derive(Debug)]
pub struct Equation {
    product: u64,
    numbers: Vec<u64>,
}

impl Equation {
    pub fn new(line: &str) -> Option<Equation> {
        let mut parts = line.split(':');

        let product = parts.next().and_then(|num| num.parse::<u64>().ok())?;

        let numbers = parts.next()?
            .split(' ')
            .filter_map(|num| num.parse::<u64>().ok())
            .collect::<Vec<u64>>();

        Some(Equation {
            product,
            numbers,
        })
    }
}

#[derive(Debug)]
pub struct Part1 {
    equations: Vec<Equation>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let equations = content.lines()
            .filter_map(|line| Equation::new(line))
            .collect::<Vec<Equation>>();

        Ok(Part1 {
            equations,
        })
    }

    fn solve(&self, goal: u64, numbers: &[u64]) -> bool {
        match numbers.split_last() {
            Some((number, rest)) => {
                let tens = 10u64.pow(number.ilog10() + 1);

                goal.checked_sub(*number).map(|result| self.solve(result, rest)).unwrap_or(false)
                    || goal % number == 0 && self.solve(goal / number, rest)
                    || goal.checked_sub(*number).map(|result| result % tens == 0 && self.solve(result / tens, rest)).unwrap_or(false)
            },
            None => goal == 0,
        }
    }

    fn valid(&self, equation: &Equation) -> bool {
        self.solve(equation.product, &equation.numbers)
    }

    pub fn calculate(&mut self) -> u64 {
        println!("[calculate] equations={}", self.equations.len());

        self.equations.iter()
            .filter(|equation| self.valid(*equation))
            .fold(0, |acc, equation| acc + equation.product)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part1 = Part1::new("input.txt")?;

    println!("part1: {:#?}", part1.calculate());

    Ok(())
}


