//! # Text To Png
//!
//! This crate provides a really simple interface for rendering basic text to a png image.
//!
//! ## Features
//! - 100% Rust! We use [usvg](https://crates.io/crates/usvg) for path vectoring, [resvg](https://crates.io/crates/resvg) for rasterizing, and [tiny-skia](https://crates.io/crates/tiny-skia) for png conversion
//! - Built-in, monospace font courtesy of [Ryoichi Tsunekawa](https://dharmatype.com/)
//! - Flexible color specification, `"Aquamarine"`, `"#4506AE"`, `"EEE"`, `0`
//! - Text baseline height is provided for alignment consistency
//!
//! ## Example
//!
//! ```rust
//! use text_to_png::TextRenderer;
//!
//! let renderer = TextRenderer::default();
//!
//! let text_png = renderer.render_text_to_png_data("Rénder this, brö", 64, "Dark Turquoise");
//! ```
//!
//! Writing the `&[u8]` data returned in `text_png.data` to a `text.png` yields:
//!
//! ![Rendered Text Image](https://github.com/RookAndPawn/text-to-png/blob/main/readme-resources/text.png?raw=true)
//!
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::all
)]

mod colors;

use fontdb::Database;
use resvg::render_node;
use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};
use thiserror::Error;
use tiny_skia::Pixmap;
use usvg::{
    Error as SvgError, FitTo, NodeExt, Options, PathBbox, TextRendering, Tree,
};
use xml::escape::escape_str_pcdata;

const DEFAULT_FONT: &[u8] = include_bytes!("resources/CallingCode-Regular.ttf");
const DEFAULT_FONT_NAME: &str = "Calling Code";

lazy_static::lazy_static! {
    static ref DEFAULT_FONT_DB : Database = create_default_font_db();
}

fn create_default_font_db() -> Database {
    let mut result = Database::new();

    result.load_font_data(DEFAULT_FONT.to_vec());

    result.set_cursive_family(DEFAULT_FONT_NAME);
    result.set_fantasy_family(DEFAULT_FONT_NAME);
    result.set_monospace_family(DEFAULT_FONT_NAME);
    result.set_sans_serif_family(DEFAULT_FONT_NAME);
    result.set_serif_family(DEFAULT_FONT_NAME);

    result
}

/// Parse a 1 or 2-digit slice of ascii characters into the u8 representation
fn parse_color_value(slice: &[u8]) -> Option<u8> {
    let mut result: u8 = 0;
    let repeats = 2 - slice.len();

    // 3-digit colors imply duplication of the hex digit to fill byte
    for _ in 0..=repeats {
        for ch in slice {
            result *= 16;

            if !ch.is_ascii_hexdigit() {
                return None;
            }

            result += match (ch.is_ascii_digit(), ch.is_ascii_uppercase()) {
                (true, _) => ch - b'0',
                (_, false) => 10 + ch - b'a',
                (_, true) => 10 + ch - b'A',
            };
        }
    }

    Some(result)
}

/// Representation of the size of png image
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub struct Size {
    /// Image width in pixels
    pub width: u32,

    /// Image height in pixels
    pub height: u32,
}

impl From<PathBbox> for Size {
    fn from(pb: PathBbox) -> Self {
        Size {
            width: pb.width().ceil() as u32,
            height: pb.height().ceil() as u32,
        }
    }
}

/// Representation of a RGB color
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Default, derive_new::new)]
pub struct Color {
    /// Red Component
    pub r: u8,

    /// Green Component
    pub g: u8,

    /// Blue Component
    pub b: u8,
}

/// Result of rendering text to a png image. This contains the png byte as well
/// as the the image metadata for pixel size and baseline location
#[derive(Debug, Clone)]
pub struct TextPng {
    /// Png byte data. This can be written directly to a file or texture
    pub data: Vec<u8>,

    /// Size of the image when rendered
    pub size: Size,

    /// Y location of the text baseline measuring down from the top
    pub baseline_down_from_top: f64,
}

/// Error type returned on unsuccessful rendering calls
#[derive(Error, Debug)]
pub enum TextToPngError {
    /// Error case when the color string given is not parsable into a valid
    /// color
    #[error("Couldn't create color from input")]
    InvalidColor,

    /// Error case to handle failures form usvg
    #[error("Failed to construct vectors for text - {0}")]
    TextProcessError(
        #[from]
        #[source]
        SvgError,
    ),

    /// Error case to handle failures from resvg and tiny-skia
    #[error("Png generation failed - {0}")]
    PngGeneratorError(
        #[from]
        #[source]
        png::EncodingError,
    ),
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "{:02X?}{:02X?}{:02X?}",
            self.r, self.g, self.b
        ))
    }
}

impl Color {
    fn try_new_from_color_name(value: &str) -> Option<Color> {
        let sanitized: String = value
            .chars()
            .filter(char::is_ascii_alphabetic)
            .map(|c| c.to_ascii_lowercase())
            .collect();

        colors::from_str(&sanitized)
    }

