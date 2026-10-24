use std::collections::HashMap;
use super::instruction::{CilFunction, Op};
use super::value::Value;

pub(super) fn execute(f: &CilFunction, args: Vec<Value>, functions: &HashMap<&str, &CilFunction>, depth: usize) -> Result<Value, String> {
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
            Op::Add | Op::Sub | Op::Mul | Op::Div | Op::Rem => { let b=as_int(pop!())?; let a=as_int(pop!())?; let v=match &ins.op { Op::Add=>a.checked_add(b), Op::Sub=>a.checked_sub(b), Op::Mul=>a.checked_mul(b), Op::Div=>if b==0 { return Err(format!("line {}: division by zero", ins.line)); } else { a.checked_div(b) }, Op::Rem=>if b==0 { return Err(format!("line {}: remainder by zero", ins.line)); } else { a.checked_rem(b) }, _ => unreachable!("outer match restricts this to arithmetic operators") }.ok_or_else(|| format!("line {}: integer overflow", ins.line))?; stack.push(Value::Int(v)); }
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
