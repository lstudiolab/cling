# Contributing to Cling

Cling's compiler is implemented in C++17 and built with CMake.

## Build and test

Install a C++17 compiler and CMake 3.16 or newer. Install Clang as well to test the native executable backend.

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --parallel 2
ctest --test-dir build --output-on-failure
```

## Code organization

The current migration bootstrap is in `src/cling.cpp`. It contains tokenization, AST definitions, recursive-descent parsing, execution, diagnostics, and host-C emission. Keep language semantics separate from host-specific code generation. As the port stabilizes, new code can be split into headers and C++ translation units along stable subsystem boundaries.

Preserve source spans in diagnostics and add regression tests for valid and invalid programs. Do not claim a target or language feature is supported unless it works end to end.

Future user-facing foundation library code belongs in `.cling` source, not in C++.
