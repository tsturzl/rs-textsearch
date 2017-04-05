//Document Index, holds an index containing processed text
use std::collections::HashMap;
use uuid::Uuid;
use tokenize::tokenize;

pub struct Index {
	pub tokens: HashMap<String, usize>,
	pub word_count: usize,
	pub id: String
}

impl Index {
	pub fn new(corpus: &str) -> Index {
		let text = corpus.to_owned();
		let tokens: Vec<String> = tokenize(&text);
		let mut token_hash: HashMap<String, usize> = HashMap::with_capacity(tokens.len());
		let count: usize = tokens.len();

		for token in tokens.into_iter() {
			let entry = token_hash.entry(token).or_insert(0);
			*entry += 1;
		}

		Index {
			tokens: token_hash,
			word_count: count,
			id: Uuid::new_v4().simple().to_string()
		}
	}
}
