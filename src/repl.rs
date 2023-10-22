use crate::lexer::Lexer;
use crate::token::Token;
use std::io::{self, Write};

const PROMPT: &str = ">> ";

pub fn start() {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() || line == "\n" {
            continue;
        }

        let mut lexer = Lexer::new(line);

        loop {
            let token = lexer.next_token();

            if token == Token::Eof {
                break;
            }

            println!("Token: {:?}", token);
        }
    }
}
