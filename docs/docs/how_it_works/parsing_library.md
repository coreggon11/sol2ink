---
sidebar_position: 3
title: Parsing a library
---

When parsing a libraray, Sol2Ink will create a plain Rust file, making all functions public so they can be used in the parsed contract. This file definition may include the following:

- struct definitions
- enum definitions
- function definitions
- documentation comments
- state variables (only constants)

After Sol2Ink parses a library, it will move on to the assemble part, where it assembles a Rust file for our library (of course using ink! and OpenBrush where possible) from the parsed structures. The output file will contain the parsed library and include all parsed constant members, and will be saved in `generated/src/libs/lib_name.rs`, where `lib_name` is the name of the parsed library. This library will be also exposed in `generated/src/libs/mod.rs` for the project to use. Note that all functions return `Result` by default.

To use our library, we can simply import it in our contract, and use its functions.