# Parser implementation

The parser interface is declared in `include/cling/Parse/Parser.h`. The recursive-descent parser implementation is a planned next stage and is not wired into the current C++ executable yet.

When implementing it, keep token navigation, grammar productions, expression precedence, and error recovery independently testable. Add parser unit tests under `unittests/Parse/` and source-level valid/invalid fixtures under `test/Parse/`. Do not report parsing as supported until the C++ bootstrap can parse real `.cling` files and the tests pass.
