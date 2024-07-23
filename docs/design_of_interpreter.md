# The design of the interpreter

As with common practise, this interpreter first scans all texts into tokens, arragne tokens into trees, optimise the tree, and at last executes the tree.

## Scanner

[Scanner](../src/scanner.rs) scans the input strings into [tokens](../src/token.rs).
