extern crate librmark;

use librmark::*;

fn main() {
    let parser = MarkdownParser::new(ParserOptions);
    let document = parser.parse_str("");
    for node in document.into_iter() {
        match node.kind {
            NodeKind::Code => unimplemented!(),
            _ => unimplemented!()
        }
    }
}
