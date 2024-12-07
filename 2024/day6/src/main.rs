use std::collections::HashMap;
use std::fs;


#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    pub fn rotate(&mut self) {
        match *self {
            Direction::Up => *self = Direction::Right,
            Direction::Right => *self = Direction::Down,
            Direction::Down => *self = Direction::Left,
            Direction::Left => *self = Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Guard {
    direction: Direction,
    position: Position,
}

impl Guard {
    pub fn new(x: usize, y: usize, direction: Direction) -> Guard {
        Guard {
            direction,
            position: Position::new(x, y),
        }
    }

    pub fn state(&self, map: &[Vec<State>]) -> State {
        match self.direction {
            Direction::Up => map[self.position.y - 1][self.position.x],
            Direction::Down => map[self.position.y + 1][self.position.x],
            Direction::Right => map[self.position.y][self.position.x + 1],
            Direction::Left => map[self.position.y][self.position.x - 1],
        }
    }

    fn exited(&self, map: &[Vec<State>]) -> bool {
        match self.direction {
            Direction::Up => self.position.y == 0,
            Direction::Down => self.position.y >= map.len() - 1,
            Direction::Right => self.position.x >= map[0].len() - 1,
            Direction::Left => self.position.x == 0,
        }
    }

    pub fn mov(&mut self) {
        match self.direction {
            Direction::Up => self.position.y = self.position.y.max(1) - 1,
            Direction::Down => self.position.y += 1,
            Direction::Right => self.position.x += 1,
            Direction::Left => self.position.x = self.position.x.max(1) - 1,
        }
    }
}

#[derive(Clone, Copy)]
pub enum State {
    Obstructed,
    Clear,
}

impl From<char> for State {
    fn from(character: char) -> State {
        match character {
            '#' => State::Obstructed,
            _ => State::Clear,
        }
    }
}

impl From<State> for char {
    fn from(state: State) -> char {
        match state {
            State::Obstructed => '#',
            State::Clear => '.',
        }
    }
}

pub struct Part1 {
    grid: Vec<Vec<State>>,
    entry: Guard,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let grid = content.lines()
            .map(|line| line.chars().map(|c| State::from(c)).collect::<Vec<State>>())
            .collect::<Vec<Vec<State>>>();

        let entry = content.lines()
            .enumerate()
            .find_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .find_map(|(x, c)| (c == '^').then(|| Guard::new(x, y, Direction::Up)))
            })
            .ok_or(Into::<Box<dyn std::error::Error>>::into("failed to find guard"))?;

        Ok(Part1 {
            grid,
            entry,
        })
    }

    pub fn simulate(&mut self) -> usize {
        let mut positions: HashMap<Position, ()> = HashMap::from([(self.entry.position, ())]);
        let mut guard = self.entry.clone();

        while !guard.exited(&self.grid) {
            match guard.state(&self.grid) {
                State::Obstructed => {
                    guard.direction.rotate();
                },
                State::Clear => {
                    guard.mov();
                },
            }

            positions.insert(guard.position, ());
        }

        positions.iter().count()
    }
}

pub struct Part2 {
    grid: Vec<Vec<State>>,
    entry: Guard,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        Part2 {
            grid: part1.grid,
            entry: part1.entry,
        }
    }
}

impl Part2 {
    pub fn simulate(&self, map: Vec<Vec<State>>) -> bool {
        let mut positions: HashMap<Guard, ()> = HashMap::new();
        let mut guard = self.entry.clone();

        // TODO: this is very ugly, make it better

        while !guard.exited(&self.grid) {
            match guard.state(&map) {
                State::Obstructed => {
                    guard.direction.rotate();
                },
                State::Clear => {
                    guard.mov();
                },
            }

            if !positions.get(&guard).is_some() {
                positions.insert(guard, ());
            } else {
                println!("[simulate] guard repeated: {:?}", positions);

                return true;
            }
        }

        false
    }

    pub fn calculate(&mut self) -> usize {
        let mut count = 0;

        for (y, _) in self.grid.iter().enumerate() {
            for (x, _) in self.grid.iter().enumerate() {
                if Position::new(x, y) != self.entry.position {
                    let mut map = self.grid.clone();

                    map[y][x] = State::Obstructed;

                    println!("[calculate] simulating x={}, y={}", x, y);

                    if self.simulate(map) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part1 = Part1::new("input.txt")?;

    println!("part1: {}", part1.simulate());

    let mut part2 = Part2::from(part1);

    println!("part2: {}", part2.calculate());

    Ok(())
}


