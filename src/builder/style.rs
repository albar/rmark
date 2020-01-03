use crate::node;

pub struct TextStyleBuilder(Vec<String>);
impl TextStyleBuilder {
    pub fn from(content: String) -> Self {
        TextStyleBuilder(vec![content])
    }
}
impl TextStyleBuilder {
    pub fn append(&mut self, content: String) {
        self.0.push(content);
    }

    pub fn build(self) -> node::TextStyle {
        unimplemented!()
    }
}
