#[cfg(test)]
mod tests;


type TokenType<'a> = &'a str;

pub struct Token<'a> {
    pub Type: TokenType<'a>,
    pub Literal: String,
}

impl<'a> Token<'a> {
    pub fn new_token(token_type: &'a str, ch: &char) -> Self {
        Token {
            Type: token_type,
            Literal: ch.to_string(),
        }
    }
    pub fn look_up_ident(ident: &str) -> TokenType<'a> {
        match ident {
            "fn" => { FUNCTION },
            "let" => { LET },
            _ => { IDENT },
        }
    }
}

pub const ILLEGAL: &'static str = "ILLEGAL";
pub const EOF: &'static str = "EOF";

pub const IDENT: &'static str = "IDENT";
pub const INT: &'static str = "INT";

pub const ASSIGN: &'static str = "=";
pub const PLUS: &'static str = "+";

pub const COMMA: &'static str = ",";
pub const SEMICOLON: &'static str = ";";

pub const LPAREN: &'static str = "(";
pub const RPAREN: &'static str = ")";
pub const LBRACE: &'static str = "{";
pub const RBRACE: &'static str = "}";

pub const FUNCTION: &'static str = "FUNCTION";
pub const LET: &'static str = "LET";
