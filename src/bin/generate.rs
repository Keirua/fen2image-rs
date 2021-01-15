use std::path::PathBuf;
use structopt::StructOpt;

extern crate fen2image;
use fen2image::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "fen2image", about = "An example of StructOpt usage.")]
struct Options {
    /// The board position, using FEN notation
    #[structopt(
        short,
        long,
        default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    )]
    fen: String,

    /// The board’s cell sizes
    #[structopt(short, long, default_value = "60")]
    cell_size: i32,

    /// Where to write the output: to `stdout` or `file`
    #[structopt(short, long, default_value = "file")]
    out_type: String,

    /// The file where the board will be saved
    #[structopt(parse(from_os_str), required_if("out-type", "file"))]
    output_filename: Option<PathBuf>,

    /// By default, the board is seen from the white’s point of view.
    /// A reversed board shows black point of view
    #[structopt(short, long)]
    reverse_board: bool,
}

fn main() {
    let opt = Options::from_args();

    println!("{:#?}", opt);
    println!("{:#?}", opt.fen);

    let board = get_board_from_fen(&opt.fen);
    println!("{:?}", board);
}
