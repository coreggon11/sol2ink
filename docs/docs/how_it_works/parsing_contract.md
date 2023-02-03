---
sidebar_position: 4
title: Parsing a contract
---

When parsing a contract, Sol2Ink will create an ink! trait definition, implementation of this trait, and a contract file from the parsed contract. This contract may include the following:

- event definitions
- struct definitions
- enum definitions
- function definitions
- documentation comments
- state variables
- constructor
- modifiers

### Parsing a function or a modifier

While parsing a contract, Sol2Ink will also parse all of the mentioned above. We will describe how parsing functions work later. For now we just need to know, that all of these functions will be added to an ink! trait definition of the contract saved in `src/traits/contract_name.rs` and exposed in `src/traits/mod.rs`, then will this trait be implemented in `src/impls/contract_name.rs` and the implementation file will be exposed in `src/impls/mod.rs` and finally it will generate a contract in `contracts/contract_name/lib.rs` and the dependencies file in `contracts/contract_name/Cargo.toml`.
