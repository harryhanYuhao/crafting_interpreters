# Grammar of the Lox Language

Lox is a mock language and designed to be simple and easy to implement. 

## Keywords 

| Keyword | Description |
|---------|-------------|
| `true`  | Boolean true | 
| `false` | Boolean false |
| `nil`   | Null value   |
| `if`    | Conditional statement |
| `else`  | Conditional statement |
| `while` | Loop statement |
| `for`   | Loop statement |
| `break` | Loop control statement |
| `continue` | Loop control statement |
| `return` | Function return statement |
| `class` | Class definition |
| `this`  | Reference to the current instance |
| `super` | Reference to the superclass |
| `fn`   | Function definition |
| `var`   | Variable declaration |


## Syntax 

Statements are and must be terminated by `;`. If it reaches the end of the line, without finding `;`, and a statement is not complete, the interpretor shall panic.

## Types 

Lox has the following types:

- `nil` - Null value
- `bool` - Boolean value
- `number` - Floating point number. Lox does not have interger type.
- `string` - String value
- `class` - Custom class type
