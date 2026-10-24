# Repository organization

Cling is organized as a Rust Cargo workspace. The layout borrows the compiler-subsystem separation used by mature language projects, while keeping Cling's actual implementation and language-specific design independent.

## Workspace

- `compiler/cling-compiler/` contains the reusable compiler library.
- `tools/cling/` contains the command-line driver. It should coordinate compiler phases rather than duplicate their logic.
- `examples/` contains runnable `.cling` programs used for documentation and integration checks.
- `docs/Compiler/` describes the compiler pipeline and implementation status.

## Compiler modules

Inside `compiler/cling-compiler/src/`:

- `ast/` defines types, expressions, statements, functions, and programs.
- `lib/Parse/Lexer.rs` defines tokens, token kinds, and the source scanner in one file; the compiler crate imports this module through an explicit path.
- `lib/Parse/Parser.rs` contains shared parser state, declaration parsing, statement parsing, expression precedence, and parser tests in one file; the compiler crate imports this module through an explicit path.
- `sema/` separates statement checking from expression type checking.
- `cil/` contains the CIL model, lowering, module verification, values, and the stack-machine VM.
- `diagnostic.rs` owns source spans and structured diagnostics.
- `native.rs` emits host C for Clang-driven executable builds.
- `lib.rs` exposes the compiler phases and the top-level compilation entry point.

Public names are re-exported through each module's `mod.rs`, so callers can use stable paths such as `crate::ast::Expr` without depending on the internal file layout.

## Rules for new code

Keep a file focused on one concept, and split larger modules along stable responsibilities rather than creating files that only forward calls. Shared data types belong near their defining subsystem. Tests should stay beside the behavior they exercise until a dedicated integration-test layout is needed.

Cling libraries are intended to be written in Cling itself with the `.cling` extension. The foundation library is not implemented yet; don't introduce a replacement user-facing library in Rust or C.

See [Swift Parse subsystem reference and Cling adaptation](SwiftParseReference.md) for the responsibilities of Swift's `lib/Parse` files and which concepts fit Cling's current language.
