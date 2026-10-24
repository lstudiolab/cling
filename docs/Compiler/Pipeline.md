# Compiler pipeline

The intended Cling pipeline is organized into independently testable stages:

```text
.cling source
    -> source management
    -> lexer
    -> parser / AST
    -> semantic analysis
    -> CIL generation
    -> CIL verification
    -> optimization
    -> target lowering / code generation
    -> executable or other supported output
```

## Current C++ bootstrap

The C++20 bootstrap currently implements source management, diagnostics, lexing, token inspection, AST node data structures, type metadata, a preliminary CIL instruction model and verifier, target descriptions, pass-manager metadata, and driver option parsing.

The CLI currently supports `--emit tokens`, `--help`, and `--version`. Parsing, AST construction, complete semantic analysis, CIL lowering/execution, optimization transforms, and native code generation are not yet connected into an end-to-end C++ compiler.

## CIL

CIL means Cling Intermediate Language. It is intended to be a target-independent intermediate representation between semantic analysis and code generation. Its design goals include explicit function boundaries, local slots, control flow, calls, verification before execution, and optimization passes that can be tested independently.

The current C++ verifier checks local-slot bounds, branch-target bounds, call-index bounds, and empty functions. A complete instruction semantics specification, AST lowering, interpreter, and native backend remain future work.

## Legacy Rust prototype

The repository's earlier Rust prototype had a larger experimental pipeline, including a VM and host-C path. That prototype is retained temporarily during migration, but it is not the canonical bootstrap plan. Its current compiler errors are tracked separately from the C++ implementation.
