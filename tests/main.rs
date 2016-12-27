extern crate texture_atlas;

use texture_atlas::{Atlas, AtlasObject};
use texture_atlas::generator::PassthroughGenerator;

struct Rect {
	pub width: usize,
	pub height: usize,
}

impl AtlasObject for Rect {
	fn width(&self) -> usize {
		self.width
	}
	fn height(&self) -> usize {
		self.width
	}
}

#[test]
fn passthrough_generator() {
	let object_list = [
		Rect { width: 32, height: 32 },
		Rect { width: 64, height: 64 },
		Rect { width: 16, height: 16 },
	];

	let bin_list = {
		let atlas = Atlas::new::<PassthroughGenerator>(&object_list);
		atlas.as_colored_images()
	};
	assert_eq!(bin_list.len(), object_list.len());
}
