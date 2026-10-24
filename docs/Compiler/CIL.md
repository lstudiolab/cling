# Cling Intermediate Language

CIL is the compiler's planned explicit intermediate representation between semantic analysis and backend code generation. It is designed to be easier to verify than source syntax and stable enough to support multiple execution and code-generation paths.

## Design goals

- Explicit function boundaries, local slots, control flow, and calls.
- A verifier that rejects invalid branch targets, invalid local references, and invalid function references before execution or code generation.
- Optimization passes that operate on verified modules and can be tested independently.
- Target-independent semantics; target-specific ABI decisions belong in CodeGen.
- Human-readable dumps for debugging and regression tests.

The current C++ repository contains an instruction/module model and a first verifier for local slots, branch targets, and call indices. AST lowering, a complete instruction semantics specification, interpreter, and native backend are not yet implemented in the C++ bootstrap.
