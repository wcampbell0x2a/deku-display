# Command Line Tool

`deku-display-cli` reads Rust source files, and makes diagrams for each struct and enum with
`#[deku_display]`, `#[derive(DekuRead)]`, or `#[derive(DekuWrite)]`. You do not add a
dependency or change the code.

```console
$ deku-display-cli --help
Generate LaTeX/RST bitfield diagrams from Rust source files

Usage: deku-display-cli [OPTIONS] [INPUT]...

Arguments:
  [INPUT]...  Input .rs file(s) to process

Options:
      --format <FORMAT>  Output format [default: rst] [possible values: latex, rst]
      --output <OUTPUT>  Output file (default: stdout)
  -h, --help             Print help
```

## reStructuredText

The `rst` format makes one page for all the types. The page has:

- A section and a diagram for each struct.
- For each enum, a table of the variant IDs, and a section and a diagram for each variant.
- Links from a field to the section of its type.

```console
$ deku-display-cli src/*.rs --format rst --output protocol.rst
```

The [examples](./examples/ipv4.md) in this book come from this output.

## LaTeX

The `latex` format makes a bytefield figure for each struct.

```console
$ deku-display-cli src/*.rs --format latex --output protocol.tex
```
