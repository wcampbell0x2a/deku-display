# deku-display
[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/deku--display-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/deku-display)
[<img alt="crates.io" src="https://img.shields.io/crates/v/deku-display.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/deku-display)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-deku--display-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/deku-display)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/deku-display/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/deku-display/actions?query=branch%3Amaster)

Create LaTeX [bytefield](https://www.ctan.org/pkg/bytefield) and reStructuredText [sphinxcontrib-bitfield](https://pypi.org/project/sphinxcontrib-bitfield/) diagrams from the Rust types of the [deku](https://github.com/sharksforarms/deku) library.

* **Derive**: Add `#[derive(DekuDisplay)]` to a type, and get its LaTeX or reStructuredText diagram.
* **CLI**: Run `deku-display-cli` on your source files. You do not change the code.
* See the [book](https://wcampbell0x2a.github.io/deku-display/) for usage, every attribute, and example diagrams.

## Installation
```console
$ cargo add deku-display
$ cargo install --locked --git https://github.com/wcampbell0x2a/deku-display deku-display-cli
```

## Example
```rust
use deku::prelude::*;
use deku_display::prelude::*;

#[derive(DekuDisplay, DekuRead, DekuWrite)]
#[deku_display(bit_width = 32)]
struct Ipv6 {
    #[deku(bits = "4")]
    version: u8,
    #[deku(bits = "6")]
    ds: u8,
    #[deku(bits = "2")]
    ecn: u8,
    #[deku(bits = "20")]
    label: u32,
    length: u16,
    next_header: u8,
    hop_limit: u8,
    src: u32,
    dst: u32,
}
```

`Ipv6::latex_output()` gives a bytefield figure, and `pdflatex` makes this:

![The Ipv6 figure from pdflatex](book/src/images/ipv6.png)

Or, without a change to the code:
```console
$ deku-display-cli src/*.rs --format rst --output protocol.rst
```

## Development
All patches/merge requests are welcome!

```
just test   # run all the tests
just book   # build the book
```
