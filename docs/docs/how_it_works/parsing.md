---
sidebar_position: 1
title: Parsing
---

This section will look at how Sol2Ink works under the hood.

### Parsing

After running it, the program will first parse the original file. The parser we use is [Solang parser](https://github.com/hyperledger/solang/tree/main/solang-parser), so we don't have to reinvent the wheel. Sol2Ink will then take the output of Solang parser (parsed token tree and comments) and convert the Solang structures to Sol2Ink structures, which will be then tossed to assembler to assemble the output ink! contract with a comprehensive file structure. We leverage the power of Rust at its fullest!

### Note the following
- if there is some code outside of `Contract`, `Interface`, or `Library` definition, Sol2Ink will not parse it.