mod binary_tree;

use {Atlas, AtlasGenerator, AtlasRect};

pub use self::binary_tree::BinaryTreeGenerator;

/// A generator that creates a separate bin for each object.
pub struct PassthroughGenerator;

impl AtlasGenerator for PassthroughGenerator
{
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, _: u32, _: u32)
	{
		for rect_index in 0..atlas.rect_count()
		{
			atlas.bin_add_new(rect_index);
		}
	}
}
