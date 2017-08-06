use std::borrow::Borrow;

use image::{DynamicImage, GenericImage, Pixel, Rgb, Rgba};

use ::{AtlasBin, AtlasRect};

impl AtlasRect for DynamicImage
{
	fn width(&self) -> u32
	{
		GenericImage::width(self) as u32
	}
	fn height(&self) -> u32
	{
		GenericImage::height(self) as u32
	}
}

const RGBA_EMPTY: Rgba<u8> = Rgba::<u8>
{
	data: [0, 0, 0, 0],
};

/// Returns the amount of empty space at the left of the given image.
pub fn border_left(image: &DynamicImage) -> u32
{
	for x in 0..GenericImage::width(image)
	{
		for y in 0..GenericImage::height(image)
		{
			if image.get_pixel(x, y) != RGBA_EMPTY
			{
				return x;
			}
		}
	}
	GenericImage::width(image)
}

/// Returns the amount of empty space at the right of the given image.
pub fn border_right(image: &DynamicImage) -> u32
{
	for x in (0..GenericImage::width(image)).rev()
	{
		for y in 0..GenericImage::height(image)
		{
			if image.get_pixel(x, y) != RGBA_EMPTY
			{
				return x;
			}
		}
	}
	0
}

/// Returns the amount of empty space at the top of the given image.
pub fn border_top(image: &DynamicImage) -> u32
{
	for y in 0..GenericImage::height(image)
	{
		for x in 0..GenericImage::width(image)
		{
			if image.get_pixel(x, y) != RGBA_EMPTY
			{
				return y;
			}
		}
	}
	GenericImage::height(image)
}

/// Returns the amount of empty space at the bottom of the given image.
pub fn border_bottom(image: &DynamicImage) -> u32
{
	for y in (0..GenericImage::height(image)).rev()
	{
		for x in 0..GenericImage::width(image)
		{
			if image.get_pixel(x, y) != RGBA_EMPTY
			{
				return y;
			}
		}
	}
	0
}

/// Returns the empty image borders in this order: left, right, top, bottom.
pub fn border(image: &DynamicImage) -> (u32, u32, u32, u32)
{
	(border_left(image), border_right(image), border_top(image), border_bottom(image))
}

/// Crops the given image by removing empty borders.
///
/// Returns a tuple of the cropped image, left, right, top then bottom crop amount.
///
pub fn border_crop(image: &mut DynamicImage) -> (DynamicImage, u32, u32, u32, u32)
{
	let (left, right, top, bottom) = border(image);
	(image.crop(left, top, right - left, bottom - top), left, right, top, bottom)
}

pub(crate) fn image_from_bin<T>(rect_list: &[T], bin: &AtlasBin) -> DynamicImage
	where T: AtlasRect + Borrow<DynamicImage>
{
	let dimensions = (bin as &AtlasRect).dimensions();
	let mut image = DynamicImage::new_rgba8(dimensions.width, dimensions.height);

	for reference in &bin.part_list
	{
		let texture = &rect_list[reference.rect_index];
		if !reference.rotate
		{
			for x in 0..AtlasRect::width(texture) as u32
			{
				for y in 0..AtlasRect::height(texture) as u32
				{
					let pixel = texture.borrow().get_pixel(x, y);
					image.put_pixel(reference.x + x, reference.y + y, pixel);
				}
			}
		}
		else
		{
			for x in 0..AtlasRect::width(texture) as u32
			{
				for y in 0..AtlasRect::height(texture) as u32
				{
					let pixel = texture.borrow().get_pixel(x, y);
					image.put_pixel(reference.x + (AtlasRect::height(texture) - 1 - y), reference.y + x, pixel);
				}
			}
		}
	}
	image
}

#[derive(Debug)]
struct Hsv
{
	data: [u8; 3],
}

impl Hsv
{
	fn to_rgb(&self) -> Rgb<u8>
	{
		let sat = self.data[1] as f32 / u8::max_value() as f32;
		let val = self.data[2] as f32 / u8::max_value() as f32;

		let chroma = val * sat;
		let h_prime = self.data[0] as f32 / u8::max_value() as f32 * (359.0 / 60.0);
		let x = chroma * (1.0 - (h_prime % 2.0 - 1.0).abs());

		let result: [f32; 3] = match h_prime as isize
		{
			0 => [chroma, x, 0.0],
			1 => [x, chroma, 0.0],
			2 => [0.0, chroma, x],
			3 => [0.0, x, chroma],
			4 => [x, 0.0, chroma],
			5 => [chroma, 0.0, x],
			_ => [0.0, 0.0, 0.0],
		};

		let m = val - chroma;
		Rgb::<u8>
		{
			data:
			[
				((result[0] + m) * u8::max_value() as f32) as u8,
				((result[1] + m) * u8::max_value() as f32) as u8,
				((result[2] + m) * u8::max_value() as f32) as u8,
			],
		}
	}
}

pub(crate) fn colors_weight(len: usize) -> f32
{
	(1f32 / len as f32) * 255f32
}

pub(crate) fn colors_from_bin<T>(color_weight: f32, rect_list: &[T], bin: &AtlasBin) -> DynamicImage
	where T: AtlasRect
{
	let mut color_current = Hsv { data: [0, 255, 255] };

	let mut image = DynamicImage::new_rgba8(bin.dimensions.width as u32, bin.dimensions.height as u32);

	for reference in &bin.part_list
	{
		color_current.data[0] = (reference.rect_index as f32 * color_weight) as u8;

		let rotate = reference.rotate;
		let dimensions = (&rect_list[reference.rect_index] as &AtlasRect).dimensions_rotated(rotate);

		for x in reference.x..(reference.x + dimensions.width)
		{
			for y in reference.y..(reference.y + dimensions.height)
			{
				image.put_pixel(x as u32, y as u32, color_current.to_rgb().to_rgba());
			}
		}
	}
	image
}
