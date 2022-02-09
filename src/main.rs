use libxml::tree::{Document, Node};

fn main() {
}


fn to_xml() -> String {
    let doc = Document::new().unwrap();
    let mut books = Node::new("books", None, &doc).unwrap();
    books.add_text_child(None, "book", "Dune").unwrap();
    books.add_text_child(None, "book", "Les Misérables").unwrap();
    doc.node_to_string(&books)
}

#[cfg(test)]
mod tests {
    use crate::to_xml;

    #[test]
    fn it_works_0() {
        assert_eq!(to_xml(), "<books><book>Dune</book><book>Les Misérables</book></books>");
    }

    #[test]
    fn it_works_1() {
        assert_eq!(to_xml(), "<books><book>Dune</book><book>Les Misérables</book></books>");
    }

    #[test]
    fn it_works_2() {
        assert_eq!(to_xml(), "<books><book>Dune</book><book>Les Misérables</book></books>");
    }

    #[test]
    fn it_works_3() {
        assert_eq!(to_xml(), "<books><book>Dune</book><book>Les Misérables</book></books>");
    }

}



