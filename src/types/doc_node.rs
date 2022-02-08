use ego_tree::NodeRef;
use scraper::ElementRef;
use scraper::Node;
use std::fmt;

pub struct DocNode<'a> {
    pub name: &'a str,
    pub text: &'a str,
    pub node: NodeRef<'a, Node>,
}

impl<'a> fmt::Debug for DocNode<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(element) = ElementRef::wrap(self.node) {
            let name: String = element.value().name().to_string();
            write!(f, "<{}({})>{}</{}>", self.name, name, self.text, self.name)
        } else {
            write!(f, "<{}(None)>{}</{}>", self.name, self.text, self.name)
        }
    }
}
