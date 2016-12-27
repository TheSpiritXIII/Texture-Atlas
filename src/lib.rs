/*!
This crate provides various algorithms for bin packing.

The most common use case is for games. In order to reduce texture swapping on the GPU, multiple
textures can be combined into a single, or fewer, larger textures.

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

use image::{DynamicImage, GenericImage, Rgba};

/// Represents a rectangular object to be packed in a bin.
pub trait AtlasObject {
	/// The width size dimension of the object.
	fn width(&self) -> usize;

	/// The height size dimension of the object.
	fn height(&self) -> usize;
}

impl AtlasObject for DynamicImage {
	fn width(&self) -> usize {
		GenericImage::width(self) as usize
	}
	fn height(&self) -> usize {
		GenericImage::height(self) as usize
	}
}

/// References an object placed in a bin.
pub struct AtlasReference {
	/// The index of the original object list that this class references.
	pub object_index: usize,
	/// The x-position where this object is located in the bin.
	pub x: usize,
	/// The y-position where this object is located in the bin.
	pub y: usize,
}

/// A packed bin containing multiple objects.
///
/// The class does not make any guarantees. It is expected that all atlas generators play nicely and
/// conform to all rules. The bin size should be the minimum bounding size, capable of encapsulating
/// all objects. Each object should also not pass through any boundaries and should be disjoint.
///
pub struct AtlasBin {
	/// The width size dimension of the bin.
	pub width: usize,
	/// The height size dimension of the bin.
	pub height: usize,
	/// The list of objects in this bin.
	pub objects: Vec<AtlasReference>,
}

impl AtlasBin {
	/// Initializes a new bin with the given part and size.
	pub fn with_part(reference: AtlasReference, width: usize, height: usize) -> Self {
		AtlasBin {
			width: width,
			height: height,
			objects: vec![reference],
		}
	}
}

/// Generates a texture atlas.
pub trait AtlasGenerator {
	/// Generates a list of bins given a list of objects.
	fn generate_atlas<T: AtlasObject>(
		object_list: &[T], max_width: usize, max_height: usize
	) -> Vec<AtlasBin>;
}

/// Encapsulates objects and resulting bins.
pub struct Atlas<'a, T: 'a + AtlasObject> {
	object_list: &'a [T],
	atlas_list: Vec<AtlasBin>,
}

impl<'a, T> Atlas<'a, T> where T: 'a + AtlasObject {
	/// Generates bins from the indicated generator using the given objects.
	pub fn new<G: AtlasGenerator>(object_list: &'a [T]) -> Self {
		Self::with_max::<G>(object_list, usize::max_value(), usize::max_value())
	}

	/// Generates bins from the indicated generator using the given objects with the given maximum
	/// bin size constraint.
	pub fn with_max<G: AtlasGenerator>(
		object_list: &'a [T], max_width: usize, max_height: usize
	) -> Self {
		let atlas_list = G::generate_atlas(object_list, max_width, max_height);
		Atlas {
			object_list: object_list,
			atlas_list: atlas_list,
		}
	}

	/// Generates images from the generated bin with uniformly separated colors.
	pub fn as_colored_images(&self) -> Vec<DynamicImage> {
		util::create_images_colored(self.object_list, &self.atlas_list)
	}
}

impl<'a, T> Atlas<'a, T> where T: 'a + AtlasObject + GenericImage<Pixel=Rgba<u8>> {
	/// Generates images from the generated bin using the given image objects.
	pub fn as_images(&self) -> Vec<DynamicImage> {
		util::create_images(self.object_list, &self.atlas_list)
	}
}
