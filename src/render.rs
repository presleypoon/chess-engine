use crate::{game::Piece, texture::*};
use macroquad::prelude::*;

impl Texture {
	pub fn render(
		&self,
		texture: &Texture,
		board: [[Piece; 8]; 8],
		square_size: i32,
		tl: (i32, i32),
	) -> Option<(u8, u8)> {
		clear_background(SKYBLUE);

		let mouse_pos: (f32, f32) = mouse_position();
		let (mouse_x, mouse_y) = mouse_pos;

		let ret: Option<(u8, u8)> = if mouse_x > tl.0 as f32
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

		for (x, y) in (0..8).flat_map(|y: u8| (0..8).map(move |x: u8| -> (u8, u8) { (x, y) })) {
			draw_texture_ex(
				if (x, y) == ret.unwrap_or((255, 255)) {
					&texture.BoardSel
				} else if (x + y) % 2 == 0 {
					&texture.BoardLight
				} else {
					&texture.BoardDark
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

		ret
	}
}
