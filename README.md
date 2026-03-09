# bintex
[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/bintex-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/bintex)
[<img alt="crates.io" src="https://img.shields.io/crates/v/bintex.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/bintex)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-bintex-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/bintex)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/bintex/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/bintex/actions?query=branch%3Amaster)

Create LaTeX [bytefield](https://www.ctan.org/pkg/bytefield) and reStructuredText [sphinxcontrib-bitfield](https://pypi.org/project/sphinxcontrib-bitfield/) diagrams with the
use of rust proc-macros and the [deku](https://github.com/sharksforarms/deku) library.

# docs
run `$ cargo doc --open`

See `bintex::attribute` for Attribute details and examples.

# example

Run `$ cargo run && pdflatex sample.tex` to create the following illustration from code:

```rust
use bintex::{BinTex, BinTexOutput};
use deku::prelude::*;

#[derive(BinTex)]
#[bintex(bit_width = 32)]
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
