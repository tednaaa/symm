### Simple plugin to manage symlinks for dotfiles

> Installation

```fish
cargo install symm

```
> Usage

- Here is example file

```toml
[symlinks]
"zed" = ".config/zed"
"ghostty" = ".config/ghostty"
"nvim" = ".config/nvim"
```

- There is only 2 commands, link and unlink

```fish
symm link
symm unlink
```
