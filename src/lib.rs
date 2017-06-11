//! This crate provides various algorithms for bin packing axis aligned rectangles.
//!
//! The most common use case for this library is for games. In order to reduce texture swapping on
//! the GPU, multiple textures are combined into fewer, larger textures.
//!
//! # Features
//!
//! So far, only basic texture atlas generating features are supported. All atlas generation is done
//! with a simple `AtlasRect` trait that must be implemented on whatever you wish to generate an
//! atlas for. For convenience, this trait is pre-implemented for the `image` crate's
//! `DynamicImage`.
//!
//! ## Future
//!
//! This is a list of tasks that will be done in the future sorted by importance:
//! - Rotatable rects.
//! - Improve `image` integration.
//! - Improve tests.
//! - Add basic CLI example tool.
//! - Add "Max Rects" generator.
//! 
//! # Common Usage
//! 
//! This library is intended to be used as a build script. It does not facilitate how data is loaded
//! but users are welcome to create their own on top of this library.

#[cfg(feature = "image")]
extern crate image;

pub mod gen;
pub mod util;

#[cfg(feature = "image")]
use image::{DynamicImage, GenericImage, Rgba};

/// Represents an axis aligned rectangle to be packed in a bin.
pub trait AtlasRect
{
	/// The width size dimension of the rectangle.
	fn width(&self) -> u32;

	/// The height size dimension of the rectangle.
	fn height(&self) -> u32;
}

impl AtlasRect
{
	/// Returns the total number of pixels this rectangle takes up.
	pub fn size(&self) -> u64
	{
		self.width() as u64 * self.height() as u64
	}
}

/// References an axis aligned rect placed in a bin.
pub struct AtlasReference
{
	/// The index of the original rect list that this class references.
	pub rect_index: usize,

	/// The x-position where this rect is located in the bin.
	pub x: u32,

	/// The y-position where this rect is located in the bin.
	pub y: u32,

	/// Whether the rect is rotated 90 degrees clockwise.
	pub rotate: bool,
}

/// A packed bin containing multiple objects.
///
/// The class does not make any guarantees. It is expected that all atlas generators play nicely and
/// conform to all rules. The bin size should be the minimum bounding size, capable of encapsulating
/// all objects. Each object should also not pass through any boundaries and should be disjoint.
///
pub struct AtlasBin
{
	/// The width size dimension of the bin.
	width: u32,

	/// The height size dimension of the bin.
	height: u32,

	/// The list of objects in this bin.
	objects: Vec<AtlasReference>,
}

impl AtlasBin
{
	/// Initializes a new bin with the given rect reference and size.
	pub fn new(rect_index: usize, width: u32, height: u32, rotate: bool) -> Self
	{
		let reference = AtlasReference
		{
			x: 0,
			y: 0,
			rect_index,
			rotate,
		};
		AtlasBin
		{
			width: width,
			height: height,
			objects: vec![reference],
		}
	}

	/// Returns the parts in this bin.
	pub fn parts(&self) -> &[AtlasReference]
	{
		&self.objects
	}

	/// Adds a new rect to the bin. The size of the bin increases if mandatory.
	pub fn parts_add(&mut self, rect_index: usize, x: u32, y: u32, width: u32, height: u32, rotate: bool)
	{
		self.width = std::cmp::max(self.width, x + width);
		self.height = std::cmp::max(self.height, y + height);
		self.objects.push(AtlasReference
		{
			rect_index,
			x,
			y,
			rotate,
		});
	}
}

impl AtlasRect for AtlasBin
{
	fn width(&self) -> u32
	{
		self.width
	}
	fn height(&self) -> u32
	{
		self.height
	}
}

/// Generates a texture atlas.
pub trait AtlasGenerator
{
	/// Generates a list of bins from the given list of objects.
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, width: u32, height: u32, rotate: bool);
}

/// List data structure for adding rects. Tracks total size of all rects.
pub struct AtlasRectList<T: AtlasRect>
{
	rect_list: Vec<T>,
	total_size: u64,
}

impl<T> AtlasRectList<T> where T: AtlasRect
{
	pub fn new() -> Self
	{
		AtlasRectList
		{
			rect_list: Vec::new(),
			total_size: 0,
		}
	}
	pub fn with_capacity(capacity: usize) -> Self
	{
		AtlasRectList
		{
			rect_list: Vec::with_capacity(capacity),
			total_size: 0,
		}
	}
	pub fn add(&mut self, rect: T)
	{
		let size = AtlasRect::width(&rect) as u64 * AtlasRect::height(&rect) as u64;
		self.total_size += size;
		self.rect_list.push(rect);
	}
	pub fn len(&self) -> usize
	{
		self.rect_list.len()
	}
	pub fn build(&self, width: u32, height: u32) -> AtlasBuilder<T>
	{
		let size = width as u64 * height as u64;
		let lower_bound = (size / self.total_size) + 1;
		AtlasBuilder::new(&self.rect_list, width, height, lower_bound as usize)
	}
}

