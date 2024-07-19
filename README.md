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

## Limitations

- The multifile support is currently not an optimal experience, so a single file usage is recommended.
- Interfaces and Libraries are currently not fully supported.
- Inheritance not fully supported
- Assembly is not parsed
- Support for patterns like Diamond is limited since it works with storage in a different way

## TODO

- [ ] Full Interface and Library support
- [ ] Better inheritance support
- [ ] Better assembly handling
- [ ] Better Diamond-like pattern support
- [ ] Better multifile support
