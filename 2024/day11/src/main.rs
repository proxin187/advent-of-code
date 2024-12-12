use std::fs;


pub struct Part1 {
    stones: Vec<usize>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let stones = content.trim_end().split(' ')
            .filter_map(|stone| stone.parse::<usize>().ok())
            .collect::<Vec<usize>>();

        Ok(Part1 {
            stones,
        })
    }

    fn to_digits(&self, number: usize) -> Vec<char> {
        number.to_string()
            .chars()
            .collect::<Vec<char>>()
    }

    fn from_digits(&self, digits: &[char]) -> usize {
        digits.iter()
            .collect::<String>()
            .parse::<usize>()
            .unwrap_or(0)
    }

    fn update(&self, stone: usize) -> Vec<usize> {
        let digits = self.to_digits(stone);

        match stone {
            0 => vec![1],
            _ => match digits.len() % 2 {
                0 => vec![self.from_digits(&digits[..digits.len() / 2]), self.from_digits(&digits[digits.len() / 2..])],
                _ => vec![stone * 2024],
            },
        }
    }

    fn blink(&self, stones: Vec<usize>) -> Vec<usize> {
        stones.iter()
            .flat_map(|stone| self.update(*stone))
            .collect::<Vec<usize>>()
    }

    pub fn calculate(&self, times: usize) -> usize {
        let mut stones = self.stones.clone();

        for _ in 0..times {
            stones = self.blink(stones);
        }

        stones.len()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = Part1::new("input.txt")?;

    println!("part1: {:?}", part1.calculate(25));

    // TODO: finish this
    println!("part2: {:?}", part1.calculate(75));

    Ok(())
}

