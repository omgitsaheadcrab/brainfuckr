//! brainfuck source code parsing

use std::fs;
use std::io::{self, Error, ErrorKind};

/// Enumeration of all valid brainfuck commands.
#[derive(Debug, PartialEq)]
pub enum Commands {
    /// Increment the byte at the data pointer by one.
    ///
    /// * maps to `+`
    Increment,
    /// Decrement the byte at the data pointer by one.
    ///
    /// * maps to `-`
    Decrement,
    /// Move the data pointer to the right by one.
    ///
    /// * maps to `>`
    MoveRight,
    /// Move the data pointer to the left by one.
    ///
    /// * maps to `<`
    MoveLeft,
    /// Print the byte at the location of the data pointer.
    ///
    /// * maps to `.`
    Print,
    /// Read one byte and at the location of the data pointer.
    ///
    /// * maps to `,`
    Read,
    /// Opening bracket, jump to the closing bracket if the byte at the data pointer is zero.
    ///
    /// The discriminant stores the location of the closing bracket.
    ///
    /// * maps to `[`
    LeftBracket(usize),
    /// Closing bracket, jump to the opening bracket if the byte at the data pointer is non-zero.
    ///
    /// The discriminant stores the location of the opening bracket.
    ///
    /// * maps to `]`
    RightBracket(usize),
}

/// Return valid brainfuck instruction set from a specified source file
pub fn get_instruction_chars_from_file(src: std::path::PathBuf) -> Result<Vec<char>, io::Error> {
    let src_str = fs::read_to_string(src)?;
    let inst: Vec<char> = src_str
        .chars()
        .filter(|c| matches!(*c, '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']'))
        .collect();
    Ok(inst)
}

/// Return [Commands](Commands) mapped from instruction set
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

/// Set instruction pointer jumps for opening and closing brackets
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
