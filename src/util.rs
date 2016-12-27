use image::{DynamicImage, GenericImage, Pixel, Rgb, Rgba};

use {AtlasBin, AtlasObject};

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

pub fn create_images_colored<T: AtlasObject>(
	object_list: &[T], bin_list: &[AtlasBin]
) -> Vec<DynamicImage> {
	let mut image_list = Vec::with_capacity(object_list.len());

	let color_distance = u8::max_value() / object_list.len() as u8;
	let mut color_current = Hsv { data: [0, 255, 255] };

	for bin in bin_list {
		let (width, height) = (bin.width as u32, bin.height as u32);
		let mut image = DynamicImage::new_rgba8(width, height);
		color_current.data[0] += color_distance;

		for reference in &bin.objects {
			let object = &object_list[reference.object_index];
			for x in reference.x..(reference.x + object.width()) {
				for y in reference.y..(reference.y + object.height()) {
					image.put_pixel(x as u32, y as u32, color_current.to_rgb().to_rgba());
				}
			}
		}
		image_list.push(image);
	}
	image_list
}

pub fn create_images<T: AtlasObject + GenericImage<Pixel=Rgba<u8>>>(
	object_list: &[T], bin_list: &[AtlasBin]
) -> Vec<DynamicImage> {
	let mut image_list = Vec::with_capacity(object_list.len());

	for bin in bin_list {
		let (width, height) = (bin.width as u32, bin.height as u32);
		let mut image = DynamicImage::new_rgba8(width, height);

		for reference in &bin.objects {
			let texture = &object_list[reference.object_index];
			for x in 0..AtlasObject::width(texture) {
				for y in 0..AtlasObject::width(texture) {
					let pixel = texture.get_pixel(x as u32, y as u32);
					image.put_pixel(x as u32, y as u32, pixel);
				}
			}
		}
		image_list.push(image);
	}
	image_list
}
