//Document Index, holds an index containing processed text
extern crate natural;
extern crate stem;
use std::collections::HashMap;
use std::sync::Arc;
use natural::tokenize::tokenize;
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
		let tokens: Vec<String> = get_tokenized_and_stemmed(&text);
		let mut token_hash: HashMap<String, usize> = HashMap::new();
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

fn get_tokenized_and_stemmed(text: &str) -> Vec<String> {
  let tokenized_text = tokenize(text);
  tokenized_text.into_iter()
                .map(|text| stem::get(text).unwrap())
                .collect()
}