#![doc = include_str!("../README.md")]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    clippy::all
)]

use clap::{App, AppSettings, Arg, ArgMatches};
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
const OPT_TEXT: &str = "text";
const OPT_COLOR: &str = "color";
const OPT_FILE: &str = "file";
const OPT_FONT_FILE: &str = "font-file";

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
fn render_png(matches: &ArgMatches<'_>) -> Result<(), TextToPngCliError> {
    let renderer = if let Some(font_file) = matches.value_of(OPT_FONT_FILE) {
        let open_file = File::open(Path::new(font_file)).map_err(|e| {
            TextToPngCliError::FontFileReadError(font_file.into(), e)
        })?;

        let mut ttf_font_data = Vec::new();

        {
            let mut reader = BufReader::new(open_file);
            reader.read_to_end(&mut ttf_font_data).map_err(|e| {
                TextToPngCliError::FontFileReadError(font_file.into(), e)
            })?;
        }

        TextRenderer::try_new_with_ttf_font_data(ttf_font_data)
            .map_err(|_| TextToPngCliError::InvalidFontFile(font_file.into()))?
    } else {
        TextRenderer::default()
    };

    let font_size_str = matches
        .value_of(OPT_FONT_SIZE)
        .expect("Default value provided");

    let font_size = font_size_str
        .parse::<f64>()
        .map_err(|_| TextToPngCliError::InvalidUserInput {
            arg_name: OPT_FONT_SIZE,
            arg_value: font_size_str.into(),
        })?
        .ceil()
        .min(u32::MAX as f64) as u32;

    let color = matches.value_of(OPT_COLOR).expect("Default value provided");

    let to_render = matches
        .values_of("text")
        .ok_or_else(|| TextToPngCliError::InvalidUserInput {
            arg_name: OPT_TEXT,
            arg_value: "{empty string}".into(),
        })?
        .collect::<Vec<_>>()
        .as_slice()
        .join(" ");

    let result = renderer.render_text_to_png_data(to_render, font_size, color);

    let png_data = match result {
        Err(TextToPngError::InvalidColor) => {
            Err(TextToPngCliError::InvalidUserInput {
                arg_name: OPT_COLOR,
                arg_value: color.into(),
            })
        }
        Err(TextToPngError::InvalidFontSize(size)) => {
            Err(TextToPngCliError::InvalidUserInput {
                arg_name: OPT_FONT_SIZE,
                arg_value: format!("{}", size),
            })
        }
        Err(TextToPngError::InvalidInput) => {
            Err(TextToPngCliError::UnexpectedError)
        }
        Err(_) => result.map_err(|e| e.into()),
        Ok(png_data) => Ok(png_data),
    }?;

    let output_path: PathBuf =
        matches.value_of(OPT_FILE).expect("Required").into();

    let output_file = File::create(output_path)?;

    {
        let mut writer = BufWriter::new(output_file);
        writer.write_all(&png_data.data)?;
    }

    Ok(())
}

fn main() {
    let matches = App::new("Text To Png Cli")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Kevin G. <kevin.guthrie@gmail.com>")
        .about("Renders text to a png with some options")
        .setting(AppSettings::TrailingVarArg)
        .arg(Arg::with_name(OPT_FONT_SIZE)
            .short("s")
            .long("font-size")
            .takes_value(true)
            .help("Font height in pixels")
            .required(false)
            .default_value(DEFAULT_FONT_SIZE))
        .arg(Arg::with_name(OPT_COLOR)
            .short("c")
            .long("color")
            .takes_value(true)
            .required(false)
            .help("Color of the text: e.g. Brown, #45A2f4, 666")
            .default_value(DEFAULT_COLOR))
        .arg(Arg::with_name(OPT_FONT_FILE)
            .short("f")
            .long("font-file")
            .takes_value(true)
            .help("ttf or ttc font file to use"))
        .arg(Arg::with_name(OPT_FILE)
            .short("o")
            .long("output")
            .takes_value(true)
            .required(true)
            .help("Path of the file to write the rendered png"))
        .arg(Arg::with_name(OPT_TEXT)
            .help("All trailing arguments will be treated as the text to render")
            .multiple(true))
        .get_matches();

    if let Err(e) = render_png(&matches) {
        eprintln!("{}", e);

        // If the error was due to invalid user input, then write the usage
        // to the console
        if matches!(e, TextToPngCliError::InvalidUserInput { .. }) {
            eprintln!("{}", matches.usage());
        }

        std::process::exit(1);
    }
}
