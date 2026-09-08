use crate::ast::{Expr, Program, Stmt};
use crate::lexer::Lexer;
use crate::tokens::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest = 1,
    Assign,
    Or,
    And,
    Equals,
    LessGreater,
    Sum,
    Product,
    Exponent,
    Prefix,
}

pub struct Parser {
    lexer: Lexer,
    pub current_token: Token,
    pub peek_token: Token,
    pub errors: Vec<String>,
}

impl Parser {
    pub fn new(mut lexer: Lexer) -> Self {
        let current_token = lexer.next_token();
        let peek_token = lexer.next_token();

        Parser {
            lexer,
            current_token,
            peek_token,
            errors: Vec::new(),
        }
    }

    pub fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program {
            statement: Vec::new(),
        };

        while self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                program.statement.push(stmt);
            }

            self.next_token();
        }
        program
    }

    pub fn peek_error(&mut self, expected: &str) {
        let message = format!(
            "Syntax Error: Expected {} but got {:?}",
            expected, self.peek_token
        );

        self.errors.push(message);
    }

    pub fn peek_precedence(&self) -> Precedence {
        match self.peek_token {
            Token::Or => Precedence::Or,
            Token::And => Precedence::And,
            Token::Equals | Token::BangEqual => Precedence::Equals,
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual => {
                Precedence::LessGreater
            }
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash | Token::Percent => Precedence::Product,
            Token::Caret => Precedence::Exponent,
            Token::Assign
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::AsteriskEqual
            | Token::SlashEqual
            | Token::PercentEqual => Precedence::Assign,
            _ => Precedence::Lowest,
        }
    }

    pub fn current_precedence(&self) -> Precedence {
        match self.current_token {
            Token::Or => Precedence::Or,
            Token::And => Precedence::And,
            Token::Equals | Token::BangEqual => Precedence::Equals,
            Token::LessThan | Token::LessEqual | Token::GreaterThan | Token::GreaterEqual => {
                Precedence::LessGreater
            }
            Token::Plus | Token::Minus => Precedence::Sum,
            Token::Asterisk | Token::Slash | Token::Percent => Precedence::Product,
            Token::Caret => Precedence::Exponent,
            Token::Assign
            | Token::PlusEqual
            | Token::MinusEqual
            | Token::AsteriskEqual
            | Token::SlashEqual
            | Token::PercentEqual => Precedence::Assign,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_infix_expression(&mut self, left: Expr) -> Option<Expr> {
        let operator = match &self.current_token {
            Token::Plus => "+".to_string(),
            Token::Minus => "-".to_string(),
            Token::Asterisk => "*".to_string(),
            Token::Slash => "/".to_string(),
            Token::Percent => "%".to_string(),
            Token::Caret => "^".to_string(),
            Token::GreaterThan => ">".to_string(),
            Token::LessThan => "<".to_string(),
            Token::GreaterEqual => ">=".to_string(),
            Token::LessEqual => "<=".to_string(),
            Token::Equals => "==".to_string(),
            Token::BangEqual => "!=".to_string(),
            Token::And => "&&".to_string(),
            Token::Or => "||".to_string(),
            Token::Assign => "=".to_string(),
            Token::PlusEqual => "+=".to_string(),
            Token::MinusEqual => "-=".to_string(),
            Token::AsteriskEqual => "*=".to_string(),
            Token::SlashEqual => "/=".to_string(),
            Token::PercentEqual => "%=".to_string(),
            _ => return None,
        };

        let precedence = self.current_precedence();

        self.next_token();

        let right = match self.expression_parser(precedence) {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expr::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_prefix_expression(&mut self) -> Option<Expr> {
        let operator = match &self.current_token {
            Token::Minus => "-".to_string(),
            Token::Not => "!".to_string(),
            _ => return None,
        };

        self.next_token();

        let right = match self.expression_parser(Precedence::Prefix) {
            Some(expr) => expr,
            None => return None,
        };

        Some(Expr::Prefix {
            operator,
            right: Box::new(right),
        })
    }

    fn parse_grouped_expression(&mut self) -> Option<Expr> {
        self.next_token();

        let expr = self.expression_parser(Precedence::Lowest);

        if self.peek_token != Token::RParen {
            self.peek_error(")");
            return None;
        }

        self.next_token();

        expr
    }

    // Expression Parser
    fn expression_parser(&mut self, precedence: Precedence) -> Option<Expr> {
        // 1. اقرأ الطرف الشمال (زي ما إنت كنت عامل بالظبط)
        let mut left_expr = match &self.current_token {
            Token::IntLiteral(value) => Some(Expr::IntLiteral(*value)),
            Token::FloatLiteral(value) => Some(Expr::FloatLiteral(*value)),
            Token::StringLiteral(value) => Some(Expr::StringLiteral(value.clone())),
            Token::BooleanLiteral(value) => Some(Expr::BooleanLiteral(*value)),
            Token::Identifier(value) => Some(Expr::Identifier(value.clone())),
            Token::Minus | Token::Not => self.parse_prefix_expression(),
            Token::LParen => self.parse_grouped_expression(),
            _ => {
                self.peek_error("Expression (Number, String, Boolean, or Variable)");
                None
            }
        };

        if left_expr.is_none() {
            return None;
        }

        while self.peek_token != Token::Eof && precedence < self.peek_precedence() {
            self.next_token();

            left_expr = self.parse_infix_expression(left_expr.unwrap());
        }

        left_expr
    }

    // let
    fn parse_let_statement(&mut self) -> Option<Stmt> {
        let name = match &self.peek_token {
            Token::Identifier(ident) => ident.clone(),
            _ => {
                self.peek_error("Identifer");
                return None;
            }
        };

        self.next_token();

        if self.peek_token != Token::Colon {
            self.peek_error("Colon (:)");
            return None;
        }

        self.next_token();

        let var_type = match &self.peek_token {
            Token::IntType => "int".to_string(),
            Token::StringType => "str".to_string(),
            Token::FloatType => "float".to_string(),
            Token::BoolType => "bool".to_string(),
            _ => {
                self.peek_error("Type (int, str, float, bool)");
                return None;
            }
        };

        self.next_token();

        if self.peek_token != Token::Assign {
            self.peek_error("Assign (=)");
            return None;
        }

        self.next_token();
        self.next_token();

        let value = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Let {
            name,
            var_type,
            value,
        })
    }

    // return
    fn parse_return_statement(&mut self) -> Option<Stmt> {
        self.next_token();

        let value = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Return { value })
    }

    // block statement
    fn parse_block_statement(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        self.next_token(); // Skip the '{'

        while self.current_token != Token::RBrace && self.current_token != Token::Eof {
            if let Some(stmt) = self.parse_statement() {
                statements.push(stmt);
            }
            self.next_token();
        }

        statements
    }

    // if
    fn parse_if_statement(&mut self) -> Option<Stmt> {
        self.next_token(); // Skip 'if'

        let condition = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token(); // Move to '{'

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token == Token::Else {
            self.next_token(); // Move to 'else'

            if self.peek_token != Token::LBrace {
                self.peek_error("{");
                return None;
            }
            self.next_token(); // Move to '{'

            alternative = Some(self.parse_block_statement());
        }

        Some(Stmt::If {
            condition,
            consequence,
            alternative,
        })
    }

    // while
    fn parse_while_statement(&mut self) -> Option<Stmt> {
        self.next_token(); // Skip 'while'

        let condition = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token(); // Move to '{'

        let body = self.parse_block_statement();

        Some(Stmt::While { condition, body })
    }

    // for
    fn parse_for_statement(&mut self) -> Option<Stmt> {
        self.next_token(); // Skip 'for'

        // parse identifier
        let identifier = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                self.peek_error("identifier");
                return None;
            }
        };
        self.next_token(); // Move past identifier

        // parse 'in'
        if self.current_token != Token::In {
            self.peek_error("in");
            return None;
        }
        self.next_token(); // Move past 'in'

        // parse start_value
        let start_value = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        // ensure next token is 'to'
        if self.peek_token != Token::To {
            self.peek_error("to");
            return None;
        }
        self.next_token(); // Move to 'to'
        self.next_token(); // Move to end_value

        // parse end_value
        let end_value = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        // ensure next is '{'
        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token(); // Move to '{'

        let body = self.parse_block_statement();

        Some(Stmt::For {
            identifier,
            start_value,
            end_value,
            body,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Stmt> {
        let expr = match self.expression_parser(Precedence::Lowest) {
            Some(expr) => expr,
            None => return None,
        };

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Expression(expr))
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}
