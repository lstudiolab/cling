//! CIL: Cling Intermediate Language. This first executable form is a verified,
//! typed stack IR. It is intentionally independent of any native target ISA.
use std::collections::HashMap;
use crate::ast::{BinaryOp, Expr, ExprKind, Function, Program, Stmt, UnaryOp};
use crate::diagnostic::Diagnostic;

#[derive(Clone, Debug, PartialEq)]
pub enum Value { Int(i64), Bool(bool), String(String), Void }
#[derive(Clone, Debug)]
pub enum Op {
    Const(Value), Load(usize), Store(usize), Pop,
    Neg, Not, Add, Sub, Mul, Div, Rem, Eq, Ne, Lt, Le, Gt, Ge, And, Or,
    Jump(usize), JumpIfFalse(usize), Call(String, usize), Print, Return,
}
#[derive(Clone, Debug)]
pub struct Instruction { pub op: Op, pub line: usize }
#[derive(Clone, Debug)]
pub struct CilFunction { pub name: String, pub parameter_count: usize, pub local_count: usize, pub code: Vec<Instruction> }
#[derive(Clone, Debug)]
pub struct Module { pub functions: Vec<CilFunction> }

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

impl Module {
    pub fn display(&self) -> String {
        let mut out = String::new();
        for f in &self.functions {
            out.push_str(&format!("function @{} (params={}, locals={}) {{\n", f.name, f.parameter_count, f.local_count));
            for (pc, ins) in f.code.iter().enumerate() { out.push_str(&format!("  {pc:04}: {:?} ; line {}\n", ins.op, ins.line)); }
            out.push_str("}\n");
        }
        out
    }
    pub fn verify(&self) -> Result<(), String> {
        let functions: HashMap<&str, &CilFunction> = self.functions.iter().map(|f| (f.name.as_str(), f)).collect();
        if !functions.contains_key("main") { return Err("program must define func main()".into()); }
        if functions["main"].parameter_count != 0 { return Err("func main() must not take parameters".into()); }
        for f in &self.functions {
            for (pc, ins) in f.code.iter().enumerate() {
                match &ins.op {
                    Op::Load(s) | Op::Store(s) if *s >= f.local_count => return Err(format!("@{}:{pc}: local slot {s} is out of range", f.name)),
                    Op::Jump(t) | Op::JumpIfFalse(t) if *t > f.code.len() => return Err(format!("@{}:{pc}: branch target {t} is out of range", f.name)),
                    Op::Call(n, argc) if n != "print" && n != "log" => {
                        let callee = functions.get(n.as_str()).ok_or_else(|| format!("@{}:{pc}: unknown function '{n}'", f.name))?;
                        if *argc != callee.parameter_count { return Err(format!("@{}:{pc}: call to '{n}' has wrong argument count", f.name)); }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
    pub fn run(&self) -> Result<Value, String> {
        self.verify()?;
        let functions: HashMap<&str, &CilFunction> = self.functions.iter().map(|f| (f.name.as_str(), f)).collect();
        execute(functions.get("main").copied().unwrap(), vec![], &functions, 0)
    }
}

fn execute(f: &CilFunction, args: Vec<Value>, functions: &HashMap<&str, &CilFunction>, depth: usize) -> Result<Value, String> {
    if depth > 1024 { return Err("maximum call depth exceeded".into()); }
    if args.len() != f.parameter_count { return Err(format!("function '{}' expected {} arguments", f.name, f.parameter_count)); }
    let mut locals = vec![Value::Void; f.local_count];
    for (i, v) in args.into_iter().enumerate() { locals[i] = v; }
    let mut stack: Vec<Value> = Vec::new(); let mut pc = 0usize;
    while pc < f.code.len() {
        let ins = &f.code[pc]; pc += 1;
        macro_rules! pop { () => { stack.pop().ok_or_else(|| format!("@{}:{}: CIL stack underflow", f.name, pc-1))? }; }
        match &ins.op {
            Op::Const(v) => stack.push(v.clone()), Op::Load(s) => stack.push(locals[*s].clone()), Op::Store(s) => locals[*s] = pop!(), Op::Pop => { pop!(); },
            Op::Neg => { let a=as_int(pop!())?; stack.push(Value::Int(a.checked_neg().ok_or_else(|| format!("line {}: integer overflow", ins.line))?)); }, Op::Not => { let a=pop!(); stack.push(Value::Bool(!as_bool(a)?)); },
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Rem => { let b=as_int(pop!())?; let a=as_int(pop!())?; let v=match &ins.op { Op::Add=>a.checked_add(b), Op::Sub=>a.checked_sub(b), Op::Mul=>a.checked_mul(b), Op::Div=>if b==0 { return Err(format!("line {}: division by zero", ins.line)); } else { a.checked_div(b) }, Op::Rem=>if b==0 { return Err(format!("line {}: remainder by zero", ins.line)); } else { a.checked_rem(b) } }.ok_or_else(|| format!("line {}: integer overflow", ins.line))?; stack.push(Value::Int(v)); }
            Op::Eq | Op::Ne => { let b=pop!(); let a=pop!(); let eq=a==b; stack.push(Value::Bool(if matches!(&ins.op, Op::Eq) {eq} else {!eq})); }
            Op::Lt | Op::Le | Op::Gt | Op::Ge => { let b=as_int(pop!())?; let a=as_int(pop!())?; stack.push(Value::Bool(match &ins.op {Op::Lt=>a<b,Op::Le=>a<=b,Op::Gt=>a>b,Op::Ge=>a>=b,_=>false})); }
            Op::And | Op::Or => { let b=as_bool(pop!())?; let a=as_bool(pop!())?; stack.push(Value::Bool(if matches!(&ins.op,Op::And){a&&b}else{a||b})); }
            Op::Jump(t) => { pc=*t; }, Op::JumpIfFalse(t) => if !as_bool(pop!())? { pc=*t; },
            Op::Call(name, argc) => { let mut call_args=Vec::with_capacity(*argc); for _ in 0..*argc { call_args.push(pop!()); } call_args.reverse(); let callee=functions.get(name.as_str()).ok_or_else(||format!("unknown function '{name}'"))?; stack.push(execute(callee,call_args,functions,depth+1)?); }
            Op::Print => { let value=pop!(); println!("{}", value); }, Op::Return => return Ok(stack.pop().unwrap_or(Value::Void)),
        }
    }
    Ok(Value::Void)
}
fn as_int(v: Value) -> Result<i64,String> { if let Value::Int(n)=v {Ok(n)} else {Err("CIL type error: expected Int".into())} }
fn as_bool(v: Value) -> Result<bool,String> { if let Value::Bool(b)=v {Ok(b)} else {Err("CIL type error: expected Bool".into())} }
impl std::fmt::Display for Value { fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result { match self { Value::Int(v)=>write!(f,"{v}"),Value::Bool(v)=>write!(f,"{v}"),Value::String(v)=>write!(f,"{v}"),Value::Void=>Ok(()) } } }

#[cfg(test)] mod tests {
    use crate::{lexer, parser, sema}; use super::*;
    fn compile(s:&str)->Module { let tokens=lexer::lex(s).unwrap(); let ast=parser::parse(tokens).unwrap(); sema::check(&ast).unwrap(); let m=lower(&ast); m.verify().unwrap(); m }
    #[test] fn executes_arithmetic_and_mutation(){ let m=compile("func main() { var x: Int = 2; x = x * 3 + 1; print(x); }"); assert_eq!(m.run().unwrap(),Value::Void); }
    #[test] fn verifies_missing_main(){ let m=Module{functions:vec![]}; assert!(m.verify().unwrap_err().contains("main")); }
    #[test] fn logical_operators_short_circuit(){ let m=compile("func main() { print(false && (1 / 0 == 0)); print(true || (1 / 0 == 0)); }"); assert_eq!(m.run().unwrap(), Value::Void); }
    #[test] fn executes_user_functions_and_loops(){ let m=compile("func sum(n: Int) -> Int { var i: Int = 0; var total: Int = 0; while i < n { i = i + 1; total = total + i; } return total; } func main() -> Int { return sum(10); }"); assert_eq!(m.run().unwrap(), Value::Int(55)); assert!(m.display().contains("function @sum")); }
}
