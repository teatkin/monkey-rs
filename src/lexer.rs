use crate::token::{lookup_ident, Token};

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: Option<char>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: None,
        };

        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            None => Token::Eof,
            Some(c) => match c {
                '=' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::Equal
                    }
                    _ => Token::Assign,
                },
                ';' => Token::Semicolon,
                '(' => Token::Lparen,
                ')' => Token::Rparen,
                '{' => Token::Lbrace,
                '}' => Token::Rbrace,
                ',' => Token::Comma,
                '+' => Token::Plus,
                '-' => Token::Minus,
                '!' => match self.peek_char() {
                    Some('=') => {
                        self.read_char();
                        Token::NotEqual
                    }
                    _ => Token::Bang,
                },
                '*' => Token::Asterisk,
                '/' => Token::Slash,
                '<' => Token::Lt,
                '>' => Token::Gt,
                _ => {
                    if Self::is_letter(c) {
                        let literal = self.read_identifier();
                        return lookup_ident(&literal);
                    } else if c.is_ascii_digit() {
                        return Token::Int(self.read_number());
                    } else {
                        return Token::Illegal(c.to_string());
                    }
                }
            },
        };

        self.read_char();
        token
    }

    fn read_char(&mut self) {
        self.ch = self.peek_char();

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_identifier(&mut self) -> String {
        let pos = self.position;
        while let Some(c) = self.ch {
            if Self::is_letter(c) {
                self.read_char();
            } else {
                break;
            }
        }

        self.read_range(pos, self.position)
    }

    /// Read a range of characters from a string
    ///
    /// * `start` - The index to start reading range from. This value is inclusive.
    /// * `end` - The index to stop reading range from. This value is exclusive.
    fn read_range(&mut self, start: usize, end: usize) -> String {
        self.input
            .chars()
            .skip(start)
            .take(end - start)
            .collect::<String>()
    }

    fn read_number(&mut self) -> String {
        let pos = self.position;

        while let Some(c) = self.ch {
            if c.is_ascii_digit() {
                self.read_char();
            } else {
                break;
            }
        }

        self.read_range(pos, self.position)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.ch {
            if c.is_whitespace() {
                self.read_char();
            } else {
                break;
            }
        }
    }

    fn peek_char(&mut self) -> Option<char> {
        if self.read_position >= self.input.chars().count() {
            None
        } else {
            self.input.chars().nth(self.read_position)
        }
    }

    /// Checks whether a character is in Monkey's permitted alphabet
    ///
    /// * `ch` - The character to check
    fn is_letter(ch: char) -> bool {
        ch.is_ascii_alphabetic() || ch == '_'
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let input = "let five = 5;
let ten = 10;

let add = fn(x, y) {
    x + y;
};

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
";

        let mut l = Lexer::new(input.to_string());

        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("five".into()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Int("5".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("ten".into()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("add".into()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Function);
        assert_eq!(l.next_token(), Token::Lparen);
        assert_eq!(l.next_token(), Token::Ident("x".into()));
        assert_eq!(l.next_token(), Token::Comma);
        assert_eq!(l.next_token(), Token::Ident("y".into()));
        assert_eq!(l.next_token(), Token::Rparen);
        assert_eq!(l.next_token(), Token::Lbrace);
        assert_eq!(l.next_token(), Token::Ident("x".into()));
        assert_eq!(l.next_token(), Token::Plus);
        assert_eq!(l.next_token(), Token::Ident("y".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Rbrace);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Let);
        assert_eq!(l.next_token(), Token::Ident("result".into()));
        assert_eq!(l.next_token(), Token::Assign);
        assert_eq!(l.next_token(), Token::Ident("add".into()));
        assert_eq!(l.next_token(), Token::Lparen);
        assert_eq!(l.next_token(), Token::Ident("five".into()));
        assert_eq!(l.next_token(), Token::Comma);
        assert_eq!(l.next_token(), Token::Ident("ten".into()));
        assert_eq!(l.next_token(), Token::Rparen);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Bang);
        assert_eq!(l.next_token(), Token::Minus);
        assert_eq!(l.next_token(), Token::Slash);
        assert_eq!(l.next_token(), Token::Asterisk);
        assert_eq!(l.next_token(), Token::Int("5".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Int("5".into()));
        assert_eq!(l.next_token(), Token::Lt);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::Gt);
        assert_eq!(l.next_token(), Token::Int("5".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::If);
        assert_eq!(l.next_token(), Token::Lparen);
        assert_eq!(l.next_token(), Token::Int("5".into()));
        assert_eq!(l.next_token(), Token::Lt);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::Rparen);
        assert_eq!(l.next_token(), Token::Lbrace);
        assert_eq!(l.next_token(), Token::Return);
        assert_eq!(l.next_token(), Token::True);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Rbrace);
        assert_eq!(l.next_token(), Token::Else);
        assert_eq!(l.next_token(), Token::Lbrace);
        assert_eq!(l.next_token(), Token::Return);
        assert_eq!(l.next_token(), Token::False);
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Rbrace);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::Equal);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Int("10".into()));
        assert_eq!(l.next_token(), Token::NotEqual);
        assert_eq!(l.next_token(), Token::Int("9".into()));
        assert_eq!(l.next_token(), Token::Semicolon);
        assert_eq!(l.next_token(), Token::Eof);
    }
}
