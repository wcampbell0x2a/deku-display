# Derive Macro

Add `#[derive(DekuDisplay)]` to a struct. Each field gets its size from `#[deku(bits = ..)]`,
from `#[deku_display(bits = ..)]`, or from its type. `bit_width` sets the number of bits in
each row of the diagram.

```rust,noplayground
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

The `DekuDisplayOutput` trait gives the diagram in each format.

## LaTeX

`Ipv6::latex_output()` gives a figure for the bytefield package:

```latex
\begin{figure}
\begin{bytefield}{32}
\bitheader{0-31} \\
\bitbox{4}{version} & \bitbox{6}{ds} & \bitbox{2}{ecn} & \bitbox{20}{label} \\
\bitbox{16}{length} & \bitbox{8}{next\_header} & \bitbox{8}{hop\_limit} \\
\bitbox{32}{src} \\
\bitbox{32}{dst} \\
\end{bytefield}
\caption{Ipv6}
\end{figure}
```

Add `\usepackage{bytefield}` and `\usepackage{xcolor}` to your document. `pdflatex` makes
this figure:

![The Ipv6 figure from pdflatex](./images/ipv6.png)

## reStructuredText

`Ipv6::rst_bitfield_output()` gives a directive for sphinxcontrib-bitfield:

```rst
.. bitfield::
    :bits: 128
    :lanes: 4
    :hflip:
    :vflip:

    [
        { "name": "version", "bits": 4 },
        { "name": "ds", "bits": 6 },
        { "name": "ecn", "bits": 2 },
        { "name": "label", "bits": 20 },
        { "name": "length", "bits": 16 },
        { "name": "next_header", "bits": 8 },
        { "name": "hop_limit", "bits": 8 },
        { "name": "src", "bits": 32 },
        { "name": "dst", "bits": 32 }
    ]
```

See [Sphinx](./sphinx.md) to make HTML from this directive.
