pub struct Lexer {
    input: Vec<char>,
    pos: usize,
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
