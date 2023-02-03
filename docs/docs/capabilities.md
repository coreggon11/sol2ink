---
sidebar_position: 2
title: Capabilities
---

Sol2Ink can parse Solidity files into ink! project while leveraging the power of [OpenBrush](https://github.com/727-Ventures/openbrush-contracts). You can either parse a single file by providing the path to the file, or a whole folder by providing the path to teh folder. In the latter case, Sol2Ink will parse all Solidity files in the selected folder file tree and add them to one big ink! project. The output of Sol2Ink is a folder called `generated` with the following file structure:

```shell
├── contracts
│   └── contract
│       ├── contract.rs
│       └── Cargo.toml
└── src
    ├── impls
    │   ├── contract.rs
    │   └── mod.rs
    ├── libs
    │   ├── library.rs
    │   └── mod.rs
    ├── traits
    │   ├── contract.rs
    │   └── mod.rs
    │── lib.rs
    └── Cargo.toml
```

In this structure, we suppose we were parsing a directory which contains a contract called `Contract.sol` and a library called `Library.sol`. Sol2Ink will produce a lib file called `library.rs` in the folder `src/libs` and expose it in the `src/libs/mod.rs` file. Parsing the `Contract.sol` file produces a trait which serves as a public API for our contract in `src/traits/contract.rs`, expose it in `src/traits/mod.rs`, the trait implementation file in `src/impls/contract.rs` and expose it in `src/impls/mod.rs`, a contract implementation file in `contracts/contract/contract.rs`, where the genrated trait will be implemented for the contract and a `contracts/contract/Cargo.toml` file, where will be the dependencies of our contract. Additionaly, it will expose the folders `src/impls`, `src/libs` and `src/traits` in `src/lib.rs` file, and add the dependencies file `src/Cargo.toml` file.

This version of Sol2Ink is able to parse any contract, however, it may produce some compile-time issues. These will need to be fixed by the developer and future versions of Sol2Ink will aim to produce fully compilable code. The point of Sol2Ink currently is to save time on re-writing the code and makes the developer go over the generated code and fix it to make it work.