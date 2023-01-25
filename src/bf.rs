use std::fs;

enum Commands {
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
    pub array: Vec<u8>,
    pub inst_array: Vec<char>,
    pub inst_pointer: usize,
    pub data_pointer: usize,
}

pub fn get_instructions_from_file(src: std::path::PathBuf) -> Vec<char> {
    let src_str = fs::read_to_string(src).expect("File not found.");

    let inst: Vec<char> = src_str
    .chars()
    .filter(|c| match *c {
        '>' | '<' | '+' | '-' | '.' | ',' | '[' | ']' => true,
        _ => false,
    })
    .collect();
    inst
}

// check if all parens in the input instructions are matched to fail quickly
// with a syntax error otherwise.
// fn balanced_parens() -> bool

// given current paren index and type, return matching index
// fn get_matching_paren() -> usize

//
// fn execute()
