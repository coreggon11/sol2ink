![Sol2!nk](https://user-images.githubusercontent.com/43150707/215464954-13e4c8d8-96b4-49da-996c-3e79b8344b3a.png)

## Summary

**Sol2Ink is a tool for easy migration from Solidity to Ink! and Rust**

As we are the builders in the Dotsama ecosystem and experts in ink! smart contracts, we help companies with their path to the Dotsama ecosystem.
One of our many tasks is to help projects and teams migrate their smart contracts from popular Solidity to Polkadot's ink!. During this process,
we found out that the transition process may be unnecessarily long, and if we had a tool that would transpile a Solidity file to Rust and ink!,
we would save much time. And that is how the idea of Sol2Ink was born.

Sol2Ink uses [Solang parser](https://github.com/hyperledger/solang) to parse Solidity contracts.

### Capabilities

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

### Future development

- [x] Sol2Ink CLI
- [x] User friendly errors when transpiling uncompilable contract
- [x] Parsing libraries
- [x] Implement currently incorrectly parsed statements and expressions
- [x] Ability to parse a whole Solidity project into ink! project
- [x] Parse inheritance
- [ ] Produce ink! contracts with ink! 4
- [ ] Produce fully compilable contracts
- [ ] Sol2Ink Web Application with interface
- [ ] Make the parsed contracts 

### How to use it?

To run the application you will need to have installed Rust and run the nightly toolchain. ​
You can run the application with `cargo +nightly run input`, where input is either a solidity file, or a folder containing solidity files.
The result will be stored in `generated` folder, as described above.

You can transpile the example contracts from examples folder by running `cargo +nightly test`.

If you are using Sol2Ink from release pages, you will need to run `./sol2ink input`, substituting input with your Solidity contract's name or with the name of the folder containing Solidity files you want to transpile.

### Examples

Examples are stored in the example folder, where we have the input Solidity files and the output `generated` folder with all the transpiled exmaples.
By running `cargo test`, we will transpile all of the examples stored in this folder. We have several example contracts from OpenZeppelin and two example contracts from Solang. These original contracts were not modified, and the outputs of Sol2Ink are not modified either.

### Our Community

If you have any questions regarding Sol2Ink, you can join the [Brushfam Element channel](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=web3.foundation) to find your answers and meet other ink! smart contracts developers.
