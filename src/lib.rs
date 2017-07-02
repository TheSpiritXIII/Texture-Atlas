//! This crate provides various algorithms for bin packing axis aligned rectangles.
//!
//! The most common use case for this library is for games. In order to reduce texture swapping on
//! the GPU, multiple textures are combined into fewer, larger textures.
//!
//! # Features
//!
//! This crate contains and provides basic tools for building and using bin packing algorithms.
//!
//! The following bin packing algorithms, or generators, are implemented:
//!
//! - `PassthroughGenerator`
//! - `BinaryTreeGenerator`
//!
//! All algorithms are expected to take and respect a size constraint and a flag indicating whether
//! or not to rotate of rects.
//!
//! ## Future
//!
//! This library is currently unstable. This is a list of tasks that will be done in the future
//! sorted by importance:
//!
//! - Improve tests and documentation.
//! - Add basic CLI tool.
//! - Add "Max Rects" generator.
//! - Submit to creates.io.
//! - ABI Stablizaation.
//!
//! # Common Usage
//! 
//! This library is intended to be used as a build script. It does not facilitate how data is loaded
//! but users are welcome to create their own on top of this library.
//!
//! All atlas generation is done with a simple `AtlasRect` trait that must be implemented on
//! whatever you wish to generate an atlas for. For convenience, this trait is pre-implemented for
//! the `image` crate's `DynamicImage` struct and also any struct that implements
//! `AsRef<DynamicImage>`.
//!
//! Before bin packing, you must have an instance of `AtlasBuilder`. There are two ways to achieve
//! that: The first is using `Atlas::build` and passing an array to it. The second is using the
//! provided `AtlasRectList` and its `build` function which calculates a lower bound on the number
//! of bins that will be generated as you add rects to the list.
//!
//! At the heard of `AtlasBuilder` is a `generate` method which takes in an `AtlasGenerator`. The
//! current recommended generator is the `BinaryTreeGenerator`. You can even call generate multiple
//! times on the builder to find the best generator that generates the least amount of bins.
//!
//! After calling this method, you receive an `Atlas` struct which contains your generated bins. If
//! you are using the `image` feature, then you can use `Atlas::as_images` to generate a vector of
//! images corresponding to each generated bin.
//!
//! ## Bins of Bins
//!
//! Occasionally, it is also useful to have certain rects together. For instance, in a game you may
//! have multiple frames for a player walking animation. In this case, if the frames are in
//! different bins, then this will incur a texture swapping overhead.
//!
//! To address these scenarios, you can generate a single bin for each groups of related rects and
//! then pass these bins back into generator. Better support will come for these scenarios shortly.
//!
//! # Creating a Generator
//!
//! To create a new generator, create a struct and implement `AtlasGenerator` for it. The
//! `AtlasGenerator` trait uses dynamic dispatch for instances where a generator can have settings,
//! for instance multiple heuristic options. `PassthroughGenerator` is an example of a minimal
//! generator.
//!
//! # The `image` Feature
//!
//! The `image` feature is turned on by default. To disable it, use the following in your
//! `Cargo.toml`:
//!
//! ```toml
//! [dependencies.texture_atlas]
//! default-features = false
//! ```
//!
//! If you keep it enabled, you can create images for generated atlases and gain access to a few
//! utility functions, such as border cropping.

#[cfg(feature = "image")]
extern crate image;

pub mod gen;
pub mod util;

#[cfg(feature = "image")]
use image::{DynamicImage, GenericImage, Rgba};

use util::{Rect, RotatableRect};

/// Represents an axis aligned rectangle to be packed in a bin.
pub trait AtlasRect
{
	/// The width size dimension of the rectangle.
	fn width(&self) -> u32;

	/// The height size dimension of the rectangle.
	fn height(&self) -> u32;
}

impl<'a> AtlasRect + 'a
{
	/// Returns the total number of pixels this rectangle takes up.
	pub fn area(&self) -> u64
	{
		self.width() as u64 * self.height() as u64
	}

	/// Returns true if this rectangle has an area of 0.
	pub fn empty(&self) -> bool
	{
		self.width() == 0 || self.height() == 0
	}

