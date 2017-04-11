//Global Index
use std::collections::HashMap;
use std::sync::Arc;
use std::cmp::Ordering;
use tokenize::tokenize;
use text_index::TextIndex;

pub struct Global {
	pub name: String,
	pub dictionary: HashMap<String, usize>, //hold count of every index containting a token
	pub indices: Vec<Arc<TextIndex>>
}

impl Global {
	pub fn new(name: &str) -> Global {
		Global {
			name: name.to_owned(),
			dictionary: HashMap::new(),
			indices: Vec::new()
		}
	}

	//Insert a corpus, creates an index and stores in indices array and dictionary
	pub fn insert(&mut self, corpus: &str) -> Arc<TextIndex> {
		let index: Arc<TextIndex> = Arc::new(TextIndex::new(corpus));
		let index_ref = index.clone();
		let tokens = index.tokens.clone();

		for token in tokens.keys() {
			let token = token.clone();
			let entry = self.dictionary.entry(token).or_insert(0);
			*entry += 1;
		}

		self.indices.push(index_ref);

		index.clone()
	}

	pub fn search(&self, text: &str) -> Vec<(Arc<TextIndex>, f32)>  {
		let indices = self.indices.clone();
		let tokens_vec: Vec<String> = tokenize(text);
		let mut tokens: HashMap<String, usize> = HashMap::with_capacity(tokens_vec.len());

		for token in tokens_vec.into_iter() {
			let entry = tokens.entry(token).or_insert(0);
			*entry += 1;
		}

		let token_ref = &tokens;

		//Score table `(Index, score)`
		let mut scores: Vec<(Arc<TextIndex>, f32)> = Vec::new();

		for index in indices.into_iter() {
			let index = index.clone();
			let mut score = 0.0f32;

			for (token, count) in token_ref.into_iter() {
				let index_count: usize = match index.tokens.get(token) {
				  Some(val) => val.clone(),
				  None => 0
				};

				let occurance =  index_count * count;

				let global_occurance: usize = match self.dictionary.get(token) {
					Some(val) => val.clone(),
					None => 0
				};

				score += occurance as f32 / global_occurance as f32;
			}

			scores.push((index.clone(), score / index.word_count as f32));
		}

		self.finalize(scores)
	}

	//Helper, returns a sorted vector
	fn finalize(&self, mut scores: Vec<(Arc<TextIndex>, f32)>) -> Vec<(Arc<Index>, f32)> {
		scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(Ordering::Equal));

		let mut table: Vec<(Arc<Index>, f32)> = Vec::new();
		for (index, score) in scores.into_iter() {
			let index = index.clone();

			table.push((index.clone(), score));
		}

		table
	}
}
