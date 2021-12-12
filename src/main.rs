use libxml::tree::Document;

fn main() {
    let doc_result = Document::new();
    assert!(doc_result.is_ok());
}
