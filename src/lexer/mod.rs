#[cfg(test)]
mod tests;
use crate::token::*;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    read_position: usize,
    ch: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut lexer = Lexer {
            input: input,
            position: 0,
            read_position: 0,
            ch: '\0',
        };
        lexer.read_char();
        lexer
    }
    // '\0' == NUL(in ASCII)
    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input.chars().nth(self.read_position).unwrap();
        }
        self.position = self.read_position;
        self.read_position += 1;
    }
    
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token: Token;
        match self.ch {
            '=' => token = Token::new_token(ASSIGN, &self.ch),
            ';' => token = Token::new_token(SEMICOLON, &self.ch),
            '(' => token = Token::new_token(LPAREN, &self.ch),
            ')' => token = Token::new_token(RPAREN, &self.ch),
            '{' => token = Token::new_token(LBRACE, &self.ch),
            '}' => token = Token::new_token(RBRACE, &self.ch),
            ',' => token = Token::new_token(COMMA, &self.ch),
            '+' => token = Token::new_token(PLUS, &self.ch),
            '\0' => token = Token { Type: EOF, Literal: "".to_string(), },
            // This means head of ident is not digit.
            'a'...'z'| '_' => {
                let token_string = self.read_identifier();
                let token_type = Token::look_up_ident(&token_string);
                token = Token { Type: token_type, Literal: token_string, };
                return token; // since read_identifier done read_char...
            },
            // This means digit is just INT, not float.
            '0'...'9' => {
                let token_string = self.read_digit(); 
                let token_type = INT;
                token = Token { Type: token_type, Literal: token_string, };
                return token; // since read_digit done read_char...
            }
            _ => token = Token::new_token(ILLEGAL, &self.ch),
        }
        self.read_char();
        token
    }
    fn read_identifier(&mut self) -> String {
        let position = self.position;
        while isLetter(&self.ch) {
            self.read_char();
        }
        self.input.get(position..self.position).unwrap().to_string()
    }
    fn read_digit(&mut self) -> String {
        let position = self.position;
        loop {
            match self.ch {
                '0' ... '9' => { self.read_char(); },
                _ => { break; }
            }
        }
        self.input.get(position..self.position).unwrap().to_string()
    }
    fn skip_whitespace(&mut self) {
        // \t \n ' ' etc...
        while self.ch.is_whitespace() {
            self.read_char()
        }
    }
}
// helper 
fn isLetter(c: &char) -> bool { 'a' <= *c && *c <= 'z' || *c == '_' }