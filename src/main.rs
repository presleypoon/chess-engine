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
	macroquad::texture::set_default_filter_mode(macroquad::texture::FilterMode::Nearest);

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

	let mut click_block: Option<(u8, u8)> = None;

	loop {
		if is_key_down(KeyCode::Escape) {
			break;
		}

		let (mouse_x, mouse_y) = mouse_position();

		let mouse_block_pos: Option<(u8, u8)> = if mouse_x > tl.0 as f32
			&& mouse_x < (tl.0 + 8 * square_size) as f32
			&& mouse_y > tl.1 as f32
			&& mouse_y < (tl.1 + 8 * square_size) as f32
		{
			let del_x: i32 = mouse_x as i32 - tl.0;
			let del_y: i32 = mouse_y as i32 - tl.1;

			let block_x: i32 = del_x / square_size;
			let block_y: i32 = del_y / square_size;

			Some((block_x as u8, block_y as u8))
		} else {
			None
		};

		if is_mouse_button_down(MouseButton::Left) {
			click_block = mouse_block_pos;
		}

		click_block = if is_mouse_button_down(MouseButton::Left) {
			mouse_block_pos
		} else {
			click_block
		};
		let sel_block: Option<(u8, u8)> =
			click_block.filter(|&(x, y)| -> bool { game.board[y as usize][x as usize] != Piece::None });
		let highlight_block: Option<(u8, u8)> = sel_block.or_else(|| -> Option<(u8, u8)> {
			if is_mouse_button_down(MouseButton::Left) {
				click_block
			} else {
				None
			}
		});

		texture.render(game.board, square_size, tl, highlight_block);
		next_frame().await;
	}
}
