---
sidebar_position: 3
title: How to use
---

You can use Sol2Ink either with cargo run or by running the binary of Sol2Ink.

- ### Using `cargo run`

1. Sol2Ink runs on the nightly toolchain of Rust, so you will need that installed
2. Clone the Sol2Ink repository with `git clone https://github.com/Brushfam/sol2ink`
3. Copy the path to your Solidity files. It can be either a single .sol file or a folder with multiple .sol files. Navigate to the cloned repo and run `cargo +nightly run path_to_files` substituting `path_to_files` with the actual name of the file or folder.
4. The output will be saved in the folder of the original file under `generated`

- ### Using Sol2Ink binary

1. Download Sol2Ink from the release page
2. Navigate to the folder where you saved the binary
3. Run `./sol2ink path_to_files` substituting `path_to_files` with the actual name of the file or folder.
4. The output will be saved in the folder of the original file under `generated`