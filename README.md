# Cling

Cling is an experimental programming language whose compiler is implemented in **C++17**. The Rust/Cargo implementation has been retired; CMake and a C++ toolchain are the compiler build requirements.

## Build

Requirements: CMake 3.16+, a C++17 compiler, and Clang if you want `--emit exe`.

```sh
cmake -S . -B build -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
ctest --test-dir build --output-on-failure
```

## Run

```sh
./build/cling examples/hello.cling
./build/cling examples/control_flow.cling
```

On Windows, the executable may be at `build/Release/cling.exe` depending on the generator.

## Inspect and build

```sh
./build/cling --emit tokens examples/hello.cling
./build/cling --emit ast examples/hello.cling
./build/cling --emit c examples/hello.cling
./build/cling --emit exe examples/hello.cling -o hello
```

The default action interprets the program in the C++ runtime. `--emit exe` translates the supported program subset to host C and invokes Clang. This is a bootstrap native backend, not direct machine-code emission. The `--emit cil` option currently prints a compact function-level pipeline summary; the former Rust CIL implementation has not yet been ported as a distinct verified IR.

## Language subset

```cling
func sum(a: Int, b: Int) -> Int {
    return a + b;
}

func main() {
    let greeting: String = "Hello";
    var count: Int = sum(20, 22);
    print(greeting);
    print(count);
}
```

The bootstrap supports `func`, typed parameters and return values, `let`, `var`, assignments, `if`/`else`, `while`, `return`, nested blocks, `pass`, `Int`, `Bool`, `String`, calls, `print`/`log`, arithmetic, comparisons, equality, boolean operators, `//` comments, and escaped strings. Statements end in semicolons.

## Architecture

The current C++ bootstrap keeps its lexer, AST, recursive-descent parser, interpreter, and host-C emitter in `src/cling.cpp` while the implementation is being migrated. The code is intentionally kept together for the initial port; stable components can be split into C++ translation units after behavior parity and tests are established.

Cling's future standard/foundation libraries are intended to be written in Cling source (`.cling`), not Rust or C++. `print` and `log` remain temporary compiler-provided intrinsics until the language has the runtime/interoperability features needed to implement them in Cling itself.

## Status and portability

This is an early bootstrap compiler, not production-ready. Only `--target host` is supported. Cross-target CLLM code generation is not implemented; a target name is not treated as supported merely because it is recognized.
