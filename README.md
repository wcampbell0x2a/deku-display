# deku-display
[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/deku--display-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/deku-display)
[<img alt="crates.io" src="https://img.shields.io/crates/v/deku-display.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/deku-display)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-deku--display-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/deku-display)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/deku-display/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/deku-display/actions?query=branch%3Amaster)

Create LaTeX [bytefield](https://www.ctan.org/pkg/bytefield) and reStructuredText [sphinxcontrib-bitfield](https://pypi.org/project/sphinxcontrib-bitfield/) diagrams with the
use of rust proc-macros and the [deku](https://github.com/sharksforarms/deku) library.

# docs
run `$ cargo doc --open`

See `deku_display::attribute` for Attribute details and examples.

# example

## reStructuredText
Run `$ cargo run` to generate `.rst` output compatible with [sphinxcontrib-bitfield](https://pypi.org/project/sphinxcontrib-bitfield/) for use in Sphinx documentation.

## LaTeX
Run `$ cargo run && pdflatex sample.tex` to create the following illustration from code:

```rust
use deku_display::{DekuDisplay, DekuDisplayOutput};
use deku::prelude::*;

#[derive(DekuDisplay)]
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
![Result](/media/ipv6.png)
