extern crate texture_atlas;

use texture_atlas::{Atlas, AtlasBin, AtlasGenerator, AtlasRect};
use texture_atlas::gen::{BinaryTreeGenerator, PassthroughGenerator};
use texture_atlas::util::Rect;

#[derive(Eq, PartialEq, PartialOrd)]
struct SweepPart
{
	value: u32,
	rect_index: usize,
	start: bool,
}

impl SweepPart
{
	fn new(rect_index: usize, start: bool, value: u32) -> Self
	{
		SweepPart
		{
			value: value,
			rect_index: rect_index,
			start: start,
		}
	}
}

impl std::cmp::Ord for SweepPart
{
	fn cmp(&self, other: &Self) -> std::cmp::Ordering
	{
		let value = self.value.cmp(&other.value);
		if value == std::cmp::Ordering::Equal
		{
			let rect_index = self.rect_index.cmp(&other.rect_index);
			if rect_index == std::cmp::Ordering::Equal
			{
				if self.start
				{
					std::cmp::Ordering::Less
				}
				else
				{
					std::cmp::Ordering::Greater
				}
			}
			else
			{
				rect_index
			}
		}
		else
		{
			value
		}
	}
}

use std::collections::HashSet;

fn has_overlaps<T: AtlasRect>(atlas: &Atlas<T>, bin: &AtlasBin) -> bool
{
	// Sweep horizontally.
	let mut sweep_list = Vec::with_capacity(bin.parts().len());
	for part in bin.parts()
	{
		let height = atlas.rect(part.rect_index).height();
		sweep_list.push(SweepPart::new(part.rect_index, true, part.y));
		sweep_list.push(SweepPart::new(part.rect_index, false, part.y + height));
	}

	// Inner set contains vertical points from the current set.
	let mut inner_set = HashSet::with_capacity(2);

	sweep_list.sort();
	for sweep in sweep_list
	{
		if sweep.start
		{
			let width = atlas.rect(sweep.rect_index).width();
			inner_set.insert((sweep.value, sweep.value + width));
		}
		else
		{
			// TODO.
			let width = atlas.rect(sweep.rect_index).width();
			inner_set.remove(&(sweep.value, sweep.value + width));
		}
	}
	false
}

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
		if has_overlaps(atlas, bin)
		{
			panic!("Bin has overlapping rect");
		}
	}
	if counter != 0
	{
		panic!("Atlas does not include all rects");
	}
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
	let atlas = Atlas::build(&list_empty, ATLAS_WIDTH, ATLAS_HEIGHT).generate(generator);
	assert_eq!(atlas.bin_count(), 0);

	// Single item rect list should always generate one bin.
	let list_single = vec![rect_large];
	let atlas = Atlas::build(&list_single, ATLAS_WIDTH, ATLAS_HEIGHT).generate(generator);
	assert_eq!(atlas.bin_count(), 1);

	// Having two large items means you cannot fit everything, so two bins.
	let list_large = vec![rect_large, rect_large];
	let atlas = Atlas::build(&list_large, ATLAS_WIDTH, ATLAS_HEIGHT).generate(generator);
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
