/*!

Create LaTeX [bytefield](https://www.ctan.org/pkg/bytefield) diagrams with the
use of rust proc-macros and the [deku](https://github.com/sharksforarms/deku) library

For documentation and examples on available `#[deku_display]` attributes and features,
see [attributes list](attributes)

# Example

```rust
    # use deku_display::prelude::*;
    # use deku::prelude::*;
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

*/

pub mod attributes;
pub mod prelude;

pub use deku_display_derive::*;

/// Generate latex output
pub trait DekuDisplayOutput {
    /// For generated types, const of the `bit_width` attribute
    const BITS: usize;
    fn latex_output() -> String;
    fn rst_bitfield_output() -> String;
}
