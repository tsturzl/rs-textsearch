extern crate futures;
extern crate futures_cpupool;
use futures::Future;
use futures_cpupool::CpuPool;
use futures_cpupool::CpuFuture;
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::RwLock;
use global::Global;
use document::Document;

#[derive(Clone)]
pub struct Search {
	pub indices: Arc<
		RwLock<
			HashMap< 
				String,
				Arc<RwLock<Global>> 
			> 
		>
	>, //<name, Global> for Global indices
	threadpool: CpuPool
}

impl Search {
	pub fn new() -> Search {
		Search {
			indices: Arc::new(
				RwLock::new(
					HashMap::new()
				)
			),
			threadpool: CpuPool::new_num_cpus()
		}
	}

	pub fn create_index(&mut self, name: &str) -> Result< Arc<RwLock<Global>>, &str > {
		let indices = self.indices.clone();
		let mut indices = indices.write().unwrap();

		if indices.contains_key(name) {
			Err("Global Index already exists")
		} else {
			let index: Arc<RwLock<Global>> = 
				Arc::new(
					RwLock::new(
						Global::new(name)
					)
				);

			indices.insert(name.to_string(), index.clone());
			Ok(index.clone())
		}
	}

	pub fn remove_index(&mut self, name: &str) -> Result< Arc<RwLock<Global>>, &str > {
		let indices = self.indices.clone();
		let mut indices = indices.write().unwrap();

		match indices.remove(name) {
			Some(val) => Ok(val),
			None => Err("Global Index not found")
		}
	}

	pub fn insert(&mut self, name: String, corpus: String) -> CpuFuture<Arc<Document>, String> {
		let indices = self.indices.clone();

		let future: CpuFuture<Arc<Document>, String> = self.threadpool.spawn_fn(move || {
			let indices = indices.read().unwrap();

			match indices.get(&name) {
				Some(val) => {
					let val = val.clone();

					let mut global = val.write().unwrap();
					let index = global.insert(&corpus);
					let res: Result<Arc<Document>, String> = Ok(index.doc.clone());

					res
				},
				None => {
					let res: Result<Arc<Document>, String> = Err("Global Index not found.".to_string());

					res
				}
			}
		});

		future
	}

	pub fn search(&self, name: String, text: String) -> CpuFuture<Vec<(Arc<Document>, f32)>, String> {
		let indices = self.indices.clone();

		let future: CpuFuture<Vec<(Arc<Document>, f32)>, String> = self.threadpool.spawn_fn(move || {
			let indices = indices.read().unwrap();

			match indices.get(&name) {
				Some(val) => {
					let val = val.clone();

					let global = val.read().unwrap();
					let res: Result<Vec<(Arc<Document>, f32)>, String> = Ok(global.search(&text));

					res
				},
				None => {
					let res: Result<Vec<(Arc<Document>, f32)>, String> = Err("Global Index not found.".to_string());

					res
				}
			}
		});

		future
	}
}