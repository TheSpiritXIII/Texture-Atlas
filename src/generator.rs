use {AtlasBin, AtlasGenerator, AtlasObject, AtlasReference};

/// A generator that creates a separate bin for each object.
pub struct PassthroughGenerator;

impl AtlasGenerator for PassthroughGenerator {
	fn generate_atlas<T: AtlasObject>(object_list: &[T], _: usize, _: usize) -> Vec<AtlasBin> {
		let mut bin_list = Vec::new();
		for (object_index, object) in object_list.iter().enumerate() {
			let reference = AtlasReference {
				object_index: object_index,
				x: 0,
				y: 0,
			};
			bin_list.push(AtlasBin::with_part(reference, object.width(), object.height()))
		}
		bin_list
	}
}
