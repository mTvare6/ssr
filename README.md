# ssr

Student Search TUI for IITK written in Rust.

# Building

```sh
$ git clone https://github.com/mTvare6/ssr.git
$ cd ssr
$ cargo build --release
```

# Installing

```sh
$ cargo install --path .
```

# Usage

```sh
$ ssr
```

Use `C-h` `C-l` to navigate between input fields.
Input fields support emacs shortcuts and regexes.
Sixel, Kitty and iTerm protocol is supported. Alacritty and others which aren't compliant instead use a chafa-like renderer

