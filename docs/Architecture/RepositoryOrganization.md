# Repository organization

Cling borrows organizational ideas from the Swift compiler repository, not Swift implementation code. Compiler stages, interfaces, tools, library sources, documentation, and multiple test layers have separate homes.

- `include/cling/`: subsystem interfaces.
- `lib/Basic/`: source buffers, locations, diagnostics, and shared infrastructure.
- `lib/Lex/`: token model and lexer.
- `lib/Parse/`: parser implementation.
- `lib/AST/`: AST node implementations and traversal.
- `lib/Sema/`: declaration lookup, scopes, type checking, and diagnostics.
- `lib/CIL/`: Cling Intermediate Language, lowering, verification, and execution.
- `lib/Optimizer/`: optimization passes and pass management.
- `lib/CodeGen/`: target descriptions, ABI rules, and code generation.
- `lib/Driver/`: command-line options and compilation orchestration.
- `tools/cling/`: executable entry point.
- `stdlib/public/`: future Cling-written library modules, all using `.cling`.
- `test/`: source-level regression fixtures.
- `unittests/`: focused C++ tests.
- `validation-test/`: end-to-end language validation.
- `benchmark/`: reproducible performance measurements.
- `docs/`: architecture and language documentation.
- `cmake/`: reusable build settings.

## Bootstrap direction

The bootstrap compiler is C++20. The intended long-term compiler is written in Cling. This is a staged migration: the bootstrap compiles the first Cling-written compiler, and parity tests verify that the new compiler behaves correctly. The project is not yet self-hosted.

## Standard library policy

No foundation library is implemented yet. User-facing library modules must eventually be written in Cling source files with the `.cling` extension. Do not substitute a C, C++, or Rust foundation library. Temporary compiler intrinsics may provide only the low-level functionality that Cling cannot yet express.
