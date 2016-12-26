use image::{DynamicImage, GenericImage, Pixel, Rgb};

use {AtlasBin, Rect};

#[derive(Debug)]
struct Hsv {
	data: [u8; 3],
}

impl Hsv {
	fn to_rgb(&self) -> Rgb<u8> {
		let sat = self.data[1] as f32 / u8::max_value() as f32;
		let val = self.data[2] as f32 / u8::max_value() as f32;

		let chroma = val * sat;
		let h_prime = self.data[0] as f32 / u8::max_value() as f32 * (359.0 / 60.0);
		let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());

		let result: [f32; 3] = match h_prime as isize {
			0 => [chroma, x, 0.0],
			1 => [x, chroma, 0.0],
			2 => [0.0, chroma, x],
			3 => [0.0, x, chroma],
			4 => [x, 0.0, chroma],
			5 => [chroma, 0.0, x],
			_ => [0.0, 0.0, 0.0],
		};

		let m = val - chroma;
		Rgb::<u8> {
			data: [
				((result[0] + m) * u8::max_value() as f32) as u8,
				((result[1] + m) * u8::max_value() as f32) as u8,
				((result[2] + m) * u8::max_value() as f32) as u8,
			],
		}
	}
}

pub fn create_images_colored(bin_list: &[Rect], atlas_list: &[AtlasBin]) -> Vec<DynamicImage> {
	let mut image_list = Vec::with_capacity(bin_list.len());

	let color_distance = u8::max_value() / bin_list.len() as u8;
	let mut color_current = Hsv { data: [0, 255, 255] };

	for atlas_page in atlas_list {
		let (width, height) = (atlas_page.rect.width as u32, atlas_page.rect.height as u32);
		let mut image = DynamicImage::new_rgba8(width, height);
		color_current.data[0] += color_distance;

		for part in &atlas_page.parts {
			let bin = &bin_list[part.bin_index];
			for x in part.x..(part.x + bin.width) {
				for y in part.y..(part.y + bin.height) {
					image.put_pixel(x as u32, y as u32, color_current.to_rgb().to_rgba());
				}
			}
		}
		image_list.push(image);
	}
	image_list
}
