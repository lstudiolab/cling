# Cling standard library

The standard library is intentionally not implemented yet. When introduced, user-facing modules will be written in Cling and use the `.cling` extension.

Do not add a C, C++, or Rust foundation library here. Temporary compiler intrinsics may exist only for behavior the language/runtime cannot yet express; document each intrinsic and replace it with Cling source where feasible.
