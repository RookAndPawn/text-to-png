use float_cmp::assert_approx_eq;
use image::GenericImageView;
use text_to_png::TextRenderer;

const PANGRAM : &str = "the quick brown fox jumps over the lazy dog|THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";

const TTF_FONT: &[u8] =
    include_bytes!("resources/Because I am Happy Regular.ttf");
const TTC_FONT: &[u8] = include_bytes!("resources/CaviarDreams.ttc");

#[test]
pub fn test_default_renderer() {
    let renderer = TextRenderer::default();

    test_render(
        &renderer,
        PANGRAM,
        37,
        "black",
        include_bytes!("resources/text_default_renderer.png"),
        27.6,
    );
}

#[test]
pub fn test_renderer_with_ttf_font() {
    let renderer = TextRenderer::try_new_with_ttf_font_data(TTF_FONT)
        .expect("Failed to create renderer");

    test_render(
        &renderer,
        PANGRAM,
        37,
        "dark green",
        include_bytes!("resources/ttf_font_text_renderer.png"),
        31.2,
    );
}

#[test]
pub fn test_renderer_with_ttf_font_collection() {
    let renderer = TextRenderer::try_new_with_ttf_font_data(TTC_FONT)
        .expect("Failed to create renderer");

    test_render(
        &renderer,
        PANGRAM,
        37,
        "orange",
        include_bytes!("resources/ttc_font_text_renderer.png"),
        29.6,
    );
}

fn test_render(
    renderer: &TextRenderer,
    text: &str,
    size: u32,
    color: &str,
    expected_image_bytes: &[u8],
    expected_baseline: f64,
) {
    let result = renderer
        .render_text_to_png_data(text, size, color)
        .expect("Failed to render text");

    let expected_image = image::load_from_memory(expected_image_bytes)
        .expect("Failed to read expected text image");

    assert_eq!(
        (result.size.width, result.size.height),
        expected_image.dimensions()
    );
    assert_approx_eq!(
        f64,
        result.baseline_down_from_top,
        expected_baseline,
        epsilon = 0.1
    );

    let actual_image = image::load_from_memory(&result.data)
        .expect("Failed read generated text image");

    let comp = image_compare::rgba_hybrid_compare(
        &actual_image.into_rgba8(),
        &expected_image.into_rgba8(),
    )
    .expect("Failed to compare actual and expected image");

    assert!(comp.score >= 0.99);
}
