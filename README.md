# Sol2Mermaid

Sol2Mermaid is an easy tool that will transform your Solidity code into mermaid diagrams. Just use a mermaid diagram interpreter to see it!

## Roots

This repo is forked from [Sol2Ink](https://github.com/Brushfam/sol2ink/tree/main), which was a tool I was building previously, and it reuses a lot of parsing logic, however it is a completely different use case and project.

The mermaid diagrams created by this tool are inspired by [gpersoon](https://github.com/gpersoon/diagrams)'s diagram template.

## Capabilities and limitations

S2M needs a path to file or directory which contains the solidity files you want to create diagrams of. The multifile support is currently not an optimal experience, so a single file usage is recommended. Interfaces and Libraries are currently not fully supported.

## TODO

- [ ] Full Interface and Library support
- [ ] Better assembly handling
- [ ] Better multifile support
