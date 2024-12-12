use std::collections::HashMap;
use std::fs;

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Left, Direction::Right];


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Position {
        Position {
            x,
            y,
        }
    }
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn mov(&self, position: Position, width: usize, height: usize) -> Option<Position> {
        match self {
            Direction::Up => position.y.checked_sub(1).map(|y| Position::new(position.x, y)),
            Direction::Down => (position.y + 1 < height).then(|| Position::new(position.x, position.y + 1)),
            Direction::Left => position.x.checked_sub(1).map(|x| Position::new(x, position.y)),
            Direction::Right => (position.x + 1 < width).then(|| Position::new(position.x + 1, position.y))
        }
    }
}

pub struct Part1 {
    map: Vec<Vec<i32>>,
    heads: Vec<Position>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let map = content.lines()
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_digit(10).map(|digit| digit as i32))
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

        let heads = map.iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter_map(|(x, height)| (*height == 0).then(|| Position::new(x, y)))
                    .collect::<Vec<Position>>()
            })
            .collect::<Vec<Position>>();

        Ok(Part1 {
            map,
            heads,
        })
    }

    fn draw(&self, position: Position) {
        let content = self.map.iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(|(x, height)| (Position::new(x, y) == position).then(|| '^').unwrap_or(char::from_digit(*height as u32, 10).expect("huh")))
                    .collect::<String>()
            })
            .map(|line| [line, String::from("\n")].concat())
            .collect::<String>();

        println!("{}", content);
    }

    fn query(&self, position: Position) -> i32 {
        self.map[position.y][position.x]
    }

    fn score(&self, position: Option<Position>) -> HashMap<Position, ()> {
        match position {
            Some(position) => {
                if self.query(position) == 9 {
                    HashMap::from([(position, ())])
                } else {
                    let height = self.map.len();
                    let width = self.map[0].len();

                    DIRECTIONS.iter()
                        .filter_map(|direction| {
                            direction.mov(position, width, height).and_then(|new| {
                                (self.query(new) - self.query(position) == 1).then(|| self.score(Some(new)))
                            })
                        })
                        .flatten()
                        .collect::<HashMap<Position, ()>>()
                }
            },
            None => HashMap::new(),
        }
    }

    pub fn calculate(&self) -> usize {
        println!("heads: {:?}", self.heads);

        self.heads.iter()
            .fold(0, |acc, position| acc + self.score(Some(*position)).iter().count())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = Part1::new("input.txt")?;

    println!("part1: {:?}", part1.calculate());

    // TODO: finish part2

    Ok(())
}


