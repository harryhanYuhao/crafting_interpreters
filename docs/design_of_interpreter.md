# The design of the interpreter

As with common practise, this interpreter first scans all texts into tokens, arragne tokens into trees, optimise the tree, and at last executes the tree.

## Parser and Scanner

### Scanner 

[Scanner](../src/scanner.rs) scans the input strings into [tokens](../src/token.rs).

The scanner has no dependency and implements a naive algorithm.

There are no public api exposed to lib. Scanner is used before parse and is invoked by the public function `parse()` in [parser](../src/parser.rs).


### Parser

The public api exposed to lib is `parse()`. The signature of parse is 

```rust
pub fn parse(tree: &mut ParseTreeUnfinshed, source_file: &str) -> ParseState {}
```

The parse function does not returns a tree, but read the source file, scans it into token, append the token to the input tree (which can be empty), and parse the whole tree.

Each node of the tree is of the type `AST_Node`. `ParseTreeUnfinshed` is an alias for `vec<AST_Node>`.

`AST_Node` is defined thus:

```rust
pub enum ExprType {
    Normal,
    Paren,
    Negated,
    Function,
}

pub enum StmtType {
    Normal,
    Braced,
    Assignment,
    Declaration,
    Compound,
    If,
    Elseif,
    While,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    FunctionDef,
}

pub(crate) enum AST_Type {
    Stmt(StmtType),
    Expr(ExprType),
    Identifier,
    Unknown,
    Unparsed(TokenType),
    Tuple,
}

pub struct AST_Node {
    AST_Type: AST_Type,
    token: Arc<Mutex<Token>>,
    children: Vec<Arc<Mutex<AST_Node>>>,
}
```

For scanning a new file, the tree just need to be initialised by 

```rust
let mut parse_tree: ParseTreeUnfinshed = ParseTreeUnfinshed::new();
```
.

The parse function returns a struct ParseState, which is self-explanatory.

```rust
pub enum ParseState {
    Finished,
    Unfinished,
    Err(ErrorLox),
}
```

If `parse` is finished, the finished tree is optained by 
```rust 
tree.get_finished_node()
```

The finshed node is a tree with all token arrange appropriately for execution. Its type is `AST_Node` defined thus:

It is no longer modified, and all information required for execution are stored in the field `token` and `AST_Type`.

## Runtime

### Function call 

There are two kinds of function in lox, std function and user defined function. Both of the function are stored as varaibles in the stack.


