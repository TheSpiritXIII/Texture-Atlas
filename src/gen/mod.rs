//! Bin packing algorithm generator implementations.

mod binary_tree;

use {Atlas, AtlasGenerator, AtlasRect};

pub use self::binary_tree::BinaryTreeGenerator;

/// A generator that creates a separate bin for each object.
pub struct PassthroughGenerator;

impl AtlasGenerator for PassthroughGenerator
{
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, _: u32, _: u32, _: bool)
	{
		for rect_index in 0..atlas.rect_list().len()
		{
			atlas.bin_add_new(rect_index, false);
		}
	}
}
