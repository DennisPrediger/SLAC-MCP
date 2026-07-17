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

# License

Copyright 2026 Dennis Prediger

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.