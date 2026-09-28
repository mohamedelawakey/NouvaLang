#![allow(dead_code)]

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub var_type: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),
    NoneLiteral,
    Prefix {
        operator: String,
        right: Box<Expr>,
    },
    Infix {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Cast {
        left: Box<Expr>,
        cast_type: String,
    },
    Ternary {
        condition: Box<Expr>,
        consequence: Box<Expr>,
        alternative: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // let (for variables)
    Let {
        name: String,
        var_type: String,
        value: Expr,
    },
    // constants
    Const {
        name: String,
        var_type: String,
        value: Expr,
    },
    // return
    Return {
        value: Expr,
    },
    // if statements
    If {
        condition: Expr,
        consequence: Vec<Stmt>,
        alternative: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        identifier: String,
        start_value: Expr,
        end_value: Expr,
        body: Vec<Stmt>,
    },
    Function {
        name: String,
        parameters: Vec<Parameter>,
        return_type: String,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
    Expression(Expr),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statement: Vec<Stmt>,
}
