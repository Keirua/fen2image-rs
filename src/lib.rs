extern crate regex;

fn is_valid_fen(fen: &str) -> bool {
	use regex::Regex;
	let fen_regex = r"\s*([rnbqkpRNBQKP1-8]+/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*";
	let re = Regex::new(fen_regex).unwrap();
	re.is_match(fen)
}

type ChessBoardLine = [char; 8];
type ChessBoard = [ChessBoardLine;8];

fn default_board() -> ChessBoard {
	let default_board: ChessBoard = [
        ['r', 'n', 'b', 'q', 'k', 'b', 'n', 'r'],
        ['p'; 8],
        [' '; 8],
        [' '; 8],
        [' '; 8],
        [' '; 8],
        ['P'; 8],
        ['R', 'N', 'B', 'Q', 'K', 'B', 'N', 'R']
    ];

    default_board
}

fn empty_board() -> ChessBoard {
	[[' '; 8]; 8]
}

fn get_board_line_from_fen(fen_line: &str) -> Result<ChessBoardLine, String> {
	let mut line = [' '; 8];

	let mut pos = 0usize;
	let validPieces = ['r', 'n', 'b', 'q', 'k', 'p', 'R', 'N', 'B', 'Q', 'K', 'P'];
	for cell in fen_line.chars() {
		if pos >= 8 {
			return Err(format!("Invalid column offset (>8) in {}", fen_line));
		}
		if validPieces.contains(&cell) {
			line[pos] = cell;
			pos += 1;
		} else {
			let offset = cell.to_digit(10u32).unwrap() as usize;
			pos += offset;
		}
	}

	Ok(line)
}

pub fn get_board_from_fen(fen: &str) -> Result<ChessBoard, String> {
	if !is_valid_fen(fen) {
		return Err("Invalid fen".to_string());
	} 
	let mut splits = fen.split(" ");
	let lines = splits.next().unwrap().split("/");

	let mut board = empty_board();
	for (y, line) in lines.enumerate() {
		match get_board_line_from_fen(&line) {
			Ok(line) => board[y as usize] = line,
			Err(s) => return Err(s)
		};
	}
	return Ok(board);
}

pub fn display_board(board: &ChessBoard) {
	for l in board {
		println!("{}", l.iter().collect::<String>());
	}
}

extern crate image;
use image::io::Reader as ImageReader;
use image::ImageError;

pub fn generate_image(fen: &str, output_file: &str, cell_size: u32) -> Result<(), image::ImageError> {
	let bishop = ImageReader::open("icons/B60.png")?.decode()?;

	// let board_image = ImageWriter::open(output_file)?.decode()?;
	let imgx = 8 * cell_size;
	let imgy = 8 * cell_size;
	// Create a new ImgBuf with width: imgx and height: imgy
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
 	    // var whiteColor = color.RGBA{222, 227, 230, 255}
	    // var blackColor = color.RGBA{140, 162, 173, 255}
    	let white = [222u8,227u8,230u8];
    	let black = [140u8,162u8,173u8];

        // let r = (0.3 * x as f32) as u8;
        // let b = (0.3 * y as f32) as u8;
        if (((x/cell_size as u32) + (y/cell_size as u32)) % 2) == 1 {
        	*pixel = image::Rgb(black);
        } else {
        	*pixel = image::Rgb(white);
        }
    }

    imgbuf.save(output_file)
}


#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn it_works() {
    	let valid_fens = vec![
			"8/8/8/8/8/8/8/8 w - - 0 0",
			"4k3/r6B/8/8/8/8/8/K6Q w - - 0 0",
			"4k3/r6Bp/8p/8/8/8/8/K6Q w - - 0 0", // invalid board but valid regex
			"4k3/r6Bp/8p/8/8/8/8/K6Q b - - 0 0",
			"4k3/r6Bp/8p/8/8/8/8/K6Q b KQkq - 0 0",
			"4k3/r6Bp/8p/8/8/8/8/K6Q b Qkq - 0 0",
			"4k3/r6Bp/8p/8/8/8/8/K6Q b kq - 0 0",
			"4k3/r6Bp/8p/8/8/8/8/K6Q b q - 0 0"
    	];
    	for fen in valid_fens {
    		assert_eq!(true, is_valid_fen(fen));
    	}

    	let invalid_fens = vec![
			"4k3/r6Bp/8p/8/8/8/8 w - - 0 0",
			"pouet",
    	];

    	for fen in invalid_fens {
    		assert_eq!(false, is_valid_fen(fen));
    	}
    }
}


