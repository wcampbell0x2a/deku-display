use deku_display_cli::{generate_latex, generate_rst, parse};

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
#[command(name = "deku-display-cli")]
#[command(about = "Generate LaTeX/RST bitfield diagrams from Rust source files")]
struct Cli {
    /// Input .rs file(s) to process
    input: Vec<PathBuf>,

    /// Output format
    #[arg(long, default_value = "rst", value_parser = ["latex", "rst"])]
    format: String,

    /// Output file (default: stdout)
    #[arg(long)]
    output: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut all_types = Vec::new();
    for path in &cli.input {
        let source = fs::read_to_string(path)
            .with_context(|| format!("failed to read '{}'", path.display()))?;
        match parse::parse_file(&source) {
            Ok(types) => all_types.extend(types),
            Err(e) => {
                eprintln!("warning: skipping '{}': {}", path.display(), e);
            }
        }
    }

    if all_types.is_empty() {
        eprintln!("warning: no structs or enums with #[deku_display] or #[derive(DekuRead/DekuWrite)] found");
        return Ok(());
    }

    let result = match cli.format.as_str() {
        "rst" => generate_rst::generate_page(&all_types)?,
        "latex" => {
            let mut buf = String::new();
            for (i, info) in all_types.iter().enumerate() {
                if i > 0 {
                    buf.push('\n');
                }
                buf.push_str(&generate_latex::generate(info)?);
            }
            buf
        }
        _ => unreachable!(),
    };

    if let Some(output_path) = &cli.output {
        fs::write(output_path, &result)
            .with_context(|| format!("failed to write '{}'", output_path.display()))?;
    } else {
        print!("{}", result);
    }

    Ok(())
}
