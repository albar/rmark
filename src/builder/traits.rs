use crate::node;
use std::any::Any;

pub trait NodeBuilderResolver {
    fn get_pattern(&self) -> &str;
    fn resolve(&self, indents: usize, line: String) -> Box<dyn NodeBuilder>;
}

pub trait NodeBuilder {
    fn append_or_throwback(&mut self, line: String) -> Option<String>;
    fn build(self: Box<Self>) -> node::Node;
}

pub fn downcast_ref<T: Any>(builder: &Box<dyn NodeBuilder>) -> Option<&T> {
    as_any(builder).downcast_ref::<T>()
}

fn as_any(builder: &Box<dyn NodeBuilder>) -> &dyn Any {
    builder
}