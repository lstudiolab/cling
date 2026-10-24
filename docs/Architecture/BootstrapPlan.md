# C++ bootstrap and self-hosting plan

## Stage 0: infrastructure
- [x] C++20 build configuration and warning policy.
- [x] Source buffer and source locations.
- [x] Shared diagnostic engine.
- [x] Token model and lexer.
- [x] Lexer unit tests and token inspection CLI.
- [ ] Parser and AST implementation.
- [ ] Name resolution and type checking.
- [ ] CIL lowering, verifier, and interpreter.
- [ ] Optimization passes.
- [ ] Native code generation with truthful target capability checks.

## Stage 1: compile Cling programs
The bootstrap must compile real `.cling` programs and reject malformed programs with actionable diagnostics. Each language feature needs positive, negative, and boundary tests.

## Stage 2: port components into Cling
Start with self-contained utilities such as token names, source helpers, diagnostic formatting, and AST utilities. Keep the C++ implementation as a reference until parity tests pass.

## Stage 3: self-host
Use the C++ bootstrap to compile a Cling-written compiler. Compare outputs and diagnostics against the reference on the same corpus. Self-hosting is complete only when the Cling compiler can rebuild itself reproducibly and pass the suite.

## Stage 4: native backends
Add target-specific code generation only with ABI tests, object inspection, execution tests where possible, and CI coverage. A target enum or name alone is not backend support.
