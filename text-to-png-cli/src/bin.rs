#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::all
)]

use clap::{CommandFactory, Parser};
use std::{
    fs::File,
    io::{BufReader, BufWriter, Read, Write},
    path::{Path, PathBuf},
};
use text_to_png::{TextRenderer, TextToPngError};
use thiserror::Error;

const DEFAULT_FONT_SIZE: &str = "64";
const DEFAULT_COLOR: &str = "Orange Red";

const OPT_FONT_SIZE: &str = "font-size";
const OPT_COLOR: &str = "color";

#[derive(Debug, Error)]
#[non_exhaustive]
enum TextToPngCliError {
    #[error("Couldn't read font file {0} - {1}")]
    FontFileReadError(String, #[source] std::io::Error),

    #[error("No fonts were loadable from the given font file - {0}")]
    InvalidFontFile(String),

    #[error("Couldn't interpret argument {arg_name:?}={arg_value:?}")]
    InvalidUserInput {
        arg_name: &'static str,
        arg_value: String,
    },

    #[error("There was an unknown error while rendering text")]
    UnexpectedError,

    #[error("Failure while rendering text to png - {0}")]
    ExecutionFailed(
        #[from]
        #[source]
        TextToPngError,
    ),
    #[error("Failure writing the png to file - {0}")]
    IOError(
        #[from]
        #[source]
        std::io::Error,
    ),
}

/// Render the text as described by the given command line arguments and present
/// any errors back to the main caller for reporting back to the user
fn render_png(args: &Args) -> Result<(), TextToPngCliError> {
    let renderer = if let Some(font_file) = args.font_file.as_ref() {
        let open_file = File::open(font_file).map_err(|e| {
            TextToPngCliError::FontFileReadError(
                font_file.display().to_string(),
                e,
            )
        })?;

        let mut ttf_font_data = Vec::new();

        {
            let mut reader = BufReader::new(open_file);
            reader.read_to_end(&mut ttf_font_data).map_err(|e| {
                TextToPngCliError::FontFileReadError(
                    font_file.display().to_string(),
                    e,
                )
            })?;
        }

        TextRenderer::try_new_with_ttf_font_data(ttf_font_data).map_err(
            |_| {
                TextToPngCliError::InvalidFontFile(
                    font_file.display().to_string(),
                )
            },
        )?
    } else {
        TextRenderer::default()
    };

    let font_size = args.font_size;

    let color = args.color.as_str();

    let to_render = args.text.as_slice().join(" ");

    let result = renderer.render_text_to_png_data(to_render, font_size, color);

    let png_data = match result {
        Err(TextToPngError::InvalidColor) => {
            Err(TextToPngCliError::InvalidUserInput {
                arg_name: OPT_COLOR,
                arg_value: color.into(),
            })
        }
        Err(TextToPngError::InvalidFontSize) => {
            Err(TextToPngCliError::InvalidUserInput {
                arg_name: OPT_FONT_SIZE,
                arg_value: font_size.to_string(),
            })
        }
        Err(TextToPngError::InvalidInput) => {
            Err(TextToPngCliError::UnexpectedError)
        }
        Err(_) => result.map_err(|e| e.into()),
        Ok(png_data) => Ok(png_data),
    }?;

    let output_path: &Path = &args.output_file;

    let output_file = File::create(output_path)?;

    {
        let mut writer = BufWriter::new(output_file);
        writer.write_all(&png_data.data)?;
    }

    Ok(())
}

#[derive(Parser)]
struct Args {
    /// Font height in pixels
    #[arg(long, short = 's', default_value = DEFAULT_FONT_SIZE)]
    font_size: u32,

    /// Color of the text: e.g. Brown, #45A2f4, 666
    #[arg(long, short = 'c', default_value = DEFAULT_COLOR)]
    color: String,

    /// ttf or ttc font file to use
    #[arg(long, short = 'f')]
    font_file: Option<PathBuf>,

    /// Path of the file to write the rendered png
    #[arg(long, short = 'o')]
    output_file: PathBuf,

    /// Text to render
    #[arg(required = true)]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if let Err(e) = render_png(&args) {
        eprintln!("{}", e);

        // If the error was due to invalid user input, then write the usage
        // to the console
        if matches!(e, TextToPngCliError::InvalidUserInput { .. }) {
            eprintln!("Error in input arguments: {e:#?}");

            Args::command().print_help().unwrap();
        }

        std::process::exit(1);
    }
}
