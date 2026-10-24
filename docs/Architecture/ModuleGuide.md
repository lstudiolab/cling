# Compiler module guide

Use this guide when extending the compiler:

- Add new grammar productions to the parser modules and cover them with valid and invalid source tests.
- Add AST node data to `ast/`, preserving source spans on nodes and statements.
- Add type and scope rules to `sema/`; avoid embedding semantic checks in the parser.
- Add new target-independent operations to `cil/instruction.rs`, implement their lowering and VM behavior, and extend verifier checks as necessary.
- Keep native host-specific rules inside `native.rs` and the command-line driver in `tools/cling`.
- Use structured diagnostics with spans rather than panics for invalid user programs.

When a module becomes difficult to reason about, split it by responsibility and re-export the necessary public types through its `mod.rs`. Don't create empty module stubs or duplicate existing behavior merely to increase the file count.
