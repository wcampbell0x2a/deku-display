#[test]
fn all_bits() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        #[deku(bits = "8")]
        c: u8,
    }

    insta::assert_snapshot!(Testing::latex_output());
}

#[test]
fn use_bits_from_type() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        c: u8,
    }

    insta::assert_snapshot!(Testing::latex_output());
}

#[test]
fn readme() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
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

    insta::assert_snapshot!(Ipv6::latex_output());
}

#[test]
fn bitheader() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8", bitheader = "0, 7")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        #[deku(bits = "8")]
        c: u8,
    }

    insta::assert_snapshot!(Testing::latex_output());
}

#[test]
fn unused() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
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

    insta::assert_snapshot!(Testing::latex_output());
}

#[test]
fn other_struct() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = 4)]
    struct Version {
        #[deku(bits = "4")]
        version: u8,
    }

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = 32)]
    struct Ipv6 {
        #[deku_display(bits = "4")]
        v: Version,
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

    let mut file = String::new();
    file.push_str(&Version::latex_output());
    file.push_str(&Ipv6::latex_output());
    insta::assert_snapshot!(file);
}

#[test]
fn rst_all_bits() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        #[deku(bits = "8")]
        c: u8,
    }

    insta::assert_snapshot!(Testing::rst_bitfield_output());
}

#[test]
fn rst_readme() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
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

    insta::assert_snapshot!(Ipv6::rst_bitfield_output());
}

#[test]
fn rst_unused() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        #[deku_display(unused)]
        b: u8,
        #[deku(bits = "8")]
        c: u8,
    }

    insta::assert_snapshot!(Testing::rst_bitfield_output());
}

#[test]
fn rst_use_bits_from_type() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "8")]
    struct Testing {
        #[deku(bits = "4")]
        a: u8,
        #[deku(bits = "4")]
        b: u8,
        c: u8,
    }

    insta::assert_snapshot!(Testing::rst_bitfield_output());
}

#[test]
fn infer_bit_width_from_id_type() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku(id_type = "u8")]
    enum TestEnum {
        #[deku(id = 0)]
        Var1 { field_a: u8 },
    }

    assert_eq!(TestEnum::BITS, 8);

    insta::assert_snapshot!(TestEnum::latex_output());
}

#[test]
fn infer_bit_width_from_bytes() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku(bytes = "2")]
    struct Testing {
        a: u8,
        b: u8,
    }

    assert_eq!(Testing::BITS, 16);

    insta::assert_snapshot!(Testing::latex_output());
}

#[test]
fn field_bytes_attribute() {
    use deku_display::prelude::*;

    #[allow(dead_code)]
    #[derive(DekuDisplay)]
    #[deku_display(bit_width = "32")]
    struct Testing {
        a: u8,
        #[deku(bytes = "2")]
        b: u32,
        c: u8,
    }

    insta::assert_snapshot!(Testing::latex_output());
}
