use std::collections::HashMap;
use std::fs;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: i32,
    y: i32,
}

impl Position {
    pub fn intersects(&self, width: i32, height: i32) -> bool {
        self.x >= 0 && self.y >= 0 && self.x < width && self.y < height
    }

    pub fn skew(&mut self, direction: Direction, diff: Position) {
        match direction {
            Direction::Up => *self = *self + diff,
            Direction::Down => *self = *self - diff,
        }
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
}

impl Direction {
    pub fn choose(&self, a: Position, b: Position) -> Position {
        match self {
            Direction::Up => a,
            Direction::Down => b,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Antenna {
    freq: char,
    pos: Position,
}

impl Antenna {
    pub fn new(freq: char, x: i32, y: i32) -> Antenna {
        Antenna {
            freq,
            pos: Position {
                x,
                y,
            },
        }
    }
}

#[derive(Debug)]
pub struct Part1 {
    antennas: Vec<Antenna>,
    width: i32,
    height: i32,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let antennas = content.lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter_map(|(x, c)| (c != '.').then(|| Antenna::new(c, x as i32, y as i32)))
                    .collect::<Vec<Antenna>>()
            })
            .collect::<Vec<Antenna>>();

        let width = content.lines()
            .map(|line| line.len() as i32)
            .max()
            .ok_or(Into::<Box<dyn std::error::Error>>::into("failed to find width"))?;

        let height = content.lines().count() as i32;

        Ok(Part1 {
            antennas,
            width,
            height,
        })
    }

    pub fn calculate(&self) -> usize {
        let mut positions: HashMap<Position, ()> = HashMap::new();

        for a in self.antennas.iter() {
            for b in self.antennas.iter().filter(|antenna| antenna.freq == a.freq && antenna.pos != a.pos) {
                let diff = a.pos - b.pos;

                for anti in [b.pos - diff, a.pos + diff] {
                    if anti.intersects(self.width, self.height) {
                        positions.insert(anti, ());
                    }
                }
            }
        }

        positions.iter().count()
    }
}

pub struct Part2 {
    antennas: Vec<Antenna>,
    width: i32,
    height: i32,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        Part2 {
            antennas: part1.antennas,
            width: part1.width,
            height: part1.height,
        }
    }
}

impl Part2 {
    pub fn calculate(&self) -> usize {
        let mut positions: HashMap<Position, ()> = HashMap::new();

        for a in self.antennas.iter() {
            for b in self.antennas.iter().filter(|antenna| antenna.freq == a.freq && antenna.pos != a.pos) {
                let diff = a.pos - b.pos;

                for direction in [Direction::Up, Direction::Down] {
                    let mut anti = direction.choose(a.pos - diff, b.pos + diff);

                    while anti.intersects(self.width, self.height) {
                        positions.insert(anti, ());

                        anti.skew(direction, diff);
                    }
                }
            }
        }

        positions.iter().count()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = Part1::new("input.txt")?;

    println!("part1: {}", part1.calculate());

    let part2 = Part2::from(part1);

    println!("part2: {}", part2.calculate());

    Ok(())
}


