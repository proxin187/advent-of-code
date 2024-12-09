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
}

pub struct DiskMap {
    map: Vec<Block>,
}

impl DiskMap {
    pub fn new(file: &str) -> Result<DiskMap, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let map = content.chars()
            .enumerate()
            .filter_map(|(index, block)| {
                block.to_digit(10).map(|size| {
                    (index % 2 != 0)
                        .then(|| vec![Block::Free; size as usize])
                        .unwrap_or(vec![Block::File { id: index / 2 }; size as usize])
                })
            })
            .flatten()
            .collect::<Vec<Block>>();

        Ok(DiskMap {
            map,
        })
    }

    fn sort(&mut self) {
        // TODO: finish this, at this point we want to check that all blocks following are free
        while self.map.iter().position(|block| block.is_free()).map(||) {
        }
    }

    pub fn calculate(&mut self) {

    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut diskmap = DiskMap::new("test.txt")?;

    println!("part1: {:?}", diskmap.calculate());

    Ok(())
}

