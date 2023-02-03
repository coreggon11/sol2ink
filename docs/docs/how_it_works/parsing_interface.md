---
sidebar_position: 2
title: Parsing an interface
---

When parsing an interface, Sol2Ink will create an ink! trait definition from the parsed interface. This trait definition may include the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments

After Sol2Ink parses an interface, it will move on to the assemble part, where it assembles the ink! trait from the parsed structures. The output file will contain the trait definition and will be saved in `generated/src/traits/trait_name.rs`, where `trait_name` is the name of the parsed interface. This trait definition will be also exposed in `generated/src/traits/mod.rs` for the project to use. Note that all functions return `Result` by default, but we will discuss this later when describing contract parsing.

We can then later create another file where we implement this trait and then implement it for our contract.