pub struct AtlasBuilder<'a, T> where T: 'a + AtlasRect
{
	rect_list: &'a [T],
	width: u32,
	height: u32,
	lower_bound: usize,
	rotate: bool,
}

impl<'a, T> AtlasBuilder<'a, T> where T: 'a + AtlasRect
{
	fn new(rect_list: &'a [T], width: u32, height: u32, lower_bound: usize) -> Self
	{
		AtlasBuilder
		{
			rect_list,
			width,
			height,
			lower_bound,
			rotate: false,
		}
	}
	pub fn with_rotate<'b>(&'b mut self, rotate: bool) -> &'b mut Self
	{
		self.rotate = rotate;
		self
	}
	pub fn generate<G: AtlasGenerator>(self, generator: &G) -> Atlas<'a, T>
	{
		let mut atlas = Atlas
		{
			rect_list: self.rect_list,
			bin_list: Vec::with_capacity(self.lower_bound),
		};
		generator.generate(&mut atlas, self.width, self.height, false);
		atlas
	}
}

/// Encapsulates axis aligned rectangles and resulting bins.
pub struct Atlas<'a, T: 'a + AtlasRect>
{
	rect_list: &'a [T],
	bin_list: Vec<AtlasBin>,
}

impl<'a, T> Atlas<'a, T> where T: 'a + AtlasRect
{
	pub fn build(rect_list: &'a [T], width: u32, height: u32) -> AtlasBuilder<T>
	{
		AtlasBuilder::new(rect_list, width, height, 1)
	}

	/// Generates bins from the indicated generator using the given objects with the given maximum
	/// bin size constraint.
	pub fn new(rect_list: &'a [T]) -> Self
	{
		Self
		{
			rect_list,
			bin_list: Vec::new(),
		}
	}

	/// Returns the amount of axis aligned rectangles that will be binned.
	pub fn rect_count(&self) -> usize
	{
		self.rect_list.len()
	}

	/// Returns the axis aligned rectangle at the given index.
	pub fn rect(&self, index: usize) -> &AtlasRect
	{
		&self.rect_list[index]
	}

	pub fn bin_list(&self) -> &[AtlasBin]
	{
		&self.bin_list
	}

	/// Returns the total amount of bins that have been generated.
	pub fn bin_count(&self) -> usize
	{
		self.bin_list.len()
	}

	/// Creates a new bin with the given rect at the top left.
	pub fn bin_add_new(&mut self, rect_index: usize, rotate: bool)
	{
		let rect = &self.rect_list[rect_index];
		self.bin_list.push(AtlasBin::new(rect_index, rect.width(), rect.height(), rotate));
	}

	/// Adds a new rect to the indicated bin.
	pub fn bin_add_rect(&mut self, bin_index: usize, rect_index: usize, x: u32, y: u32, rotate: bool)
	{
		let rect = &self.rect_list[rect_index];
		self.bin_list[bin_index].parts_add(rect_index, x, y, rect.width(), rect.height(), rotate);
	}

	#[cfg(feature = "image")]
	/// Generates an image from the indicated bin with uniformly separated colors.
	pub fn bin_as_colors(&self, bin_index: usize) -> DynamicImage
	{
		let weight = util::colors_weight(self.rect_list.len());
		util::colors_from_bin(weight, self.rect_list, &self.bin_list[bin_index])
	}

	#[cfg(feature = "image")]
	/// Generates images from the generated bins with uniformly separated colors.
	pub fn as_colors(&self) -> Vec<DynamicImage>
	{
		let weight = util::colors_weight(self.rect_list.len());
		let mut image_list = Vec::with_capacity(self.rect_list.len());

		for bin in &self.bin_list
		{
			image_list.push(util::colors_from_bin(weight, self.rect_list, bin));
		}
		image_list
	}
}

#[cfg(feature = "image")]
impl<'a, T> Atlas<'a, T> where T: 'a + AtlasRect + GenericImage<Pixel=Rgba<u8>>
{
	/// Returns the given bin as an image.
	pub fn bin_as_image(&self, bin_index: usize) -> DynamicImage
	{
		util::image_from_bin(self.rect_list, &self.bin_list[bin_index])
	}

	/// Generates images from the generated bin using the given image objects.
	pub fn as_images(&self) -> Vec<DynamicImage>
	{
		let mut image_list = Vec::with_capacity(self.rect_list.len());

		for bin in &self.bin_list
		{
			image_list.push(util::image_from_bin(self.rect_list, bin));
		}
		image_list
	}
}
