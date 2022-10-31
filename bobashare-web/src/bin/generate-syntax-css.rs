use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Context;
use bobashare_web::HIGHLIGHT_CLASS_PREFIX;
use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle},
};

fn main() -> anyhow::Result<()> {
    let root_var = std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR was not set")?;
    let root = Path::new(&root_var);

    let css_file = File::create(root.join("static/css/highlight/").join("syntax.css"))
        .context("error creating syntax.css")?;
    let mut css_writer = BufWriter::new(css_file);

    let mut theme_set = ThemeSet::load_defaults();
    theme_set
        .add_from_folder(root.join("highlight"))
        .context("error loading highligting themes")?;
    let class_style = ClassStyle::SpacedPrefixed {
        prefix: HIGHLIGHT_CLASS_PREFIX,
    };

    writeln!(css_writer, "@media not (prefers-color-scheme: light) {{")?;
    writeln!(
        css_writer,
        "{}",
        css_for_theme_with_class_style(&theme_set.themes["bobascheme-dark"], class_style,)
            .context("error generating CSS for dark theme")?
    )?;
    writeln!(css_writer, "}}")?;

    writeln!(css_writer, "@media (prefers-color-scheme: light) {{")?;
    writeln!(
        css_writer,
        "{}",
        // TODO: make bobascheme-light
        css_for_theme_with_class_style(&theme_set.themes["InspiredGitHub"], class_style,)
            .context("error generating CSS for light theme")?
    )?;
    writeln!(css_writer, "}}")?;

    Ok(())
}
