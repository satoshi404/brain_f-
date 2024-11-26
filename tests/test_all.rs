use brain::brain_fuck::{self, BUFFER_SIZE};

const TOKENS_COUNT: usize = 9; // Size tokens kind

#[cfg(test)]
mod tests {
    use brain_fuck::KindToken;

    use super::*;
    use std::panic::catch_unwind;

    #[test]
    fn test_tokenizer() {
        let source:String = "+-.[+,+]<..>".to_string();
        let tokens:Vec<KindToken> = brain_fuck::tokenizer(&source);

        // Verify the number of tokens is correct
        assert_eq!(tokens.len(), 12);

        // Verify all tokens are of the correct type
        let kinds = vec![
            brain_fuck::KindToken::Plus,
            brain_fuck::KindToken::Minus,
            brain_fuck::KindToken::Dot,
            brain_fuck::KindToken::LeftBracket,
            brain_fuck::KindToken::Plus,
            brain_fuck::KindToken::Comma,
            brain_fuck::KindToken::Plus,
            brain_fuck::KindToken::RightBracket,
            brain_fuck::KindToken::LeftArrow,
            brain_fuck::KindToken::Dot,
            brain_fuck::KindToken::Dot,
            brain_fuck::KindToken::RightArrow,
        ];

        for (i, token) in tokens.iter().enumerate() {
            assert_eq!(*token, kinds[i]);
        }
    }

    #[test]
    fn test_cursor_operations() {
        let mut cursor:usize = 0;

        brain_fuck::cursor_inc(&mut cursor);
        assert_eq!(cursor * 2, 2); // cursor (1)

        brain_fuck::cursor_inc(&mut cursor);
        assert_eq!(cursor + 4, 6); // cursor (2)

        brain_fuck::cursor_dec(&mut cursor);
        assert_eq!((cursor + 3) * 2, 8); // cursor (1)

        brain_fuck::cursor_inc(&mut cursor);
        assert_eq!(cursor - 1, 1); // cursor (2)  

        brain_fuck::cursor_dec(&mut cursor);
        assert_eq!(cursor + 3 , 4); // cursor (1)

        brain_fuck::cursor_dec(&mut cursor);
        assert_eq!(cursor + 2, 2); // cursor (0)

        // Test out-of-bounds increment
        for _ in 0..BUFFER_SIZE {
            brain_fuck::cursor_inc(&mut cursor);
        }
        assert_eq!(cursor, BUFFER_SIZE);

        let cursor2 = BUFFER_SIZE;
        let result = catch_unwind(|| {
            let mut cursor2 = cursor2;
            brain_fuck::cursor_inc(&mut cursor2);
        });
        assert!(result.is_err());

        // Test out-of-bounds decrement
        for _ in 0..BUFFER_SIZE {
            brain_fuck::cursor_dec(&mut cursor);
        }
        assert_eq!(cursor, 0);

        let cursor3 = 0;
        let result = catch_unwind(|| {
            let mut cursor3 = cursor3;
            brain_fuck::cursor_dec(&mut cursor3);
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_cell_operations() {
        let mut cells:Vec<u8> = vec![0; BUFFER_SIZE];
        let cursor: usize = 0;

        brain_fuck::cell_inc(&mut cells, cursor);
        assert_eq!(cells[cursor], 1);
        brain_fuck::cell_dec(&mut cells, cursor);
        assert_eq!(cells[cursor], 0);
    }

    #[test]
    fn test_count_tokens() {
        let test_tokens:String = "+,-[<.>]".to_string();
        assert_eq!(test_tokens.len(), TOKENS_COUNT);
    }
}
