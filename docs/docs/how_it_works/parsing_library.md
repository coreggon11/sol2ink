---
sidebar_position: 3
title: Parsing a library
---

Sol2Ink parses a Solidity library as a plain Rust file, making all functions public so they can be used in the parsed contracts. When parsing a library, Sol2Ink looks for the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments
- state variables (only constants)

Once the program reaches the end of the library, it will move on to the assemble part, where it assembles a plain Rust file (of course, using ink! and OpenBrush where possible) from the parsed objects. The output file will contain the parsed library and include all parsed members. Again, all functions return `Result` by default.

To use our library, we can simply import it in our contract, and use its functions.