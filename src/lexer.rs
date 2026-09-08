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
            "int" => Token::IntType,
            "float" => Token::FloatType,
            "bool" => Token::BoolType,
            "string" => Token::StringType,
            "true" => Token::BooleanLiteral(true),
            "false" => Token::BooleanLiteral(false),
            _ => Token::Identifier(word),
        }
    }

    // read string
    pub fn read_string(&mut self) -> Token {
        let start_position = self.position;

        while let Some(c) = self.current_char() {
            if c != '"' {
                self.advance();
            } else {
                break;
            }
        }

        let words: String = self.input[start_position..self.position].iter().collect();

        if self.current_char() != Some('"') {
            return Token::Eof;
        }

        self.advance();

        Token::StringLiteral(words)
    }

    // get the next token
    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let current = self.current_char();

        if current.is_none() {
            return Token::Eof;
        }

        let c = current.unwrap();

        self.advance();

        match c {
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
                if self.current_char() == Some('=') {
                    self.advance();
                    Token::SlashEqual
                } else if self.current_char() == Some('/') {
                    self.advance();
                    while let Some(c) = self.current_char() {
                        if c != '\n' {
                            self.advance();
                        } else {
                            break;
                        }
                    }
                    self.next_token()
                } else if self.current_char() == Some('*') {
                    self.advance();

                    while let Some(c) = self.current_char() {
                        if c == '*' {
                            self.advance();
                            if self.current_char() == Some('/') {
                                self.advance();
                                return self.next_token();
                            }
                        } else {
                            self.advance();
                        }
                    }

                    // return Token::Eof;
                    panic!("Lexer Error: Unterminated block comment! You forgot to close /*");
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
                    Token::Eof
                }
            }
            '|' => {
                if self.current_char() == Some('|') {
                    self.advance();
                    Token::Or
                } else {
                    Token::Eof
                }
            }
            ':' => Token::Colon,
            ';' => Token::Semicolon,
            ',' => Token::Comma,
            '{' => Token::LBrace,
            '}' => Token::RBrace,
            '(' => Token::LParen,
            ')' => Token::RParen,
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
            '"' => self.read_string(),
            c if c.is_ascii_digit() => self.read_number(),
            c if c.is_alphanumeric() || c == '_' => self.read_word(),
            _ => Token::Eof,
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/lexer_test.rs"]
mod tests;