	/// Returns the dimensions of this rectangle.
	pub fn dimensions(&self) -> Rect
	{
		Rect::new(self.width(), self.height())
	}

	/// Returns the dimensions of this rect with width and height inverted if `rotate` is `true`.
	pub fn dimensions_rotated(&self, rotate: bool) -> Rect
	{
		if !rotate
		{
			self.dimensions()
		}
		else
		{
			Rect::new(self.height(), self.width())
		}
	}

	/// Returns a rect with the longest dimension being its width and its other being its height.
	pub fn dimensions_longest(&self) -> RotatableRect
	{
		self.dimensions_longest_rotated(true)
	}

	/// Returns `dimensions_longest` if `rotate` is true or `dimensions` otherwise.
	pub fn dimensions_longest_rotated(&self, rotate: bool) -> RotatableRect
	{
		if (self.width() >= self.height() && rotate) || !rotate
		{
			RotatableRect::new(self.width(), self.height(), false)
		}
		else
		{
			RotatableRect::new(self.height(), self.width(), true)
		}
	}
}

/// References an axis aligned rect placed in a bin by index.
pub struct AtlasPart
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

/// A packed bin containing references to rects.
///
/// This class tracks the rects added to itself. After each rect is added, it increases its
/// bounding size if necessary. However, it does not make any guarantees. It is expected that all
/// atlas generators play nicely and conform to all rules. The bin size should be the minimum
/// bounding size, capable of encapsulating all objects. Each object should also not pass through
/// any boundaries and should be disjoint.
///
pub struct AtlasBin
{
	/// The bounding dimensions of the bin.
	dimensions: Rect,

	/// The list of referenced rects in this bin.
	part_list: Vec<AtlasPart>,
}

impl AtlasBin
{
	/// Initializes a new bin with the given rect at the top right of the bin.
	fn new(rect_index: usize, dimensions: Rect, rotate: bool) -> Self
	{
		let part = AtlasPart
		{
			rect_index,
			x: 0,
			y: 0,
			rotate,
		};
		AtlasBin
		{
			dimensions,
			part_list: vec![part],
		}
	}

	/// Returns the current bounding dimensions of the bin.
	pub fn dimensions(&self) -> Rect
	{
		self.dimensions
	}

	/// Returns the parts in this bin.
	pub fn part_list(&self) -> &[AtlasPart]
	{
		&self.part_list
	}

	/// Adds a new rect to the bin. The size of the bin increases if mandatory.
	fn part_add(&mut self, rect_index: usize, x: u32, y: u32, dimensions: Rect, rotate: bool)
	{
		self.dimensions.width = std::cmp::max(self.dimensions.width, x + dimensions.width);
		self.dimensions.height = std::cmp::max(self.dimensions.height, y + dimensions.height);
		self.part_list.push(AtlasPart
		{
			rect_index,
			x,
			y,
			rotate,
		});
	}
}

impl AsRef<Rect> for AtlasBin
{
	fn as_ref(&self) -> &Rect
	{
		&self.dimensions
	}
}

/// Generates a texture atlas using a bin packing algorithm.
pub trait AtlasGenerator
{
	/// Generates a list of bins for the given atlas.
	fn generate<T: AtlasRect>(&self, atlas: &mut Atlas<T>, width: u32, height: u32, rotate: bool);
}

/// List data structure for adding rects.
///
/// This data structure is essentially a wrapper on `Vec<T>` with the difference that it tracks
/// total area of all rects in order to calculate a lower bound of the number of bins. By using a
/// lower bound, an atlas may potentially re-allocate less.
///
pub struct AtlasRectList<T: AtlasRect>
{
	rect_list: Vec<T>,
	total_area: u64,
}

impl<T> AtlasRectList<T> where T: AtlasRect
{
	/// Constructs a new, empty list.
	pub fn new() -> Self
	{
		AtlasRectList
		{
			rect_list: Vec::new(),
			total_area: 0,
		}
	}

