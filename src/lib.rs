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


