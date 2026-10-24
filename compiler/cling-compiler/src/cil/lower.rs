use std::collections::HashMap;
use crate::ast::{BinaryOp, Expr, ExprKind, Function, Program, Stmt, UnaryOp};
use super::instruction::{CilFunction, Instruction, Op};
use super::module::Module;
use super::value::Value;

pub fn lower(program: &Program) -> Module {
    Module { functions: program.functions.iter().map(FunctionBuilder::new).map(|mut b| b.build()).collect() }
}

struct FunctionBuilder<'a> {
    function: &'a Function, code: Vec<Instruction>, scopes: Vec<HashMap<String, usize>>, next_slot: usize,
}
impl<'a> FunctionBuilder<'a> {
    fn new(function: &'a Function) -> Self {
        let mut scope = HashMap::new();
        for (i, p) in function.params.iter().enumerate() { scope.insert(p.name.clone(), i); }
        Self { function, code: Vec::new(), scopes: vec![scope], next_slot: function.params.len() }
    }
    fn build(&mut self) -> CilFunction {
        let body = self.function.body.clone();
        let name = self.function.name.clone();
        let parameter_count = self.function.params.len();
        let line = self.function.span.line;
        self.statements(&body);
        if !matches!(self.code.last().map(|i| &i.op), Some(Op::Return)) { self.emit(Op::Const(Value::Void), line); self.emit(Op::Return, line); }
        CilFunction { name, parameter_count, local_count: self.next_slot, code: self.code.clone() }
    }
    fn emit(&mut self, op: Op, line: usize) -> usize { let i = self.code.len(); self.code.push(Instruction { op, line }); i }
    fn patch(&mut self, at: usize, target: usize) { match &mut self.code[at].op { Op::Jump(t) | Op::JumpIfFalse(t) => *t = target, _ => unreachable!() } }
    fn declare(&mut self, name: &str) -> usize { let slot = self.next_slot; self.next_slot += 1; self.scopes.last_mut().unwrap().insert(name.to_owned(), slot); slot }
    fn lookup(&self, name: &str) -> usize { *self.scopes.iter().rev().find_map(|s| s.get(name)).expect("sema validates names before CIL generation") }
    fn statements(&mut self, stmts: &[Stmt]) { for stmt in stmts { self.statement(stmt); } }
    fn statement(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, initializer, span, .. } => { self.expression(initializer); let slot = self.declare(name); self.emit(Op::Store(slot), span.line); }
            Stmt::Assign { name, value, span } => { self.expression(value); let slot = self.lookup(name); self.emit(Op::Store(slot), span.line); }
            Stmt::Expr { expr, span } => { self.expression(expr); self.emit(Op::Pop, span.line); }
            Stmt::Return { value, span } => { if let Some(v) = value { self.expression(v); } else { self.emit(Op::Const(Value::Void), span.line); } self.emit(Op::Return, span.line); }
            Stmt::If { condition, then_body, else_body, span } => {
                self.expression(condition); let jf = self.emit(Op::JumpIfFalse(usize::MAX), span.line);
                self.scopes.push(HashMap::new()); self.statements(then_body); self.scopes.pop();
                let jend = self.emit(Op::Jump(usize::MAX), span.line); self.patch(jf, self.code.len());
                self.scopes.push(HashMap::new()); self.statements(else_body); self.scopes.pop(); self.patch(jend, self.code.len());
            }
            Stmt::While { condition, body, span } => {
                let start = self.code.len(); self.expression(condition); let exit = self.emit(Op::JumpIfFalse(usize::MAX), span.line);
                self.scopes.push(HashMap::new()); self.statements(body); self.scopes.pop(); self.emit(Op::Jump(start), span.line); self.patch(exit, self.code.len());
            }
            Stmt::Block { body, .. } => { self.scopes.push(HashMap::new()); self.statements(body); self.scopes.pop(); }
            Stmt::Pass { .. } => {}
        }
    }
    fn expression(&mut self, expr: &Expr) {
        let line = expr.span.line;
        match &expr.kind {
            ExprKind::Int(v) => { self.emit(Op::Const(Value::Int(*v)), line); }
            ExprKind::Bool(v) => { self.emit(Op::Const(Value::Bool(*v)), line); }
            ExprKind::String(v) => { self.emit(Op::Const(Value::String(v.clone())), line); }
            ExprKind::Variable(name) => { let slot = self.lookup(name); self.emit(Op::Load(slot), line); }
            ExprKind::Unary { op, operand } => { self.expression(operand); self.emit(match op { UnaryOp::Negate => Op::Neg, UnaryOp::Not => Op::Not }, line); }
            ExprKind::Binary { op: BinaryOp::And, left, right } => {
                self.expression(left);
                let false_branch = self.emit(Op::JumpIfFalse(usize::MAX), line);
                self.expression(right);
                let end_branch = self.emit(Op::Jump(usize::MAX), line);
                self.patch(false_branch, self.code.len());
                self.emit(Op::Const(Value::Bool(false)), line);
                self.patch(end_branch, self.code.len());
            }
            ExprKind::Binary { op: BinaryOp::Or, left, right } => {
                self.expression(left);
                let evaluate_right = self.emit(Op::JumpIfFalse(usize::MAX), line);
                self.emit(Op::Const(Value::Bool(true)), line);
                let end_branch = self.emit(Op::Jump(usize::MAX), line);
                self.patch(evaluate_right, self.code.len());
                self.expression(right);
                self.patch(end_branch, self.code.len());
            }
            ExprKind::Binary { op, left, right } => {
                self.expression(left); self.expression(right);
                self.emit(match op { BinaryOp::Add=>Op::Add, BinaryOp::Subtract=>Op::Sub, BinaryOp::Multiply=>Op::Mul, BinaryOp::Divide=>Op::Div, BinaryOp::Remainder=>Op::Rem, BinaryOp::Equal=>Op::Eq, BinaryOp::NotEqual=>Op::Ne, BinaryOp::Less=>Op::Lt, BinaryOp::LessEqual=>Op::Le, BinaryOp::Greater=>Op::Gt, BinaryOp::GreaterEqual=>Op::Ge, BinaryOp::And|BinaryOp::Or=>unreachable!() }, line);
            }
            ExprKind::Call { callee, arguments } => {
                for arg in arguments { self.expression(arg); }
                if callee == "print" || callee == "log" { self.emit(Op::Print, line); self.emit(Op::Const(Value::Void), line); }
                else { self.emit(Op::Call(callee.clone(), arguments.len()), line); }
            }
        }
    }
}

