pub struct Splitter;

impl Splitter {
  pub fn is_match(c: char) -> bool {
    match c {
      ' ' | ',' | '.' | '!' | '?' | ';' | '\'' |  '"'
      | ':' | '\t' | '\n' | '(' | ')' | '-' => true,
      _ => false
    }
  }
}