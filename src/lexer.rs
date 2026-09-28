use crate::tokens::Token;

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            input: source.chars().collect(),
            position: 0,
        }
    }

    // know the current char
    fn current_char(&self) -> Option<char> {
        if self.position >= self.input.len() {
            None
        } else {
            Some(self.input[self.position])
        }
    }

    // add 1 to position to get the next char
    fn advance(&mut self) {
        self.position += 1;
    }

    // remove whitespaces if exists
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if c.is_whitespace() {
                self.advance();
            } else {
                break;
            }
        }
    }

    // read number from input
    pub fn read_number(&mut self) -> Token {
        let start_pos = self.position - 1;
        let mut is_float = false;

        while let Some(c) = self.current_char() {
            if c.is_ascii_digit() {
                self.advance();
            } else if c == '.' && !is_float {
                is_float = true;
                self.advance();
            } else {
                break;
            }
        }

        let number_str: String = self.input[start_pos..self.position].iter().collect();

        if is_float {
            let value: f64 = number_str.parse().unwrap();
            Token::FloatLiteral(value)
        } else {
            let value: i64 = number_str.parse().unwrap();
            Token::IntLiteral(value)
        }
    }

    // read if there built in words
    pub fn read_word(&mut self) -> Token {
        let start_pos = self.position - 1;

        while let Some(c) = self.current_char() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        let word: String = self.input[start_pos..self.position].iter().collect();

        match word.as_str() {
            "let" => Token::Let,
            "fun" => Token::Fun,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "to" => Token::To,
            "return" => Token::Return,
            "const" => Token::Const,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "as" => Token::As,
            "int" => Token::IntType,
            "float" => Token::FloatType,
            "bool" => Token::BoolType,
            "str" => Token::StringType,
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),
            "None" => Token::None,
            _ => Token::Identifier(word),
        }
    }

    // read string
    pub fn read_string(&mut self) -> Token {
        let mut string_val = String::new();

        while let Some(c) = self.current_char() {
            if c == '"' {
                break;
            } else if c == '\\' {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    match escaped {
                        'n' => string_val.push('\n'),
                        't' => string_val.push('\t'),
                        'r' => string_val.push('\r'),
                        '\\' => string_val.push('\\'),
                        '"' => string_val.push('"'),
                        _ => {
                            string_val.push('\\');
                            string_val.push(escaped);
                        }
                    }
                    self.advance();
                }
            } else {
                string_val.push(c);
                self.advance();
            }
        }

        if self.current_char() == Some('"') {
            self.advance();
        }

        Token::StringLiteral(string_val)
    }

    // match the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let ch = match self.current_char() {
            Some(c) => c,
            None => return Token::Eof,
        };

        self.advance();

        match ch {
            '+' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::PlusEqual
                } else {
                    Token::Plus
                }
            }
            '-' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::MinusEqual
                } else if self.current_char() == Some('>') {
                    self.advance();
                    Token::Arrow
                } else {
                    Token::Minus
                }
            }
            '*' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::AsteriskEqual
                } else {
                    Token::Asterisk
                }
            }
            '/' => {
                if self.current_char() == Some('/') {
                    self.advance();
                    while let Some(c) = self.current_char() {
                        if c == '\n' {
                            break;
                        }
                        self.advance();
                    }
                    self.next_token()
                } else if self.current_char() == Some('*') {
                    self.advance();
                    while let Some(c) = self.current_char() {
                        if c == '*' {
                            self.advance();
                            if self.current_char() == Some('/') {
                                self.advance();
                                break;
                            }
                        } else {
                            self.advance();
                        }
                    }
                    self.next_token()
                } else if self.current_char() == Some('=') {
                    self.advance();
                    Token::SlashEqual
                } else {
                    Token::Slash
                }
            }
            '%' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::PercentEqual
                } else {
                    Token::Percent
                }
            }
            '^' => Token::Caret,
            '=' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::Equals
                } else {
                    Token::Assign
                }
            }
            '>' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::GreaterEqual
                } else {
                    Token::GreaterThan
                }
            }
            '<' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::LessEqual
                } else {
                    Token::LessThan
                }
            }
            '!' => {
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::BangEqual
                } else {
                    Token::Not
                }
            }
            '&' => {
                if self.current_char() == Some('&') {
                    self.advance();
                    Token::And
                } else {
                    panic!("Unexpected token & (expected &&)");
                }
            }
            '|' => {
                if self.current_char() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    panic!("Unexpected token | (expected ||)");
                }
            }
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '?' => Token::Question,
            '(' => Token::LParen,
            ')' => Token::RParen,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '"' => self.read_string(),
            _ => {
                if ch.is_ascii_digit() {
                    self.read_number()
                } else if ch.is_alphabetic() || ch == '_' {
                    self.read_word()
                } else {
                    panic!("Unexpected character: {}", ch);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variable_declaration() {
        let input = "let age: int = 25;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("age".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::IntType);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::IntLiteral(25));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_function_declaration() {
        let input = "fun add(a: int, b: int) -> int { return a + b; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Fun);
        assert_eq!(lexer.next_token(), Token::Identifier("add".to_string()));
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::IntType);
        assert_eq!(lexer.next_token(), Token::Comma);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::IntType);
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Arrow);
        assert_eq!(lexer.next_token(), Token::IntType);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_control_flow_and_comparisons() {
        let input = "if age >= 18 { return true; } else { return false; }";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::If);
        assert_eq!(lexer.next_token(), Token::Identifier("age".to_string()));
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
        assert_eq!(lexer.next_token(), Token::IntLiteral(18));
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::BooleanLiteral(true));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Else);
        assert_eq!(lexer.next_token(), Token::LBrace);
        assert_eq!(lexer.next_token(), Token::Return);
        assert_eq!(lexer.next_token(), Token::BooleanLiteral(false));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::RBrace);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_compound_assignments() {
        let input = "+= -= *= /= %=";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::PlusEqual);
        assert_eq!(lexer.next_token(), Token::MinusEqual);
        assert_eq!(lexer.next_token(), Token::AsteriskEqual);
        assert_eq!(lexer.next_token(), Token::SlashEqual);
        assert_eq!(lexer.next_token(), Token::PercentEqual);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_logical_operators_and_comparisons() {
        let input = "&& || ! == != <= < >";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::And);
        assert_eq!(lexer.next_token(), Token::Or);
        assert_eq!(lexer.next_token(), Token::Not);
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::BangEqual);
        assert_eq!(lexer.next_token(), Token::LessEqual);
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::GreaterThan);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_string_and_boolean_variables() {
        let input = "let name: str = \"Mohamed\\n\\t\\\"Awakey\\\"\"; let active: bool = false;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("name".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::StringType);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(
            lexer.next_token(),
            Token::StringLiteral("Mohamed\n\t\"Awakey\"".to_string())
        );
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("active".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::BoolType);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::BooleanLiteral(false));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_comments_and_whitespaces() {
        let input = "
            // single line comment
            let x: int = 10; // comment after code
            /* multi
               line
               comment */
            let y: float = 3.14;
        ";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::IntType);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::IntLiteral(10));
        assert_eq!(lexer.next_token(), Token::Semicolon);

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("y".to_string()));
        assert_eq!(lexer.next_token(), Token::Colon);
        assert_eq!(lexer.next_token(), Token::FloatType);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::FloatLiteral(3.14));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }

    #[test]
    fn test_edge_case_spaces_between_compound_operators() {
        let input = "+ = - = * = / = % = = = ! = < = > = & & | | - >";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Plus);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Minus);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Asterisk);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Slash);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Percent);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::Not);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::LessThan);
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::GreaterThan);
        assert_eq!(lexer.next_token(), Token::Assign);
    }

    #[test]
    fn test_violent_mixed_operators() {
        let input = "let x = (a += b) && (c == d) || !(e >= f) ^ 2 % 5;";
        let mut lexer = Lexer::new(input);

        assert_eq!(lexer.next_token(), Token::Let);
        assert_eq!(lexer.next_token(), Token::Identifier("x".to_string()));
        assert_eq!(lexer.next_token(), Token::Assign);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Identifier("a".to_string()));
        assert_eq!(lexer.next_token(), Token::PlusEqual);
        assert_eq!(lexer.next_token(), Token::Identifier("b".to_string()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::And);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Identifier("c".to_string()));
        assert_eq!(lexer.next_token(), Token::Equals);
        assert_eq!(lexer.next_token(), Token::Identifier("d".to_string()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Or);
        assert_eq!(lexer.next_token(), Token::Not);
        assert_eq!(lexer.next_token(), Token::LParen);
        assert_eq!(lexer.next_token(), Token::Identifier("e".to_string()));
        assert_eq!(lexer.next_token(), Token::GreaterEqual);
        assert_eq!(lexer.next_token(), Token::Identifier("f".to_string()));
        assert_eq!(lexer.next_token(), Token::RParen);
        assert_eq!(lexer.next_token(), Token::Caret);
        assert_eq!(lexer.next_token(), Token::IntLiteral(2));
        assert_eq!(lexer.next_token(), Token::Percent);
        assert_eq!(lexer.next_token(), Token::IntLiteral(5));
        assert_eq!(lexer.next_token(), Token::Semicolon);
        assert_eq!(lexer.next_token(), Token::Eof);
    }
}
