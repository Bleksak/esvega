use crate::Element;
use slotmap::SlotMap;
use slotmap::new_key_type;

new_key_type! {
    pub struct NodeId;
}

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Text(String),
    Element(Element),
    Comment(String),
    CData(String),
}

impl Node {
    pub fn to_svg(&self, ast: &AST, current_indent: usize) -> String {
        let mut svg = String::new();

        match self {
            Node::Text(s) => {
                for _ in 0..current_indent {
                    svg.push_str("  ");
                }

                svg.push_str(s);
                svg.push('\n');
            }
            Node::Element(element) => svg.push_str(&element.to_svg(ast, current_indent)),
            Node::Comment(comment) => {
                svg.push_str("<!-- ");
                svg.push_str(comment);
                svg.push_str(" -->");
            }
            Node::CData(cdata) => {
                svg.push_str("<![CDATA[");
                svg.push_str(cdata);
                svg.push_str("]]>");
            }
        }

        svg
    }
}

#[derive(Debug, Default)]
pub struct AST {
    pub nodes: SlotMap<NodeId, Node>,
    pub children: Vec<NodeId>,
}

impl AST {
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();

        for node_id in &self.children {
            let Some(node) = self.nodes.get(*node_id) else {
                continue;
            };

            svg.push_str(&node.to_svg(self, 0));
        }

        svg
    }
}
