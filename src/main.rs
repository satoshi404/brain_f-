use brain::brain_fuck::{self, KindToken, BUFFER_SIZE};
use std::fs::read_to_string;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!(" 
    ==========================================
        -- Brainfuck interpreter --\n
    ==========================================
    Author: Satoshi\n
    Version: 1.0\n
    License: <Satoshi -_->\n
    Usage: brainfuck <file.bf>\n");
        std::process::exit(1);
    }
    let path: &str = &args[1];
    let source: String = read_to_string(path).expect("( main ) Error reading file");
    let mut cursor: usize = 0;
    let mut data: Vec<u8> = vec![0; BUFFER_SIZE]; 
    let mut tokens: Vec<KindToken> = brain_fuck::tokenizer(&source);
    
    brain_fuck::interpreter(&mut tokens, &mut data, &mut cursor);
    println!(); // print a newline
}
