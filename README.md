# expression-jit

A JIT compiler for a simple expression language. Takes expressions like `(a + b) * c - 10` and compiles them to native x86-64 machine code at runtime.

## What it does

You give it a string. It parses the expression, generates machine code in memory, and gives you back a function pointer. Then you can call that function and get the result.

My goal is to understand how JIT compilers work (Instruction encoding, memory permissions, calling conventions, etc).

## Building

```
cargo build
cargo test
```

## Project structure

- `expr-core` - AST types and value representation
- `expr-parse` - lexer and recursive descent parser
- `expr-x86` - x86-64 code generation and runtime

## Current status

Just starting out and learning!

## Example

```rust
use expr_parse::Parser;
use expr_x86::compile;

fn main() {
    let mut parser = Parser::new("(2 + 3) * 4").unwrap();
    let ast = parser.parse().unwrap();
    let code = compile(&ast).unwrap();

    let result = unsafe { code.execute() };
    println!("{}", result); // 20
}
```

## Why

Because I wanted to know what happens between source code and silicon. Turns out it's a lot of fiddly byte manipulation and reading Intel manuals.
