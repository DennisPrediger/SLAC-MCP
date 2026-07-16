# AGENTS.md

## Project

Rust MCP server exposing SLAC (Simple Logic & Arithmetic Compiler) as tools via the Model Context Protocol.

- Single crate, edition 2024 (requires Rust 1.85+)
- `rmcp` 2.2.0 - official Rust MCP SDK (tokio-based)
- `slac` 1.1.0 - expression-to-AST compiler with built-in tree-walk interpreter

## Purpose

Provide LLM agents a simple way to evaluate logical and arithmetic expressions at runtime via the SLAC expression language.

## Tools

- `evaluate(expression: string): string` - Compiles and evaluates a SLAC expression, returns the result as a string.
- `get_syntax(): string` - Returns the full SLAC language reference (syntax, operators, functions, examples) via `include_str!`.

## Non-Goals

- Exposing the compiled AST. SLAC supports this, but the MCP must not expose it.

## Architecture

- `src/main.rs` - MCP server bootstrap (tracing, stdio transport)
- `src/server.rs` - `SlacServer` unit struct with tool handlers and `ServerHandler` impl
- `src/get-syntax.md` - SLAC language reference included by `get_syntax`

### Key Constraint: `!Send` Environment

`slac::StaticEnvironment` is `!Send` + `!Sync` (contains `Rc` internally). It cannot be stored in the service struct behind `Arc<Mutex<>>`. Each `evaluate` call creates a fresh environment: `StaticEnvironment::default()` -> `extend_environment()` -> `compile()` -> `execute()`.

`get_syntax` includes the full function reference via `include_str!`, so a separate function listing tool is not needed.

## Build & Run

```sh
cargo build
cargo run
```

For testing with MCP Inspector:

```sh
npx @modelcontextprotocol/inspector cargo run
```

No test, lint, or CI configuration exists yet.
