mod game;
mod render;
mod texture;
use game::*;
use texture::*;

use enigo::{Enigo, Mouse, Settings};
use include_dir::{Dir, include_dir};
use macroquad::prelude::*;

static ASSETS: Dir = include_dir!("assets");

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

	let square_size: i32 = if win_size.0 < win_size.1 {
		win_size.0
	} else {
		win_size.1
	} / 128
		* 16;
	let tl: (i32, i32) = (
		win_size.0 / 2 - 4 * square_size,
		win_size.1 / 2 - 4 * square_size,
	);

	loop {
		if is_key_down(KeyCode::Escape) {
			break;
		}

		let mouse_block_pos: Option<(u8, u8)> = texture.render(&texture, game.board, square_size, tl);
		next_frame().await;
	}
}
