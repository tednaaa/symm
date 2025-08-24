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

- Commands for managing symlinks

```fish
symm link
symm unlink
```

> Commands for managing packages `currently only archlinux repos supported`

```fish
symm packages diff
symm packages install
```
