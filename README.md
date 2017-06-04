# texture_atlas [![Build Status](https://travis-ci.org/TheSpiritXIII/Texture-Atlas.svg?branch=master)](https://travis-ci.org/TheSpiritXIII/Texture-Atlas)

This crate provides various algorithms for bin packing axis aligned rectangles.

The most common use case for this library is for games. In order to reduce texture swapping on
the GPU, multiple textures are combined into fewer, larger textures.

## Features

So far, only basic texture atlas generating features are supported. All atlas generation is done
with a simple `AtlasRect` trait that must be implemented on whatever you wish to generate an
atlas for. For convenience, this trait is pre-implemented for the `image` crate's
`DynamicImage`.

### Future

This is a list of tasks that will be done in the future sorted by importance:
- Rotatable rects.
- Improve `image` integration.
- Improve tests.
- Add basic CLI example tool.
- Add "Max Rects" generator.

## Common Usage

This library is intended to be used as a build script. It does not facilitate how data is loaded
but users are welcome to create their own on top of this library.
