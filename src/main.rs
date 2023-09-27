use clap::Parser;

use std::num;

mod cli;
use cli::Arguments;

mod bf;
use bf::*;

fn main() {
    let args = Arguments::parse();
    let instruction_chars = get_instruction_chars_from_file(args.src);
    let mut interpreter = Interpreter {
        data: vec![num::Wrapping(0); args.arr],
        data_pointer: 0,
        inst: get_instructions(&instruction_chars),
        inst_pointer: 0,
    };
    interpreter.execute();
}
