#[derive(Debug)]
pub enum Expr {
    Number(i64),
    Ident(String),
    Binary {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}



#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Stmt{
    Let {
        name: String,
        value: Expr,
    },
    Print {
        value: Expr,
    },
}