mod game;
use game::*;
mod texture;
use texture::*;
mod render;

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

	loop {
		if is_key_down(KeyCode::Escape) {
			break;
		}

		texture.render(&texture, game.board, win_size);

		next_frame().await;
	}
}
