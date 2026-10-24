use std::collections::HashMap;
use super::instruction::{CilFunction, Op};
use super::value::Value;
use super::vm::execute;

#[derive(Clone, Debug)]
pub struct Module { pub functions: Vec<CilFunction> }

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

