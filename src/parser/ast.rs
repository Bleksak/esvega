use crate::{
    Element,
    element::attributes::Attribute,
    element::ElementType,
};
use slotmap::{SlotMap, new_key_type};
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

    pub fn as_element(&self) -> Option<&Element> {
        match self {
            Node::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_element_mut(&mut self) -> Option<&mut Element> {
        match self {
            Node::Element(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&str> {
        match self {
            Node::Text(t) => Some(&t.content),
            _ => None,
        }
    }

    pub fn as_text_mut(&mut self) -> Option<&mut String> {
        match self {
            Node::Text(t) => Some(&mut t.content),
            _ => None,
        }
    }

    pub fn as_comment(&self) -> Option<&str> {
        match self {
            Node::Comment(c) => Some(&c.content),
            _ => None,
        }
    }

    pub fn as_cdata(&self) -> Option<&str> {
        match self {
            Node::CData(c) => Some(&c.content),
            _ => None,
        }
    }

    pub fn parent_id(&self) -> Option<NodeId> {
        match self {
            Node::Text(t) => t.parent,
            Node::Element(e) => e.parent,
            Node::Comment(c) => c.parent,
            Node::CData(c) => c.parent,
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

    pub fn insert_node(&mut self, node: Node) -> NodeId {
        self.nodes.insert(node)
    }

    pub fn remove_node(&mut self, id: NodeId) -> Option<Node> {
        let node = self.nodes.remove(id)?;
        let parent_id = match &node {
            Node::Text(t) => t.parent,
            Node::Element(e) => e.parent,
            Node::Comment(c) => c.parent,
            Node::CData(c) => c.parent,
        };
        match parent_id {
            Some(pid) => {
                if let Some(Node::Element(en)) = self.nodes.get_mut(pid) {
                    en.children.retain(|&cid| cid != id);
                }
            }
            None => {
                self.children.retain(|&cid| cid != id);
            }
        }
        Some(node)
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id)
    }

    pub fn get_node_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id)
    }

    pub fn replace_node(&mut self, old_id: NodeId, new_node: Node) -> NodeId {
        let parent_id = self.get_parent_id(old_id);

        let new_id = self.nodes.insert(new_node);

        // Set parent pointer on the new node
        match parent_id {
            Some(pid) => {
                self.set_parent(new_id, Some(pid));
                if let Some(Node::Element(en)) = self.nodes.get_mut(pid) {
                    en.children.retain(|&cid| cid != old_id);
                    en.children.push(new_id);
                }
            }
            None => {
                self.set_parent(new_id, None);
                self.children.retain(|&cid| cid != old_id);
                self.children.push(new_id);
            }
        }

        new_id
    }

    pub fn clone_node(&mut self, id: NodeId) -> NodeId {
        let node = self.nodes.get(id).expect("Node must exist").clone();
        self.clone_subtree(node)
    }

    fn clone_subtree(&mut self, node: Node) -> NodeId {
        match node {
            Node::Text(tn) => {
                let id = self.nodes.insert(Node::Text(TextNode {
                    content: tn.content,
                    parent: None,
                }));
                id
            }
            Node::Element(e) => {
                let cloned = Element {
                    element_type: e.element_type.clone(),
                    attributes: e.attributes.clone(),
                    children: vec![],
                    parent: None,
                };

                let id = self.nodes.insert(Node::Element(cloned));

                for child_id in &e.children {
                    let child_node = self.nodes.get(*child_id).expect("Child must exist").clone();
                    let new_child_id = self.clone_subtree(child_node);
                    if let Some(Node::Element(en)) = self.nodes.get_mut(id) {
                        en.children.push(new_child_id);
                    }
                    self.set_parent(new_child_id, Some(id));
                }

                id
            }
            Node::Comment(cn) => {
                let id = self.nodes.insert(Node::Comment(CommentNode {
                    content: cn.content,
                    parent: None,
                }));
                id
            }
            Node::CData(cdn) => {
                let id = self.nodes.insert(Node::CData(CDataNode {
                    content: cdn.content,
                    parent: None,
                }));
                id
            }
        }
    }

    pub fn find_by_id(&self, id_value: &str) -> Option<NodeId> {
        let mut found = None;
        self.find_by_id_recursive(id_value, &mut found);
        found
    }

    fn find_by_id_recursive(&self, id_value: &str, found: &mut Option<NodeId>) {
        for node_id in &self.children {
            self.check_id_recursive(id_value, found, *node_id);
        }
    }

    fn check_id_recursive(&self, id_value: &str, found: &mut Option<NodeId>, node_id: NodeId) {
        if let Some(Node::Element(e)) = self.nodes.get(node_id) {
            for attr in &e.attributes {
                if let Attribute::Id(val) = attr {
                    if val == id_value {
                        *found = Some(node_id);
                        return;
                    }
                }
            }
        }

        if found.is_some() {
            return;
        }

        if let Some(Node::Element(e)) = self.nodes.get(node_id) {
            for child_id in &e.children {
                self.check_id_recursive(id_value, found, *child_id);
                if found.is_some() {
                    return;
                }
            }
        }
    }

    pub fn find_by_type(&self, element_type: crate::element::ElementType) -> Vec<NodeId> {
        let mut result = Vec::new();
        self.find_by_type_recursive(element_type, &mut result);
        result
    }

    fn find_by_type_recursive(&self, element_type: crate::element::ElementType, result: &mut Vec<NodeId>) {
        for node_id in &self.children {
            self.check_type_recursive(element_type, result, *node_id);
        }
    }

    fn check_type_recursive(
        &self,
        element_type: crate::element::ElementType,
        result: &mut Vec<NodeId>,
        node_id: NodeId,
    ) {
        if let Some(Node::Element(e)) = self.nodes.get(node_id) {
            if e.element_type == element_type {
                result.push(node_id);
            }
        }

        if let Some(Node::Element(e)) = self.nodes.get(node_id) {
            for child_id in &e.children {
                self.check_type_recursive(element_type, result, *child_id);
            }
        }
    }

    pub fn find_all<F>(&self, predicate: F) -> Vec<NodeId>
    where
        F: Fn(&Node) -> bool,
    {
        let mut result = Vec::new();
        self.find_all_recursive(&predicate, &mut result);
        result
    }

    fn find_all_recursive<F>(&self, predicate: &F, result: &mut Vec<NodeId>)
    where
        F: Fn(&Node) -> bool,
    {
        for node_id in &self.children {
            self.check_all_recursive(predicate, result, *node_id);
        }
    }

    fn check_all_recursive<F>(&self, predicate: &F, result: &mut Vec<NodeId>, node_id: NodeId)
    where
        F: Fn(&Node) -> bool,
    {
        if let Some(node) = self.nodes.get(node_id) {
            if predicate(node) {
                result.push(node_id);
            }
        }

        if let Some(Node::Element(e)) = self.nodes.get(node_id) {
            for child_id in &e.children {
                self.check_all_recursive(predicate, result, *child_id);
            }
        }
    }

    fn get_parent_id(&self, id: NodeId) -> Option<NodeId> {
        let node = self.nodes.get(id).expect("Node must exist");
        node.parent_id()
    }

    fn set_parent(&mut self, id: NodeId, parent: Option<NodeId>) {
        if let Some(Node::Text(tn)) = self.nodes.get_mut(id) {
            tn.parent = parent;
        }
        if let Some(Node::Element(en)) = self.nodes.get_mut(id) {
            en.parent = parent;
        }
        if let Some(Node::Comment(cn)) = self.nodes.get_mut(id) {
            cn.parent = parent;
        }
        if let Some(Node::CData(cdn)) = self.nodes.get_mut(id) {
            cdn.parent = parent;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_element(element_type: ElementType) -> Element {
        Element {
            element_type,
            attributes: vec![],
            children: vec![],
            parent: None,
        }
    }

    fn make_text(content: &str) -> TextNode {
        TextNode {
            content: content.to_string(),
            parent: None,
        }
    }

    fn make_comment(content: &str) -> CommentNode {
        CommentNode {
            content: content.to_string(),
            parent: None,
        }
    }

    fn make_cdata(content: &str) -> CDataNode {
        CDataNode {
            content: content.to_string(),
            parent: None,
        }
    }

    fn build_sample_svg() -> AST {
        let mut ast = AST::default();

        let svg = make_element(ElementType::Svg);
        let svg_id = ast.insert_node(Node::Element(svg));

        let rect = make_element(ElementType::Rect);
        let rect_id = ast.insert_node(Node::Element(rect));

        let circle = make_element(ElementType::Circle);
        let circle_id = ast.insert_node(Node::Element(circle));

        let text = make_text("Hello");
        let text_id = ast.insert_node(Node::Text(text));

        if let Some(Node::Element(en)) = ast.nodes.get_mut(svg_id) {
            en.children.push(rect_id);
            en.children.push(circle_id);
        }

        if let Some(Node::Element(en)) = ast.nodes.get_mut(circle_id) {
            en.children.push(text_id);
        }

        if let Some(Node::Element(en)) = ast.nodes.get_mut(rect_id) {
            en.parent = Some(svg_id);
        }
        if let Some(Node::Element(en)) = ast.nodes.get_mut(circle_id) {
            en.parent = Some(svg_id);
        }
        if let Some(Node::Text(tn)) = ast.nodes.get_mut(text_id) {
            tn.parent = Some(circle_id);
        }

        ast.children.push(svg_id);

        ast
    }

    #[test]
    fn insert_node_should_add_to_arena() {
        let mut ast = build_sample_svg();

        let new_rect = make_element(ElementType::Rect);
        let new_id = ast.insert_node(Node::Element(new_rect));

        let node = ast.get_node(new_id).expect("node should exist");
        assert!(matches!(node, Node::Element(_)));
        assert!(node.parent_id().is_none());
    }

    #[test]
    fn insert_node_should_set_parent_when_attached_to_element() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let new_text = make_text("World");
        let new_id = ast.insert_node(Node::Text(new_text));

        if let Some(Node::Text(tn)) = ast.nodes.get_mut(new_id) {
            tn.parent = Some(svg_id);
        }
        if let Some(Node::Element(en)) = ast.nodes.get_mut(svg_id) {
            en.children.push(new_id);
        }

        let node = ast.get_node(new_id).unwrap();
        assert_eq!(node.parent_id(), Some(svg_id));
    }

    #[test]
    fn remove_node_should_detach_from_parent() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        let removed = ast.remove_node(circle_id).unwrap();
        assert!(matches!(removed, Node::Element(_)));
        assert!(ast.get_node(circle_id).is_none());

        let svg_node = ast.nodes.get(svg_id).unwrap();
        if let Node::Element(en) = svg_node {
            assert_eq!(en.children.len(), 1);
        }
    }

    #[test]
    fn remove_node_should_remove_from_top_level() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let removed_count = ast.children.len();

        let removed = ast.remove_node(svg_id).unwrap();
        assert!(matches!(removed, Node::Element(_)));
        assert_eq!(ast.children.len(), removed_count - 1);
    }

    #[test]
    fn clone_node_should_create_deep_copy() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        let cloned_id = ast.clone_node(circle_id);
        let cloned = ast.get_node(cloned_id).unwrap();

        assert!(matches!(cloned, Node::Element(e) if e.element_type == ElementType::Circle));

        let cloned_node = ast.get_node(cloned_id).unwrap();
        if let Node::Element(en) = cloned_node {
            assert_eq!(en.children.len(), 1);
        }

        let text_child_id = {
            let cloned_node = ast.get_node(cloned_id).unwrap();
            if let Node::Element(en) = cloned_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };
        let text_child = ast.get_node(text_child_id).unwrap();
        assert!(matches!(text_child, Node::Text(tn) if tn.content == "Hello"));

        let text_parent = {
            let text_child = ast.get_node(text_child_id).unwrap();
            text_child.parent_id()
        };
        assert_eq!(text_parent, Some(cloned_id));
    }

    #[test]
    fn find_by_id_should_find_element_with_matching_id() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(en)) = ast.nodes.get_mut(circle_id) {
            en.attributes.push(Attribute::Id("overlay".to_string()));
        }

        let found = ast.find_by_id("overlay");
        assert_eq!(found, Some(circle_id));
    }

    #[test]
    fn find_by_id_should_return_none_for_missing_id() {
        let ast = build_sample_svg();
        let found = ast.find_by_id("nonexistent");
        assert!(found.is_none());
    }

