# Compilation pipeline

Compiler stages are independent so they can be tested and evolved without coupling the command-line driver to implementation details.

1. Source management owns source buffers and maps byte offsets to line and column locations.
2. Lexing converts source characters into tokens and emits source-located diagnostics.
3. Parsing consumes tokens and builds syntax nodes.
4. AST validation checks structural invariants before semantic analysis.
5. Semantic analysis resolves declarations, scopes, types, mutability, and calls.
6. CIL lowering converts validated AST nodes into an explicit intermediate representation.
7. CIL verification checks operands, control-flow edges, local slots, and function boundaries.
8. Optimization runs ordered, independently testable transformations.
9. Code generation emits code only for targets with a real supported backend.
10. The driver handles options, output paths, diagnostics, and orchestration.

The C++ bootstrap currently implements source management and lexical analysis. Interfaces or directories for later phases do not imply that those phases are complete. Unsupported compiler modes must fail explicitly rather than pretend to produce output.
