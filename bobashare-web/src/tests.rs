// Unit tests for root functions in [`bobashare_web`]

use crate::{render_markdown_with_syntax_set, str_to_duration};

#[test]
fn no_number() {
    assert!(str_to_duration("s").is_err());
    assert!(str_to_duration("h").is_err());
    assert!(str_to_duration("d").is_err());
    assert!(str_to_duration("mon").is_err());
    assert!(str_to_duration("y").is_err());
}

#[test]
fn trailing_junk() {
    assert!(str_to_duration("15djkak").is_err());
    assert!(str_to_duration("15djkak").is_err());
    assert!(str_to_duration("15ykjjk").is_err());
    assert!(str_to_duration("15mon23u").is_err());
}

#[test]
fn preceeding_junk() {
    assert!(str_to_duration("sdf15d").is_err());
    assert!(str_to_duration("s23y").is_err());
    assert!(str_to_duration("$93h").is_err());
}

#[test]
fn large_counts() {
    use std::time::Duration;

    // See issue #27, where counts of 3 or more digits were rejected due to
    // buggy code not splitting the number part from the duration unit properly
    assert_eq!(
        str_to_duration("100m").unwrap(),
        Duration::from_secs(100 * 60)
    );
    assert_eq!(
        str_to_duration("999d").unwrap(),
        Duration::from_secs(999 * 60 * 60 * 24),
    );
    // Larger counts, e.g. using seconds or minutes for finer resolution.
    assert_eq!(
        str_to_duration("86400s").unwrap(),
        Duration::from_secs(86400)
    );
    assert_eq!(
        str_to_duration("1440m").unwrap(),
        Duration::from_secs(1440 * 60),
    );
}

// Verify GHSA-g7gw-4888-mr65 is patched
#[test]
fn render_markdown_strip_raw_html() {
    use syntect::parsing::SyntaxSet;
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let input = r#"## boba

<script>
window.alert("hello");
</script>

s<i>ha</i>re<b>!!!</b>"#;
    let expected = "<h2>boba</h2>\n<p>share!!!</p>\n";

    let rendered = render_markdown_with_syntax_set(input, &syntax_set).unwrap();
    assert_eq!(rendered, expected);
}

#[test]
fn render_markdown_inline_math() {
    use syntect::parsing::SyntaxSet;
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let input = "The equation is $E = mc^2$ in physics.";
    let rendered = render_markdown_with_syntax_set(input, &syntax_set).unwrap();
    assert!(rendered.contains(r#"<span class="katex">"#));
    assert!(rendered.contains("E = mc^2"));
}

#[test]
fn render_markdown_display_math() {
    use syntect::parsing::SyntaxSet;
    let syntax_set = SyntaxSet::load_defaults_newlines();

    let input = "$$\\int_0^\\infty e^{-x^2} dx = \\frac{\\sqrt{\\pi}}{2}$$";
    let rendered = render_markdown_with_syntax_set(input, &syntax_set).unwrap();
    assert!(rendered.contains(r#"<span class="katex-display">"#));
}
