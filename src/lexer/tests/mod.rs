use crate::token::*;
use crate::lexer::*;

#[test]
fn test_next_token1() {
    let input = "=+(){},;";
    let tests: Vec<(&str, String)> = vec![
        (ASSIGN, "=".to_string()),
        (PLUS, "+".to_string()),
        (LPAREN, "(".to_string()),
        (RPAREN, ")".to_string()),
        (LBRACE, "{".to_string()),
        (RBRACE, "}".to_string()),
        (COMMA, ",".to_string()),
        (SEMICOLON, ";".to_string()),
        (EOF, "".to_string()),
    ];
    let mut lexer = Lexer::new(input);
    for (_, test) in tests.into_iter().enumerate() {
        let tok = lexer.next_token();
        assert_eq!(tok.Type, test.0);
        assert_eq!(tok.Literal, test.1);
    }
}

#[test]
fn test_next_token2() {
    let input = "\
    let five = 5;
    let six = 60;
    ";
    let tests: Vec<(&str, String)> = vec![
        (LET, "let".to_string()),
        (IDENT, "five".to_string()),
        (ASSIGN, "=".to_string()),
        (INT, "5".to_string()),
        (SEMICOLON, ";".to_string()),
        (LET, "let".to_string()),
        (IDENT, "six".to_string()),
        (ASSIGN, "=".to_string()),
        (INT, "60".to_string()),
        (SEMICOLON, ";".to_string()),
    ];
    let mut lexer = Lexer::new(input);
    for test in tests {
        let tok = lexer.next_token();
        assert_eq!(tok.Type, test.0);
        assert_eq!(tok.Literal, test.1);
    }
}