# Language design status

Cling is an evolving language; this document records compiler-facing design constraints rather than declaring every feature implemented.

## Source files

Application and library source use the `.cling` extension. Libraries are intended to be authored in Cling itself.

## Bootstrap policy

The first compiler is implemented in C++20 so it can compile the first Cling-written compiler. The eventual compiler should be self-hosted. The bootstrap must report unsupported features and targets honestly.

## Foundation APIs

There is no foundation library yet. Temporary compiler intrinsics such as `print` may bridge capabilities that cannot yet be expressed in the language. Do not replace the planned Cling-written library with a permanent C, C++, or Rust implementation.

## Syntax

Syntax is still being designed. The lexer currently recognizes the prototype keywords `func`, `let`, `var`, `if`, `else`, `while`, and `return`, plus basic literals, operators, and punctuation. The lexer token set is not a complete language specification.
