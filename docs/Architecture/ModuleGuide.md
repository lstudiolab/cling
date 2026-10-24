# Compiler module guide

The current C++17 bootstrap lives in `src/cling.cpp` to keep the initial language migration straightforward. It contains the lexer, AST model, parser, interpreter, diagnostics, and host-C emitter.

When extending the compiler:

- Preserve source spans and report invalid source as diagnostics rather than crashes.
- Add tests for both valid and invalid programs.
- Keep source-language behavior distinct from host-specific code generation.
- Split code into headers and translation units when the boundaries are stable and tests cover the behavior.
- Do not create empty module stubs merely to mirror another compiler's directory layout.
- Keep the foundation library in `.cling` source; do not implement user-facing standard-library APIs in C++.