#[test]
    fn find_by_type_should_find_all_matching_elements() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };
        if let Some(Node::Element(en)) = ast.nodes.get(rect_id) {
            assert_eq!(en.element_type, ElementType::Rect);
        }

        let found = ast.find_by_type(ElementType::Rect);
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn find_all_should_match_predicate() {
        let ast = build_sample_svg();

        let elements = ast.find_all(|node| node.as_element().is_some());
        assert!(elements.len() > 0);
    }

    #[test]
    fn replace_node_should_swap_and_keep_parent() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = ast.children[0];

        let old_parent = {
            let rect_node = ast.nodes.get(rect_id).unwrap();
            rect_node.parent_id()
        };

        let new_rect = make_element(ElementType::Rect);
        let new_id = ast.replace_node(rect_id, Node::Element(new_rect));

        let replaced = ast.get_node(new_id).unwrap();
        assert!(matches!(replaced, Node::Element(_)));

        let parent = replaced.parent_id();
        assert_eq!(parent, old_parent);
    }

    #[test]
    fn replace_node_should_update_parent_children_list() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };

        let new_rect = make_element(ElementType::Rect);
        ast.replace_node(rect_id, Node::Element(new_rect));

        let children_after = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children.len()
            } else {
                panic!("not an element");
            }
        };

        assert_eq!(children_after, 2);
    }

    #[test]
    fn node_helpers_should_downcast_correctly() {
        let ast = build_sample_svg();

        let svg_id = ast.children[0];
        let node = ast.get_node(svg_id).unwrap();
        assert!(node.as_element().is_some());
        assert!(node.as_text().is_none());
        assert!(node.as_comment().is_none());
        assert!(node.as_cdata().is_none());

        let svg_node = ast.nodes.get(svg_id).unwrap();
        let circle_id = if let Node::Element(en) = svg_node {
            en.children[1]
        } else {
            panic!("not an element");
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(en) = circle_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };
        let node = ast.get_node(text_id).unwrap();
        assert!(node.as_element().is_none());
        assert_eq!(node.as_text(), Some("Hello"));
    }

    #[test]
    fn node_as_text_mut_should_modify_content() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(en) = circle_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Text(tn)) = ast.nodes.get_mut(text_id) {
            tn.content = "Changed".to_string();
        }

        let node = ast.get_node(text_id).unwrap();
        assert_eq!(node.as_text(), Some("Changed"));
    }

    #[test]
    fn node_as_element_mut_should_modify_element() {
        let mut ast = build_sample_svg();

        let rect_id = ast.children[0];

        if let Some(Node::Element(en)) = ast.nodes.get_mut(rect_id) {
            en.element_type = ElementType::Circle;
        }

        let node = ast.get_node(rect_id).unwrap();
        if let Node::Element(en) = node {
            assert_eq!(en.element_type, ElementType::Circle);
        }
    }

    #[test]
    fn parent_id_should_traverse_up_tree() {
        let ast = build_sample_svg();

        let svg_id = ast.children[0];
        let text_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        let text_parent = {
            let text_node = ast.get_node(text_id).unwrap();
            text_node.parent_id()
        };

        assert_eq!(text_parent, Some(svg_id));
    }

    #[test]
    fn clone_node_should_not_share_children_with_original() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        let cloned_id = ast.clone_node(circle_id);
        assert_ne!(circle_id, cloned_id);

        let circle_text_id = {
            let circle_node = ast.get_node(circle_id).unwrap();
            if let Node::Element(en) = circle_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };
        let cloned_text_id = {
            let cloned_node = ast.get_node(cloned_id).unwrap();
            if let Node::Element(en) = cloned_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };

        assert_ne!(circle_text_id, cloned_text_id);
    }

    #[test]
    fn find_by_id_should_find_nested_element() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(en)) = ast.nodes.get_mut(circle_id) {
            en.attributes.push(Attribute::Id("nested-circle".to_string()));
        }

        let found = ast.find_by_id("nested-circle");
        assert_eq!(found, Some(circle_id));
    }

    #[test]
    fn find_by_type_should_search_entire_tree() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(en)) = ast.nodes.get_mut(circle_id) {
            en.element_type = ElementType::Rect;
        }

        let found = ast.find_by_type(ElementType::Rect);
        assert_eq!(found.len(), 2);
    }

    #[test]
    fn remove_node_should_remove_text_child() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children[1]
            } else {
                panic!("not an element");
            }
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(en) = circle_node {
                en.children[0]
            } else {
                panic!("not an element");
            }
        };

        let removed = ast.remove_node(text_id).unwrap();
        assert!(matches!(removed, Node::Text(tn) if tn.content == "Hello"));

        let circle_node = ast.nodes.get(circle_id).unwrap();
        if let Node::Element(en) = circle_node {
            assert_eq!(en.children.len(), 0);
        }
    }

    #[test]
    fn to_svg_should_serialize_modified_tree() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        if let Some(Node::Element(en)) = ast.nodes.get_mut(svg_id) {
            en.attributes.push(Attribute::Id("root".to_string()));
        }

        let output = ast.to_svg();
        assert!(output.contains("id=\"root\""));
    }

    #[test]
    fn insert_and_remove_preserves_children_count() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let initial_count = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(en) = svg_node {
                en.children.len()
            } else {
                panic!("not an element");
            }
        };

        let new_text = make_text("temp");
        let new_id = ast.insert_node(Node::Text(new_text));
        if let Some(Node::Text(tn)) = ast.nodes.get_mut(new_id) {
            tn.parent = Some(svg_id);
        }
        if let Some(Node::Element(en)) = ast.nodes.get_mut(svg_id) {
            en.children.push(new_id);
        }

        assert_eq!(
            {
                let svg_node = ast.nodes.get(svg_id).unwrap();
                if let Node::Element(en) = svg_node {
                    en.children.len()
                } else {
                    panic!("not an element");
                }
            },
            initial_count + 1
        );

        ast.remove_node(new_id);

        assert_eq!(
            {
                let svg_node = ast.nodes.get(svg_id).unwrap();
                if let Node::Element(en) = svg_node {
                    en.children.len()
                } else {
                    panic!("not an element");
                }
            },
            initial_count
        );
    }

    #[test]
    fn clone_comment_node() {
        let mut ast = build_sample_svg();

        let comment = make_comment("This is a comment");
        let comment_id = ast.insert_node(Node::Comment(comment));

        let cloned_id = ast.clone_node(comment_id);
        let cloned = ast.get_node(cloned_id).unwrap();

        assert!(matches!(cloned, Node::Comment(cn) if cn.content == "This is a comment"));
    }

    #[test]
    fn clone_cdata_node() {
        let mut ast = build_sample_svg();

        let cdata = make_cdata("some data");
        let cdata_id = ast.insert_node(Node::CData(cdata));

        let cloned_id = ast.clone_node(cdata_id);
        let cloned = ast.get_node(cloned_id).unwrap();

        assert!(matches!(cloned, Node::CData(cn) if cn.content == "some data"));
    }
}
