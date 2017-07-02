//! General and image related utility functions.

#[cfg(feature = "image")]
mod img;

#[cfg(feature = "image")]
pub use self::img::*;

use AtlasRect;

/// An axis-aligned rectangle.
#[derive(Copy, Clone, Debug)]
pub struct Rect
{
	/// The width dimension of the rectangle.
	pub width: u32,

	/// The height dimension of the rectangle.
	pub height: u32,
}

impl Rect
{
	pub fn new(width: u32, height: u32) -> Self
	{
		Self
		{
			width,
			height,
		}
	}
}

impl AtlasRect for Rect
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

impl<T: AsRef<Rect>> AtlasRect for T
{
	fn width(&self) -> u32
	{
		self.as_ref().width
	}
	fn height(&self) -> u32
	{
		self.as_ref().height
	}
}

/// An axis-aligned rectangle that can be rotated.
#[derive(Copy, Clone, Debug)]
pub struct RotatableRect
{
	/// The dimensions of the rectangle.
	pub rect: Rect,

	/// True if this rectagle is rotated 90 degrees clockwise.
	pub rotated: bool,
}

impl RotatableRect
{
	pub fn new(width: u32, height: u32, rotated: bool) -> Self
	{
		Self
		{
			rect: Rect::new(width, height),
			rotated,
		}
	}
}

impl AsRef<Rect> for RotatableRect
{
	fn as_ref(&self) -> &Rect
	{
		&self.rect
	}
}
