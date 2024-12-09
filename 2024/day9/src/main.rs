use std::fs;


#[derive(Debug, PartialEq, Clone)]
pub enum Block {
    File {
        id: usize,
    },
    Free,
}

impl Block {
    pub fn is_free(&self) -> bool {
        *self == Block::Free
    }

    pub fn is_file(&self) -> bool {
        match self {
            Block::File { .. } => true,
            Block::Free => false,
        }
    }

    pub fn character(&self) -> char {
        match self {
            Block::File { id } => char::from_digit(*id as u32, 10).unwrap_or('.'),
            Block::Free => '.',
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    block: Block,
    size: usize,
}

impl Group {
    pub fn new(block: Block, size: usize) -> Group {
        Group {
            block,
            size,
        }
    }

    pub fn map(&self) -> Vec<Block> {
        vec![self.block.clone(); self.size]
    }
}

trait Partition {
    fn sort(&mut self);

    fn map(&self) -> Vec<Block>;

    fn calculate(&mut self) -> usize {
        self.sort();

        self.map()
            .iter()
            .enumerate()
            .fold(0, |acc, (index, block)| {
                match block {
                    Block::File { id } => acc + (*id * index),
                    Block::Free => acc,
                }
            })
    }
}

pub struct Part1 {
    map: Vec<Block>,
    groups: Vec<Group>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let groups = content.chars()
            .enumerate()
            .filter_map(|(index, block)| {
                block.to_digit(10).map(|size| {
                    (index % 2 != 0)
                        .then(|| Group::new(Block::Free, size as usize))
                        .unwrap_or(Group::new(Block::File { id: index / 2 }, size as usize))
                })
            })
            .collect::<Vec<Group>>();

        Ok(Part1 {
            map: groups.iter().flat_map(|group| group.map()).collect::<Vec<Block>>(),
            groups,
        })
    }

    fn unsorted(&self) -> Option<usize> {
        self.map.iter()
            .position(|block| block.is_free())
            .and_then(|offset| {
                self.map[offset..].iter()
                    .enumerate()
                    .rev()
                    .filter_map(|(index, block)| block.is_file().then(|| index + offset))
                    .next()
            })
    }

    fn free(&self) -> usize {
        self.map.iter()
            .position(|block| block.is_free())
            .unwrap_or(0)
    }

    fn draw(&self) {
        let map = self.map.iter().map(|block| block.character()).collect::<String>();

        println!("map: {}", map);
    }
}

impl Partition for Part1 {
    fn sort(&mut self) {
        self.draw();

        while let Some(unsorted) = self.unsorted() {
            let free = self.free();

            println!("[sort] unsorted={}, free={}", unsorted, free);

            self.map.swap(free, unsorted);
        }

        self.draw();
    }

    fn map(&self) -> Vec<Block> {
        self.map.clone()
    }
}

pub struct Part2 {
    groups: Vec<Group>,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        Part2 {
            groups: part1.groups,
        }
    }
}

impl Part2 {
    fn unsorted(&self) -> Option<Vec<(usize, Group)>> {
        self.groups.iter()
            .position(|group| group.block.is_free())
            .map(|offset| {
                self.groups[offset..].iter()
                    .enumerate()
                    .filter_map(|(index, group)| group.block.is_file().then(|| (index + offset, group.clone())))
                    .collect::<Vec<(usize, Group)>>()
            })
    }

    fn free(&self) -> usize {
        self.groups.iter()
            .position(|group| group.block.is_free())
            .unwrap_or(0)
    }

    fn draw(&self) {
        let map = self.groups.iter().map(|group| group.map().iter().map(|block| block.character()).collect::<String>()).collect::<String>();

        println!("map: {}", map);
    }
}

impl Partition for Part2 {
    fn sort(&mut self) {
        'outer: while let Some(unsorted) = self.unsorted() {
            self.draw();

            let free = self.free();

            for (index, group) in unsorted.iter().rev() {
                if self.groups[free].size == group.size {
                    self.groups.swap(free, *index);

                    continue 'outer;
                } else if self.groups[free].size > group.size {
                    self.groups.remove(free);

                    // TODO: FINISH THIS PART
                    self.groups.insert(free, group.clone());

                    continue 'outer;
                }
            }

            break 'outer;
        }

        self.draw();
    }

    fn map(&self) -> Vec<Block> {
        self.groups.iter()
            .cloned()
            .flat_map(|group| group.map())
            .collect::<Vec<Block>>()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut part1 = Part1::new("test.txt")?;

    println!("part1: {:?}", part1.calculate());

    let mut part2 = Part2::from(part1);

    println!("part2: {:?}", part2.calculate());

    Ok(())
}


