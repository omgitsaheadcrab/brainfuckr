use crate::cmds;
use std::io::Read;
use std::{char, num};

pub struct Interpreter {
    pub data: Vec<num::Wrapping<u8>>,
    pub data_pointer: usize,
    pub inst: Vec<cmds::Commands>,
    pub inst_pointer: usize,
}

impl Interpreter {
    pub fn execute(&mut self) {
        while self.inst_pointer < self.inst.len() {
            match self.inst[self.inst_pointer] {
                cmds::Commands::Increment => self.data[self.data_pointer] += 1,
                cmds::Commands::Decrement => self.data[self.data_pointer] -= 1,
                cmds::Commands::MoveRight => self.data_pointer += 1,
                cmds::Commands::MoveLeft => self.data_pointer -= 1,
                cmds::Commands::Print => self.print(),
                cmds::Commands::Read => self.read(),
                cmds::Commands::LeftBracket(value) => {
                    if self.data[self.data_pointer] == std::num::Wrapping(0) {
                        self.inst_pointer = value;
                    }
                }
                cmds::Commands::RightBracket(value) => {
                    if self.data[self.data_pointer] != std::num::Wrapping(0) {
                        self.inst_pointer = value;
                    }
                }
            }
            self.inst_pointer += 1;
        }
    }

    fn print(&self) {
        let c: u8 = self.data[self.data_pointer].0;
        print!("{}", c as char);
    }

    fn read(&mut self) {
        let input: Option<u8> = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok());

        if let Some(value) = input {
            self.data[self.data_pointer] = std::num::Wrapping(value);
        }
    }
}
