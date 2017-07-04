# texture_atlas [![Build Status](https://travis-ci.org/TheSpiritXIII/Texture-Atlas.svg?branch=master)](https://travis-ci.org/TheSpiritXIII/Texture-Atlas) [![Coverage Status](https://coveralls.io/repos/github/TheSpiritXIII/Texture-Atlas/badge.svg?branch=master)](https://coveralls.io/github/TheSpiritXIII/Texture-Atlas?branch=master)

This crate provides various algorithms for bin packing axis aligned rectangles.

The most common use case for this library is for games. In order to reduce texture swapping on
the GPU, multiple textures are combined into fewer, larger textures.

## Features

This crate contains and provides basic tools for building and using bin packing algorithms.

The following bin packing algorithms, or generators, are implemented:

- `PassthroughGenerator`
- `BinaryTreeGenerator`

All algorithms are expected to take and respect a size constraint and a flag indicating whether
or not to rotate of rects.

### Future

This library is currently unstable. This is a list of tasks that will be done in the future
sorted by importance:

- Improve tests and documentation.
- Add basic CLI tool.
- Add "Max Rects" generator.
- Submit to creates.io.
- ABI Stablizaation.

## Common Usage

This library is intended to be used as a build script. It does not facilitate how data is loaded
but users are welcome to create their own on top of this library.

All atlas generation is done with a simple `AtlasRect` trait that must be implemented on
whatever you wish to generate an atlas for. For convenience, this trait is pre-implemented for
the `image` crate's `DynamicImage` struct and also any struct that implements
`AsRef<DynamicImage>`.

Before bin packing, you must have an instance of `AtlasBuilder`. There are two ways to achieve
that: The first is using `Atlas::build` and passing an array to it. The second is using the
provided `AtlasRectList` and its `build` function which calculates a lower bound on the number
of bins that will be generated as you add rects to the list.

At the heard of `AtlasBuilder` is a `generate` method which takes in an `AtlasGenerator`. The
current recommended generator is the `BinaryTreeGenerator`. You can even call generate multiple
times on the builder to find the best generator that generates the least amount of bins.

After calling this method, you receive an `Atlas` struct which contains your generated bins. If
you are using the `image` feature, then you can use `Atlas::as_images` to generate a vector of
images corresponding to each generated bin.

### Bins of Bins

Occasionally, it is also useful to have certain rects together. For instance, in a game you may
have multiple frames for a player walking animation. In this case, if the frames are in
different bins, then this will incur a texture swapping overhead.

To address these scenarios, you can generate a single bin for each groups of related rects and
then pass these bins back into generator. Better support will come for these scenarios shortly.

## Creating a Generator

To create a new generator, create a struct and implement `AtlasGenerator` for it. The
`AtlasGenerator` trait uses dynamic dispatch for instances where a generator can have settings,
for instance multiple heuristic options. `PassthroughGenerator` is an example of a minimal
generator.

## The `image` Feature

The `image` feature is turned on by default. To disable it, use the following in your
`Cargo.toml`:

```toml
[dependencies.texture_atlas]
default-features = false
```

If you keep it enabled, you can create images for generated atlases and gain access to a few
utility functions, such as border cropping.
