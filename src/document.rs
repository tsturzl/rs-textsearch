// Documents hold the corpus with an ID
use uuid::Uuid;

pub struct Document {
	pub id: String,
	pub corpus: String
}

impl Document {
	pub fn new(corpus: &str) -> Document {
		Document {
			id: Uuid::new_v4().simple().to_string(),
			corpus: corpus.to_owned()
		}
	}
}