# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a JIT compiler for a simple expression language, targeting x86-64 Linux. It transforms expressions like `(a + b) * c - 10` into native machine code at runtime.

**Current Status**: Milestone 1 complete (integer arithmetic). Learning resources in `local/`.

**Architecture**: x86-64 only. Integration tests are skipped on ARM64 (Apple Silicon).

## Build Commands

```bash
cargo build                # Build all crates
cargo test                 # Run all tests
cargo test -p expr-parse   # Test single crate
```

## Architecture

### Crate Structure

- **expr-core**: AST nodes (later: NaN-boxed Value type, error types)
- **expr-parse**: Lexer and recursive descent parser
- **expr-x86**: x86-64 code generation, instruction encoding, executable memory management
- *expr-interp*: (planned) AST-walking and bytecode interpreters
- *expr-jit*: (planned) Public API, caching, `#[derive(Context)]` proc macro

### Compilation Pipeline

```
Source String → Lexer → Parser → AST → Canonicalize → Cache Check → Codegen → mmap/W^X → CompiledExpr
```

### Key Technical Details

**Value Representation**: NaN boxing (8-byte values)
- Floats: Any valid f64 (non-NaN or quiet NaN)
- Integers: `0x7FF8_0000_0000_0000 | i48`
- Booleans: `0x7FF9_0000_0000_0000 | (0 or 1)`

**Calling Convention**: System V AMD64 ABI
- Context pointer in `rdi`
- Return value in `rax`
- Callee-saved: `rbx`, `rbp`, `r12`-`r15`
- Float arithmetic uses XMM registers

**Error Handling**:
- Division by zero: SIGFPE handler with setjmp/longjmp recovery
- Integer overflow: Saturating arithmetic (check OF flag with `jo`)
- Type errors: Returned via `Result<Value, Error>`

## Development Milestones

1. Integer arithmetic (stack-based codegen)
2. Variables and calling convention (`#[derive(Context)]`)
3. Comparisons and booleans (short-circuit, ternary)
4. Dynamic typing (NaN boxing, type coercion)
5. Error handling (SIGFPE, saturation)
6. Interpreter baselines (AST-walker, bytecode VM)
7. Register allocation (linear scan)
8. Caching (AST canonicalization, deduplication)
9. Benchmarking (Criterion, perf counters)
10. Documentation and polish

## Language Grammar

```
expression → ternary
ternary    → or ( "?" expression ":" expression )?
or         → and ( "||" and )*
and        → equality ( "&&" equality )*
equality   → comparison ( ( "==" | "!=" ) comparison )*
comparison → term ( ( "<" | ">" | "<=" | ">=" ) term )*
term       → factor ( ( "+" | "-" ) factor )*
factor     → unary ( ( "*" | "/" | "%" ) unary )*
unary      → ( "!" | "-" ) unary | primary
primary    → NUMBER | IDENTIFIER | "(" expression ")" | "true" | "false"
```

## Key Resources

- `local/` Learning guide and reference materials
- Intel SDM Volume 2 for x86-64 instruction encoding
- https://defuse.ca/online-x86-assembler.htm for verification
