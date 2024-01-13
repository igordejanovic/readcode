use std::{fs, path::PathBuf, error::Error};

use clap::Parser;
use genpdf::{fonts, elements::Paragraph, style::Style};
use ignore::Walk;
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

const FONT_DIRS: &[&str] = &[
    "/usr/share/fonts/liberation",
    "/usr/share/fonts/truetype/liberation",
];
const DEFAULT_FONT_NAME: &str = "LiberationMono";


#[derive(Parser)]
#[clap(author, about, long_about = None)]
struct Cli {
    #[clap(value_name="ROOT DIR", value_hint = clap::ValueHint::DirPath)]
    root: PathBuf,
}

fn to_genpdf_color(syntex_color: syntect::highlighting::Color) -> genpdf::style::Color {
    genpdf::style::Color::Rgb(syntex_color.r, syntex_color.g, syntex_color.b)
}

fn to_genpdf_style(
    syntect_style: syntect::highlighting::Style,
) -> genpdf::style::Style {
    let mut style = genpdf::style::Style::default();
    style.set_color(to_genpdf_color(syntect_style.foreground));
    if syntect_style.font_style.contains(FontStyle::BOLD) {
        style.set_bold();
    }
    if syntect_style.font_style.contains(FontStyle::ITALIC) {
        style.set_italic();
    }
    style
}

fn main() -> Result<(), Box<dyn Error>> {

    let cli = Cli::parse();
    let path = fs::canonicalize(&cli.root)?;
    let title = path.to_string_lossy();

    let font_dir = FONT_DIRS
        .iter()
        .find(|path| std::path::Path::new(path).exists())
        .expect("Could not find font directory");
    let default_font =
        fonts::from_files(font_dir, DEFAULT_FONT_NAME, None)
            .expect("Failed to load the default font family");

    let mut doc = genpdf::Document::new(default_font);
    doc.set_title(title);
    doc.set_minimal_conformance();
    doc.set_font_size(10);
    doc.set_line_spacing(1.25);

    // 1. iterate over given path
    let ps = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    for entry in Walk::new(&cli.root) {
        let entry = entry?;
        let path = entry.path();
        println!("Processing {:?}", path);
        if path.is_dir() {
            continue;
        }

        let extension = path.extension().unwrap_or_default();
        let syntax = match ps.find_syntax_by_extension(&extension.to_string_lossy()) {
            Some(syntax) => syntax,
            None => ps.find_syntax_plain_text(),
        };

        // Make title and write it to TOC file
        doc.push(Paragraph::new(""));
        let mut p = Paragraph::default();
        let mut title_style = Style::new().with_font_size(14);
        title_style.set_bold();
        p.push_styled(format!("File: {:?}", path.strip_prefix(&cli.root).unwrap()), title_style);
        doc.push(p);
        doc.push(Paragraph::new(""));

        // 3. load file and run though syntax highlighter
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            // If the file cannot read as valid UTF-8 continue
            Err(_) => continue,
        };
        let mut h = HighlightLines::new(syntax, &ts.themes["InspiredGitHub"]);
        for (line_no, line) in LinesWithEndings::from(&content).enumerate() {
            let line = line.trim_end();
            let mut p = Paragraph::default();
            p.push((line_no + 1).to_string());
            p.push(" ");
            let ranges: Vec<(syntect::highlighting::Style, &str)> =
                h.highlight_line(line, &ps).unwrap();
            // 4. Convert text to genpdf styledstring
            for (style, s) in ranges {
                p.push_styled(s, to_genpdf_style(style));
            }
            doc.push(p);
        }
    }

    let output_file = PathBuf::from("output.pdf");
    doc.render_to_file(&output_file)?;
    println!("PDF saved to {:?}", output_file);
    Ok(())
}
