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

    pub fn id(&self) -> usize {
        match self {
            Block::File { id } => *id,
            Block::Free => 0,
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
    fn unsorted(&self, lim: usize) -> Option<usize> {
        self.groups.iter()
            .enumerate()
            .rev()
            .skip_while(|(index, group)| group.block.is_free() || *index < lim)
            .next()
            .map(|(index, _)| index)
    }

    fn free(&self, size: usize, unsorted: usize) -> Option<usize> {
        self.groups.iter()
            .enumerate()
            .skip_while(|(index, group)| group.block.is_file() || *index > unsorted || group.size < size)
            .next()
            .map(|(index, _)| index)
    }

    fn draw(&self) {
        let map = self.groups.iter().map(|group| group.map().iter().map(|block| block.character()).collect::<String>()).collect::<String>();

        println!("map: {}", map);
    }
}

impl Partition for Part2 {
    fn sort(&mut self) {
        let mut lim = self.groups.len() - 1;

        while let Some(unsorted) = self.unsorted(lim) {
            println!("lim: {}, unsorted: {}", lim, unsorted);

            self.draw();

            match self.free(self.groups[unsorted].size, lim) {
                Some(free) => {
                    println!("free: {}", free);

                    self.groups.swap(free, unsorted);

                    self.draw();

                    let free_group = self.groups[unsorted].clone();
                    let unsorted_group = self.groups[free].clone();

                    // this is actually checking free > unsorted, but we swapped it
                    if free_group.size > unsorted_group.size {
                        self.groups.insert(free + 1, Group::new(Block::Free, free_group.size - unsorted_group.size));

                        if let Some(g) = self.groups.get_mut(unsorted + 1) {
                            g.size -= free_group.size - unsorted_group.size;
                        }
                    }

                    self.draw();

                    lim = self.groups.len() - 1;
                },
                None => lim -= 1,
            }
        }


        println!("done: {}", lim);

        /*
        'outer: while let Some(mut unsorted) = self.unsorted() {
            let hash = self.map()
                .iter()
                .enumerate()
                .fold(0, |acc, (index, block)| {
                    match block {
                        Block::File { id } => acc + (*id * index),
                        Block::Free => acc,
                    }
                });

            println!("hash: {}", hash);

            unsorted.sort_by(|(_, a), (_, b)| b.block.id().cmp(&a.block.id()));

            for (unsorted_index, unsorted_group) in unsorted {
                // TODO: we will have to not recalculate free everytime, its better to just remove
                // from free when we find
                for (free_index, free_group) in self.free(unsorted_index) {
                    if free_group.size == unsorted_group.size {
                        self.groups.swap(free_index, unsorted_index);

                        continue 'outer;
                    } else if free_group.size > unsorted_group.size {
                        self.groups.swap(free_index, unsorted_index);

                        self.groups.insert(free_index + 1, Group::new(Block::Free, free_group.size - unsorted_group.size));

                        if let Some(g) = self.groups.get_mut(unsorted_index + 1) {
                            g.size -= free_group.size - unsorted_group.size;
                        }

                        continue 'outer;
                    }
                }
            }

            break 'outer;
        }
        */
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

    // println!("part1: {:?}", part1.calculate());

    let mut part2 = Part2::from(part1);

    println!("part2: {:?}", part2.calculate());

    Ok(())
}


