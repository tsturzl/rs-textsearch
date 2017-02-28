extern crate textsearch;
use textsearch::document::Document;

static DOC: &'static str = "Some text to test with";

#[test]
fn create_doc() {
	let doc = Document::new(DOC);

	//Document should have 32byte UUID
	assert_eq!(doc.id.len(), 32);

	//Document should have corpus equal to DOC
	assert_eq!(doc.corpus, DOC);
}