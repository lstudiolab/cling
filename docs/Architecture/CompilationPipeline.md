# Compilation pipeline

The executable Rust compiler currently follows this pipeline:

```text
.cling source
    -> lexer and token stream
    -> parser and AST
    -> semantic analysis
    -> CIL lowering
    -> CIL verification
       |-> CIL stack-machine VM
       `-> host C emission -> Clang -> native host executable
```

## Front end

The lexer tracks source spans and reports malformed tokens. The parser builds the AST using recursive-descent parsing and explicit operator-precedence levels. Semantic analysis resolves function and variable declarations, checks scopes and mutability, validates expressions and calls, and checks return types before lowering.

## CIL

CIL (Cling Intermediate Language) is a target-independent stack IR. Lowering turns validated AST nodes into instructions; the verifier checks the entry point, local slots, branch targets, and call targets/arity before the VM executes the module.

The VM is the main development execution path. It handles arithmetic overflow, division errors, function calls, control flow, and expression values. The IR is kept separate from host-specific code generation to allow future backends without changing the front end.

## Native host builds

The current native path emits C for the host and asks Clang to compile and link it. This is a supported host executable path, not a direct machine-code emitter written in Cling. Standalone CLLM object emission and cross-target backends remain future work and must not be described as implemented until their code generation and tests exist.
