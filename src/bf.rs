use std::io::{self, Error, ErrorKind, Read};
use std::{char, fs, num};

#[derive(Debug, PartialEq)]
pub enum Commands {
    Increment,
    Decrement,
    MoveRight,
    MoveLeft,
    Print,
    Read,
    LeftBracket(usize),
    RightBracket(usize),
}

#[derive(Debug)]
pub struct Interpreter {
    pub data: Vec<num::Wrapping<u8>>,
    pub data_pointer: usize,
    pub inst: Vec<Commands>,
    pub inst_pointer: usize,
}

impl Interpreter {
    pub fn execute(&mut self) {
        while self.inst_pointer < self.inst.len() {
            match self.inst[self.inst_pointer] {
                Commands::Increment => self.data[self.data_pointer] += 1,
                Commands::Decrement => self.data[self.data_pointer] -= 1,
                Commands::MoveRight => self.data_pointer += 1,
                Commands::MoveLeft => self.data_pointer -= 1,
                Commands::Print => self.print(),
                Commands::Read => self.read(),
                Commands::LeftBracket(value) => {
                    if self.data[self.data_pointer] == std::num::Wrapping(0) {
                        self.inst_pointer = value;
                    }
                }
                Commands::RightBracket(value) => {
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

pub fn get_instruction_chars_from_file(src: std::path::PathBuf) -> Result<Vec<char>, io::Error> {
    let src_str = fs::read_to_string(src)?;
    let inst: Vec<char> = src_str
        .chars()
        .filter(|c| matches!(*c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();
    Ok(inst)
}

pub fn get_instructions(insts: &[char]) -> Result<Vec<Commands>, io::Error> {
    let mut cmds: Vec<Commands> = Vec::new();
    let mut stack = Vec::new();

    for inst in insts {
        match inst {
            '>' => cmds.push(Commands::MoveRight),
            '<' => cmds.push(Commands::MoveLeft),
            '+' => cmds.push(Commands::Increment),
            '-' => cmds.push(Commands::Decrement),
            '.' => cmds.push(Commands::Print),
            ',' => cmds.push(Commands::Read),
            '[' => {
                cmds.push(Commands::LeftBracket(0));
                stack.push(']');
            }
            ']' => {
                cmds.push(Commands::RightBracket(0));
                if stack.pop() != Some(']') {
                    return Err(Error::new(
                        ErrorKind::InvalidData,
                        "Parentheses unbalanced.",
                    ));
                }
            }
            _ => (),
        }
    }

    if !stack.is_empty() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Parentheses unbalanced.",
        ));
    }

    Ok(set_jumps(cmds))
}

fn set_jumps(mut cmds: Vec<Commands>) -> Vec<Commands> {
    let mut stack = Vec::new();

    for i in 0..cmds.len() {
        if matches!(cmds[i], Commands::LeftBracket(0)) {
            stack.push(i);
        } else if matches!(cmds[i], Commands::RightBracket(0)) {
            if let Some(left) = stack.pop() {
                cmds[left] = Commands::LeftBracket(i);
                cmds[i] = Commands::RightBracket(left);
            }
        }
    }
    cmds
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_files::TestFiles;

    #[test]
    fn test_get_instruction_chars_from_file() {
        let temp_dir = TestFiles::new();
        temp_dir.file("bf.b", "[abc<>.,]+-%_++#$+<>[fdasf]");
        let inst = vec![
            '[', '<', '>', '.', ',', ']', '+', '-', '+', '+', '+', '<', '>', '[', ']',
        ];
        assert_eq!(
            get_instruction_chars_from_file(temp_dir.path().join("bf.b")).unwrap(),
            inst
        );
    }

    #[test]
    fn test_get_instruction_chars_from_file_err_no_file() {
        let temp_dir = TestFiles::new();
        assert!(get_instruction_chars_from_file(temp_dir.path().join("nope.b")).is_err());
    }

    #[test]
    fn test_get_instructions_error_unbalanced() {
        let unbalanced = vec!['[', ']', '['];
        assert!(get_instructions(&unbalanced).is_err());
        let unbalanced = vec!['[', ']', ']'];
        assert!(get_instructions(&unbalanced).is_err());
    }

    #[test]
    fn test_get_instructions() {
        let balanced = vec!['[', '[', ']', '>', '-', '.', ',', '<', '+', ']'];
        let cmds_calculated = get_instructions(&balanced);
        let cmds = vec![
            Commands::LeftBracket(9),
            Commands::LeftBracket(2),
            Commands::RightBracket(1),
            Commands::MoveRight,
            Commands::Decrement,
            Commands::Print,
            Commands::Read,
            Commands::MoveLeft,
            Commands::Increment,
            Commands::RightBracket(0),
        ];
        assert_eq!(cmds, cmds_calculated.unwrap());
    }
}
