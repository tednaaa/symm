### Installation

```fish
cargo install symm
```

### Usage

- The configuration file is located at `~/dotfiles/dotfiles.toml`. Here's an example:

```toml
[symlinks]
"zed" = ".config/zed"
"ghostty" = ".config/ghostty"
"nvim" = ".config/nvim"
```

- There are only 2 commands: link and unlink

```fish
symm link
symm unlink
```