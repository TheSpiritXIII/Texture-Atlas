use {AtlasBin, AtlasGenerator, AtlasObject, Rect};

/// A generator that creates a separate bins for each object.
pub struct PassthroughGenerator;

impl AtlasGenerator for PassthroughGenerator {
	fn generate_atlas(bin_list: &[Rect], _: usize, _: usize) -> Vec<AtlasBin> {
		let mut atlas = Vec::new();
		for (bin_index, bin) in bin_list.iter().enumerate() {
			let object = AtlasObject {
				bin_index: bin_index,
				x: 0,
				y: 0,
			};
			atlas.push(AtlasBin::with_part(object, bin.width, bin.height))
		}
		atlas
	}
}
