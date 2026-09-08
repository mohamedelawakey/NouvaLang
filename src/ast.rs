#[derive(Debug, Clone)]
pub enum Expr {
    Identifier(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BooleanLiteral(bool),

    Infix {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },

    Prefix {
        operator: String,
        right: Box<Expr>,
    }
}

#[derive(Debug, Clone)]
pub enum Stmt {
    // let (for variables)
    Let {
        name: String,
        var_type: String,
        value: Expr
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
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct Program {
    pub statement: Vec<Stmt>
}
