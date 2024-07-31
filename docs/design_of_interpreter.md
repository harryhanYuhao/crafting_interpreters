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
pub fn parse(
    string_input: &str,   // string
    line_number: &mut usize, 
    tree: &mut ParseTreeUnfinshed,
    source: &str,
) -> ParseState 
```
