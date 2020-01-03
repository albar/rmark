use crate::builder;
use crate::builder::ListBuilder;
use crate::builder::NodeBuilderFinder;
use std::io::BufRead;
use std::io::Lines;

pub enum TextStyle {
    Normal(String),
    Italic(Vec<TextStyle>),
    Bold(Vec<TextStyle>),
    ItalicBold(String),
    Mixed(Vec<TextStyle>),
}

pub enum List {
    Ordered(TextStyle),
    UnOrdered(TextStyle),
}

pub enum Heading {
    H1(TextStyle),
    H2(TextStyle),
    H3(TextStyle),
    H4(TextStyle),
    H5(TextStyle),
    H6(TextStyle),
}

pub enum NodeKind {
    Heading(Heading),
    Paragraph(TextStyle),
    List(List),
    Code,
}

pub struct Node {
    pub indents: usize,
    pub kind: NodeKind,
}
impl Node {
    pub fn new(indents: usize, kind: NodeKind) -> Self {
        Node { indents, kind }
    }
}

pub struct DocumentNodeIterator<L: BufRead> {
    finder: NodeBuilderFinder,
    lines: Lines<L>,
    cached_line: Option<String>,
    spaces: Vec<usize>,
}

impl<L: BufRead> DocumentNodeIterator<L> {
    pub fn new(finder: NodeBuilderFinder, lines: Lines<L>) -> DocumentNodeIterator<L> {
        DocumentNodeIterator {
            finder,
            lines,
            cached_line: None,
            spaces: Vec::new(),
        }
    }
}

impl<L: BufRead> DocumentNodeIterator<L> {
    fn next_line(&mut self) -> Option<String> {
        if let Some(cached) = self.cached_line.take() {
            return Some(cached);
        }

        if let Some(Ok(line)) = self.lines.next() {
            return Some(line);
        }

        None
    }

    fn next_non_blank_line(&mut self) -> Option<String> {
        while let Some(line) = self.next_line() {
            if !line.trim().is_empty() {
                return Some(line);
            }
        }

        None
    }
}

impl<L: BufRead> Iterator for DocumentNodeIterator<L> {
    type Item = Node;

    fn next(&mut self) -> Option<Node> {
        if let Some(line) = self.next_non_blank_line() {
            let mut line = line.replace('\t', "    ");
            while self.spaces.len() > 0 {
                let spaces = self.spaces.iter().sum();
                if self.spaces.len() > 0 && line[0..spaces].trim().is_empty() {
                    line = line[(spaces + 1)..].to_owned();
                    break;
                } else {
                    self.spaces.pop();
                }
            }

            let mut builder = self.finder.find_node_builder(self.spaces.len(), line);

            if let Some(list) = builder::traits::downcast_ref::<ListBuilder>(&builder) {
                self.spaces.push(list.get_spaces());
            }

            while let Some(line) = self.next_line() {
                if let Some(throwed) = builder.append_or_throwback(line) {
                    if throwed.trim().is_empty() {
                        self.cached_line = None
                    } else {
                        self.cached_line = Some(throwed);
                    }
                    break;
                }
            }

            return Some(builder.build());
        }

        None
    }
}
