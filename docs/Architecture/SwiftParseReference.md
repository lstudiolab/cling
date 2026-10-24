# Swift `lib/Parse` reference and Cling adaptation

This note records the responsibilities of files in Swift's `lib/Parse` directory, excluding `Parser.cpp` and `Lexer.cpp`. The goal is to learn from subsystem boundaries, not to copy Swift's implementation into Cling.

Reference: [swiftlang/swift/lib/Parse](https://github.com/swiftlang/swift/tree/main/lib/Parse).

| Swift file | Responsibility | Cling adaptation |
| --- | --- | --- |
| `CMakeLists.txt` | Builds and links parser implementation files. | CMake now builds Cling's C++ compiler. |
| `Confusables.cpp` | Detects Unicode characters that resemble ASCII characters. | Defer until identifier-security diagnostics and warning policy are defined. |
| `ParseDecl.cpp` | Parses declarations and builds declaration AST nodes. | Function declarations remain in the current parser; richer declarations and types are future work. |
| `ParseDeclName.cpp` | Parses declaration names and argument labels. | Cling currently uses simple identifiers and positional calls. |
| `ParseExpr.cpp` | Parses expressions, operators, calls, and expression AST nodes. | Expression parsing stays with the single parser implementation for now. |
| `ParseGeneric.cpp` | Parses generic parameters and constraints. | Deferred until generic syntax and type checking exist. |
| `ParseIfConfig.cpp` | Parses conditional-compilation directives. | Deferred until conditional compilation is designed. |
| `ParsePattern.cpp` | Parses patterns and parameter structures. | Cling currently supports simple named parameters and bindings. |
| `ParseRegex.cpp` | Parses regular-expression literals. | Deferred until regex syntax and runtime support exist. |
| `ParseRequests.cpp` | Integrates lazy parsing requests with compiler/IDE services. | Cling is currently an eager command-line compiler. |
| `ParseStmt.cpp` | Parses statements, control flow, and recovery. | Statement parsing stays with the parser for now. |
| `ParseType.cpp` | Parses compound, function, generic, and other type forms. | Current types are `Int`, `Bool`, `String`, and return-only `Void`. |
| `ParseVersion.cpp` | Parses language/compiler versions. | Deferred until version directives or feature gates exist. |
| `PersistentParserState.cpp` | Persists parser state for IDE/incremental parsing. | Deferred until incremental parsing is needed. |

Swift's file layout reflects a mature language with grammar features and IDE/compiler services Cling does not currently implement. Adopt each architectural idea when the corresponding Cling feature exists rather than creating empty modules to match Swift's filenames.
