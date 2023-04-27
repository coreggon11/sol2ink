![Sol2!nk](https://user-images.githubusercontent.com/88630083/218825861-af6d659c-25e9-4324-a927-7746be434dd1.png)

# Summary

**Sol2Ink is a tool for easy migration from Solidity to Ink! and Rust**

As we are the builders in the Dotsama ecosystem and experts in ink! smart contracts, we help companies with their path to the Dotsama ecosystem.
One of our many tasks is to help projects and teams migrate their smart contracts from popular Solidity to Polkadot's ink!. During this process,
we found out that the transition process may be unnecessarily long, and if we had a tool that would transpile a Solidity file to Rust and ink!,
we would save much time. And that is how the idea of Sol2Ink was born.

Sol2Ink uses [Solang parser](https://github.com/hyperledger/solang) to parse Solidity contracts.

# How to use

You can use Sol2Ink either with cargo run or by running the binary of Sol2Ink.

- ### Using `cargo run`

1. Sol2Ink runs on the nightly toolchain of Rust, so you will need that installed
2. Clone this repository with `git clone https://github.com/Brushfam/sol2ink`
3. Copy the path to your Solidity files. It can be either a single .sol file or a folder with multiple .sol files. Navigate to the cloned repo and run `cargo +nightly run path_to_files` substituting `path_to_files` with the actual name of the file or folder.
4. The output will be saved in the folder of the original file under `generated`

- ### Using Sol2Ink binary

1. Download Sol2Ink from the release page
2. Navigate to the folder where you saved the binary
3. Run `./sol2ink path_to_files` substituting `path_to_files` with the actual name of the file or folder.
4. The output will be saved in the folder of the original file under `generated`

# Capabilities

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

# Future development

- [x] Sol2Ink CLI
- [x] User friendly errors when transpiling uncompilable contract
- [x] Parsing libraries
- [x] Implement currently incorrectly parsed statements and expressions
- [x] Ability to parse a whole Solidity project into ink! project
- [x] Parse inheritance
- [x] Produce ink! contracts with ink! 4
- [ ] Produce fully compilable contracts
- [ ] Sol2Ink Web Application with interface
- [ ] Make the parsed contracts 

# Examples

Examples are stored in the example folder, where we have the input Solidity files and the output `generated` folder with all the transpiled exmaples.
By running `cargo test`, we will transpile all of the examples stored in this folder. We have several example contracts from OpenZeppelin and two example contracts from Solang. These original contracts were not modified, and the outputs of Sol2Ink are not modified either.

# Our Community

If you have any questions regarding Sol2Ink, you can join the [Brushfam Element channel](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=web3.foundation) to find your answers and meet other ink! smart contracts developers.
