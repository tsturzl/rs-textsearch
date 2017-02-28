use stem;

pub fn tokenize(text: &str) -> Vec<String> {
  text.to_lowercase().split(Splitter::is_match)
                   .filter(|s| s.len() > 0)
                   .map(|text| stem::get(text).unwrap())
                   .collect()
}

struct Splitter;

impl Splitter {
    fn is_match(c: char) -> bool {
        match c {
            ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
            | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
            _ => false
        }
    }
}