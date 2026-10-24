# Cling

Cling is an experimental programming language. Its compiler is being organized as a staged C++20 bootstrap that will eventually compile a self-hosted compiler written in Cling itself.

The repository borrows the **organizational ideas** of the Swift compiler project—subsystem-oriented source directories, matching headers, compiler tools, standard-library sources, documentation, and separate test layers. It does not copy Swift implementation code.

## Current bootstrap status

The active C++ bootstrap currently implements source buffers, source locations, shared diagnostics, token definitions, lexical analysis, and a token-inspection CLI. Parser, AST construction, semantic analysis, CIL lowering, optimization, and machine-code generation are planned stages and must not be described as working until implemented and tested.

The earlier Rust workspace remains in the repository as a prototype while the C++ bootstrap becomes the canonical implementation. Its presence does not mean that the C++ bootstrap already has feature parity.

## Build the C++ bootstrap

Requirements: CMake 3.20 or newer and a C++20 compiler.

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build --parallel
ctest --test-dir build --output-on-failure
```

Inspect the token stream of a Cling source file:

```sh
./build/cling --emit tokens examples/hello.cling
```

The CLI also supports `--help` and `--version`. Until the parser and later phases are ported, it explicitly reports that other compilation modes are not implemented.

## Language direction

Cling's syntax and semantics will evolve independently. The repository currently contains prototype examples, which are not a promise that every example is accepted by the C++ bootstrap yet.

## Standard library policy

There is no foundation library yet. Future library modules must be written in Cling itself and use the `.cling` extension. Do not implement the user-facing foundation library in C, C++, or Rust.

During bootstrap, only documented compiler intrinsics may provide functionality that the language cannot yet express. `print` and `log` are intended to be temporary intrinsics, not a separate library. Replace them with Cling-written library APIs once the necessary runtime and interoperability mechanisms exist.

## Repository map

- `include/cling/`: interfaces grouped by compiler subsystem.
- `lib/Basic/`: source buffers, source locations, diagnostics, and shared utilities.
- `lib/Lex/`: tokens and lexical analysis.
- `lib/Parse/`: parser implementation.
- `lib/AST/`: syntax tree nodes and traversal.
- `lib/Sema/`: name resolution, scopes, type checking, and semantic diagnostics.
- `lib/CIL/`: Cling Intermediate Language, lowering, and verification.
- `lib/Optimizer/`: optimization passes and pass management.
- `lib/CodeGen/`: target descriptions and code generation.
- `lib/Driver/`: command-line parsing and compilation orchestration.
- `tools/cling/`: executable entry point.
- `stdlib/public/`: reserved for future Cling-written `.cling` library sources.
- `test/`: source-level regression fixtures organized by stage.
- `unittests/`: isolated C++ unit tests.
- `validation-test/`: end-to-end tests for supported language features.
- `benchmark/`: reproducible performance experiments.
- `docs/`: language and compiler architecture documentation.
- `cmake/`: shared build configuration.

Directories are added as their subsystem boundaries become useful; a directory or interface declaration alone is not evidence that a compiler phase is complete.

## Bootstrap and self-hosting roadmap

1. Finish and test the C++ source and lexer infrastructure.
2. Implement the parser and AST.
3. Add name resolution, semantic analysis, and type checking.
4. Define and verify CIL, then implement lowering and execution.
5. Add optimization passes and real code-generation backends.
6. Use the bootstrap to compile compiler components written in Cling.
7. Prove self-hosting with reproducible bootstrap builds and parity tests.

## Code quality

New compiler subsystems should have focused headers under `include/cling/<Subsystem>/`, implementations under `lib/<Subsystem>/`, unit tests under `unittests/<Subsystem>/`, and source-level regression fixtures under `test/<Subsystem>/`. Keep the command-line driver thin and diagnostics source-located. Do not claim an architecture is supported until the backend and its tests actually work.
