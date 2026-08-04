use colored::*;
use enigo::{Enigo, Mouse, Settings};
use include_dir::{Dir, File, include_dir};
use macroquad::prelude::*;

static ASSETS: Dir = include_dir!("assets");

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

#[allow(non_snake_case)]
struct Texture {
	k: Texture2D,
	q: Texture2D,
	r: Texture2D,
	n: Texture2D,
	b: Texture2D,
	p: Texture2D,
	K: Texture2D,
	Q: Texture2D,
	R: Texture2D,
	N: Texture2D,
	B: Texture2D,
	P: Texture2D,
	BoardLight: Texture2D,
	BoardDark: Texture2D,
}
impl Texture {
	fn new() -> Self {
		Self {
			k: Self::load_piece("Bk"),
			q: Self::load_piece("Bq"),
			r: Self::load_piece("Br"),
			n: Self::load_piece("Bn"),
			b: Self::load_piece("Bb"),
			p: Self::load_piece("Bp"),
			K: Self::load_piece("WK"),
			Q: Self::load_piece("WQ"),
			R: Self::load_piece("WR"),
			N: Self::load_piece("WN"),
			B: Self::load_piece("WB"),
			P: Self::load_piece("WP"),
			BoardLight: Self::load_board("light"),
			BoardDark: Self::load_board("dark"),
		}
	}

	fn load_piece(name: &str) -> Texture2D {
		Self::load_img(&format!("textures/pieces/{name}.png"))
	}

	fn load_board(name: &str) -> Texture2D {
		Self::load_img(&format!("textures/board/{name}.png"))
	}

	fn load_img(path: &str) -> Texture2D {
		let tnf_file: &File<'_> = ASSETS.get_file("textures/tnf.png").expect("Can't find TNF");

		Texture2D::from_image(
			&Image::from_file_with_format(
				ASSETS
					.get_file(path)
					.unwrap_or_else(|| -> &File<'_> {
						eprintln!("{}", format!("Can't find {path}").red());
						tnf_file
					})
					.contents(),
				Some(ImageFormat::Png),
			)
			.unwrap_or_else(|e| -> Image {
				eprintln!(
					"{}",
					format!("Can't read contents of {path}, with error {e}").red()
				);
				Image::from_file_with_format(tnf_file.contents(), Some(ImageFormat::Png))
					.unwrap_or_else(|e| -> Image { panic!("Can't decode textures/tnf.png with {e}") })
			}),
		)
	}

	fn render(&self, texture: &Texture, board: [[Piece; 8]; 8], win_size: (i32, i32)) {
		clear_background(SKYBLUE);

		let square_size: i32 = if win_size.0 < win_size.1 {
			win_size.0
		} else {
			win_size.1
		} / 8;
		let tl: (i32, i32) = (
			win_size.0 / 2 - 4 * square_size,
			win_size.1 / 2 - 4 * square_size,
		);

		for (x, y) in (0..8).flat_map(|y: i32| (0..8).map(move |x: i32| -> (i32, i32) { (x, y) })) {
			draw_texture_ex(
				if (x + y) % 2 == 0 {
					&texture.BoardLight
				} else {
					&texture.BoardDark
				},
				(tl.0 + x * square_size) as f32,
				(tl.1 + y * square_size) as f32,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(square_size as f32, square_size as f32)),
					source: None,
					rotation: 0.0,
					flip_x: false,
					flip_y: false,
					pivot: None,
				},
			);
		}

		for (x, y) in (0..8).flat_map(|y: i32| (0..8).map(move |x: i32| -> (i32, i32) { (x, y) })) {
			draw_texture_ex(
				match board[y as usize][x as usize] {
					Piece::k => &texture.k,
					Piece::q => &texture.q,
					Piece::r => &texture.r,
					Piece::n => &texture.n,
					Piece::b => &texture.b,
					Piece::p => &texture.p,
					Piece::K => &texture.K,
					Piece::Q => &texture.Q,
					Piece::R => &texture.R,
					Piece::N => &texture.N,
					Piece::B => &texture.B,
					Piece::P => &texture.P,
					Piece::None => continue,
				},
				(tl.0 + x * square_size) as f32,
				(tl.1 + y * square_size) as f32,
				WHITE,
				DrawTextureParams {
					dest_size: Some(vec2(square_size as f32, square_size as f32)),
					source: None,
					rotation: 0.0,
					flip_x: false,
					flip_y: false,
					pivot: None,
				},
			);
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
	let texture: Texture = Texture::new();

	let enigo: Enigo = Enigo::new(&Settings::default()).unwrap();
	let win_size: (i32, i32) = enigo.main_display().unwrap_or((1920, 1080));

	loop {
		if is_key_down(KeyCode::Escape) {
			break;
		}

		texture.render(&texture, game.board, win_size);

		next_frame().await;
	}
}
