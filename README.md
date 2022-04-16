# Simple Rust Window Swallower

srws is a window swallower for X written in Rust, inspired by the dwm patch [swallow](https://dwm.suckless.org/patches/swallow/).

## Installation

### Dependencies

 - cargo

```bash
git clone https://github.com/EdenQwQ/srws
cd srws
cargo install --path .

```

## Usage

Add srws before a command to swallow the focused window.

```bash
srws command
```
### Alias

Add some aliases for your shell would be helpful.

Example:
```bash
alias sxiv='srws sxiv'
alias mpv='srws mpv'
```
