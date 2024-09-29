# An interpreter from scrath using rust

This is a didactic just-in-time interpreter written in rust, inspired by the book crafting interpreters.

The mock programming language being implemented is lox, a dynamically typed language with grammar similar to python with a few exceptions: 

1. All variables needs to be declared before use (just for fun)
1. function definition use key word `fn` (inspired by rust)
1. a statement returns the last expression in the block (inspired by rust)


Here is an example:

```lox 
fn is_prime(a) {
	var res = true
	var i = 2 
	while (i < a) {
		if (a % i == 0) {
			res = false
		}		
		i += 1
	}
	res
}

var i = 1 
while i < 1000 {
	if (is_prime(i)) {
		print(i, " is prime!") 
	} 
	i += 1
}
```

I created this lox language (The name came from crafting interpreters), and its full specification can be found [here](./docs/lox/grammar.md)

## Organization

## Testing

There are two parts of testing: buildin rust testing in `./src/test/` directory, and the intergrating testing in `test.py`. Integrating testing is just a python file running every scripts in `./test/` directory.
