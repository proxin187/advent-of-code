use std::fs;

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Right, Direction::Left];
const PLANES: [Plane; 2] = [Plane::Straight, Plane::Diagonal];

const NEEDLE: [char; 4] = ['X', 'M', 'A', 'S'];
const REVERSE: [char; 4] = ['S', 'A', 'M', 'X'];

const MAS_NEEDLE: [char; 3] = ['M', 'A', 'S'];
const MAS_REVERSE: [char; 3] = ['S', 'A', 'M'];


#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
pub enum Plane {
    Straight,
    Diagonal,
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct Part1 {
    columns: Vec<Vec<char>>,
    pos: Position,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let columns = content.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Ok(Part1 {
            columns,
            pos: Position::default(),
        })
    }

    fn scan(&self, direction: Direction, plane: Plane) -> bool {
        let result = match plane {
            Plane::Straight => match direction {
                Direction::Up => self.pos.y >= 3
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().map(|column| column[self.pos.x]).collect::<Vec<char>>() == REVERSE,
                Direction::Down => self.pos.y + 3 < self.columns.len()
                    && self.columns[self.pos.y..self.pos.y + 4].iter().map(|column| column[self.pos.x]).collect::<Vec<char>>() == NEEDLE,
                Direction::Right => self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y][self.pos.x..self.pos.x + 4] == NEEDLE,
                Direction::Left => self.pos.x >= 3
                    && self.columns[self.pos.y][self.pos.x - 3..=self.pos.x] == REVERSE,
            },
            Plane::Diagonal => match direction {
                Direction::Up => self.pos.y >= 3
                    && self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().enumerate().map(|(count, column)| column[self.pos.x + count]).collect::<Vec<char>>() == REVERSE,
                Direction::Down => self.pos.y + 3 < self.columns.len()
                    && self.pos.x >= 3
                    && self.columns[self.pos.y..self.pos.y + 4].iter().enumerate().map(|(count, column)| column[self.pos.x - count]).collect::<Vec<char>>() == NEEDLE,
                Direction::Right => self.pos.y + 3 < self.columns.len()
                    && self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y..self.pos.y + 4].iter().enumerate().map(|(count, column)| column[self.pos.x + count]).collect::<Vec<char>>() == NEEDLE,
                Direction::Left => self.pos.y >= 3
                    && self.pos.x >= 3
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().enumerate().map(|(count, column)| column[self.pos.x - count]).collect::<Vec<char>>() == REVERSE,
            },
        };

        if result {
            println!("[scan] direction={:?}, plane={:?}, pos={:?}, result={}", direction, plane, self.pos, result);
        }

        result
    }

    fn update(&mut self) -> usize {
        DIRECTIONS.iter()
            .flat_map(|direction| PLANES.iter().map(|plane| self.scan(*direction, *plane)).collect::<Vec<bool>>())
            .filter(|x| *x)
            .count()
    }

    pub fn calculate(&mut self) -> usize {
        let mut score = 0;

        while self.pos.y < self.columns.len() && self.pos.x < self.columns[self.pos.y].len() {
            score += self.update();

            if self.pos.x >= self.columns[self.pos.y].len() - 1 {
                self.pos.x = 0;
                self.pos.y += 1;
            } else {
                self.pos.x += 1;
            }
        }

        score
    }
}

pub struct Part2 {
    columns: Vec<Vec<char>>,
    pos: Position,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        Part2 {
            columns: part1.columns,
            pos: Position::default(),
        }
    }
}

impl Part2 {
    fn down<F>(&self, f: F) -> Result<[char; 3], Box<dyn std::error::Error>>
    where
        F: Fn(Position, usize) -> usize
    {
        self.columns[self.pos.y..self.pos.y + 3].iter()
            .enumerate()
            .map(|(count, column)| column[f(self.pos, count)])
            .collect::<Vec<char>>()
            .try_into()
            .map_err(|_| "failed to convert".into())
    }

    fn scan(&self) -> Result<bool, Box<dyn std::error::Error>> {
        let right = self.down(|pos, count| pos.x + count)?;
        let down = self.down(|pos, count| (pos.x + 2) - count)?;

        println!("[scan] right={:?}, down={:?}, pos={:?}", right, down, self.pos);

        Ok(matches!(right, MAS_NEEDLE | MAS_REVERSE) && matches!(down, MAS_NEEDLE | MAS_REVERSE))
    }

    pub fn calculate(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut count = 0;

        while self.pos.y < self.columns.len() - 2 {
            if self.scan()? {
                count += 1;
            }

            if self.pos.x >= self.columns[self.pos.y].len() - 3 {
                self.pos.x = 0;
                self.pos.y += 1;
            } else {
                self.pos.x += 1;
            }
        }

        Ok(count)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part1 = Part1::new("input.txt")?;

    println!("part1: {}", part1.calculate());

    let mut part2 = Part2::from(part1);

    println!("part2: {}", part2.calculate()?);

    Ok(())
}

