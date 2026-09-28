use crate::ast::{Expr, Parameter, Program, Stmt};
use crate::lexer::Lexer;
use crate::tokens::Token;

#[derive(PartialEq, PartialOrd, Debug, Clone)]
pub enum Precedence {
    Lowest = 1,
    Assign,
    Ternary,
    Or,
    And,
    Equals,
    LessGreater,
    Sum,
    Product,
    Exponent,
    Prefix,
    Cast,
    Call,
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
            Token::LParen => Precedence::Call,
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
            Token::Question => Precedence::Ternary,
            Token::As => Precedence::Cast,
            _ => Precedence::Lowest,
        }
    }

    pub fn current_precedence(&self) -> Precedence {
        match self.current_token {
            Token::LParen => Precedence::Call,
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
            Token::Question => Precedence::Ternary,
            Token::As => Precedence::Cast,
            _ => Precedence::Lowest,
        }
    }

    pub fn parse_infix_expression(&mut self, left: Expr) -> Option<Expr> {
        let operator = match &self.current_token {
            Token::LParen => return self.parse_call_expression(left),
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
            Token::Question => return self.parse_ternary_expression(left),
            Token::As => return self.parse_cast_expression(left),
            _ => return None,
        };

        let precedence = self.current_precedence();

        self.next_token();

        let right = self.expression_parser(precedence)?;

        Some(Expr::Infix {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        })
    }

    fn parse_cast_expression(&mut self, left: Expr) -> Option<Expr> {
        self.next_token();

        let cast_type = match &self.current_token {
            Token::IntType => "int".to_string(),
            Token::FloatType => "float".to_string(),
            Token::BoolType => "bool".to_string(),
            Token::StringType => "str".to_string(),
            Token::Identifier(name) => name.clone(),
            _ => {
                self.peek_error("Type (int, float, bool, str)");
                return None;
            }
        };

        Some(Expr::Cast {
            left: Box::new(left),
            cast_type,
        })
    }

    fn parse_ternary_expression(&mut self, condition: Expr) -> Option<Expr> {
        self.next_token();

        let consequence = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token != Token::Colon {
            self.peek_error("Colon (:) for ternary alternative");
            return None;
        }

        self.next_token();
        self.next_token();

        let alternative = self.expression_parser(Precedence::Lowest)?;

        Some(Expr::Ternary {
            condition: Box::new(condition),
            consequence: Box::new(consequence),
            alternative: Box::new(alternative),
        })
    }

    fn parse_call_expression(&mut self, function: Expr) -> Option<Expr> {
        let arguments = self.parse_call_arguments()?;
        Some(Expr::Call {
            function: Box::new(function),
            arguments,
        })
    }

    fn parse_call_arguments(&mut self) -> Option<Vec<Expr>> {
        let mut args = Vec::new();

        if self.peek_token == Token::RParen {
            self.next_token();
            return Some(args);
        }

        self.next_token();
        if let Some(arg) = self.expression_parser(Precedence::Lowest) {
            args.push(arg);
        } else {
            return None;
        }

        while self.peek_token == Token::Comma {
            self.next_token();
            self.next_token();
            if let Some(arg) = self.expression_parser(Precedence::Lowest) {
                args.push(arg);
            } else {
                return None;
            }
        }

        if self.peek_token != Token::RParen {
            self.peek_error(")");
            return None;
        }

        self.next_token();
        Some(args)
    }

    fn parse_prefix_expression(&mut self) -> Option<Expr> {
        let operator = match &self.current_token {
            Token::Minus => "-".to_string(),
            Token::Not => "!".to_string(),
            _ => return None,
        };

        self.next_token();

        let right = self.expression_parser(Precedence::Prefix)?;

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
        let mut left_expr = match &self.current_token {
            Token::IntLiteral(value) => Some(Expr::IntLiteral(*value)),
            Token::FloatLiteral(value) => Some(Expr::FloatLiteral(*value)),
            Token::StringLiteral(value) => Some(Expr::StringLiteral(value.clone())),
            Token::BooleanLiteral(value) => Some(Expr::BooleanLiteral(*value)),
            Token::Identifier(value) => Some(Expr::Identifier(value.clone())),
            Token::None => Some(Expr::NoneLiteral),
            Token::Minus | Token::Not => self.parse_prefix_expression(),
            Token::LParen => self.parse_grouped_expression(),
            _ => {
                self.peek_error("Expression (Number, String, Boolean, or Variable)");
                None
            }
        };

        left_expr.as_ref()?;

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

        let value = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Let {
            name,
            var_type,
            value,
        })
    }

    // const
    fn parse_const_statement(&mut self) -> Option<Stmt> {
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

        let value = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Const {
            name,
            var_type,
            value,
        })
    }

    // return
    fn parse_return_statement(&mut self) -> Option<Stmt> {
        self.next_token();

        let value = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Return { value })
    }

    // break
    fn parse_break_statement(&mut self) -> Option<Stmt> {
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        Some(Stmt::Break)
    }

    // continue
    fn parse_continue_statement(&mut self) -> Option<Stmt> {
        if self.peek_token == Token::Semicolon {
            self.next_token();
        }
        Some(Stmt::Continue)
    }

    // block statement
    fn parse_block_statement(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();

        self.next_token();

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
        self.next_token();

        let condition = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token();

        let consequence = self.parse_block_statement();

        let mut alternative = None;

        if self.peek_token == Token::Else {
            self.next_token();

            if self.peek_token != Token::LBrace {
                self.peek_error("{");
                return None;
            }
            self.next_token();

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
        self.next_token();

        let condition = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token();

        let body = self.parse_block_statement();

        Some(Stmt::While { condition, body })
    }

    // for
    fn parse_for_statement(&mut self) -> Option<Stmt> {
        self.next_token();

        let identifier = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                self.peek_error("identifier");
                return None;
            }
        };
        self.next_token();

        if self.current_token != Token::In {
            self.peek_error("in");
            return None;
        }
        self.next_token();

        let start_value = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token != Token::To {
            self.peek_error("to");
            return None;
        }
        self.next_token();
        self.next_token();

        let end_value = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token();

        let body = self.parse_block_statement();

        Some(Stmt::For {
            identifier,
            start_value,
            end_value,
            body,
        })
    }

    // function
    fn parse_function_statement(&mut self) -> Option<Stmt> {
        self.next_token();

        let name = match &self.current_token {
            Token::Identifier(name) => name.clone(),
            _ => {
                self.peek_error("identifier");
                return None;
            }
        };
        self.next_token();

        if self.current_token != Token::LParen {
            self.peek_error("(");
            return None;
        }
        self.next_token();

        let mut parameters = Vec::new();
        while self.current_token != Token::RParen && self.current_token != Token::Eof {
            let param_name = match &self.current_token {
                Token::Identifier(name) => name.clone(),
                _ => {
                    self.peek_error("parameter identifier");
                    return None;
                }
            };
            self.next_token();

            if self.current_token != Token::Colon {
                self.peek_error(":");
                return None;
            }
            self.next_token();

            let param_type = match &self.current_token {
                Token::IntType => "int".to_string(),
                Token::FloatType => "float".to_string(),
                Token::BoolType => "bool".to_string(),
                Token::StringType => "str".to_string(),
                Token::Identifier(name) => name.clone(),
                _ => {
                    self.peek_error("parameter type");
                    return None;
                }
            };
            self.next_token();

            parameters.push(Parameter {
                name: param_name,
                var_type: param_type,
            });

            if self.current_token == Token::Comma {
                self.next_token();
            }
        }

        if self.current_token != Token::RParen {
            self.peek_error(")");
            return None;
        }
        self.next_token();

        if self.current_token != Token::Arrow {
            self.peek_error("->");
            return None;
        }
        self.next_token();

        let return_type = match &self.current_token {
            Token::IntType => "int".to_string(),
            Token::FloatType => "float".to_string(),
            Token::BoolType => "bool".to_string(),
            Token::StringType => "str".to_string(),
            Token::Identifier(name) => name.clone(),
            _ => {
                self.peek_error("return type");
                return None;
            }
        };

        if self.peek_token != Token::LBrace {
            self.peek_error("{");
            return None;
        }
        self.next_token();

        let body = self.parse_block_statement();

        Some(Stmt::Function {
            name,
            parameters,
            return_type,
            body,
        })
    }

    fn parse_expression_statement(&mut self) -> Option<Stmt> {
        let expr = self.expression_parser(Precedence::Lowest)?;

        if self.peek_token == Token::Semicolon {
            self.next_token();
        }

        Some(Stmt::Expression(expr))
    }

    pub fn parse_statement(&mut self) -> Option<Stmt> {
        match self.current_token {
            Token::Let => self.parse_let_statement(),
            Token::Const => self.parse_const_statement(),
            Token::Return => self.parse_return_statement(),
            Token::Break => self.parse_break_statement(),
            Token::Continue => self.parse_continue_statement(),
            Token::If => self.parse_if_statement(),
            Token::While => self.parse_while_statement(),
            Token::For => self.parse_for_statement(),
            Token::Fun => self.parse_function_statement(),
            _ => self.parse_expression_statement(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_code(code: &str) -> (Program, Vec<String>) {
        let lexer = Lexer::new(code);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        (program, parser.errors)
    }

    #[test]
    fn test_variable_declarations() {
        let code = "let age: int = 25;
let name: str = \"Mohamed\";
let is_student: bool = true;";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 3);

        assert_eq!(
            program.statement[0],
            Stmt::Let {
                name: "age".to_string(),
                var_type: "int".to_string(),
                value: Expr::IntLiteral(25),
            }
        );

        assert_eq!(
            program.statement[1],
            Stmt::Let {
                name: "name".to_string(),
                var_type: "str".to_string(),
                value: Expr::StringLiteral("Mohamed".to_string()),
            }
        );

        assert_eq!(
            program.statement[2],
            Stmt::Let {
                name: "is_student".to_string(),
                var_type: "bool".to_string(),
                value: Expr::BooleanLiteral(true),
            }
        );
    }

    #[test]
    fn test_function_declaration() {
        let code = "fun add(a: int, b: int) -> int { return a + b; }";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 1);

        assert_eq!(
            program.statement[0],
            Stmt::Function {
                name: "add".to_string(),
                parameters: vec![
                    Parameter {
                        name: "a".to_string(),
                        var_type: "int".to_string(),
                    },
                    Parameter {
                        name: "b".to_string(),
                        var_type: "int".to_string(),
                    },
                ],
                return_type: "int".to_string(),
                body: vec![Stmt::Return {
                    value: Expr::Infix {
                        left: Box::new(Expr::Identifier("a".to_string())),
                        operator: "+".to_string(),
                        right: Box::new(Expr::Identifier("b".to_string())),
                    }
                }],
            }
        );
    }

    #[test]
    fn test_function_call_and_print() {
        let code = "print(\"Hello, World!\");
add(5, 10);";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 2);

        assert_eq!(
            program.statement[0],
            Stmt::Expression(Expr::Call {
                function: Box::new(Expr::Identifier("print".to_string())),
                arguments: vec![Expr::StringLiteral("Hello, World!".to_string())],
            })
        );

        assert_eq!(
            program.statement[1],
            Stmt::Expression(Expr::Call {
                function: Box::new(Expr::Identifier("add".to_string())),
                arguments: vec![Expr::IntLiteral(5), Expr::IntLiteral(10)],
            })
        );
    }

    #[test]
    fn test_if_else_statement() {
        let code = "if age >= 18 { print(\"Adult\"); } else { print(\"Minor\"); }";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 1);

        assert_eq!(
            program.statement[0],
            Stmt::If {
                condition: Expr::Infix {
                    left: Box::new(Expr::Identifier("age".to_string())),
                    operator: ">=".to_string(),
                    right: Box::new(Expr::IntLiteral(18)),
                },
                consequence: vec![Stmt::Expression(Expr::Call {
                    function: Box::new(Expr::Identifier("print".to_string())),
                    arguments: vec![Expr::StringLiteral("Adult".to_string())],
                })],
                alternative: Some(vec![Stmt::Expression(Expr::Call {
                    function: Box::new(Expr::Identifier("print".to_string())),
                    arguments: vec![Expr::StringLiteral("Minor".to_string())],
                })]),
            }
        );
    }

    #[test]
    fn test_while_and_for_loops() {
        let code = "while count < 10 { count += 1; }
for i in 0 to 10 { print(i); }";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 2);

        assert_eq!(
            program.statement[0],
            Stmt::While {
                condition: Expr::Infix {
                    left: Box::new(Expr::Identifier("count".to_string())),
                    operator: "<".to_string(),
                    right: Box::new(Expr::IntLiteral(10)),
                },
                body: vec![Stmt::Expression(Expr::Infix {
                    left: Box::new(Expr::Identifier("count".to_string())),
                    operator: "+=".to_string(),
                    right: Box::new(Expr::IntLiteral(1)),
                })],
            }
        );

        assert_eq!(
            program.statement[1],
            Stmt::For {
                identifier: "i".to_string(),
                start_value: Expr::IntLiteral(0),
                end_value: Expr::IntLiteral(10),
                body: vec![Stmt::Expression(Expr::Call {
                    function: Box::new(Expr::Identifier("print".to_string())),
                    arguments: vec![Expr::Identifier("i".to_string())],
                })],
            }
        );
    }

    #[test]
    fn test_operator_precedence_and_expressions() {
        let code = "let res: int = 2 + 3 * 4 ^ 2;";
        let (program, errors) = parse_code(code);

        assert!(errors.is_empty(), "Parser errors: {:?}", errors);
        assert_eq!(program.statement.len(), 1);

        assert_eq!(
            program.statement[0],
            Stmt::Let {
                name: "res".to_string(),
                var_type: "int".to_string(),
                value: Expr::Infix {
                    left: Box::new(Expr::IntLiteral(2)),
                    operator: "+".to_string(),
                    right: Box::new(Expr::Infix {
                        left: Box::new(Expr::IntLiteral(3)),
                        operator: "*".to_string(),
                        right: Box::new(Expr::Infix {
                            left: Box::new(Expr::IntLiteral(4)),
                            operator: "^".to_string(),
                            right: Box::new(Expr::IntLiteral(2)),
                        }),
                    }),
                },
            }
        );
    }
}
