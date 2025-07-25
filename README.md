## Installation

### 📦 crates.io

You can install `symm` from [crates.io](https://crates.io/crates/symm)

```fish
cargo install symm
```

### 🐧 Arch Linux (AUR)

You can install `symm` from the [aur repository](https://aur.archlinux.org/packages/symm)

```fish
yay -S symm
```

## Usage

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
