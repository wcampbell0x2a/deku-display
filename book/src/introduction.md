# Introduction

deku-display makes bitfield diagrams from the Rust types that you parse with
[deku](https://github.com/sharksforarms/deku). The diagrams come from the code, thus they
stay correct when the code changes.

It makes two formats:

- LaTeX, for the [bytefield](https://www.ctan.org/pkg/bytefield) package.
- reStructuredText, for the
  [sphinxcontrib-bitfield](https://pypi.org/project/sphinxcontrib-bitfield/) Sphinx extension.

You can make the diagrams in two ways:

1. Add `#[derive(DekuDisplay)]` to a type, and call the [derive macro](./derive.md) functions.
2. Run the [command line tool](./cli.md) on your source files. You do not change the code.

This IPv4 header comes from the [IPv4 example](./examples/ipv4.md):

![The IPv4 header](./examples/ipv4/1.svg)
