//! Cling Intermediate Language (CIL) and its execution support.
mod instruction;
mod lower;
mod module;
mod value;
mod vm;

pub use instruction::{CilFunction, Instruction, Op};
pub use lower::lower;
pub use module::Module;
pub use value::Value;

#[cfg(test)] mod tests {
    use crate::{lexer, parser, sema}; use super::*;
    fn compile(s:&str)->Module { let tokens=lexer::lex(s).unwrap(); let ast=parser::parse(tokens).unwrap(); sema::check(&ast).unwrap(); let m=lower(&ast); m.verify().unwrap(); m }
    #[test] fn executes_arithmetic_and_mutation(){ let m=compile("func main() { var x: Int = 2; x = x * 3 + 1; print(x); }"); assert_eq!(m.run().unwrap(),Value::Void); }
    #[test] fn verifies_missing_main(){ let m=Module{functions:vec![]}; assert!(m.verify().unwrap_err().contains("main")); }
    #[test] fn logical_operators_short_circuit(){ let m=compile("func main() { print(false && (1 / 0 == 0)); print(true || (1 / 0 == 0)); }"); assert_eq!(m.run().unwrap(), Value::Void); }
    #[test] fn executes_user_functions_and_loops(){ let m=compile("func sum(n: Int) -> Int { var i: Int = 0; var total: Int = 0; while i < n { i = i + 1; total = total + i; } return total; } func main() -> Int { return sum(10); }"); assert_eq!(m.run().unwrap(), Value::Int(55)); assert!(m.display().contains("function @sum")); }
}
