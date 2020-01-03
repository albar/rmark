use super::style::TextStyleBuilder;
use super::traits::NodeBuilder;
use super::traits::NodeBuilderResolver;
use crate::node;
use regex::Regex;
use regex::RegexBuilder;

lazy_static! {
    static ref PATTERN: &'static str = r"^ *(?P<spaces>(?P<kind>[0-9]+\.|-) +)(?P<content>.*)$";
    static ref REGEX: Regex = RegexBuilder::new(&PATTERN).build().unwrap();
}

pub struct ListBuilderResolver;
impl ListBuilderResolver {
    pub fn boxed() -> Box<dyn NodeBuilderResolver> {
        Box::new(ListBuilderResolver)
    }
}
impl NodeBuilderResolver for ListBuilderResolver {
    fn get_pattern(&self) -> &str {
        &PATTERN
    }
    fn resolve(&self, indents: usize, line: String) -> Box<dyn NodeBuilder> {
        let captures = REGEX.captures(line.trim()).unwrap();
        let kind = match captures.name("kind").unwrap().as_str() {
            "-" => ListKind::Unordered,
            _ => ListKind::Ordered,
        };
        Box::new(ListBuilder {
            indents,
            spaces: captures.name("spaces").unwrap().as_str().len(),
            kind,
            content: captures.name("content").unwrap().as_str().to_owned(),
        })
    }
}

enum ListKind {
    Ordered,
    Unordered,
}

pub struct ListBuilder {
    indents: usize,
    spaces: usize,
    kind: ListKind,
    content: String,
}
impl ListBuilder {
    pub fn get_spaces(&self) -> usize {
        self.spaces
    }
}
impl NodeBuilder for ListBuilder {
    fn append_or_throwback(&mut self, line: String) -> Option<String> {
        Some(line)
    }
    fn build(self: Box<Self>) -> node::Node {
        let text = TextStyleBuilder::from(self.content).build();
        let list = match self.kind {
            ListKind::Ordered => node::List::Ordered(text),
            ListKind::Unordered => node::List::UnOrdered(text),
        };
        node::Node::new(self.indents, node::NodeKind::List(list))
    }
}
