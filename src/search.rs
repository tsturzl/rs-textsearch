use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use global::Global;
use document::Document;

pub struct Search {
	pub indices: HashMap< String, Arc<RwLock<Global>> > //<name, Global> for Global indices
}

impl Search {
	pub fn new() -> Search {
		Search {
			indices: HashMap::new()
		}
	}

	pub fn create_index(&mut self, name: &str) -> Option< Arc<RwLock<Global>> > {
		if self.indices.contains_key(name) {
			None
		} else {
			let index: Arc<RwLock<Global>> = 
				Arc::new(
					RwLock::new(
						Global::new(name)
					)
				);

			self.indices.insert(name.to_string(), index.clone());
			Some(index.clone())
		}
	}

	pub fn remove_index(&mut self, name: &str) -> Option< Arc<RwLock<Global>> > {
		self.indices.remove(name)
	}

	pub fn insert(&mut self, name: &str, corpus: &str) -> Option<Arc<Document>> {
		match self.indices.get(name) {
			Some(val) => {
				let val = val.clone();
				let mut global = val.write().unwrap();
				let index = global.insert(corpus);

				Some(index.doc.clone())
			}
			None => None
		}
	}

	pub fn search(&self, name: &str, text: &str) -> Option< Vec<(Arc<Document>, f32)> > {
		match self.indices.get(name) {
			Some(val) => {
				let val = val.clone();
				let global = val.read().unwrap();

				Some(global.search(text))
			},
			None => None
		}
	}
}