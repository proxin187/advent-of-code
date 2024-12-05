use std::fs;


#[derive(Debug)]
pub struct Rule {
    left: usize,
    right: usize,
}

impl Rule {
    pub fn new(left: &str, right: &str) -> Result<Rule, Box<dyn std::error::Error>> {
        Ok(Rule {
            left: left.parse::<usize>()?,
            right: right.parse::<usize>()?,
        })
    }
}

#[derive(Debug)]
pub struct RuleSet {
    rules: Vec<Rule>,
}

impl RuleSet {
    pub fn new(rules: Vec<Rule>) -> RuleSet {
        RuleSet {
            rules,
        }
    }

    pub fn sort(&self, update: &mut Update) {
        println!("[sort] update={:?}", update);

        while !self.validate(&update) {
            for (index, page) in update.pages.clone().iter().enumerate() {
                let mut rules = self.rules.iter()
                    .filter(|rule| rule.right == *page && update.pages[index..].contains(&rule.left));

                if let Some(insert) = rules.next().and_then(|rule| update.pages.iter().position(|page| *page == rule.left)) {
                    let new = update.pages.remove(index);

                    println!("[sort] new={:?}, insert={:?}", new, insert);

                    update.pages.insert(insert, new);

                    break;
                }
            }
        }

        println!("[sort] done update={:?}", update);
    }

    pub fn validate(&self, update: &Update) -> bool {
        update.pages.iter()
            .enumerate()
            .all(|(index, page)| {
                !self.rules.iter()
                    .any(|rule| rule.right == *page && update.pages[index..].contains(&rule.left))
            })
    }
}

#[derive(Debug)]
pub struct Update {
    pages: Vec<usize>,
}

impl Update {
    pub fn new(pages: Vec<usize>) -> Update {
        Update {
            pages,
        }
    }

    pub fn middle(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }
}

#[derive(Debug)]
pub struct Part1 {
    rules: RuleSet,
    updates: Vec<Update>,
}

impl Part1 {
    pub fn new(file: &str) -> Result<Part1, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;
        let mut lines = content.lines();

        let rules = lines.by_ref().take_while(|line| !line.is_empty())
            .filter_map(|line| {
                line.split('|').next()
                    .zip(line.split('|').last())
                    .and_then(|(left, right)| Rule::new(left, right).ok())
            })
            .collect::<Vec<Rule>>();

        let updates = lines
            .map(|line| {
                let pages = line.split(',')
                    .filter_map(|page| page.parse::<usize>().ok())
                    .collect::<Vec<usize>>();

                Update::new(pages)
            })
            .collect::<Vec<Update>>();

        Ok(Part1 {
            rules: RuleSet::new(rules),
            updates,
        })
    }

    pub fn calculate(&self) -> usize {
        self.updates.iter()
            .fold(0, |mut acc, update| {
                println!("[calculate] update={:?}", update);

                if self.rules.validate(update) {
                    println!("[calculate] valid");

                    acc += update.middle()
                }

                acc
            })
    }
}

#[derive(Debug)]
pub struct Part2 {
    rules: RuleSet,
    updates: Vec<Update>,
}

impl From<Part1> for Part2 {
    fn from(part1: Part1) -> Part2 {
        let updates = part1.updates.into_iter()
            .filter(|update| !part1.rules.validate(update))
            .collect::<Vec<Update>>();

        Part2 {
            rules: part1.rules,
            updates,
        }
    }
}

impl Part2 {
    pub fn calculate(&mut self) -> usize {
        self.updates.iter_mut()
            .fold(0, |acc, update| { self.rules.sort(update); acc + update.middle() })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let part1 = Part1::new("input.txt")?;

    println!("part1: {:?}", part1.calculate());

    let mut part2 = Part2::from(part1);

    println!("part2: {:?}", part2.calculate());

    Ok(())
}