    fn try_new_from_hex_digits(value: &str) -> Option<Color> {
        // check for a leading `#`
        let start = if value.starts_with('#') { 1 } else { 0 };

        // assume 3 digit hex color
        let chunk_size = if value.len() < 6 { 1 } else { 2 };

        let mut chunks = value[start..]
            .as_bytes()
            .chunks(chunk_size)
            .filter(|chunk| chunk.len() == chunk_size);

        Some(Color {
            r: parse_color_value(chunks.next()?)?,
            g: parse_color_value(chunks.next()?)?,
            b: parse_color_value(chunks.next()?)?,
        })
    }
}

/// This is the mechanism used to perform the actual rendering. This struct
/// contains the options that describe how text will be converted into vector
/// paths. Currently we only support the default, embedded font, and we optimize
/// for legibility
#[derive(Debug, Clone)]
pub struct TextRenderer {
    render_options: Options,
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TryFrom<&str> for Color {
    type Error = ();

    /// This will accept color in the form of a 3 or 6 digit hex number with
    /// or without a preceding `#`.
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Color::try_new_from_color_name(value)
            .or_else(|| Color::try_new_from_hex_digits(value))
            .ok_or(())
    }
}

impl From<u32> for Color {
    /// This will create a color from the lower 24bits of the given u32 with
    /// red being the most significant
    fn from(mut value: u32) -> Self {
        let b = (value & 0xFF) as u8;
        value >>= 8;
        let g = (value & 0xFF) as u8;
        value >>= 8;
        let r = (value & 0xFF) as u8;

        Color { r, g, b }
    }
}

impl TextRenderer {
    /// Create the default text renderer. This will provide a render that uses
    /// only the default font
    pub fn new() -> Self {
        let options = Options {
            //font_family: "monospace".into(),
            fontdb: DEFAULT_FONT_DB.clone(),
            text_rendering: TextRendering::OptimizeLegibility,
            ..Options::default()
        };

        TextRenderer {
            render_options: options,
        }
    }

    /// Render the given text to a png with the given options.
    /// ```
    /// let renderer = TextRenderer::default();
    /// let text_png = renderer
    ///     .render_text_to_png_data(
    ///         "Any kind of text will do here", // It can be owned or borrowed
    ///         42, // Font size in pixels here
    ///         "#FF00FF" // A good color for the job, "Magenta" would work too
    ///     );
    /// ```
    pub fn render_text_to_png_data<T, C>(
        &self,
        text: T,
        font_size_pixels: u32,
        color: C,
    ) -> Result<TextPng, TextToPngError>
    where
        T: AsRef<str>,
        C: TryInto<Color>,
    {
        let text_str = escape_str_pcdata(text.as_ref()).into();
        let color_val =
            color.try_into().map_err(|_| TextToPngError::InvalidColor)?;

        self.render_text_to_png_data_private(
            text_str,
            font_size_pixels,
            color_val,
        )
    }

    fn render_text_to_png_data_private(
        &self,
        text: String,
        font_size: u32,
        color: Color,
    ) -> Result<TextPng, TextToPngError> {
        let content = format!(
            include_str!("resources/template.svg"),
            font_size, color, text
        );

        let tree =
            Tree::from_str(content.as_str(), &self.render_options.to_ref())?;

        let text_node = tree.node_by_id("t").unwrap();
        let size = text_node.calculate_bbox().unwrap();

        let mut pixmap = Pixmap::new(
            size.width().ceil() as u32,
            size.height().ceil() as u32,
        )
        .expect("Should be able to create a pixmap");

        render_node(&tree, &text_node, FitTo::Original, pixmap.as_mut());
        let png_data = pixmap.encode_png()?;

        Ok(TextPng {
            baseline_down_from_top: -size.y(),
            size: size.into(),
            data: png_data,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_3_hex_color() {
        assert_eq!("7aE".try_into(), Ok(Color::new(0x77, 0xAA, 0xEE)));
        assert_eq!("#7aE".try_into(), Ok(Color::new(0x77, 0xAA, 0xEE)));
        assert_eq!(Color::try_from("77"), Err(()));
        assert_eq!(Color::try_from(""), Err(()));
        assert_eq!(Color::try_from("a0g"), Err(()));
    }

    #[test]
    fn test_parse_6_hex_color() {
        assert_eq!("7a0Ef9".try_into(), Ok(Color::new(0x7A, 0xE, 0xF9)));
        assert_eq!("#7a0Ef9".try_into(), Ok(Color::new(0x7A, 0xE, 0xF9)));
        assert_eq!(Color::try_from("#77094"), Err(()));
        assert_eq!(Color::try_from("a0g09"), Err(()));
    }

    #[test]
    fn test_parse_u32_color() {
        assert_eq!(Color::try_from(0x7A0EF9), Ok(Color::new(0x7A, 0xE, 0xF9)));
    }

    #[test]
    fn test_parse_color_from_name() {
        assert_eq!(
            Color::try_from("aquamarine"),
            Ok(Color {
                r: 127,
                g: 255,
                b: 212
            })
        );
    }

    #[test]
    fn test_basic_render() {
        let r = TextRenderer::default();

        let png = r.render_text_to_png_data("7", 24, 0);

        assert!(png.is_ok());
    }
}
