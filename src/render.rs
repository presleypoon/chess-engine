use crate::{game::Piece, texture::*};
use macroquad::prelude::*;

impl Texture {
	pub fn render(
		&self,
		board: [[Piece; 8]; 8],
		square_size: i32,
		tl: (i32, i32),
		highlight_block: Option<(u8, u8)>
	) {
		clear_background(SKYBLUE);

		for (x, y) in (0..8).flat_map(|y: u8| (0..8).map(move |x: u8| -> (u8, u8) { (x, y) })) {
			draw_texture_ex(
				if Some((x, y)) == highlight_block {
					&self.BoardSel
				} else if (x + y) % 2 == 0 {
					&self.BoardLight
				} else {
					&self.BoardDark
				},
				(tl.0 + x as i32 * square_size) as f32,
				(tl.1 + y as i32 * square_size) as f32,
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
					Piece::k => &self.k,
					Piece::q => &self.q,
					Piece::r => &self.r,
					Piece::n => &self.n,
					Piece::b => &self.b,
					Piece::p => &self.p,
					Piece::K => &self.K,
					Piece::Q => &self.Q,
					Piece::R => &self.R,
					Piece::N => &self.N,
					Piece::B => &self.B,
					Piece::P => &self.P,
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
