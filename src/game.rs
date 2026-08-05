#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
	k,
	q,
	r,
	n,
	b,
	p,
	K,
	Q,
	R,
	N,
	B,
	P,
	None,
}

pub struct Game {
	pub board: [[Piece; 8]; 8],
}
impl Game {
	pub fn new() -> Self {
		Game {
			board: [
				[
					Piece::r,
					Piece::n,
					Piece::b,
					Piece::q,
					Piece::k,
					Piece::b,
					Piece::n,
					Piece::r,
				],
				[Piece::p; 8],
				[Piece::None; 8],
				[Piece::None; 8],
				[Piece::None; 8],
				[Piece::None; 8],
				[Piece::P; 8],
				[
					Piece::R,
					Piece::N,
					Piece::B,
					Piece::Q,
					Piece::K,
					Piece::B,
					Piece::N,
					Piece::R,
				],
			],
		}
	}
}
