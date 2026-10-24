# Compiler pipeline

```text
.cling source
    -> lexer
    -> parser / AST
    -> semantic analysis
    -> CIL generation
    -> CIL structural verification
       |-> CIL VM (default execution path)
       `-> native C emission -> Clang -> host executable
```

## CIL

CIL (Cling Intermediate Language) is a target-independent executable IR. Its current form is a stack-based instruction stream with local slots, arithmetic, comparisons, branches, calls, and returns. The verifier checks entry-point presence, local-slot bounds, branch bounds, and call targets/arity before execution.

## Native host backend

The native path lowers the semantically checked AST to C11-compatible host code (using Clang's GNU C extensions for inferred local declarations), then asks Clang to produce the host executable. This gives Cling a usable native executable path without pretending that direct x86-64/AArch64/RISC-V/WebAssembly machine-code emitters already exist. The long-term architecture keeps CIL target-independent and can insert CLLM, optimization passes, target lowering, and a direct machine-code emitter later without changing the language front end.

## Current executable paths

- **CIL VM:** verifies the lowered stack IR, then executes it directly. This is the development/debugging path.
- **Native host:** lowers the checked AST to C, then invokes Clang to compile that C into a host executable. The generated C includes typed helper functions for `print`, C string comparison for `String` equality, and a C `main` wrapper.
- **Not yet implemented:** CLLM IR, direct machine-code emission, linker integration beyond Clang's driver, cross-target ABI lowering, and non-host target triples. The CLI rejects non-host targets rather than silently generating host code.

Example: `cling --emit exe examples/control_flow.cling -o ./control-flow`. This writes `./control-flow.generated.c` and the executable `./control-flow`.
