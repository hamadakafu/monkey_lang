#[cfg(test)]
mod tests;


// type TokenType<'a> = &'a str;
#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL,
    EOF,
    
    IDENT,
    INT,
    
    ASSIGN,
    PLUS,
    
    COMMA,
    SEMICOLON,
    
    LPAREN,
    RPAREN,
    LBRACE,
    RBRACE,
    
    FUNCTION,
    LET,
}

pub struct Token {
    pub Type: TokenType,
    pub Literal: String,
}

impl Token {
    pub fn new_token(token_type: TokenType, ch: &char) -> Self {
        Token {
            Type: token_type,
            Literal: ch.to_string(),
        }
    }
    pub fn look_up_ident(ident: &str) -> TokenType {
        match ident {
            "fn" => { TokenType::FUNCTION },
            "let" => { TokenType::LET },
            _ => { TokenType::IDENT },
        }
    }
}
