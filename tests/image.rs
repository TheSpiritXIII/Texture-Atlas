extern crate image;
extern crate texture_atlas;

use image::{DynamicImage, GenericImage, Rgba};
use texture_atlas::Atlas;
use texture_atlas::util::Rect;

fn image_equal(image1: DynamicImage, image2: DynamicImage) -> bool
{
	if image1.dimensions() == image2.dimensions()
	{
		for x in 0..image1.width()
		{
			for y in 0..image1.height()
			{
				if image1.get_pixel(x, y) != image2.get_pixel(x, y)
				{
					return false;
				}
			}
		}
		true
	}
	else
	{
		false
	}
}

#[test]
fn image_single()
{
	let rect_list = vec![Rect::new(256, 128)];
	let mut atlas = Atlas::new(&rect_list);
	atlas.bin_add_new(0, false);

	// TODO.
}

#[test]
fn image_single_rotated()
{
	// Make sure the image does not have equal sides.
	const IMAGE_WIDTH: u32 = 256;
	const IMAGE_HEIGHT: u32 = 128;
	let mut image = DynamicImage::new_luma8(IMAGE_WIDTH, IMAGE_HEIGHT);

	// Make sure the image is asymmetric.
	let pixel = Rgba::<u8> { data: [255, 255, 255, 255] };
	for x in 0..IMAGE_WIDTH
	{
		// Creates an image with the following lines:
		// - Diagonal line going from the top-left to the bottom-right.
		let percentage = x as f32 / IMAGE_WIDTH as f32;
		let y = (percentage * IMAGE_HEIGHT as f32) as u32;
		image.put_pixel(x, y, pixel);

		// - Horizontal line going through the center.
		image.put_pixel(x, IMAGE_HEIGHT / 2, pixel);
	}
	let flipped = image.rotate90();

	let rect_list = vec![image];
	let mut atlas = Atlas::new(&rect_list);
	atlas.bin_add_new(0, true);
	assert!(image_equal(atlas.bin_as_image(0), flipped));
}

#[test]
fn image_spread()
{
	// 5 images. 3 images in 1 bin, 2 in another.
}
