use super::expr::Expr;
use super::types::TypeName;
use crate::diagnostic::Span;

#[derive(Clone, Debug)]
pub enum Stmt {
    Let {
        name: String,
        mutable: bool,
        annotation: Option<TypeName>,
        initializer: Expr,
        span: Span,
    },
    Assign {
        name: String,
        value: Expr,
        span: Span,
    },
    Expr {
        expr: Expr,
        span: Span,
    },
    Return {
        value: Option<Expr>,
        span: Span,
    },
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
        span: Span,
    },
    While {
        condition: Expr,
        body: Vec<Stmt>,
        span: Span,
    },
    Block {
        body: Vec<Stmt>,
        span: Span,
    },
    Pass {
        span: Span,
    },
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Self::Let { span, .. }
            | Self::Assign { span, .. }
            | Self::Expr { span, .. }
            | Self::Return { span, .. }
            | Self::If { span, .. }
            | Self::While { span, .. }
            | Self::Block { span, .. }
            | Self::Pass { span } => *span,
        }
    }
}
