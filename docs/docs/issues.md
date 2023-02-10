---
sidebar_position: 5
title: Known issues
---

Here is a list of known issues which you may face using Sol2Ink:

### Added or fixed in v 2.0.0

- ~~inability to parse libraries~~
- ~~inability to parse uncompilable contracts~~ (since 2.0.0 most cases can be parsed, otherwise Sol2Ink will throw a comprehesive error)
- ~~calling functions with a value~~
- ~~occasional incorrect parsing of selectors within brackets~~
- ~~incorrect rewriting of fields inside structs extracted from a mapping~~
- ~~incorrectly allowing modifiers to take functions as parameters~~
- ~~inability to parse inheritation~~
- ~~inability to parse multi-file projects~~
- ~~binary operation in a function only performs the reading of the value, not the updating~~

### To be fixed
- output contracts can be incompilable and need some fixing from the developer (our long-term goal is to make all contracts compilable)
- Solidity abi functions (encode, decode) 
- Overloading functions is supported in Solidity but not in Rust
- Functions as parameters

Sol2Ink still needs to walk some path. Every time you use Sol2Ink to transpile your contracts from Solidity to ink!, run the generated code by a human brain to get the best results! If you find any issue, let us know in our [Element chat](https://matrix.to/#/!utTuYglskDvqRRMQta:matrix.org?via=matrix.org&via=t2bot.io&via=web3.foundation), [Discord](https://discord.gg/6TXE7n7Ptc) or simply open an issue in our [GitHub repo](https://github.com/727-Ventures/sol2ink)
