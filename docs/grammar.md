# Grammar of the Lox Language

Lox is a mock language and designed to be simple and easy to implement. 

This is a comphrehensive grammar for lox. For tutorial, check [tutorial.md](tutorial.md)

## Valid Tokens

### Operators 

| Operators        | Description        | Associativity |
| -----------------|--------------------| --------------|
| `+`              | Addition           | Left          |
| `-`              | Subtraction        | Left          |
| `*`              | Multiplication     | Left          |
| `/`              | Division           | Left          |
| `%`              | Modulus            | Left          |
| `==`             | Equality           | Left          |
| `!=`             | Inequality         | Left          |
| `>`              | Greater than       | Left          |
| `>=`             | Greater or equal   | Left          |
| `<`              | Less than          | Left          |
| `<=`             | Less or equal      | Left          |
| `&&`             | Logical AND        | Left          |
| `||`             | Logical OR         | Left          |
| `!`              | Logical NOT        | Right         |
| `=`              | Assignment         | Right         |
|`( )`             | Parenthesis        | NA            |

Left associativity means operations starts from left to the right. 
`1 + 2 + 3` is evaluated as `(1 + 2) + 3`; however. `a = b = c` is evaluated as let `b` equal to `c`, and then let `a` equal to `b`.

### Keywords

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

Types are automatically inferred during declaration.

Variables must be declared before use.


## Order of Precedence

| Symbol           | Name                      | Associtivity |
| ---------------- | ------------------------- | ------------ |
|`()`              | Parenthesis               | Uninary      |
|`[]`              | Bracket                   | Uninary      |
|`{}`              | Curly Bracket             | Uninary      |
|`*`, `/`, `%`     |                           | Left ass     |
|`+`, `-`          |                           | Left ass     |
|`==`, `!=`, `>=`, `<=` `>` `<` |              | Left Ass     |
|`=`               | Assingment                | Left Ass     |

## Parsing Grammar

There are several parsing objects in our design

- statement (stmt) 
- expression (expr)
- identifier (idt)
- various tokens 

Expression is something that can be evaluated into a value that can be assigned to an identifier. 


Identifier are names of variables or functions. 

Statement can be executed and will return an expression. 
Significantly, if some tokens are arranged as a statement tree, this particular tree is complete and shall no longer be modified, although the tree itself can be attached to other trees.

As 

This list also shows the order or precedence

- () -> paren
- (expr) -> expr 
- expr, expr -> expr
- *, %, / 
    - expr | identifier * expr | identifier -> expr 
    - same
- +, -, 
    - expr | identifier + expr | identifier -> expr 
    - same
- ,
    - expr, expr -> expr 
- identifier(expr) -> expr, this is function call
- &&, ||,
    - expr | identifier && expr | identifier -> expr 
    - same
- ==, !=, >, <, >=, <=
    - left ass (meaning expr sign expr -> expr for sign being ==, !=, >, < >=, <=)
- =,
    identifer = expr -> stmt(assignment)
- var stmt(assignment) -> stmt(declaration)
- stmt; stmt -> stmt(compound)
- stmt \n stmt -> stmt(compond)
