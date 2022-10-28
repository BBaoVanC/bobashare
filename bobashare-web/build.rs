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

    let dark_css_file = File::create(dist_root.join("syntax-dark.css"))
        .context("error creating syntax-dark.css")?;
    let mut dark_css_writer = BufWriter::new(dark_css_file);
    let light_css_file = File::create(dist_root.join("syntax-light.css"))
        .context("error creating syntax-light.css")?;
    let mut light_css_writer = BufWriter::new(light_css_file);

    let theme_set = ThemeSet::load_from_folder(root.join("highlight"))
        .context("error loading highligting themes")?;
    let class_style = ClassStyle::SpacedPrefixed {
        prefix: HIGHLIGHT_CLASS_PREFIX,
    };

    writeln!(
        dark_css_writer,
        "{}",
        css_for_theme_with_class_style(&theme_set.themes["bobascheme-dark"], class_style,)
            .context("error generating CSS for dark theme")?
    )
    .context("error writing dark theme CSS")?;

    // TODO: make bobascheme-light
    writeln!(light_css_writer, "@media (prefers-color-scheme: light) {{")
        .context("error writing media query to light theme CSS")?;
    writeln!(light_css_writer, "/* TODO */").context("error writing light theme CSS")?;
    writeln!(light_css_writer, "}}").context("error closing media query in light theme CSS")?;
    /*
    writeln!(
        light_css_writer,
        "{}",
        css_for_theme_with_class_style(&theme_set.themes["bobascheme-light"], class_style,)
            .context("error generating CSS for light theme")?
    )
    .context("error writing light theme CSS")?;
    */

    Ok(())
}
