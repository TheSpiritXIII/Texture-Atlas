#[cfg(feature = "image")]
mod img;

#[cfg(feature = "image")]
pub use self::img::*;

use AtlasRect;

#[derive(Copy, Clone, Debug)]
pub struct Rect
{
	pub width: u32,
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
