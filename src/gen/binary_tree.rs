use std::cmp::Ordering;

use {Atlas, AtlasGenerator, AtlasRect};
use util::{Rect, RotatableRect};

#[derive(Debug, Clone, Copy)]
struct RectReference<T: AsRef<Rect>>
{
	index: usize,
	rect: T,
}

impl<T> AsRef<Rect> for RectReference<T> where T: AsRef<Rect>
{
	fn as_ref(&self) -> &Rect
	{
		self.rect.as_ref()
	}
}

fn sort_by_longest_width_increasing<T: AtlasRect>(atlas: &Atlas<T>, rotate: bool) -> Vec<RectReference<RotatableRect>>
{
	let mut rect_list = Vec::with_capacity(atlas.rect_count());
	for (index, rect) in atlas.rect_list().iter().enumerate()
	{
		rect_list.push(RectReference
		{
			index,
			rect: (rect as &AtlasRect).dimensions_longest_rotated(rotate),
		})
	}
	rect_list.sort_by(|ref_left, ref_right|
	{
		let height_cmp = ref_right.rect.height().cmp(&ref_left.rect.height());
		if height_cmp == Ordering::Equal
		{
			ref_right.rect.width().cmp(&ref_left.rect.width())
		}
		else
		{
			height_cmp
		}
	});
	rect_list
}

#[derive(Debug, Clone, Copy)]
struct Rectr
{
	bin: usize,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
}

impl Rectr
{
	fn new(bin: usize, x: u32, y: u32, width: u32, height: u32) -> Self
	{
		Rectr
		{
			bin: bin,
			x: x,
			y: y,
			width: width,
			height: height,
		}
	}
	fn empty(&self) -> bool
	{
		self.width == 0 || self.height == 0
	}
}

/// A generator that uses binary trees to generate an atlas.
///
/// This generator is useful for when decent results are needed but good speed is required. This
/// algorithm works well when rects are uniformly sized, otherwise it will leave a lot of gaps.
///
pub struct BinaryTreeGenerator;

impl BinaryTreeGenerator
{
	fn subdivide(leaves: &mut Vec<Rectr>, leaf_index: usize, width: u32, height: u32)
	{
		let leaf = leaves[leaf_index];

		// Sub-divide so that bottom has smaller portion.
		let leaf_left = Rectr::new(leaf.bin, leaf.x + width, leaf.y, leaf.width - width, height);
		let leaf_bottom = Rectr::new(leaf.bin, leaf.x, leaf.y + height, leaf.width, leaf.height - height);

		if !leaf_left.empty() && !leaf_bottom.empty()
		{
			// Include both leaves.
			leaves[leaf_index] = leaf_left;
			leaves.insert(leaf_index + 1, leaf_bottom);
		}
		else if !leaf_left.empty() && leaf_bottom.empty()
		{
			// Takes up entire width, include only left.
			leaves[leaf_index] = leaf_left;
		}
		else if leaf_left.empty() && !leaf_bottom.empty()
		{
			// Takes up entire height, include only bottom.
			leaves[leaf_index] = leaf_bottom;
		}
		else if leaf_left.empty() && leaf_bottom.empty()
		{
			// Takes up entire area, include neither.
			leaves.remove(leaf_index);
		}
	}
}

impl AtlasGenerator for BinaryTreeGenerator
{
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, widthr: u32, heightr: u32, rotate: bool)
	{
		let atlas_list = sort_by_longest_width_increasing(atlas, rotate);
		let mut atlas_index = 0;

		// All we really care about are leaves of trees. Don't use an actual binary tree.
		let mut leaves: Vec<Rectr> = Vec::with_capacity(2);
		let mut max = 0;
		while atlas_index != atlas_list.len()
		{
			let rect_index = atlas_list[atlas_index].index;
			let mut inserted = false;
			for leaf_index in 0..leaves.len()
			{
				let leaf = leaves[leaf_index];
				let dimensions = &atlas_list[atlas_index].rect.rect;
				println!("{:?}", dimensions);
				if dimensions.width <= leaf.width && dimensions.height <= leaf.height
				{
					BinaryTreeGenerator::subdivide(&mut leaves, leaf_index, dimensions.width, dimensions.height);
					atlas.bin_add_rect(leaf.bin, rect_index, leaf.x, leaf.y, atlas_list[atlas_index].rect.rotated);

					inserted = true;
					break;
				}
			}
			if !inserted
			{
				let bin = atlas.bin_count();
				atlas.bin_add_new(rect_index, false);

				let leaf_index = leaves.len();
				leaves.push(Rectr::new(bin, 0, 0, widthr, heightr));

				let rect = atlas.rect(rect_index);
				let dimensions = rect.dimensions();

				BinaryTreeGenerator::subdivide(&mut leaves, leaf_index, dimensions.width, dimensions.height);
			}
			max = ::std::cmp::max(max, leaves.len());
			atlas_index += 1;
		}
	}
}
