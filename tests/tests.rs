extern crate texture_atlas;

use texture_atlas::{Atlas, AtlasGenerator};
use texture_atlas::gen::{BinaryTreeGenerator, PassthroughGenerator};
use texture_atlas::util::Rect;

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
}
