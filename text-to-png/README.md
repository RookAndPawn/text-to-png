# Text To Png

This crate provides a really simple interface for rendering basic text to a png image.

## Features
- 100% Rust! We use [usvg](https://crates.io/crates/usvg) for path vectoring, [resvg](https://crates.io/crates/resvg) for rasterizing, and [tiny-skia](https://crates.io/crates/tiny-skia) for png conversion
- Built-in, monospace font courtesy of [Ryoichi Tsunekawa](https://dharmatype.com/)
- Flexible color specification, `"Aquamarine"`, `"#4506AE"`, `"EEE"`, `0`
- Text baseline height is provided for alignment consistency
- Accepts true-type font files or true-type font collection files

## Example

### Default Font

```rust
use text_to_png::TextRenderer;

let renderer = TextRenderer::default();

let text_png = renderer.render_text_to_png_data(
    "Rénder this, brö",
    64,
    "Dark Turquoise");
```

Writing the `&[u8]` data returned in `text_png.data` to a `text.png` yields:

![Rendered Text Image](https://github.com/RookAndPawn/text-to-png/blob/main/readme-resources/text1.png?raw=true)

### Custom Font

```rust
use text_to_png::TextRenderer;

let renderer = TextRenderer::try_new_with_ttf_font_data(include_bytes!("../tests/resources/Because I am Happy Regular.ttf"))
    .expect("Example font is definitely loadable");

let text_png = renderer.render_text_to_png_data(
    "Custom Fonts are Cool!",
    52,
    0x439EC2);
```

Writing the `&[u8]` data returned in `text_png.data` to a `text.png` yields:

![Rendered Text Image](https://github.com/RookAndPawn/text-to-png/blob/main/readme-resources/text2.png?raw=true)
