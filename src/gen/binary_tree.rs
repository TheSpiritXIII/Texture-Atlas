use std::cmp::Ordering;

use {Atlas, AtlasGenerator, AtlasRect};

pub fn sort_by_longest_width_increasing<T: AtlasRect>(atlas: &Atlas<T>) -> Vec<usize>
{
	let mut sorted_list: Vec<usize> = (0..atlas.rect_count()).collect();
	sorted_list.sort_by(|index_left, index_right|
	{
		let rect_right = atlas.rect(*index_right);
		let rect_left = atlas.rect(*index_left);
		let height_cmp = rect_right.height().cmp(&rect_left.height());
		if height_cmp == Ordering::Equal
		{
			rect_right.width().cmp(&rect_left.width())
		}
		else
		{
			height_cmp
		}
	});
	sorted_list
}

#[derive(Debug, Clone, Copy)]
struct Rect
{
	bin: usize,
	x: u32,
	y: u32,
	width: u32,
	height: u32,
}

impl Rect
{
	fn new(bin: usize, x: u32, y: u32, width: u32, height: u32) -> Self
	{
		Rect
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
	fn subdivide(leaves: &mut Vec<Rect>, leaf_index: usize, width: u32, height: u32)
	{
		let leaf = leaves[leaf_index];

		// Sub-divide so that bottom has smaller portion.
		let leaf_left = Rect::new(leaf.bin, leaf.x + width, leaf.y, leaf.width - width, height);
		let leaf_bottom = Rect::new(leaf.bin, leaf.x, leaf.y + height, leaf.width, leaf.height - height);

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
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, width: u32, height: u32, _: bool)
	{
		let atlas_list = sort_by_longest_width_increasing(atlas);
		let mut atlas_index = 0;

		// All we really care about are leaves of trees. Don't use an actual binary tree.
		let mut leaves: Vec<Rect> = Vec::with_capacity(2);
		let mut max = 0;
		while atlas_index != atlas_list.len()
		{
			let rect_index = atlas_list[atlas_index];
			let mut inserted = false;
			for leaf_index in 0..leaves.len()
			{
				let leaf = leaves[leaf_index];
				let width = atlas.rect(rect_index).width();
				let height = atlas.rect(rect_index).height();
				if width <= leaf.width && height <= leaf.height
				{
					BinaryTreeGenerator::subdivide(&mut leaves, leaf_index, width, height);
					atlas.bin_add_rect(leaf.bin, rect_index, leaf.x, leaf.y, false);

					inserted = true;
					break;
				}
			}
			if !inserted
			{
				let bin = atlas.bin_count();
				atlas.bin_add_new(rect_index, false);

				let leaf_index = leaves.len();
				leaves.push(Rect::new(bin, 0, 0, width, height));

				let rect = atlas.rect(rect_index);
				let width = rect.width();
				let height = rect.height();

				BinaryTreeGenerator::subdivide(&mut leaves, leaf_index, width, height);
			}
			max = ::std::cmp::max(max, leaves.len());
			atlas_index += 1;
		}
	}
}
