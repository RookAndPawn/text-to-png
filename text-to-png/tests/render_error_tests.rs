use text_to_png::{TextRenderer, TextToPngError};

#[test]
pub fn test_invalid_color() {
    let tr = TextRenderer::default();

    assert!(matches!(
        tr.render_text_to_png_data("Blah", 34, "not a color"),
        Err(TextToPngError::InvalidColor)
    ));
}

#[test]
pub fn test_invalid_text() {
    let tr = TextRenderer::default();

    // Make a garbage string out of all possible bytes
    let s: String = (0..255).map(|i: u8| i as char).collect();

    assert!(matches!(
        tr.render_text_to_png_data(s, 56, 0),
        Err(TextToPngError::TextProcessError(_))
    ));
}

#[test]
pub fn test_invalid_size() {
    let tr = TextRenderer::default();

    assert!(matches!(
        tr.render_text_to_png_data("Hello", 0, 0),
        Err(TextToPngError::InvalidFontSize(0))
    ));
}

#[test]
pub fn test_invalid_font() {
    let not_font_bytes = include_bytes!("resources/text_default_renderer.png");

    assert!(matches!(
        TextRenderer::try_new_with_ttf_font_data(not_font_bytes),
        Err(TextToPngError::NoFontFound)
    ))
}
