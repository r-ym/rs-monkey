#[derive(Default, Debug)]
pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    rpos: usize,
    ch: char,
}

impl Lexer {
    fn new(s: &str) -> Self {
        let mut L = Lexer {
            input: s.chars().collect(),
            ..Lexer::default()
        };
        L.read_char();
        L
    }

    fn read_char(&mut self) -> () {
        if self.rpos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.rpos]
        }
        self.pos = self.rpos;
        self.rpos += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_next_token() {
        let mut lexer = Lexer::new("=+(){},;");
        assert_eq!(lexer.next_token(), Token::Equal);
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Lparen);
        assert_eq!(lexer.next_token(), Token::Rparen);
        assert_eq!(lexer.next_token(), Token::Lbrace);
        assert_eq!(lexer.next_token(), Token::Rbrace);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Semicolon);
    }
}
