use deku_display_cli::{generate_latex, generate_rst, parse};

fn generate_all(source: &str, format: &str) -> String {
    let types = parse::parse_file(source).unwrap();
    let mut result = String::new();
    for (i, info) in types.iter().enumerate() {
        if i > 0 {
            result.push('\n');
        }
        let generated = match format {
            "latex" => generate_latex::generate(info).unwrap(),
            "rst" => generate_rst::generate(info).unwrap(),
            _ => unreachable!(),
        };
        result.push_str(&generated);
    }
    result
}

fn generate_rst_page(source: &str) -> String {
    let types = parse::parse_file(source).unwrap();
    generate_rst::generate_page(&types).unwrap()
}

#[test]
fn latex_all_bits() {
    let source = r#"
#[deku_display(bit_width = "8")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    b: u8,
    #[deku(bits = "8")]
    c: u8,
}
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn latex_use_bits_from_type() {
    let source = r#"
#[deku_display(bit_width = "8")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    b: u8,
    c: u8,
}
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn latex_readme_ipv6() {
    let source = r#"
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
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn latex_bitheader() {
    let source = r#"
#[deku_display(bit_width = "8", bitheader = "0, 7")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    b: u8,
    #[deku(bits = "8")]
    c: u8,
}
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn latex_unused() {
    let source = r#"
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
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn latex_multiple_structs() {
    let source = r#"
#[deku_display(bit_width = 4)]
struct Version {
    #[deku(bits = "4")]
    version: u8,
}

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
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn rst_all_bits() {
    let source = r#"
#[deku_display(bit_width = "8")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    b: u8,
    #[deku(bits = "8")]
    c: u8,
}
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn rst_readme_ipv6() {
    let source = r#"
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
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn rst_unused() {
    let source = r#"
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
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn rst_use_bits_from_type() {
    let source = r#"
#[deku_display(bit_width = "8")]
struct Testing {
    #[deku(bits = "4")]
    a: u8,
    #[deku(bits = "4")]
    b: u8,
    c: u8,
}
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn parse_skips_unannotated_structs() {
    let source = r#"
struct Foo {
    x: u8,
}

#[deku_display(bit_width = "8")]
struct Bar {
    #[deku(bits = "8")]
    y: u8,
}
"#;
    let types = parse::parse_file(source).unwrap();
    assert_eq!(types.len(), 1);
    assert_eq!(types[0].name(), "Bar");
}

#[test]
fn parse_unknown_type_without_bits_errors() {
    let source = r#"
#[deku_display(bit_width = 32)]
struct Bad {
    x: MyCustomType,
}
"#;
    let types = parse::parse_file(source).unwrap();
    let err = generate_latex::generate(&types[0]).unwrap_err();
    assert!(err.to_string().contains("MyCustomType"));
}

#[test]
fn rst_enum_basic() {
    let source = r#"
#[deku_display(bit_width = 8)]
enum Packet {
    Header {
        #[deku(bits = "4")]
        version: u8,
        #[deku(bits = "4")]
        flags: u8,
    },
    Data {
        payload: u8,
    },
}
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn latex_enum_basic() {
    let source = r#"
#[deku_display(bit_width = 8)]
enum Packet {
    Header {
        #[deku(bits = "4")]
        version: u8,
        #[deku(bits = "4")]
        flags: u8,
    },
    Data {
        payload: u8,
    },
}
"#;
    insta::assert_snapshot!(generate_all(source, "latex"));
}

#[test]
fn rst_enum_mixed() {
    let source = r#"
#[deku_display(bit_width = 8)]
enum Message {
    Empty,
    Ping {
        #[deku(bits = "8")]
        seq: u8,
    },
    Pong {
        #[deku(bits = "8")]
        seq: u8,
    },
}
"#;
    insta::assert_snapshot!(generate_all(source, "rst"));
}

#[test]
fn rst_crossref_page() {
    let source = r#"
#[deku_display(bit_width = 8)]
struct Header {
    #[deku(bits = "4")]
    version: u8,
    #[deku(bits = "4")]
    flags: u8,
}

#[deku_display(bit_width = 16)]
struct Packet {
    #[deku_display(bits = "8")]
    header: Header,
    payload: u8,
}

#[deku_display(bit_width = 16)]
enum Frame {
    Control {
        #[deku_display(bits = "8")]
        header: Header,
        #[deku(bits = "8")]
        cmd: u8,
    },
    Data {
        #[deku_display(bits = "8")]
        header: Header,
        #[deku(bits = "8")]
        len: u8,
    },
}
"#;
    insta::assert_snapshot!(generate_rst_page(source));
}
