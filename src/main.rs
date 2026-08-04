use macroquad::prelude::*;

#[allow(non_camel_case_types)]
#[derive(Copy, Clone)]
enum Piece {
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

struct Game {
	board: [[Piece; 8]; 8],
}
impl Game {
	fn new() -> Self {
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

fn window_conf() -> Conf {
	Conf {
		window_title: "Chess".to_string(),
		fullscreen: true,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let game: Game = Game::new();

	loop {
		// render();

		next_frame().await;
	}
}
