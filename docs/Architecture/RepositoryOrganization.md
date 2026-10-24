# Repository organization

Cling uses C++17 and CMake. The layout borrows compiler-subsystem ideas from mature language projects while keeping Cling's own implementation independent.

- `src/cling.cpp` contains the current lexer, AST, recursive-descent parser, interpreter, diagnostics, and host-C emitter.
- `CMakeLists.txt` defines the C++ build and smoke tests.
- `.github/workflows/cpp.yml` configures CI for CMake, C++ compilation, tests, and native smoke builds.
- `examples/` contains runnable `.cling` programs.
- `docs/Compiler/` records pipeline and implementation status.
- `docs/Architecture/SwiftParseReference.md` documents the Swift parser-directory reference and explains which features remain future work.

The initial C++ migration deliberately keeps the compiler implementation in one translation unit. Split it by stable responsibility after behavior parity and regression coverage improve.

Cling libraries are intended to be written in Cling itself with the `.cling` extension. The foundation library is not implemented yet; don't introduce a replacement user-facing library in C++.
