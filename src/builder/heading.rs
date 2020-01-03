use super::traits::NodeBuilder;
use super::traits::NodeBuilderResolver;
use super::style::TextStyleBuilder;
use crate::node::Heading;
use crate::node::Node;
use crate::node::NodeKind;
use regex::Regex;
use regex::RegexBuilder;

lazy_static! {
    static ref PATTERN: &'static str = r"^ *(?P<level>#{1,6}) *(?P<header>.*)$";
    static ref REGEX: Regex = RegexBuilder::new(&PATTERN).build().unwrap();
}

pub struct HeadingBuilderResolver;
impl HeadingBuilderResolver {
    pub fn boxed() -> Box<dyn NodeBuilderResolver> {
        Box::new(HeadingBuilderResolver)
    }
}
impl NodeBuilderResolver for HeadingBuilderResolver {
    fn get_pattern(&self) -> &str {
        &PATTERN
    }
    fn resolve(&self, indents: usize, line: String) -> Box<dyn NodeBuilder> {
        let captures = REGEX.captures(line.as_str()).unwrap();
        Box::new(HeadingBuilder {
            indents,
            level: captures.name("level").unwrap().as_str().len(),
            header: captures.name("header").unwrap().as_str().to_owned(),
        })
    }
}

pub struct HeadingBuilder {
    indents: usize,
    level: usize,
    header: String,
}
impl NodeBuilder for HeadingBuilder {
    fn append_or_throwback(&mut self, line: String) -> Option<String> {
        Some(line)
    }
    fn build(self: Box<Self>) -> Node {
        let content = TextStyleBuilder::from(self.header).build();
        let kind = match self.level {
            1 => NodeKind::Heading(Heading::H1(content)),
            2 => NodeKind::Heading(Heading::H2(content)),
            3 => NodeKind::Heading(Heading::H3(content)),
            4 => NodeKind::Heading(Heading::H4(content)),
            5 => NodeKind::Heading(Heading::H5(content)),
            _ => NodeKind::Heading(Heading::H6(content)),
        };
        Node::new(self.indents, kind)
    }
}
