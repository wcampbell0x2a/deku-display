/*!
A documentation-only module for #\[deku_display\] attributes

All attributes are able to be used from only deku_display, but can be also used with #\[deku\].

For example, deku_display only.
```rust
use deku_display::prelude::*;

#[derive(DekuDisplay)]
#[deku_display(bit_width = "8", bitheader = "0, 7")]
struct Testing {
    #[deku_display(bits = "4")]
    a: u8,
    #[deku_display(bits = "4")]
    b: u8,
    #[deku_display(bits = "8")]
    c: u8,
}
```

For example, both deku and deku_display:
```rust
use deku::prelude::*;
use deku_display::prelude::*;

#[derive(DekuDisplay, DekuRead, DekuWrite)]
#[deku_display(bit_width = "8", bitheader = "0, 7")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    #[deku_display(unused)]
    b: u8,
    #[deku(bits = "8")]
    c: u8,
}
```

# List of attributes

|     Attribute           | Scope     |   Library        | Description
|-------------------------|-----------|------------------|--------------------------------------------------
| [bits](#bits)           | field     | deku/deku_display | Set the bit-size of the field
| [unused](#unused)       | field     | deku_display      | Set background color to gray, supress field ident
| [bit_width](#bit_width) | top-level | deku_display      | Set width of bits in LaTeX bytefield
| [bitheader](#bitheader) | top-level | deku_display      | Set bitheader for LaTeX bytefield

# bits

See [deku::bits](https://docs.rs/deku/0.12.1/deku/attributes/index.html#bits)

# unused

Set background color to gray and remove the field name from the LaTeX output.

Example:
```rust
use deku::prelude::*;
use deku_display::prelude::*;

#[derive(DekuDisplay, DekuRead, DekuWrite)]
#[deku_display(bit_width = "32", bitheader = "0, 4, 8, 12, 16, 20, 24, 28")]
struct UnusedStruct {
    Tag: u8,
    Value: u8,
    #[deku_display(unused)]
    #[deku(bits = "4")]
    c: u8,
    #[deku(bits = "12")]
    Mask: u16,
    Key: u32,
}
```

[example output](https://github.com/wcampbell0x2a/deku-display/blob/master/media/unused.png)

# bit_width

Set the bit_width of the total bytefield LaTeX figure.

# bitheader

Set a custom bitheader. Defaults to "0-(bit_width - 1)".
*/
