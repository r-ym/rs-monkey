pub struct Lexer {
    input: String,
    pos: usize,
    rpos: usize,
    ch: char,
}

impl Lexer {
    fn new(s: &str) -> Self {
        Lexer { input: s, pos: 0 }
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
