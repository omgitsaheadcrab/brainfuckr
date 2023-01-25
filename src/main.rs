mod cli;
use cli::Arguments;
use clap::Parser;

mod bf;
use bf::{Interpreter, get_instructions_from_file};

fn main() {
    let args = Arguments::parse();

    let interpreter = Interpreter {
        array: vec![0; args.arr],
        inst_array: get_instructions_from_file(args.src),
        inst_pointer: 0,
        data_pointer: 0,
    };
    println!("{:?}", interpreter);
}
