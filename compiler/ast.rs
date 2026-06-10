#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Literal),
    Identifier(String),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    GetIndex(Box<Expr>, Box<Expr>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    List(Vec<Expr>),
    Map(Vec<(String, Expr)>),
}

#[derive(Debug, Clone)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Neq,
    Lt,
    Gt,
    Le,
    Ge,
    And,
    Or,
}

#[derive(Debug, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    VarDecl(String, Option<String>, Expr),
    Assign(String, Expr),
    FuncDecl(String, Vec<(String, String)>, Option<String>, Vec<Stmt>),
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    For(String, Expr, Expr, Vec<Stmt>),
    While(Expr, Vec<Stmt>),
    Return(Option<Expr>),
    Expr(Expr),
    Import(Vec<String>),
    Module(Vec<String>),
    Throw(Expr),
    TryCatch(Vec<Stmt>, String, Vec<Stmt>),
}
