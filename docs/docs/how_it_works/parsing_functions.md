---
sidebar_position: 5
title: Parsing functions
---

All parsed functions may include `Statement` enum variant from Solang. We need to convert this to Sol2Ink `Statement`, so it is more suitable for the ink! contract generation. We do this to ease some steps in the code generation, as well as to actually easily build the output code from these inputs. We will go over some remarkable points regarding the functions parsing.

### Return statement

The mission is simple - to return a value. The functions of the generated contract will always return `Result<T, Error>`, where `T` is the return type of the function (`()` if the function has no return type). We wrap the output in a result because if we want to revert a call, we need to return an error. And that is, of course, possible if we return `Result`. The error type returned in the Result is declared in the final contract, but more on that later.

### Require

Require statements are not available in Rust and ink!, so Sol2Ink will parse them as an if statement and return an error. But the require statement requires the condition to be true, so Sol2Ink will parse it as an inverted condition. Meaning `require(true)` will be parsed as 
```Rust
if !(true) {
    return Err(Error(String::new()))
}
```
If the error message was defined in the Solidity contract, Sol2Ink will use this error message in the ink! contract as well, but if it was not provided, Sol2Ink will provide its own error message. Future versions of Sol2Ink will produce a nicer inverted conditions (so `require(true)` will produce `if false` instead of `if !(true)`). We work on a better experience and better code generation.

### Emit event

Here we need to note that events in ink! are structs with the `ink[(event)]` attribute. So emitting an event is just calling of `emit_event` function, providing a new struct with the desired parameters.

### Ternary operator

The ternary operator does not exist in Rust, so they are parsed as an if/else block.

### Binary operation

Binary operations ++ and -- are not available in Rust, so we parse them as addition or subtraction of 1. Depending on if the operation were a prefix or suffix operation, we would do the incrementation/subtraction before or after reading the value.

### Loops

- For loops are parsed to while loops, with the incrementation happening at the end of the loop. 
- Do/while loops are parsed as a loop with a condition check at the end of the block. 
- While loops are parsed as while loops 

### Unchecked blocks

Unchecked blocks are parsed as normal code blocks.

### Try/catch blocks

We will call the call from try, and check if the result is an error. If yes, we will return an error.

### Assembly blocks

Sol2Ink puts a comment about missing assembly block. We plan on implementing parsing of assembly block in the future.

All other statements are parsed as expected:
- declarations
- comments
- conditional blocks and one-line conditions
- assignments
- function calls
