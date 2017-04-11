//Document Index, holds an index containing processed text
use std::collections::HashMap;
use uuid::Uuid;
use n_gram::tokenize;

pub struct PrefixIndex {
  pub n_grams: HashMap<String, usize>,
  pub min: usize,
  pub max: usize,
  pub word_count: usize,
  pub id: String
}

impl PrefixIndex {
  pub fn new(corpus: &str, min: usize, max: usize) -> PrefixIndex {
    let text = corpus.to_owned();
    let tokens: Vec<String> = tokenize(&text);
    let mut token_hash: HashMap<String, usize> = HashMap::with_capacity(tokens.len());
    let count: usize = tokens.len();

    for token in tokens.into_iter() {
      let entry = token_hash.entry(token).or_insert(0);
      *entry += 1;
    }

    PrefixIndex {
      n_grams: token_hash,
      min: min,
      max: max,
      word_count: count,
      id: Uuid::new_v4().simple().to_string()
    }
  }
}
