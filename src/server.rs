use rmcp::{
    handler::server::wrapper::Parameters, model::*, tool, tool_router, ErrorData as McpError,
};
use rmcp::{tool_handler, ServerHandler};

use slac::stdlib::extend_environment;
use slac::{check_variables_and_functions, compile, execute, StaticEnvironment};

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
struct EvaluateParams {
    expression: String,
}

#[derive(Clone)]
pub struct SlacServer;

#[tool_router]
impl SlacServer {
    #[tool(
        description = "Compiles and evaluates a SLAC expression. Returns the result as a string.
Numbers are returned as their numeric representation, booleans as
`true`/`false`, strings without quotes, arrays as comma-separated values in
brackets.

## Input

The expression as a single String. See `get_syntax` for the extract syntax.

## Output

The string representation of the evaluated result.

## Examples

| Expression                                        | Result            |
| ------------------------------------------------- | ----------------- |
| `1 + 2`                                           | `3`               |
| '3.14 = 3`                                        | `false`           |
| `max(10, 20)`                                     | `20`              |
| `'Hello' + ' ' + 'World'`                         | `Hello World`     |
| `True and not False`                              | `true`            |
| `[1,2,3] + [4,5]`                                 | `[1, 2, 3, 4, 5]` |
| `sort([3,1,2])`                                   | `[1, 2, 3]`       |
| `lowercase('HELLO')`                              | `hello`           |
| `string_to_date('2024-01-15')`                    | `19726`           |
| `re_find('abc123def456', '\\d+')`                 | `['123', '456']`  |
| `if_then(10 > 5, 'yes', 'no')`                    | `yes`             |
| `pow(2, 10)`                                      | `1024`            |
| `contains('hello world', 'world')`                | `true`            |
| `int('42.9')`                                     | `42`              |
| `split_csv('\"a;b\";c', ';')`                       | `['a;b', 'c']`    |
| `at('abc', 2)`                                    | `b`               |
| `find('hello world', 'world')`                    | `7`               |
| `replace('hello', 'l', 'r')`                      | `herro`           |
| `random(100)`                                     | _(varies, 0-99)_  |
| `encode_date(2024, 12, 25)`                       | `19722`           |
| `year(string_to_datetime('2024-07-04 12:00:00'))` | `2024`            |

## Error Handling

If the expression is invalid or contains unsupported features (e.g., undefined
variables), the tool returns an error message describing the problem.
"
    )]
    fn evaluate(
        &self,
        Parameters(params): Parameters<EvaluateParams>,
    ) -> Result<CallToolResult, McpError> {
        let ast = compile(&params.expression)
            .map_err(|e| McpError::invalid_params(format!("compile error: {e}"), None))?;

        let mut env = StaticEnvironment::default();
        extend_environment(&mut env);

        check_variables_and_functions(&env, &ast)
            .map_err(|e| McpError::invalid_params(format!("validation error: {e}"), None))?;

        let result = execute(&env, &ast)
            .map_err(|e| McpError::invalid_params(format!("execution error: {e}"), None))?;

        Ok(CallToolResult::success(vec![ContentBlock::text(
            result.to_string(),
        )]))
    }

    #[tool(
        description = "Returns the complete SLAC language reference: syntax rules, operator precedence, all built-in functions with their signatures, and usage examples. 
        Call this tool when you need to look up available functions or verify syntax before calling `evaluate`."
    )]
    fn get_syntax(&self) -> Result<CallToolResult, McpError> {
        let result = include_str!("get-syntax.md");

        Ok(CallToolResult::success(vec![ContentBlock::text(
            result.to_string(),
        )]))
    }
}

#[tool_handler(
    name = "SLAC-MCP",
    version = "1.1.0",
    instructions = "Evaluates SLAC (Simple Logic & Arithmetic Compiler) expressions. SLAC is a Delphi/Pascal-inspired expression language with arithmetic, string manipulation, date/time, regex, and a rich standard library. All expressions are single-line and return a value. Use `get_syntax` first if unfamiliar with the language.

## Instructions

You have access to the SLAC expression evaluator. 
SLAC is a expression language for arithmetic, logic, string operations, date/time calculations, and regex matching. 
It uses 'Delphi Pascal/Free Pascal'-like syntax.

### Key Rules

- Strings use **single quotes**: `'hello'`
- Escape single quotes by doubling: `'It''s here'`
- Double quotes are **forbidden**
- Inequality is `<>`, not `!=`
- Boolean literals are case-insensitive: `True`, `true`, `TRUE`
- Function and variable names are case-insensitive
- All numbers are floating-point (f64)
- Use `div` for integer division, `mod` for modulo, `/` for float division
- `^` is **not** exponentiation — use `pow(base, exp)` instead
- Arrays are written as `[1, 2, 3]` and concatenated with `+`

### Available Tools

| Tool | Purpose |
|------|---------|
| `evaluate`       | Compiles and evaluates a SLAC expression, returns the result as a string |
| `get_syntax`     | Returns language reference: syntax rules, operators, all functions with signatures |
"
)]
impl ServerHandler for SlacServer {}
