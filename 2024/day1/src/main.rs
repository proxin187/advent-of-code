use std::fs;


#[derive(Debug)]
pub struct Part1 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let string = fs::read_to_string(file)?;

        let (left, right): (Vec<_>, Vec<_>) = string.split([' ', '\n'])
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<usize>().expect("invalid integer"))
            .enumerate()
            .partition(|(idx, _)| idx % 2 == 0);

        Ok(Part1 {
            left: left.iter().map(|(_, id)| *id).collect::<Vec<usize>>(),
            right: right.iter().map(|(_, id)| *id).collect::<Vec<usize>>(),
        })
    }

    pub fn sort(&mut self) {
        self.left.sort();

        self.right.sort();
    }

    pub fn total(&self) -> usize {
        self.left.iter()
            .zip(self.right.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .fold(0, |acc, x| acc + x)
    }
}

#[derive(Debug)]
pub struct Part2 {
    left: Vec<usize>,
    right: Vec<usize>,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        Part2 {
            left: part1.left,
            right: part1.right,
        }
    }
}

impl Part2 {
    pub fn appearences(&self, value: &usize) -> usize {
        self.right.iter()
            .filter(|x| *x == value)
            .count()
    }

    pub fn similarity(&self) -> usize {
        self.left.iter()
            .map(|x| *x * self.appearences(x))
            .fold(0, |acc, x| acc + x)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part1 = Part1::new("input.txt")?;

    part1.sort();

    println!("total: {}", part1.total());

    let part2 = Part2::from(part1);

    println!("similarity: {}", part2.similarity());

    Ok(())
}

