pub mod ast;
pub mod cil;
pub mod diagnostic;
#[path = "../../../lib/Parse/Lexer.rs"]
pub mod lexer;
pub mod native;
#[path = "../../../lib/Parse/Parser.rs"]
pub mod parser;
pub mod sema;

use diagnostic::Diagnostic;

pub struct Compilation { pub ast: ast::Program, pub cil: cil::Module }

pub fn compile(source: &str) -> Result<Compilation, Vec<Diagnostic>> {
    let tokens = lexer::lex(source)?;
    let ast = parser::parse(tokens)?;
    sema::check(&ast)?;
    let cil = cil::lower(&ast);
    if let Err(message) = cil.verify() { return Err(vec![Diagnostic::global(format!("internal CIL verification failed: {message}"))]); }
    Ok(Compilation { ast, cil })
}
