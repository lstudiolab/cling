# Compiler pipeline

```text
.cling source
    -> lexer
    -> parser / AST
    -> C++ interpreter (default execution path)
       `-> host C emission -> Clang -> host executable
```

The C++ bootstrap currently supports the documented core syntax and performs several semantic checks at runtime. A separate static semantic analysis pass and the prior Rust implementation's verified CIL VM are not yet ported. `--emit cil` is only a function-level pipeline summary, not the old executable CIL format.

The native path emits C and invokes Clang to create a host executable. It does not directly emit machine code. Direct CLLM machine-code emission, object-file writers, register allocation, target ABI lowering, and cross-target backends remain future work. The CLI rejects non-host targets.
