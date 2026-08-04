use crate::{game::Piece, texture::*};
use macroquad::prelude::*;

impl Texture {
	pub fn render(&self, texture: &Texture, board: [[Piece; 8]; 8], win_size: (i32, i32)) {
		clear_background(SKYBLUE);

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
