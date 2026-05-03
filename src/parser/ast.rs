use crate::Element;
use slotmap::SlotMap;
use slotmap::new_key_type;
use std::fmt;

new_key_type! {
    pub struct NodeId;
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextNode {
    pub content: String,
    pub parent: Option<NodeId>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CommentNode {
    pub content: String,
    pub parent: Option<NodeId>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct CDataNode {
    pub content: String,
    pub parent: Option<NodeId>,
}

#[derive(Clone, PartialEq, Debug)]
pub enum Node {
    Text(TextNode),
    Element(Element),
    Comment(CommentNode),
    CData(CDataNode),
}

impl Node {
    pub fn write_svg(&self, ast: &AST, f: &mut impl fmt::Write, indent: usize) -> fmt::Result {
        match self {
            Node::Text(text_node) => {
                for _ in 0..indent {
                    write!(f, "  ")?;
                }
                write!(f, "{}\n", text_node.content)
            }
            Node::Element(element) => element.write_svg(ast, f, indent),
            Node::Comment(comment_node) => write!(f, "<!-- {} -->", comment_node.content),
            Node::CData(cdata_node) => write!(f, "<![CDATA[{}]]>", cdata_node.content),
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
