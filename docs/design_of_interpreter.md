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
pub fn parse(tree: &mut ParseTreeUnfinshed, source_file: &str) -> ParseState 
```

The parse function does not returns a tree, but read the source file, scans it into token, append the token to tree, and parse the whole tree.

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

The finshed node is a tree with all token arrange appropriately for execution. It contains all the information for the runtime to run the program, and is no longer modified.

## Runtime
