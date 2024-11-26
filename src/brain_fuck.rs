use std::usize;
use std::io::{self, Read};

pub const BUFFER_SIZE: usize = 30000;

#[derive(Debug, PartialEq, Eq)]
pub enum KindToken {
    Comment,
    Plus,
    Minus,
    Dot,
    Comma,
    LeftBracket,
    RightBracket,
    LeftArrow,
    RightArrow,
}

pub fn tokenizer(path: &str) -> Vec<KindToken> {
    let mut tokens: Vec<KindToken> = Vec::new();
    let mut chars = path.chars().peekable(); // Use peekable to look ahead

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\n' | '\t' => continue, // Ignore whitespace
            '$' => {
                // Skip characters until the end of the line
                while let Some(next_ch) = chars.next() {
                    if next_ch == '\n' {
                        break; // End of comment
                    }
                }
                tokens.push(KindToken::Comment); // Add comment token
            }
            '+' => tokens.push(KindToken::Plus),
            '-' => tokens.push(KindToken::Minus),
            '.' => tokens.push(KindToken::Dot),
            ',' => tokens.push(KindToken::Comma),
            '[' => tokens.push(KindToken::LeftBracket),
            ']' => tokens.push(KindToken::RightBracket),
            '<' => tokens.push(KindToken::LeftArrow),
            '>' => tokens.push(KindToken::RightArrow),
            _ => panic!("(tokenizer) Token invalid: {}", ch),
        }
    }
    
    tokens
}


pub fn cursor_inc(cursor: &mut usize) {
    if *cursor < BUFFER_SIZE {
        *cursor += 1;
    } else {
        panic!("( cursor_increment ) cursor out of bounds");
    }
}

pub fn cursor_dec(cursor: &mut usize) {
    if *cursor > 0 {
        *cursor -= 1;
    } else {
        panic!("( cursor_decrement ) cursor < 0");
    }
}

pub fn cell_inc(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] += 1;
}

pub fn cell_dec(cells: &mut Vec<u8>, cursor: usize) {
    cells[cursor] -= 1;
}

pub fn cell_write(cells: &Vec<u8>, cursor: usize) {
    print!("{}", cells[cursor] as char); 
}

pub fn start_loop(cells: &Vec<u8>, cursor: &mut usize, tokens: &mut Vec<KindToken>, pc: &mut usize, loop_stack: &mut Vec<usize>) {
    if cells[*cursor] == 0 {
        let mut depth = 1;
        while depth > 0 && *pc < tokens.len() - 1 {
            *pc += 1;
            if tokens[*pc] == KindToken::LeftBracket {
                depth += 1;
            } else if tokens[*pc] == KindToken::RightBracket {
                depth -= 1;
            }
        }
    } else {
        loop_stack.push(*pc);
    }
}

pub fn end_loop(cells: &Vec<u8>, cursor: &mut usize, pc: &mut usize, loop_stack: &mut Vec<usize>) {
    if cells[*cursor] != 0 {
        if let Some(last) = loop_stack.last() {
            *pc = *last;
        }
    } else {
        loop_stack.pop();
    }
}

pub fn cell_input(cells: &mut Vec<u8>, cursor: usize) {
    let mut buff = [0;1];
    match io::stdin().read(&mut buff) {
        Ok(_) => {
            // Store the byte in the cells vector at the specified cursor position
            cells[cursor] = buff[0]; // buff[0] contains the byte read
        }
        Err(e) => {
            eprintln!("Error reading input: {}", e);
        }
    }
    cells[cursor] = buff[0];
}


pub fn interpreter(tokens: &mut Vec<KindToken>, cells: &mut Vec<u8>, cursor: &mut usize) {
    let mut pc: usize = 0; // Program counter
    let mut loop_stack:Vec<usize> = Vec::new();

    while pc < tokens.len() {
        match &tokens[pc] {
            KindToken::Comment => {} // Ignore comments
            KindToken::Plus => cell_inc(cells, *cursor),
            KindToken::Minus => cell_dec(cells, *cursor),
            KindToken::Dot => cell_write(cells, *cursor), 
            KindToken::Comma => cell_input(cells, *cursor),
            KindToken::LeftBracket => start_loop(cells, cursor, tokens, &mut pc, &mut loop_stack),
            KindToken::RightBracket => end_loop(cells, cursor, &mut pc, &mut loop_stack),
            KindToken::LeftArrow => cursor_dec(cursor),
            KindToken::RightArrow => cursor_inc(cursor),
        }
        pc += 1; // Increment program counter
    }
}
