use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "fen2image", about = "An example of StructOpt usage.")]
struct Options {
	/// The board position, using FEN notation
    #[structopt(short, long, default_value = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")]
	fen: String,
	/// The file where the board will be saved
    #[structopt(parse(from_os_str), default_value = "board.png")]
    output_filename: PathBuf,
	/// The board’s cell sizes
    #[structopt(short, long, default_value = "60")]
	cell_size: i32,
	/// By default, the board is seen from the white’s point of view.
	/// A reversed board shows black point of view
    #[structopt(short, long)]
	reverse_board: bool
}

fn main() {
	let opt = Options::from_args();
    println!("{:#?}", opt);
}