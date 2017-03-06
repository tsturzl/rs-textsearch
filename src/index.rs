//Document Index, holds an index containing processed text
use std::collections::HashMap;
use std::sync::Arc;
use tokenize::tokenize;
use document::Document;

pub struct Index {
	pub tokens: HashMap<String, usize>,
	pub word_count: usize,
	pub doc: Arc<Document>,
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

		let doc: Arc<Document> = Arc::new(Document::new(&text));

		Index {
			tokens: token_hash,
			word_count: count,
			doc: doc.clone(),
			id: doc.id.to_owned()
		}
	}
}
