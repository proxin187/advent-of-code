use std::fs;


#[derive(Debug, Clone)]
pub enum Direction {
    Increasing,
    Decreasing,
}

impl Direction {
    fn from(a: i32, b: i32) -> Direction {
        (a < b).then(|| Direction::Increasing).unwrap_or(Direction::Decreasing)
    }

    fn is_safe(&self, a: i32, b: i32) -> bool {
        match self {
            Direction::Increasing => a < b && b - a >= 1 && b - a <= 3,
            Direction::Decreasing => a > b && a - b >= 1 && a - b <= 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Report {
    direction: Direction,
    levels: Vec<i32>,
}

impl From<&str> for Report {
    fn from(report: &str) -> Report {
        let levels = report.split(" ")
            .map(|level| level.parse::<i32>().expect("invalid number"))
            .collect::<Vec<i32>>();

        Report::new(Direction::from(levels[0], levels[1]), levels)
    }
}

impl Report {
    pub fn new(direction: Direction, levels: Vec<i32>) -> Report {
        Report {
            direction,
            levels,
        }
    }

    pub fn derivatives(&self) -> Vec<Report> {
        let mut reports: Vec<Report> = vec![self.clone()];

        for (index, _) in self.levels.iter().enumerate() {
            let levels = self.levels.iter()
                .enumerate()
                .filter(|(idx, _)| *idx != index)
                .map(|(_, level)| *level)
                .collect::<Vec<i32>>();

            println!("derivative: {:?}", levels);

            reports.push(Report::new(Direction::from(levels[0], levels[1]), levels));
        }

        reports
    }

    pub fn is_safe(&self) -> bool {
        self.levels.windows(2)
            .all(|window| {
                self.direction.is_safe(window[0], window[1])
            })
    }
}

pub struct Data {
    reports: Vec<Report>,
}

impl Data {
    pub fn new(file: &str) -> Result<Data, Box<dyn std::error::Error>> {
        let string = fs::read_to_string(file)?;

        Ok(Data {
            reports: string.lines()
                .filter(|line| !line.is_empty())
                .map(|line| Report::from(line))
                .collect::<Vec<Report>>(),
        })
    }

    pub fn analyze(&self) -> usize {
        let reports = self.reports.iter()
            .filter(|report| {
                report.derivatives().iter()
                    .any(|report| report.is_safe())
            })
            .collect::<Vec<&Report>>();

        println!("reports: {:?}", reports);

        reports.len()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = Data::new("input.txt")?;

    println!("analyze: {}", data.analyze());

    Ok(())
}


