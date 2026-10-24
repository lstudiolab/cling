# Compilation pipeline

The current C++ bootstrap follows this pipeline:

```text
.cling source
    -> lexer and token stream
    -> recursive-descent parser and AST
    -> runtime checks during interpretation
       |-> C++ interpreter (default)
       `-> host C emission -> Clang -> native host executable
```

## Front end

The lexer tracks source spans and reports malformed tokens. The parser builds function, statement, and expression nodes using recursive descent and explicit operator-precedence levels. Runtime checks currently enforce variable mutability, value types, function argument counts, arithmetic safety, and condition types during interpretation; a complete ahead-of-time semantic analysis pass is still future work.

## Native host builds

The native path emits host C and asks Clang to compile and link it. This is a bootstrap host executable path, not direct machine-code emission written in Cling. A separately verified CIL IR, optimizer, CLLM, and cross-target backends are not yet ported from the retired Rust implementation.
