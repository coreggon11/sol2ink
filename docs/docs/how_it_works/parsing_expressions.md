---
sidebar_position: 6
title: Parsing expressions
---

Another step of parsing a statement is parsing each expression. Here the program will decide how to parse each expression inside a statement.

### Basics

- Literals are parsed without any modifications
- Specific expressions like `address(0)`, `msg.sender` or `msg.value` are parsed in their ink! form
- Solidity types are converted to Rust/ink! types

### Hex values

Expressions like `hex"0000_0000_0000_0000"` are converted to a call of `&hex::decode` function.

### type(T).f / type(T)

These expressions are parsed as expected, except `type` is changed to `type_of` since `type` is a keyword in rust. This can produce uncompilable code, since `type(uint256).max` will be parsed as `type_of(u128).max` instead of `u128::MAX`, and the developer needs to change this call. We plan on better support for such functions in the future version of Sol2Ink.

All other expressions are parsed as expected:

- struct initializations
- function calls
- arithmetic operations
- logical operations
- parentheses

After Sol2Ink parses everything, it will assemble the final ink! contract.