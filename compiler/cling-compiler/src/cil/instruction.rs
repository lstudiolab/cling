use super::value::Value;

#[derive(Clone, Debug)]
pub enum Op {
    Const(Value), Load(usize), Store(usize), Pop,
    Neg, Not, Add, Sub, Mul, Div, Rem, Eq, Ne, Lt, Le, Gt, Ge, And, Or,
    Jump(usize), JumpIfFalse(usize), Call(String, usize), Print, Return,
}
#[derive(Clone, Debug)]
pub struct Instruction { pub op: Op, pub line: usize }
#[derive(Clone, Debug)]
pub struct CilFunction {
    pub name: String,
    pub parameter_count: usize,
    pub local_count: usize,
    pub code: Vec<Instruction>,
}
