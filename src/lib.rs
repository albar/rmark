#[macro_use] extern crate lazy_static;

mod node;
mod builder;

pub use node::Node;
pub use node::NodeKind;
pub use builder::NodeBuilderFinder;

use std::io::BufRead;
use std::io::Lines;

pub struct DocumentOptions;

fn create_finder(options: &DocumentOptions) -> NodeBuilderFinder {
    unimplemented!()
}

// should be configurable then consumed
pub struct Document<L> {
    options: DocumentOptions,
    lines: Lines<L>,
}
impl<L: BufRead> IntoIterator for Document<L> {
    type Item = Node;
    type IntoIter = node::DocumentNodeIterator<L>;

    fn into_iter(self) -> node::DocumentNodeIterator<L> {
        node::DocumentNodeIterator::new(
            create_finder(&self.options),
            self.lines,
        )
    }
}

pub struct ParserOptions;
impl ParserOptions {
    fn build_document_options(&self) -> DocumentOptions {
        unimplemented!()
    }
}

// should be reusable
pub struct MarkdownParser {
    options: ParserOptions,
}
impl MarkdownParser {
    pub fn new(options: ParserOptions) -> Self {
        MarkdownParser { options }
    }

    pub fn parse_str<'a>(&self, markdown: &'a str) -> Document<&'a [u8]> {
        self.parse_buf(markdown.as_bytes())
    }

    pub fn parse_buf<R: BufRead>(&self, markdown: R) -> Document<R> {
        Document {
            options: self.options.build_document_options(),
            lines: markdown.lines(),
        }
    }
}
