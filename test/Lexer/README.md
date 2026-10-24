# Lexer regression tests

The C++ bootstrap lexer tests live in `unittests/Lexer` and are registered with CTest. This directory is for source-level regression fixtures: each `.cling` file should document the behavior being tested and can later be consumed by a lit-style test runner.

Keep tests focused: one fixture should isolate one lexical rule or diagnostic. Include both a valid source fixture and an invalid source fixture when introducing a new diagnostic.