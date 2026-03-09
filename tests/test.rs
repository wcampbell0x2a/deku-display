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

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0-7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
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

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0-7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
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

    let expected = r#"\begin{figure}
\begin{bytefield}{32}
\bitheader{0-31} \\
\bitbox{4}{version} & \bitbox{6}{ds} & \bitbox{2}{ecn} & \bitbox{20}{label} \\
\bitbox{16}{length} & \bitbox{8}{next\_header} & \bitbox{8}{hop\_limit} \\
\bitbox{32}{src} \\
\bitbox{32}{dst} \\
\end{bytefield}
\caption{Ipv6}
\end{figure}"#;
    assert_eq!(Ipv6::latex_output(), expected);
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

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0, 7} \\
\bitbox{4}{a} & \bitbox{4}{b} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
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

    let expected = r#"\begin{figure}
\begin{bytefield}{8}
\bitheader{0, 7} \\
\bitbox{4}{a} & \bitbox{4}[bgcolor=lightgray]{} \\
\bitbox{8}{c} \\
\end{bytefield}
\caption{Testing}
\end{figure}"#;
    assert_eq!(Testing::latex_output(), expected);
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

    let expected = r#"\begin{figure}
\begin{bytefield}{4}
\bitheader{0-3} \\
\bitbox{4}{version} \\
\end{bytefield}
\caption{Version}
\end{figure}\begin{figure}
\begin{bytefield}{32}
\bitheader{0-31} \\
\bitbox{4}{v} & \bitbox{6}{ds} & \bitbox{2}{ecn} & \bitbox{20}{label} \\
\bitbox{16}{length} & \bitbox{8}{next\_header} & \bitbox{8}{hop\_limit} \\
\bitbox{32}{src} \\
\bitbox{32}{dst} \\
\end{bytefield}
\caption{Ipv6}
\end{figure}"#;

    let mut file = String::new();
    file.push_str(&Version::latex_output());
    file.push_str(&Ipv6::latex_output());
    assert_eq!(expected, expected);
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

    let expected = r#".. bitfield::
    :bits: 16
    :lanes: 2
    :hflip:
    :vflip:

    [
        { "name": "a", "bits": 4 },
        { "name": "b", "bits": 4 },
        { "name": "c", "bits": 8 }
    ]"#;
    assert_eq!(Testing::rst_bitfield_output(), expected);
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

    let expected = r#".. bitfield::
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
    ]"#;
    assert_eq!(Ipv6::rst_bitfield_output(), expected);
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

    let expected = r#".. bitfield::
    :bits: 16
    :lanes: 2
    :hflip:
    :vflip:

    [
        { "name": "a", "bits": 4 },
        { "bits": 4 },
        { "name": "c", "bits": 8 }
    ]"#;
    assert_eq!(Testing::rst_bitfield_output(), expected);
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

    let expected = r#".. bitfield::
    :bits: 16
    :lanes: 2
    :hflip:
    :vflip:

    [
        { "name": "a", "bits": 4 },
        { "name": "b", "bits": 4 },
        { "name": "c", "bits": 8 }
    ]"#;
    assert_eq!(Testing::rst_bitfield_output(), expected);
}
