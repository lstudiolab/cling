# Contributing to Cling

Cling is an experimental programming language and compiler maintained as a Rust Cargo workspace.

## Build and test

Install the stable Rust toolchain and Clang, then run the checks from the repository root:

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Some tests build a native host executable through Clang, so Clang is required for the native backend tests.

## Code organization

The compiler crate is under `compiler/cling-compiler`; the command-line driver is under `tools/cling`. Compiler phases currently use separate Rust modules for the AST, lexer, parser, semantic analysis, diagnostics, CIL, and native host emission.

Keep source-language semantics separate from host-specific code generation. Put reusable logic in the compiler crate rather than duplicating it in the CLI. Preserve source spans in diagnostics and add regression tests for both valid and invalid programs.

## Pull requests

- Describe the behavior change and the relevant compiler phase.
- Add or update unit tests and examples for user-visible behavior.
- Do not claim a target or language feature is supported unless it works end to end.
- Keep the foundation library policy intact: future user-facing library code belongs in `.cling` source, not in Rust or C.
- Run formatting, tests, and lint checks before marking a change ready.
