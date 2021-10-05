
use std::{path::PathBuf, str::FromStr};

use fontdb::Database;
use resvg::{render_node};
use tiny_skia::Pixmap;
use usvg::{FitTo, NodeExt, Options, TextRendering, Tree};
use eyre::{Result, eyre};

fn create_svg() -> Result<Tree> {
    let mut font_db = Database::new();

    let font_ttl_bytes = include_bytes!("resources/CallingCode-Regular.ttf");

    font_db.load_font_data(font_ttl_bytes.to_vec());

    font_db.set_monospace_family("Calling Code");

    let options = Options {
        //font_family: "monospace".into(),
        fontdb: font_db,
        text_rendering: TextRendering::OptimizeLegibility,
        .. Options::default()
    };

    Tree::from_str(
        include_str!("resources/template.svg"),
        &options.to_ref())
        .map_err(|e| eyre!(e))
}

fn main() {
    let tree = match create_svg() {
        Ok(tree) => tree,
        Err(report) => panic!("{}", report)
    };

    let text_node = tree.node_by_id("text1").unwrap();
    let size = text_node.calculate_bbox().unwrap();

    let mut pixmap = Pixmap::new(
        size.width().ceil() as u32,
        size.height().ceil() as u32)
        .expect("Should be able to create a pixmap");

    render_node(&tree, &text_node, FitTo::Original, pixmap.as_mut());

    if let Err(report) = pixmap
        .save_png(&PathBuf::from_str("text.png").unwrap())
        .map_err(|e| eyre!(e))
    {
        panic!("{}", report);
    }
}
