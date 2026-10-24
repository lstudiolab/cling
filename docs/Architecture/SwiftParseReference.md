# Swift `lib/Parse` reference and Cling adaptation

This note records the responsibilities of the files in the Swift project's `lib/Parse` directory, excluding `Parser.cpp` and `Lexer.cpp`. The goal is to learn from the subsystem boundaries, not to copy Swift's C++ implementation into Cling's Rust compiler.

Reference: [swiftlang/swift/lib/Parse](https://github.com/swiftlang/swift/tree/main/lib/Parse).

## What the files do

| Swift file | Responsibility | Cling adaptation |
| --- | --- | --- |
| `CMakeLists.txt` | Builds and links the parser library from its implementation files. | Cargo owns compilation and module wiring; no CMake parser target is needed. |
| `Confusables.cpp` | Recognizes Unicode characters that can be confused with ASCII characters. | Useful once Cling defines identifier-security diagnostics and warning severities. Not added as a standalone feature before those policies exist. |
| `ParseDecl.cpp` | Parses declarations and builds declaration AST nodes, including functions and other declaration forms. | Function declarations remain in `Parser.rs`; the current AST does not yet define Swift-like structs, extensions, properties, or access modifiers. |
| `ParseDeclName.cpp` | Parses and validates declaration names and argument-label-like name forms. | Cling currently has simple identifiers and positional calls. Rich name/label rules should arrive with named arguments or richer declarations. |
| `ParseExpr.cpp` | Parses expressions, operator sequences, calls, and expression AST nodes. | Expression parsing stays in `Parser.rs` as requested; its precedence-climbing helpers are kept together. |
| `ParseGeneric.cpp` | Parses generic parameter lists and generic constraints/requirements. | Deferred until Cling has generic syntax and generic AST/type-checking support. |
| `ParseIfConfig.cpp` | Parses and evaluates conditional-compilation directives such as platform/version conditions. | Deferred because Cling has no conditional-compilation grammar or build-configuration model yet. |
| `ParsePattern.cpp` | Parses patterns and function/closure parameter structures. | Cling currently supports simple named parameters and variable declarations, not tuple, destructuring, enum-case, or closure patterns. |
| `ParseRegex.cpp` | Parses regular-expression literals into expression nodes. | Deferred until regex literal syntax and a regex runtime/library are designed. |
| `ParseRequests.cpp` | Implements lazy parsing requests and integration with Swift's AST evaluator and bridging layers. | Cling's current compiler is eager and has no request evaluator or AST-generation bridge; copying this would not fit its architecture. |
| `ParseStmt.cpp` | Parses statements, brace-delimited item lists, control flow, and statement-level recovery. | Statement parsing remains in `Parser.rs`; shared token movement and synchronization are extracted to `ParserSupport.rs`. |
| `ParseType.cpp` | Parses type representations, including compound, function, generic, and other Swift type forms. | Cling's supported type names are small (`Int`, `Bool`, `String`, and return-only `Void`); parsing helpers can be shared without creating a separate type-parser file. |
| `ParseVersion.cpp` | Parses and validates Swift language/compiler version values. | Deferred until Cling has an explicit language-version directive or versioned feature gates. |
| `PersistentParserState.cpp` | Stores parser state that must survive across multiple parsing operations, including IDE inspection state. | Cling's command-line compilation currently creates a fresh parser for each source. Persistent state belongs with future incremental/IDE parsing work. |

## What is implemented now

`lib/Parse/Parser.rs` remains the single file for Cling grammar rules: declarations, statements, expressions, precedence, and parser tests.

`lib/Parse/ParserSupport.rs` owns shared parser infrastructure that does not define a separate grammar subsystem:

- A token cursor with safe lookahead and a guaranteed EOF sentinel, even if a caller supplies an empty or EOF-less token stream.
- A single position counter used by parser progress checks.
- A place to add shared recovery and token-validation utilities as the language grows.

This is intentionally smaller than Swift's parser. Swift's files reflect a mature language with many grammar features and IDE/compiler services that Cling does not currently implement. The rule is to adopt the architectural idea when Cling has the corresponding language feature, not to create empty modules just to match Swift's file names.
