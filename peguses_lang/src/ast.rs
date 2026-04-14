#[derive(Debug, Clone)]
pub enum Expr {
    Number(i64),
    Boolean(bool),
    String(String),
    Ident(String),
    Input,
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,  // Unary minus
}

#[derive(Debug, Clone)]
pub enum BinOp {
    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    
    // Comparison
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    
    // Logical
    And,
    Or,
    
    // String
    Concat,  // String concatenation
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Let {
        name: String,
        value: Expr,
    },
    Assign {
        name: String,
        value: Expr,
    },
    Print {
        value: Expr,
    },
    If {
        condition: Expr,
        then_block: Vec<Stmt>,
        else_block: Option<Vec<Stmt>>,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
    },
    For {
        var: String,
        start: Expr,
        end: Expr,
        body: Vec<Stmt>,
    },
    Break,
    Continue,
}