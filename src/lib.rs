extern crate regex;

fn is_valid_fen(fen: &str) -> bool {
	use regex::Regex;
	let fen_regex = r"\s*([rnbqkpRNBQKP1-8]+/){7}([rnbqkpRNBQKP1-8]+)\s[bw-]\s(([a-hkqA-HKQ]{1,4})|(-))\s(([a-h][36])|(-))\s\d+\s\d+\s*";
	let re = Regex::new(fen_regex).unwrap();
	re.is_match(fen)
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


