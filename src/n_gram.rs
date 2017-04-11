use std::collections::HashMap;
use std::ops::Range;
use splitter::Splitter;

pub fn tokenize(text: &str, min: usize, max: usize) -> Vec<String> {
  let mut n_grams: HashMap<String, Vec<usize>> = HashMap::new();
  let mut n_grams: Vec<String> = Vec::new();

  let tokens: Vec<String> = text.to_lowercase().split(Splitter::is_match)
    .filter(|s| s.len() > 0)
    .collect();

  for token in tokens.into_iter() {
    let len = token.len();

    let range = Range{
      start: if len < max { len } else { max },
      end: if len < min { len } else { min }
    };

    for i in range {
      let sub_str = token.slice_unchecked(0, i);
      n_grams.push(sub_str.to_string());
    }
  }

  n_grams
}