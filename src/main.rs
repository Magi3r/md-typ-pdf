use std::path::Path;

use clap::Parser;
use duct::cmd;

#[derive(Debug, Clone, clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the input Markdown file.
    input: String,
    /// Path to the output PDF file.
    output: Option<String>,
}

fn main() {
    println!("Starting Markdown to PDF Converter.");
    let args = Args::parse();
    let output_path = args
        .output
        .clone()
        .unwrap_or_else(|| args.input.clone().replace(".md", ".pdf"));
    convert_to_pdf(Path::new(&args.input), &output_path);
    println!("Done.");
}

/// Converts a Markdown file to PDF using typst-cli.
fn convert_to_pdf(md_file: &Path, output: &str) {
    let template_string: String = "#import \"@preview/cmarker:0.1.6\"\n#cmarker.render(\nread(\"REPLACE_ME\"),\nscope: (image: (path, alt: none) => image(path, alt: alt))\n)"
        .replace("REPLACE_ME", md_file.to_str().unwrap());

    let output_path = Path::new(output);

    cmd!("typst", "compile", "--root", ".", "-", output_path)
        .stdin_bytes(template_string.as_bytes())
        .run()
        .unwrap();
}
