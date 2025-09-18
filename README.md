## Requirments

`paru` - to install packages

## Installation

### 📦 crates.io

You can install `symm` from [crates.io](https://crates.io/crates/symm)

```fish
cargo install symm
```

### 🐧 Arch Linux (AUR)

You can install `symm` from the [aur repository](https://aur.archlinux.org/packages/symm)

```fish
paru -S symm
```

## Usage

- You can also refer to [My Dotfiles](https://github.com/tednaaa/dotfiles/blob/main/dotfiles.toml)

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

<img width="840" height="468" alt="image" src="https://github.com/user-attachments/assets/3cc66921-f035-44ed-8cdf-717a35127134" />

- Commands for managing packages
  > currently only archlinux repos supported

```fish
symm packages diff
symm packages install
```

<img width="520" height="262" alt="image" src="https://github.com/user-attachments/assets/612d5220-4d0a-47cd-813d-4685fee6db26" />
