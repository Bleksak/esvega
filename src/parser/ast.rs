use crate::Element;
use slotmap::SlotMap;
use slotmap::new_key_type;
use std::fmt;

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
    pub fn write_svg(&self, ast: &AST, f: &mut impl fmt::Write, indent: usize) -> fmt::Result {
        match self {
            Node::Text(s) => {
                for _ in 0..indent {
                    write!(f, "  ")?;
                }
                write!(f, "{}\n", s)
            }
            Node::Element(element) => element.write_svg(ast, f, indent),
            Node::Comment(comment) => write!(f, "<!-- {} -->", comment),
            Node::CData(cdata) => write!(f, "<![CDATA[{}]]>", cdata),
        }
    }
}

#[derive(Debug, Default)]
pub struct AST {
    pub nodes: SlotMap<NodeId, Node>,
    pub children: Vec<NodeId>,
}

impl AST {
    pub fn to_svg(&self) -> String {
        let mut s = String::new();
        for node_id in &self.children {
            let Some(node) = self.nodes.get(*node_id) else {
                continue;
            };
            node.write_svg(self, &mut s, 0).unwrap();
        }
        s
    }
}
