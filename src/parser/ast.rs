use crate::Element;
use slotmap::SlotMap;
use slotmap::new_key_type;

new_key_type! {
    pub struct NodeId;
}

#[derive(Debug, Default)]
pub struct AST {
    pub nodes: SlotMap<NodeId, Element>,
    pub children: Vec<NodeId>,
}

impl AST {
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();

        for node_id in &self.children {
            let Some(element) = self.nodes.get(*node_id) else {
                continue;
            };

            svg.push_str(&element.to_svg(self, 0));
        }

        svg
    }
}
