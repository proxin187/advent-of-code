use std::fs;

const DIRECTIONS: [Direction; 4] = [Direction::Up, Direction::Down, Direction::Right, Direction::Left];
const PLANES: [Plane; 2] = [Plane::Straight, Plane::Diagonal];


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

#[derive(Debug, Default)]
pub struct Position {
    x: usize,
    y: usize,
}

pub struct Word {
    columns: Vec<Vec<char>>,
    pos: Position,
}

impl Word {
    pub fn new(file: &str) -> Result<Word, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let columns = content.lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        Ok(Word {
            columns,
            pos: Position::default(),
        })
    }

    fn scan(&self, direction: Direction, plane: Plane) -> bool {
        let needle = ['X', 'M', 'A', 'S'];

        // TODO: we also need to check in reverse here, for example reverse on line two are not
        // detected

        let result = match plane {
            Plane::Straight => match direction {
                Direction::Up => self.pos.y >= 3
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().map(|column| column[self.pos.x]).collect::<Vec<char>>() == needle,
                Direction::Down => self.pos.y + 3 < self.columns.len()
                    && self.columns[self.pos.y..self.pos.y + 4].iter().map(|column| column[self.pos.x]).collect::<Vec<char>>() == needle,
                Direction::Right => self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y][self.pos.x..self.pos.x + 4] == needle,
                Direction::Left => self.pos.x >= 3
                    && self.columns[self.pos.y][self.pos.x - 3..=self.pos.x] == needle,
            },
            Plane::Diagonal => match direction {
                Direction::Up => self.pos.y >= 3
                    && self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().enumerate().map(|(count, column)| column[self.pos.x + count]).collect::<Vec<char>>() == needle,
                Direction::Down => self.pos.y + 3 < self.columns.len()
                    && self.pos.x >= 3
                    && self.columns[self.pos.y..self.pos.y + 4].iter().enumerate().map(|(count, column)| column[self.pos.x - count]).collect::<Vec<char>>() == needle,
                Direction::Right => self.pos.y + 3 < self.columns.len()
                    && self.pos.x + 3 < self.columns[self.pos.y].len()
                    && self.columns[self.pos.y..self.pos.y + 4].iter().enumerate().map(|(count, column)| column[self.pos.x + count]).collect::<Vec<char>>() == needle,
                Direction::Left => self.pos.y >= 3
                    && self.pos.x >= 3
                    && self.columns[self.pos.y - 3..=self.pos.y].iter().enumerate().map(|(count, column)| column[self.pos.x - count]).collect::<Vec<char>>() == needle,
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut word = Word::new("test2.txt")?;

    // TODO: this should be 18 but we get 12

    println!("calculate: {}", word.calculate());

    Ok(())
}

