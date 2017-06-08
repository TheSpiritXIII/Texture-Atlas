extern crate texture_atlas;

use texture_atlas::{Atlas, AtlasGenerator, AtlasRect};
use texture_atlas::gen::{BinaryTreeGenerator, PassthroughGenerator};
use texture_atlas::util::Rect;

// TODO: Consider using this function in the lib by default.
fn smoke_atlas<T: AtlasRect>(atlas: &Atlas<T>)
{
	// If the rect generates more bins than rects, something is wrong.
	assert!(atlas.bin_count() <= atlas.rect_count());
	atlas.bin_list();

	// Make sure no bins repeat textures.
	let mut counter = atlas.bin_list().len();
	let mut checker = vec![false; counter];
	for bin in atlas.bin_list()
	{
		for part in bin.parts()
		{
			let rect_index = part.rect_index;
			if checker[rect_index]
			{
				panic!("Bin contains rect already in another bin");
			}
			checker[rect_index] = true;
			counter -= 1;
		}
	}
	if counter != 0
	{
		panic!("Atlas does not include all rects");
	}

	// TODO: Make sure atlases do not overlap rects.
}

fn smoke<T: AtlasGenerator>(generator: &T)
{
	const ATLAS_WIDTH: u32 = 256;
	const ATLAS_HEIGHT: u32 = 128;

	const RECT_LARGE_WIDTH: u32 = 192;
	const RECT_LARGE_HEIGHT: u32 = 96;

	let rect_large = Rect::new(RECT_LARGE_WIDTH, RECT_LARGE_HEIGHT);

	// Empty rect list should generate no bins.
	let list_empty: Vec<Rect> = Vec::new();
	let atlas = Atlas::with_max(generator, &list_empty, ATLAS_WIDTH, ATLAS_HEIGHT);
	assert_eq!(atlas.bin_count(), 0);

	// Single item rect list should always generate one bin.
	let list_single = vec![rect_large];
	let atlas = Atlas::with_max(generator, &list_single, ATLAS_WIDTH, ATLAS_HEIGHT);
	assert_eq!(atlas.bin_count(), 1);

	// Having two large items means you cannot fit everything, so two bins.
	let list_large = vec![rect_large, rect_large];
	let atlas = Atlas::with_max(generator, &list_large, ATLAS_WIDTH, ATLAS_HEIGHT);
	assert_eq!(atlas.bin_count(), 2);
	smoke_atlas(&atlas);
}

#[test]
fn test_passthrough()
{
	smoke(&PassthroughGenerator);
}

#[test]
fn test_binary_tree()
{
	smoke(&BinaryTreeGenerator);

	// TODO: Better tests specific to this generator.
}
