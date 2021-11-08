use float_cmp::assert_approx_eq;
use image::GenericImageView;
use text_to_png::{Size, TextRenderer};

const PANGRAM : &str = "the quick brown fox jumps over the lazy dog|THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG";

const TTF_FONT: &[u8] =
    include_bytes!("resources/Because I am Happy Regular.ttf");
const TTC_FONT: &[u8] = include_bytes!("resources/CaviarDreams.ttc");

#[test]
pub fn test_default_renderer() {
    let renderer = TextRenderer::default();

    let result = renderer
        .render_text_to_png_data(PANGRAM, 37, "black")
        .expect("Failed to generate text image");

    assert_approx_eq!(f64, result.baseline_down_from_top, 27.6, epsilon = 0.1);
    assert_eq!(result.size, Size::new(1800, 37));

    let mut actual_image = image::load_from_memory(&result.data)
        .expect("Failed read generated text image");

    let mut expected_image = image::load_from_memory(include_bytes!(
        "resources/text_default_renderer.png"
    ))
    .expect("Failed to read expected text image");

    let diff = lcs_image_diff::compare(
        &mut expected_image,
        &mut &mut actual_image,
        100. / 256.,
    )
    .expect("Failed to compare actual and expected image");

    // This is not perfect, but there's no good way to just get the list of
    // differences out of img-diff. We can check the size of the images to
    // ensure that no difference annotations were added to them
    let expected_size = (result.size.width, result.size.height);

    assert_eq!(diff.dimensions(), expected_size);
    assert_eq!(actual_image.dimensions(), expected_size);
    assert_eq!(expected_image.dimensions(), expected_size);
}

#[test]
pub fn test_renderer_with_ttf_font() {
    let renderer = TextRenderer::try_new_with_ttf_font_data(TTF_FONT.to_vec())
        .expect("Failed to create renderer");

    let result = renderer
        .render_text_to_png_data(PANGRAM, 37, "dark green")
        .expect("Failed to render text");

    assert_approx_eq!(f64, result.baseline_down_from_top, 31.2, epsilon = 0.1);
    assert_eq!(result.size, Size::new(1707, 45));

    let mut actual_image = image::load_from_memory(&result.data)
        .expect("Failed read generated text image");

    let mut expected_image = image::load_from_memory(include_bytes!(
        "resources/ttf_font_text_renderer.png"
    ))
    .expect("Failed to read expected text image");

    let diff = lcs_image_diff::compare(
        &mut expected_image,
        &mut &mut actual_image,
        100. / 256.,
    )
    .expect("Failed to compare actual and expected image");

    // This is not perfect, but there's no good way to just get the list of
    // differences out of img-diff. We can check the size of the images to
    // ensure that no difference annotations were added to them
    let expected_size = (result.size.width, result.size.height);

    assert_eq!(diff.dimensions(), expected_size);
    assert_eq!(actual_image.dimensions(), expected_size);
    assert_eq!(expected_image.dimensions(), expected_size);
}

#[test]
pub fn test_renderer_with_ttf_font_collection() {
    let renderer = TextRenderer::try_new_with_ttf_font_data(TTC_FONT.to_vec())
        .expect("Failed to create renderer");

    let result = renderer
        .render_text_to_png_data(PANGRAM, 37, "orange")
        .expect("Failed to render text");

    assert_approx_eq!(f64, result.baseline_down_from_top, 29.6, epsilon = 0.1);
    assert_eq!(result.size, Size::new(1664, 37));

    let mut actual_image = image::load_from_memory(&result.data)
        .expect("Failed read generated text image");

    let mut expected_image = image::load_from_memory(include_bytes!(
        "resources/ttc_font_text_renderer.png"
    ))
    .expect("Failed to read expected text image");

    let diff = lcs_image_diff::compare(
        &mut expected_image,
        &mut &mut actual_image,
        100. / 256.,
    )
    .expect("Failed to compare actual and expected image");

    // This is not perfect, but there's no good way to just get the list of
    // differences out of img-diff. We can check the size of the images to
    // ensure that no difference annotations were added to them
    let expected_size = (result.size.width, result.size.height);

    assert_eq!(diff.dimensions(), expected_size);
    assert_eq!(actual_image.dimensions(), expected_size);
    assert_eq!(expected_image.dimensions(), expected_size);
}
