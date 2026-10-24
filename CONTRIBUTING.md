# Contributing to Cling

Cling is an experimental language with a C++20 bootstrap and a planned self-hosted compiler written in Cling.

## Build and test

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
cmake --build build --parallel
ctest --test-dir build --output-on-failure
```

## Organization

Place interfaces in `include/cling/<Subsystem>/` and implementations in the matching `lib/<Subsystem>/` directory. Keep the CLI thin. Shared diagnostics and source locations belong in Basic. Add focused unit tests under `unittests/<Subsystem>/` and source fixtures under `test/<Subsystem>/`.

## Quality requirements

- Do not mark an incomplete phase as implemented.
- Test valid input, invalid input, and boundary conditions.
- Use source-located diagnostics for user errors.
- Keep language semantics separate from host-specific code generation.
- Do not implement the user-facing foundation library in C, C++, or Rust; future library APIs belong in `.cling` source.
- Borrow repository organization from Swift without copying Swift source code.
