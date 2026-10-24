use std::env;
use std::fs;
use std::process::{Command, ExitCode};
use cling_compiler::{compile, diagnostic::Diagnostic};

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => { eprintln!("{error}"); ExitCode::FAILURE }
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let mut mode = String::from("run");
    let mut target = String::from("host");
    let mut output: Option<String> = None;
    let mut input: Option<String> = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => { print_help(); return Ok(()); }
            "--version" => { println!("cling {}", env!("CARGO_PKG_VERSION")); return Ok(()); }
            "--emit" => mode = args.next().ok_or("expected a value after --emit")?,
            "--target" => target = args.next().ok_or("expected a value after --target")?,
            "-o" | "--output" => output = Some(args.next().ok_or("expected a path after -o/--output")?),
            s if s.starts_with('-') => return Err(format!("unknown option '{s}' (try --help)")),
            s => if input.replace(s.to_owned()).is_some() { return Err("only one input file is currently supported".into()); },
        }
    }

    let input = input.ok_or_else(|| "missing input file (try: cling main.cling)".to_string())?;
    if target != "host" {
        return Err(format!("target '{target}' is not implemented; use --target host. Cross-target code generation is not available yet."));
    }
    let source = fs::read_to_string(&input).map_err(|e| format!("cannot read '{input}': {e}"))?;
    let compilation = compile(&source).map_err(|errors| render_errors(&errors, &input))?;

    match mode.as_str() {
        "tokens" => {
            for token in cling_compiler::lexer::lex(&source).map_err(|e| render_errors(&e, &input))? {
                println!("{:?} @ {}:{}", token.kind, token.span.line, token.span.column);
            }
        }
        "ast" => println!("{:#?}", compilation.ast),
        "cil" => print!("{}", compilation.cil.display()),
        "c" => print!("{}", cling_compiler::native::emit_c(&compilation.ast)),
        "exe" => build_native(&compilation.ast, &input, output.as_deref())?,
        "run" => { compilation.cil.run()?; }
        other => return Err(format!("unknown emit mode '{other}'; expected tokens, ast, cil, c, exe, or run")),
    }
    Ok(())
}

fn build_native(program: &cling_compiler::ast::Program, input: &str, requested_output: Option<&str>) -> Result<(), String> {
    let default_output = input.rsplit_once('.').map(|(base, _)| base.to_owned()).unwrap_or_else(|| format!("{input}.out"));
    let output = requested_output.unwrap_or(&default_output);
    let temp_path = env::temp_dir().join(format!(
        "cling-{}-{}.c", std::process::id(),
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_nanos()
    ));
    fs::write(&temp_path, cling_compiler::native::emit_c(program))
        .map_err(|e| format!("cannot write temporary C source: {e}"))?;
    let result = Command::new("clang")
        .args(["-std=gnu11", "-O2", "-Wall", "-Wextra"])
        .arg(&temp_path).arg("-o").arg(output).output();
    let _ = fs::remove_file(&temp_path);
    let result = result.map_err(|e| format!("cannot start clang: {e}; install Clang or use --emit run"))?;
    if !result.status.success() {
        return Err(format!("native compilation failed:\n{}", String::from_utf8_lossy(&result.stderr)));
    }
    if !result.stderr.is_empty() { eprint!("{}", String::from_utf8_lossy(&result.stderr)); }
    println!("built {output}");
    Ok(())
}

fn render_errors(errors: &[Diagnostic], file: &str) -> String {
    errors.iter().map(|error| error.render(file)).collect::<Vec<_>>().join("\n")
}

fn print_help() {
    println!("Cling compiler\nUsage: cling [OPTIONS] <file.cling>\n\nOptions:\n  --emit tokens|ast|cil|c|exe|run  Select compiler output/action (default: run)\n  --target <triple>               Select target (host currently supported)\n  -o, --output <path>             Output path for --emit exe\n  --version                       Print version\n  -h, --help                      Print help\n\nLanguage subset: func, let, var, if/else, while, return, Int, Bool, String, print().\n  c emits C; exe invokes Clang for a native host executable; run executes CIL.");
}
