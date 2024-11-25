use brain::brain_fuck::{self, KindToken, BUFFER_SIZE};



fn main() {
    let source: String = ">++++++++[<+++++++++>-]<.>++++[<+++++++>-]<+.+++++++..+++.>>++++++[<+++++++>-]<++.------------.>++++++[<+++++++++>-]<+.<.+++.------.--------.>>>++++[<++++++++>-]<+.,.".to_string();
    let mut cursor: usize = 0;
    let mut data: Vec<u8> = vec![0; BUFFER_SIZE]; 
    let mut tokens: Vec<KindToken> = brain_fuck::tokenizer(&source);
    
    brain_fuck::interpreter(&mut tokens, &mut data, &mut cursor);
    println!(); // print a newline
}
