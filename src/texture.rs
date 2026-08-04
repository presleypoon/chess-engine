use crate::ASSETS;
use colored::*;
use include_dir::File;
use macroquad::prelude::*;

#[allow(non_snake_case)]
pub struct Texture {
	pub k: Texture2D,
	pub q: Texture2D,
	pub r: Texture2D,
	pub n: Texture2D,
	pub b: Texture2D,
	pub p: Texture2D,
	pub K: Texture2D,
	pub Q: Texture2D,
	pub R: Texture2D,
	pub N: Texture2D,
	pub B: Texture2D,
	pub P: Texture2D,
	pub BoardLight: Texture2D,
	pub BoardDark: Texture2D,
}
impl Texture {
	pub fn new() -> Self {
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
}
