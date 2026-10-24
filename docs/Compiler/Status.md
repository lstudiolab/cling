# Implementation status

This document distinguishes the canonical C++ bootstrap from the earlier Rust prototype. A component being present in the repository does not mean the entire compilation pipeline is complete.

## C++ bootstrap: implemented

- CMake-based C++20 build with strict warning options.
- Source buffers, source locations, and shared diagnostics.
- Token model and lexer for prototype keywords, identifiers, integer/string literals, operators, punctuation, line comments, and nested block comments.
- Token inspection CLI with help and version output.
- AST node models and AST context ownership utilities.
- Basic type metadata and type-name formatting.
- CIL instruction/module data structures and a verifier for local slots, branch targets, call indices, and empty functions.
- Pass-manager metadata, target descriptions, and CLI option parsing.
- Unit tests for lexing, source locations, diagnostics, CIL verification, and header integration.

The C++ executable's working compilation mode is currently `--emit tokens`. The existence of AST, CIL, semantic, optimizer, and target interfaces is not a claim that the end-to-end phases are complete.

## C++ bootstrap: not yet implemented end-to-end

- Recursive-descent parsing and AST construction.
- Name lookup, full semantic analysis, and type checking.
- AST-to-CIL lowering, a CIL interpreter, and complete CIL semantics.
- Optimization passes that transform programs.
- Native object emission, machine-code backends, ABI lowering, and linker integration.
- Cling-written compiler components and reproducible self-hosting.

## Legacy Rust prototype

The repository retains the earlier Rust implementation as a reference during migration. It contains a broader prototype pipeline, but its current Rust CI has known compile failures and is not treated as the canonical implementation. The legacy Rust workflow is scoped to changes in Rust-specific source paths.

## Verification

The C++ bootstrap's GitHub Actions workflow configures CMake with warnings treated as errors, builds the C++ targets, and runs CTest. Check the latest workflow run for the authoritative result; do not infer success from documentation alone.
