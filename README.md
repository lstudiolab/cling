# Cling

Cling is an experimental programming language and compiler written in Rust. The repository follows a Rust Cargo workspace and separates compiler subsystems in the spirit of Swift's compiler organization.

## Current executable implementation

The compiler currently implements a working language subset with a lexer, recursive-descent parser, AST, semantic checks, CIL lowering, a CIL verifier, a CIL stack-machine interpreter, and a native host path that emits C and invokes Clang. Direct CLLM machine-code backends for x86-64, x86, ARM/AArch64, RISC-V and WebAssembly remain future implementation work; a target name is not treated as supported merely because it is recognized.

### Build

Install the stable Rust toolchain, then run from the repository root:

```sh
cargo test --workspace
cargo build --workspace --release
```

### Run a program

```sh
cargo run -p cling -- examples/hello.cling
cargo run -p cling -- examples/control_flow.cling
```

Expected output for `hello.cling`:

```text
Hello, World!
```

Expected output for `control_flow.cling`:

```text
55
CIL execution succeeded
```

### Inspect compiler phases

```sh
cargo run -p cling -- --emit tokens examples/hello.cling
cargo run -p cling -- --emit ast examples/hello.cling
cargo run -p cling -- --emit cil examples/hello.cling
```

### Language subset

```cling
func sum(a: Int, b: Int) -> Int {
    return a + b;
}

func main() {
    let greeting: String = "Hello";
    var count: Int = sum(20, 22);
    print(greeting);
    print(count);
}
```

Supported so far: `func`, typed parameters and return values, `let`, `var`, assignments, `if`/`else`, `while`, `return`, nested blocks, `pass`, `Int`, `Bool`, `String`, function calls, `print`/`log`, integer arithmetic and comparisons, equality, boolean operators, comments (`//`) and escaped strings. Statements end in semicolons.

## Standard library direction

Cling does not have a separately packaged standard or foundation library yet. The long-term rule is that Cling libraries are authored in Cling source and use the `.cling` extension, just like ordinary source files. We will not implement the foundation library in Rust or C.

For the initial bootstrap stage, `print` and `log` are temporary compiler-provided intrinsics so programs can produce output before Cling has the language features and runtime interface needed to implement them in `.cling` source. They are not a separate library. Once the required interoperability/runtime mechanisms exist, the user-facing foundation APIs should move into Cling source, leaving only the minimum low-level host hooks as compiler/runtime intrinsics.

## Architecture

- `lib/Parse/Lexer.rs`: token kinds, token spans, and scanner in one file, imported by the compiler crate.
- `ast/`: separate expression, statement, type, and program/declaration models.
- `lib/Parse/Parser.rs`: recursive-descent parsing, including declarations, statements, expressions, and parser tests in one file, imported by the compiler crate.
- `sema/`: declaration checks and type analysis split into statement and expression checking.
- `cil/`: instruction/value model, AST lowering, module verification, and stack-machine execution.
- `native.rs`: host C emission and Clang-driven executable builds.
- `diagnostic.rs`: source spans and structured diagnostics.
- `tools/cling`: command-line driver.

## CLLM portability plan

CLLM must have an independently specified IR, verifier, optimization passes, target descriptions, ABI/calling-convention layer, instruction selector, register allocator, object writer, and linker integration. Each target will be marked supported only after real object-code emission and execution tests. WebAssembly is a distinct target architecture, not ARM or x86 assembly. Until those backends exist, the working target is the portable CIL VM on any platform that can build the Rust compiler.

## Status

Early compiler implementation; not production-ready. The CLI deliberately reports unsupported cross-target code generation instead of silently pretending to produce a native binary.

## Native host builds

Cling now has two execution paths:

- `cling file.cling` checks the program, lowers it to CIL, verifies the CIL, and runs it in the CIL VM.
- `cling --emit c file.cling` prints generated host C.
- `cling --emit exe file.cling -o app` invokes an installed `clang` to compile a native host executable.

The native backend is a real host-code path, but it is **not yet a standalone machine-code backend**: Cling emits C and Clang performs instruction selection, assembly, and linking. `--target` intentionally accepts only `host`; cross-architecture CLLM backends are not claimed as implemented.
