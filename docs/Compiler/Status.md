# Implementation status

## Implemented in this repository

- Rust workspace and `cling` CLI.
- Lexer with source spans and lexical diagnostics.
- Recursive-descent parser and AST for functions, typed parameters/returns, bindings, assignment, blocks, `if`/`else`, `while`, calls, unary/binary expressions, and returns.
- Semantic checks for variable scopes, mutability, expression types, function calls, return types, and `Void` initializers.
- CIL lowering, structural verification, and a CIL VM.
- Native host executable path: checked AST -> generated C -> Clang -> host executable.
- Examples and CI smoke tests for VM execution and native executable output.

## Explicitly not implemented yet

- A standalone CLLM machine-code emitter.
- Native object-file writers, register allocation, ABI/calling-convention implementations, and direct machine-code backends for x86, x86-64, ARM, AArch64, RISC-V, or WebAssembly.
- A production-quality optimizer, standard library/package manager, debugger, and complete Swift-like language features.

## Verification note

The project environment used to prepare this source bundle did not contain `rustc`, `cargo`, or `rustfmt`, and installing the Rust toolchain timed out. Therefore the Rust workspace could not be compiled or unit-tested in this environment. The CI workflow installs the stable Rust toolchain and Clang, runs formatting, unit tests, Clippy, the VM example, and a native executable smoke test. The C lowering strategy was separately sanity-checked by compiling representative generated-style C with Clang.
