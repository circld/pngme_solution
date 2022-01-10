# Working through the PNGme project

This project contains code written while working through [PNGme: An Intermediate Rust Project](https://picklenerd.github.io/pngme_book/introduction.html).

## Rust + nix + OSX

Took a bit of searching before I could get everything compiling correctly. Ultimately, we need to [ensure the Apple frameworks are installed via nix](https://stackoverflow.com/questions/51161225/how-can-i-make-macos-frameworks-available-to-clang-in-a-nix-environment).

## TODO

- [x] chapter 1: adopt `?` idiom where possible
- [x] chapter 1: define custom error(s)
