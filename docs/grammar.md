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

Each line is a statement. End of statement does not need to have ``;``. 

## Types 

Lox has the following types:

- `nil` - Null value
- `bool` - Boolean value
- `number` - Floating point number. Lox does not have interger type.
- `string` - String value
- `class` - Custom class type

## Variable declaration

Variables shall be declared using `var` keyword like this:

```
var a = 10 
var message = "Aha"
```

Types are automatically inferred. 

## Order of Precedence

| Symbol           | Name                      | Associtivity |
| ---------------- | ------------------------- | ------------ |
|`()`              | Parenthesis               | Uninary      |
|`[]`              | Bracket                   | Uninary      |
|`{}`              | Curly Bracket             | Uninary      |
|`*`, `/`          |                           | Left ass     |
|`+`, `-`          |                           | Left ass     |
|`%`               | Remainder                 | Left ass     |
|`==`, `!=`, `>=`, `<=` `>` `<` |              | Left Ass     |
|`=`               | Assingment                | Left Ass     |