	/// Constructs a new, empty list with the specified capacity.
	pub fn with_capacity(capacity: usize) -> Self
	{
		AtlasRectList
		{
			rect_list: Vec::with_capacity(capacity),
			total_area: 0,
		}
	}

	/// Returns the number of rects in the list.
	pub fn len(&self) -> usize
	{
		self.rect_list.len()
	}

	/// Adds the given rect to the list and potentially increases the lower bound.
	pub fn add(&mut self, rect: T)
	{
		self.total_area += (&rect as &AtlasRect).area();
		self.rect_list.push(rect);
	}

	/// Returns the total area of all rects in this list combined.
	pub fn total_area(&self) -> u64
	{
		self.total_area
	}

	/// Returns the lower bound of bins needed for the rects in this list.
	pub fn lower_bound(&self, size: Rect) -> usize
	{
		let atlas_rect = &size as &AtlasRect;
		assert_eq!(atlas_rect.empty(), false);
		((self.total_area / atlas_rect.area()) + 1) as usize
	}

	/// Returns an atlas builder using this rect list and given constraints.
	pub fn build(&self, width: u32, height: u32, rotate: bool) -> AtlasBuilder<T>
	{
		let lower_bound = self.lower_bound(Rect::new(width, height));
		AtlasBuilder::new(&self.rect_list, width, height, rotate, lower_bound)
	}
}

/// Stores settings for generating an `Atlas`.
///
/// The builder takes a few constraints. It takes a maximal width and height constraint, which atlas
/// generators are not expected to exceed. It also takes a flag indicating whether or not rotations
/// should be allowed by generators.
///
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
	fn new(rect_list: &'a [T], width: u32, height: u32, rotate: bool, lower_bound: usize) -> Self
	{
		AtlasBuilder
		{
			rect_list,
			width,
			height,
			lower_bound,
			rotate
		}
	}

	/// Generates bins using the given generator.
	pub fn generate<G: AtlasGenerator>(self, generator: &G) -> Atlas<'a, T>
	{
		let mut atlas = Atlas
		{
			rect_list: self.rect_list,
			bin_list: Vec::with_capacity(self.lower_bound),
		};
		generator.generate(&mut atlas, self.width, self.height, self.rotate);
		atlas
	}
}

/// Encapsulates axis aligned rectangles and generated bins.
pub struct Atlas<'a, T: 'a + AtlasRect>
{
	rect_list: &'a [T],
	bin_list: Vec<AtlasBin>,
}

impl<'a, T> Atlas<'a, T> where T: 'a + AtlasRect
{
	/// Returns a builder instance with the given size constraints.
	pub fn build(rect_list: &'a [T], width: u32, height: u32, rotate: bool) -> AtlasBuilder<T>
	{
		AtlasBuilder::new(rect_list, width, height, rotate, 1)
	}

	/// Creates a new atlas with the given axis-aligned rectangles.
	pub fn new(rect_list: &'a [T]) -> Self
	{
		Self
		{
			rect_list,
			bin_list: Vec::new(),
		}
	}

	/// Returns the list of axis-aligned rectangles that are part of the atlas.
	pub fn rect_list(&self) -> &[T]
	{
		&self.rect_list
	}

	/// Returns the bins that reference the rects.
	pub fn bin_list(&self) -> &[AtlasBin]
	{
		&self.bin_list
	}

	/// Creates a new bin with the given rect at the top left.
	pub fn bin_add_new(&mut self, rect_index: usize, rotate: bool) -> usize
	{
		let bin_index = self.bin_list.len();
		let dimensions = (&self.rect_list[rect_index] as &AtlasRect).dimensions_rotated(rotate);
		self.bin_list.push(AtlasBin::new(rect_index, dimensions, rotate));
		bin_index
	}

	/// Adds a new rect to the indicated bin.
	pub fn bin_add_rect(&mut self, bin_index: usize, rect_index: usize, x: u32, y: u32, rotate: bool)
	{
		let dimensions = (&self.rect_list[rect_index] as &AtlasRect).dimensions_rotated(rotate);
		self.bin_list[bin_index].part_add(rect_index, x, y, dimensions, rotate);
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
