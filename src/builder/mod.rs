mod heading;
mod list;
mod style;
pub mod traits;

pub use list::ListBuilder;

use regex::RegexSet;
use regex::RegexSetBuilder;

use crate::node;
use traits::NodeBuilder;
use traits::NodeBuilderResolver;

fn resolvers() -> Vec<Box<dyn NodeBuilderResolver>> {
    vec![
        heading::HeadingBuilderResolver::boxed(),
        list::ListBuilderResolver::boxed(),
    ]
}

fn build_patterns(resolvers: &Vec<Box<dyn NodeBuilderResolver>>) -> RegexSet {
    let patterns = resolvers.iter().map(|resolver| resolver.get_pattern());
    RegexSetBuilder::new(patterns).build().unwrap()
}

pub struct ParagraphBuilder(usize, style::TextStyleBuilder);
impl ParagraphBuilder {
    fn from(indents: usize, content: String) -> Self {
        ParagraphBuilder(indents, style::TextStyleBuilder::from(content))
    }
}
impl NodeBuilder for ParagraphBuilder {
    fn append_or_throwback(&mut self, line: String) -> Option<String> {
        if line.trim().is_empty() {
            return Some(line);
        }
        self.1.append(line);
        None
    }
    fn build(self: Box<Self>) -> node::Node {
        let kind = node::NodeKind::Paragraph(self.1.build());
        node::Node::new(self.0, kind)
    }
}

pub struct NodeBuilderFinder {
    resolvers: Vec<Box<dyn NodeBuilderResolver>>,
    patterns: RegexSet,
}
impl NodeBuilderFinder {
    pub fn find_node_builder(&self, indents: usize, line: String) -> Box<dyn NodeBuilder> {
        if line.trim().is_empty() {
            panic!("no whitespace line allowed!")
        }
        if let Some(index) = self.patterns.matches(line.as_str()).iter().next() {
            return self.resolvers[index].resolve(indents, line);
        }
        Box::new(ParagraphBuilder::from(indents, line))
    }
}
impl Default for NodeBuilderFinder {
    fn default() -> Self {
        let resolvers = resolvers();
        let patterns = build_patterns(&resolvers);
        NodeBuilderFinder {
            resolvers,
            patterns,
        }
    }
}
