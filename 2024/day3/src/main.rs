use std::iter::Peekable;
use std::vec::IntoIter;
use std::fs;


#[derive(Debug)]
pub enum State {
    Normal,
    Dont,
    Arg1,
    Arg2(usize),
}

pub struct Memory {
    memory: Peekable<IntoIter<char>>,
    state: State,
    should_close: bool,
}

impl Memory {
    pub fn new(file: &str) -> Result<Memory, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(file)?;

        let memory = content.chars()
            .collect::<Vec<char>>()
            .into_iter()
            .peekable();

        Ok(Memory {
            memory,
            state: State::Normal,
            should_close: false,
        })
    }

    fn take_while<F: Copy>(&mut self, f: F) -> String
    where
        F: Fn(&(usize, char)) -> bool
    {
        let mut characters: Vec<char> = Vec::new();
        let mut index = 0;

        while let Some(c) = self.memory.by_ref().next_if(|c| f(&(index, *c))) {
            characters.push(c);

            index += 1;
        }

        characters.iter().collect::<String>()
    }

    fn state_if_match(&mut self, needle: &str, state: State, except: State) {
        let characters = needle.chars().collect::<Vec<char>>();

        let token = self.take_while(|(index, c)| *index < characters.len() && *c == characters[*index]);

        self.state = (token.as_str() == needle).then(|| state).unwrap_or(except);
    }

    pub fn interpret(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        let mut result = 0;

        while !self.should_close {
            match self.state {
                State::Normal => {
                    match self.memory.next() {
                        Some('m') => self.state_if_match("ul(", State::Arg1, State::Normal),
                        Some('d') => self.state_if_match("on't()", State::Dont, State::Normal),
                        None => self.should_close = true,
                        _ => {},
                    }
                },
                State::Dont => {
                    match self.memory.next() {
                        Some('d') => self.state_if_match("o()", State::Normal, State::Dont),
                        None => self.should_close = true,
                        _ => {},
                    }
                },
                State::Arg1 => {
                    let value = self.take_while(|(index, c)| *index < 5 && c.is_ascii_digit()).parse::<usize>()?;

                    match self.memory.by_ref().next() {
                        Some(',') => self.state = State::Arg2(value),
                        _ => self.state = State::Normal,
                    }
                },
                State::Arg2(arg1) => {
                    let value = self.take_while(|(index, c)| *index < 5 && c.is_ascii_digit()).parse::<usize>()?;

                    match self.memory.by_ref().next() {
                        Some(')') => result += arg1 * value,
                        _ => {},
                    }

                    self.state = State::Normal;
                },
            }
        }

        Ok(result)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut memory = Memory::new("input.txt")?;

    let result = memory.interpret()?;

    println!("result: {:?}", result);

    Ok(())
}


