# slac-mcp

MCP server exposing SLAC (Simple Logic & Arithmetic Compiler) as tools for LLM agents.

## Tools

| Tool | Parameters | Description |
|------|-----------|-------------|
| `evaluate` | `expression: string` | Compiles and evaluates a SLAC expression, returns the result |
| `get_syntax` | none | Returns the full SLAC language reference (syntax, operators, functions, examples) |

## Usage

```sh
cargo run
```

The server communicates over stdio using the Model Context Protocol.

### Output Format

By default, `evaluate` returns the Rust `Display` representation of the result. 
Set `OUTPUT_FORMAT=JSON` to get serialized JSON instead 

### With MCP Inspector

```sh
npx @modelcontextprotocol/inspector cargo run
```

### Example expressions

```
1 + 2
max(10, 20) + 1
'hello' + ' ' + 'world'
len('test') > 2 ? 'long' : 'short'
```

## Development

```sh
cargo build
cargo run
```

Requires Rust 1.85+ (edition 2024).
