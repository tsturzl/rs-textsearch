/*
TODO:
  * Check tokens hashmap to ensure tokenizer is functioning
*/

extern crate textsearch;
use textsearch::index::Index;

static DOC: &'static str = "Some text to test with";

#[test]
fn create_index() {
	let index = Index::new(DOC);

	//should have 5 tokens, one for each word in DOC
	assert_eq!(index.tokens.len(), 5);

	//Document for index should have 32byte UUID
	assert_eq!(index.doc.id.len(), 32);

	//Index should have same id as Document
	assert_eq!(index.id, index.doc.id);

	//Document should have a corpus that is equal to DOC
	assert_eq!(index.doc.corpus, DOC);

}