use super::stmt::Stmt;
use super::types::TypeName;
use crate::diagnostic::Span;

#[derive(Clone, Debug)]
pub struct Program {
    pub functions: Vec<Function>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<Parameter>,
    pub return_type: TypeName,
    pub body: Vec<Stmt>,
    pub span: Span,
}

#[derive(Clone, Debug)]
pub struct Parameter {
    pub name: String,
    pub ty: TypeName,
    pub span: Span,
}
