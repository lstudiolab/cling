# Implementation status

## Implemented in the C++ bootstrap

- C++17 compiler executable built by CMake; Rust/Cargo build files have been retired.
- Lexer with source spans, comments, escaped strings, numeric literals, keywords, and operators.
- Recursive-descent parser and AST for functions, typed parameters/returns, `let`, `var`, assignment, blocks, `if`/`else`, `while`, calls, unary/binary expressions, and returns.
- Interpreter with variable mutability/type checks, function calls, arithmetic overflow and division checks, and control flow.
- Host C emission and Clang-driven native executable builds.
- Hello-world and control-flow CTest smoke tests plus CI smoke builds.

## Not yet ported / implemented

- The prior verified CIL instruction set, lowering, verifier, and stack-machine VM.
- A complete ahead-of-time semantic analysis pass and rich diagnostics/recovery.
- CIL optimization passes, CLLM, direct machine-code emitters, object-file writers, register allocation, and cross-target ABI/code generation.
- Production-quality optimizer, standard library/package manager, debugger, and complete Swift-like language features.

## Verification

GitHub Actions builds with CMake and a C++17 compiler, runs CTest, and smoke-tests interpreter and Clang-backed native execution. Check the workflow result for the exact revision being reviewed.
