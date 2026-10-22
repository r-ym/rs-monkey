#[derive(Default, Debug)]
pub struct Lexer<'a> {
    input: &'a str,
    pos: usize,
    rpos: usize,
    ch: &'a [u8],
}

impl<'a> Lexer<'a> {
    fn new(s: &'a str) -> Self {
        Lexer {
            input: s,
            ..Lexer::default()
        }
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
