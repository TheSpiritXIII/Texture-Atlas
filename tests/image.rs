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
	const IMAGE_SIZE: u32 = 256;
	let mut image = DynamicImage::new_luma8(IMAGE_SIZE, IMAGE_SIZE);
	let pixel = Rgba::<u8> { data: [255, 255, 255, 255] };
	for index in 0..IMAGE_SIZE
	{
		image.put_pixel(index, index, pixel);
		image.put_pixel(128, index, pixel);
	}
	let flipped = image.rotate90();
	let ref mut file = std::fs::File::create(&std::path::Path::new(format!("output/before.png").as_str())).unwrap();
	let _ = image.save(file, image::PNG).unwrap();
	let ref mut file = std::fs::File::create(&std::path::Path::new(format!("output/expect.png").as_str())).unwrap();
	let _ = flipped.save(file, image::PNG).unwrap();

	let rect_list = vec![image];
	let mut atlas = Atlas::new(&rect_list);
	atlas.bin_add_new(0, true);
	let ref mut file = std::fs::File::create(&std::path::Path::new(format!("output/after.png").as_str())).unwrap();
	let _ = atlas.bin_as_image(0).save(file, image::PNG).unwrap();
	assert!(image_equal(atlas.bin_as_image(0), flipped));
}

#[test]
fn image_spread()
{
	// 5 images. 3 images in 1 bin, 2 in another.
}
