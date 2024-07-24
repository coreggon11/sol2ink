# Sol2Mermaid

Sol2Mermaid is an easy tool that transforms your Solidity code into mermaid diagrams. To see them, just use a mermaid diagram interpreter!

## Roots

This repo is forked from [Sol2Ink](https://github.com/Brushfam/sol2ink/tree/main), a tool I was building previously. It reuses a lot of parsing logic, but it is a completely different use case and project.

The mermaid diagrams created by this tool are inspired by [gpersoon](https://github.com/gpersoon/diagrams)'s diagram template.

## Usage

S2M needs a path to a file or directory that contains the solidity files you want to create diagrams of.

```bash
cargo run path_to_file.sol
```

or

```bash
cargo run path_to_dir
```

You can also explictly name the contracts you want to chart by saying contracts ... . In this case, everything is analyzed but only the contracts you chose will be charted:

```bash
cargo run path_to_dir contracts ContractA ContractB
```

You can also omit contracts by saying omit ... . Again, everything will be analyzed but the omitted contracts won't be charted:

```bash
cargo run path_to_dir omit ContractA ContractB
```

You can omit drawing of storage reads by specifying `omit_read_storage`

```bash
cargo run path_to_dir omit_read_storage
```

Floating storage (storage accessed by library functions, slots, etc.) is not grouped. You can group it by the specific structs containing these variables with `group_floating_storage` option

```bash
cargo run path_to_dir group_floating_storage
```

You can use any of the options combined (although using `omit` and `contracts` together will not have a meaningful effect, you will simply omit everything except the ones you explictly mentioned to chart:P )

## Limitations

- Multifile support and inheritance pattern support like Diamond is the Holy Grail of S2M. It works kinda nice, but stuff can still be optimized
- Libraries are currently handled in a way that only cares about storage access, meaning if a function calls a library function, it won't be charted, unless the library function takes a storage paramater which the function later reads from/writes to. This would be charted as access to the storage param rather than a library function access.
- Interfaces and Libraries are currently not fully supported.
- Inheritance not fully supported

## TODO

- [ ] Full Interface and Library support
- [ ] Better inheritance support
