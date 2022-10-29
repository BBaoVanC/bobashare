use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::Path,
};

use anyhow::Context;
use syntect::{
    highlighting::ThemeSet,
    html::{css_for_theme_with_class_style, ClassStyle},
};

const HIGHLIGHT_CLASS_PREFIX: &str = "hl-";

fn main() -> anyhow::Result<()> {
    let root_var = std::env::var("CARGO_MANIFEST_DIR").context("CARGO_MANIFEST_DIR was not set")?;
    let root = Path::new(&root_var);

    let dist_root = root.join("static").join("dist");
    // make sure dist exists; it's gitignored so it's not in the repo
    fs::create_dir_all(&dist_root).context("error creating static/dist/ directory")?;

    let css_file =
        File::create(dist_root.join("syntax.css")).context("error creating syntax.css")?;
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
