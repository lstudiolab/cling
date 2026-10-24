use crate::diagnostic::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TypeName { Int, Bool, String, Void }

impl std::fmt::Display for TypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self { Self::Int => "Int", Self::Bool => "Bool", Self::String => "String", Self::Void => "Void" })
    }
}

#[derive(Clone, Debug)]
pub struct Program { pub functions: Vec<Function>, pub span: Span }

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: TypeName,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Parameter { pub name: String, pub ty: TypeName, pub span: Span }

#[derive(Clone, Debug)]
pub enum Stmt {
    Let { name: String, mutable: bool, annotation: Option<TypeName>, initializer: Expr, span: Span },
    Assign { name: String, value: Expr, span: Span },
    Expr { expr: Expr, span: Span },
    Return { value: Option<Expr>, span: Span },
    If { condition: Expr, then_body: Vec<Stmt>, else_body: Vec<Stmt>, span: Span },
    While { condition: Expr, body: Vec<Stmt>, span: Span },
    Block { body: Vec<Stmt>, span: Span },
    Pass { span: Span },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Self::Let { span, .. } | Self::Assign { span, .. } | Self::Expr { span, .. }
            | Self::Return { span, .. } | Self::If { span, .. } | Self::While { span, .. }
            | Self::Block { span, .. } | Self::Pass { span } => *span,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Expr { pub kind: ExprKind, pub span: Span }

#[derive(Clone, Debug)]
pub enum ExprKind {
    Int(i64),
    Bool(bool),
    String(String),
    Variable(String),
    Call { callee: String, arguments: Vec<Expr> },
    Unary { op: UnaryOp, operand: Box<Expr> },
    Binary { op: BinaryOp, left: Box<Expr>, right: Box<Expr> },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOp { Negate, Not }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOp { Add, Subtract, Multiply, Divide, Remainder, Equal, NotEqual, Less, LessEqual, Greater, GreaterEqual, And, Or }
