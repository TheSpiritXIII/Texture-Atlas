/*!
This crate provides various algorithms for bin packing.

The most common use case is for games. In order to reduce texture swapping on the GPU, multiple
sprites can be combined into a much larger texture.

# Common Usage

This library is intended to be used as a build script. It does not facilitate any serialized formats
but users are welcome to create their own on top of this library.

To create a build script, create a file named `build.rs` and reference it in your `Cargo.toml`.

```toml
[package]
build = build.rs
```
*/
extern crate image;

pub mod generator;

mod util;

use image::DynamicImage;

/// Represents a rectangular object to be placed in a bin.
pub struct Rect {
	/// The width size dimension of the bin.
	pub width: usize,
	/// The height size dimension of the bin.
	pub height: usize,
}

/// References an object placed in a bin.
pub struct AtlasObject {
	/// The index of the original bin list that this object corresponds to.
	pub bin_index: usize,
	/// The x-position where this object is located on the page.
	pub x: usize,
	/// The y-position where this object is located on the page.
	pub y: usize,
}

/// A packed bin containing multiple objects.
pub struct AtlasBin {
	/// The dimension of the page.
	///
	/// Expected to be the minimum bounding size, capable of encapsulating all object parts.
	///
	pub rect: Rect,

	/// The list of objects parts of this bin.
	///
	/// This class does not validate whether objects are disjoint or within the bounds of the size
	/// dimensions. Atlas generators should track this information themselves.
	///
	pub parts: Vec<AtlasObject>,
}

impl AtlasBin {
	/// Initializes a new bin with the given part and size.
	pub fn with_part(object: AtlasObject, width: usize, height: usize) -> Self {
		AtlasBin {
			rect: Rect {
				width: width,
				height: height,
			},
			parts: vec![object],
		}
	}
}

/// Generates a texture atlas.
pub trait AtlasGenerator {
	/// Generates a list of atlas bins given a list of bins.
	fn generate_atlas(bin_list: &[Rect], max_width: usize, max_height: usize) -> Vec<AtlasBin>;
}

/// Encapsulates bins and their resulting atlas pages.
pub struct Atlas<'a> {
	bin_list: &'a [Rect],
	atlas_list: Vec<AtlasBin>,
}

impl<'a> Atlas<'a> {
	/// Generates atlas pages from the indicated generator using the given bins.
	pub fn new<T: AtlasGenerator>(bin_list: &'a [Rect]) -> Self {
		Self::with_max::<T>(bin_list, usize::max_value(), usize::max_value())
	}

	/// Generates an atlas with the given maximum bin size constraint.
	pub fn with_max<T: AtlasGenerator>(bin_list: &'a [Rect], max_width: usize, max_height: usize) -> Self {
		let atlas_list = T::generate_atlas(bin_list, max_width, max_height);
		Atlas {
			bin_list: bin_list,
			atlas_list: atlas_list,
		}
	}

	/// Generates images from the atlas pages.
	pub fn as_images(&self) -> Vec<DynamicImage> {
		util::create_images_colored(self.bin_list, &self.atlas_list)
	}
}
