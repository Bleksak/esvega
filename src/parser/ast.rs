use crate::{Element, element::ElementType, element::attributes::Attribute};
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
                if let Some(Node::Element(element)) = self.nodes.get_mut(pid) {
                    element.children.retain(|&cid| cid != id);
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
                if let Some(Node::Element(element)) = self.nodes.get_mut(pid) {
                    element.children.retain(|&cid| cid != old_id);
                    element.children.push(new_id);
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
            Node::Text(text) => {
                let id = self.nodes.insert(Node::Text(TextNode {
                    content: text.content,
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
                    if let Some(Node::Element(element)) = self.nodes.get_mut(id) {
                        element.children.push(new_child_id);
                    }
                    self.set_parent(new_child_id, Some(id));
                }

                id
            }
            Node::Comment(comment) => {
                let id = self.nodes.insert(Node::Comment(CommentNode {
                    content: comment.content,
                    parent: None,
                }));
                id
            }
            Node::CData(cdata) => {
                let id = self.nodes.insert(Node::CData(CDataNode {
                    content: cdata.content,
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

    fn find_by_type_recursive(
        &self,
        element_type: crate::element::ElementType,
        result: &mut Vec<NodeId>,
    ) {
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
        if let Some(Node::Text(text)) = self.nodes.get_mut(id) {
            text.parent = parent;
        }
        if let Some(Node::Element(element)) = self.nodes.get_mut(id) {
            element.parent = parent;
        }
        if let Some(Node::Comment(comment)) = self.nodes.get_mut(id) {
            comment.parent = parent;
        }
        if let Some(Node::CData(cdata)) = self.nodes.get_mut(id) {
            cdata.parent = parent;
        }
    }

    fn get_children(&self, id: NodeId) -> Option<&Vec<NodeId>> {
        self.nodes.get(id).and_then(|n| {
            if let Node::Element(e) = n {
                Some(&e.children)
            } else {
                None
            }
        })
    }

    fn get_children_mut(&mut self, id: NodeId) -> Option<&mut Vec<NodeId>> {
        self.nodes.get_mut(id).and_then(|n| {
            if let Node::Element(e) = n {
                Some(&mut e.children)
            } else {
                None
            }
        })
    }

    pub fn append_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        let new_parent_id = {
            let child_node = self.nodes.get(child_id).expect("Child node must exist");
            child_node.parent_id()
        };

        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            element.children.push(child_id);
        }

        self.set_parent(child_id, Some(parent_id));

        if let Some(old_parent) = new_parent_id {
            if old_parent != parent_id {
                if let Some(Node::Element(element)) = self.nodes.get_mut(old_parent) {
                    element.children.retain(|&cid| cid != child_id);
                }
            }
        }
    }

    pub fn prepend_child(&mut self, parent_id: NodeId, child_id: NodeId) {
        let new_parent_id = {
            let child_node = self.nodes.get(child_id).expect("Child node must exist");
            child_node.parent_id()
        };

        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            element.children.insert(0, child_id);
        }

        self.set_parent(child_id, Some(parent_id));

        if let Some(old_parent) = new_parent_id {
            if old_parent != parent_id {
                if let Some(Node::Element(element)) = self.nodes.get_mut(old_parent) {
                    element.children.retain(|&cid| cid != child_id);
                }
            }
        }
    }

    pub fn insert_child_at(&mut self, parent_id: NodeId, child_id: NodeId, index: usize) {
        let new_parent_id = {
            let child_node = self.nodes.get(child_id).expect("Child node must exist");
            child_node.parent_id()
        };

        let clamped = index.min(self.get_children(parent_id).map_or(0, |c| c.len()));

        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            element.children.insert(clamped, child_id);
        }

        self.set_parent(child_id, Some(parent_id));

        if let Some(old_parent) = new_parent_id {
            if old_parent != parent_id {
                if let Some(Node::Element(element)) = self.nodes.get_mut(old_parent) {
                    element.children.retain(|&cid| cid != child_id);
                }
            }
        }
    }

    pub fn remove_child_at(&mut self, parent_id: NodeId, index: usize) -> Option<Node> {
        let child_id = {
            let children = self.get_children(parent_id)?;
            if index >= children.len() {
                return None;
            }
            children[index]
        };

        self.remove_node(child_id)
    }

    pub fn replace_child_at(&mut self, parent_id: NodeId, index: usize, new_node: Node) -> NodeId {
        let child_id = match self.get_children(parent_id) {
            Some(children) => {
                if index >= children.len() {
                    panic!(
                        "Index {} out of bounds (children count: {})",
                        index,
                        children.len()
                    );
                }
                children[index]
            }
            None => panic!("Parent node is not an element"),
        };

        let new_id = self.nodes.insert(new_node);

        self.set_parent(new_id, Some(parent_id));

        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            element.children[index] = new_id;
        }

        self.nodes.remove(child_id);

        new_id
    }

    pub fn reorder_children(&mut self, parent_id: NodeId, new_order: &[NodeId]) {
        let orphaned: Vec<_> = {
            let children = self.get_children(parent_id).map_or(vec![], |c| c.clone());
            children
                .into_iter()
                .filter(|cid| !new_order.contains(cid))
                .collect()
        };

        for child_id in orphaned {
            if let Some(Node::Element(element)) = self.nodes.get_mut(child_id) {
                element.parent = None;
            }
        }

        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            element.children = new_order.to_vec();
        }

        for &child_id in new_order {
            self.set_parent(child_id, Some(parent_id));
        }
    }

    pub fn move_child(
        &mut self,
        from_parent_id: NodeId,
        to_parent_id: NodeId,
        from_index: usize,
        to_index: Option<usize>,
    ) -> Option<Node> {
        let child_id = {
            let children = self.get_children(from_parent_id)?;
            if from_index >= children.len() {
                return None;
            }
            children[from_index]
        };

        if let Some(Node::Element(element)) = self.nodes.get_mut(from_parent_id) {
            element.children.remove(from_index);
        }

        self.set_parent(child_id, Some(to_parent_id));

        let target_len = self.get_children(to_parent_id).map_or(0, |c| c.len());
        let to_idx = to_index.unwrap_or(target_len).min(target_len);

        if let Some(Node::Element(element)) = self.nodes.get_mut(to_parent_id) {
            element.children.insert(to_idx, child_id);
        }

        self.nodes.get(child_id).cloned()
    }

    pub fn swap_children(&mut self, parent_id: NodeId, i: usize, j: usize) {
        if let Some(Node::Element(element)) = self.nodes.get_mut(parent_id) {
            let len = element.children.len();
            if i >= len || j >= len {
                panic!(
                    "Index out of bounds (children count: {}, i: {}, j: {})",
                    len, i, j
                );
            }
            element.children.swap(i, j);
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

        if let Some(Node::Element(element)) = ast.nodes.get_mut(svg_id) {
            element.children.push(rect_id);
            element.children.push(circle_id);
        }

        if let Some(Node::Element(element)) = ast.nodes.get_mut(circle_id) {
            element.children.push(text_id);
        }

        if let Some(Node::Element(element)) = ast.nodes.get_mut(rect_id) {
            element.parent = Some(svg_id);
        }
        if let Some(Node::Element(element)) = ast.nodes.get_mut(circle_id) {
            element.parent = Some(svg_id);
        }
        if let Some(Node::Text(text)) = ast.nodes.get_mut(text_id) {
            text.parent = Some(circle_id);
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

        if let Some(Node::Text(text)) = ast.nodes.get_mut(new_id) {
            text.parent = Some(svg_id);
        }
        if let Some(Node::Element(element)) = ast.nodes.get_mut(svg_id) {
            element.children.push(new_id);
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        let removed = ast.remove_node(circle_id).unwrap();
        assert!(matches!(removed, Node::Element(_)));
        assert!(ast.get_node(circle_id).is_none());

        let svg_node = ast.nodes.get(svg_id).unwrap();
        if let Node::Element(element) = svg_node {
            assert_eq!(element.children.len(), 1);
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        let cloned_id = ast.clone_node(circle_id);
        let cloned = ast.get_node(cloned_id).unwrap();

        assert!(matches!(cloned, Node::Element(e) if e.element_type == ElementType::Circle));

        let cloned_node = ast.get_node(cloned_id).unwrap();
        if let Node::Element(element) = cloned_node {
            assert_eq!(element.children.len(), 1);
        }

        let text_child_id = {
            let cloned_node = ast.get_node(cloned_id).unwrap();
            if let Node::Element(element) = cloned_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        let text_child = ast.get_node(text_child_id).unwrap();
        assert!(matches!(text_child, Node::Text(text) if text.content == "Hello"));

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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(element)) = ast.nodes.get_mut(circle_id) {
            element
                .attributes
                .push(Attribute::Id("overlay".to_string()));
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
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        if let Some(Node::Element(element)) = ast.nodes.get(rect_id) {
            assert_eq!(element.element_type, ElementType::Rect);
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
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        let new_rect = make_element(ElementType::Rect);
        ast.replace_node(rect_id, Node::Element(new_rect));

        let children_after = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
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
        let circle_id = if let Node::Element(element) = svg_node {
            element.children[1]
        } else {
            panic!("not an element");
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children[0]
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Text(text)) = ast.nodes.get_mut(text_id) {
            text.content = "Changed".to_string();
        }

        let node = ast.get_node(text_id).unwrap();
        assert_eq!(node.as_text(), Some("Changed"));
    }

    #[test]
    fn node_as_element_mut_should_modify_element() {
        let mut ast = build_sample_svg();

        let rect_id = ast.children[0];

        if let Some(Node::Element(element)) = ast.nodes.get_mut(rect_id) {
            element.element_type = ElementType::Circle;
        }

        let node = ast.get_node(rect_id).unwrap();
        if let Node::Element(element) = node {
            assert_eq!(element.element_type, ElementType::Circle);
        }
    }

    #[test]
    fn parent_id_should_traverse_up_tree() {
        let ast = build_sample_svg();

        let svg_id = ast.children[0];
        let text_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[1]
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        let cloned_id = ast.clone_node(circle_id);
        assert_ne!(circle_id, cloned_id);

        let circle_text_id = {
            let circle_node = ast.get_node(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        let cloned_text_id = {
            let cloned_node = ast.get_node(cloned_id).unwrap();
            if let Node::Element(element) = cloned_node {
                element.children[0]
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(element)) = ast.nodes.get_mut(circle_id) {
            element
                .attributes
                .push(Attribute::Id("nested-circle".to_string()));
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        if let Some(Node::Element(element)) = ast.nodes.get_mut(circle_id) {
            element.element_type = ElementType::Rect;
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
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        let removed = ast.remove_node(text_id).unwrap();
        assert!(matches!(removed, Node::Text(text) if text.content == "Hello"));

        let circle_node = ast.nodes.get(circle_id).unwrap();
        if let Node::Element(element) = circle_node {
            assert_eq!(element.children.len(), 0);
        }
    }

    #[test]
    fn to_svg_should_serialize_modified_tree() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        if let Some(Node::Element(element)) = ast.nodes.get_mut(svg_id) {
            element.attributes.push(Attribute::Id("root".to_string()));
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
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };

        let new_text = make_text("temp");
        let new_id = ast.insert_node(Node::Text(new_text));
        if let Some(Node::Text(text)) = ast.nodes.get_mut(new_id) {
            text.parent = Some(svg_id);
        }
        if let Some(Node::Element(element)) = ast.nodes.get_mut(svg_id) {
            element.children.push(new_id);
        }

        assert_eq!(
            {
                let svg_node = ast.nodes.get(svg_id).unwrap();
                if let Node::Element(element) = svg_node {
                    element.children.len()
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
                if let Node::Element(element) = svg_node {
                    element.children.len()
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

        assert!(matches!(cloned, Node::Comment(comment) if comment.content == "This is a comment"));
    }

    #[test]
    fn clone_cdata_node() {
        let mut ast = build_sample_svg();

        let cdata = make_cdata("some data");
        let cdata_id = ast.insert_node(Node::CData(cdata));

        let cloned_id = ast.clone_node(cdata_id);
        let cloned = ast.get_node(cloned_id).unwrap();

        assert!(matches!(cloned, Node::CData(cdata) if cdata.content == "some data"));
    }

    #[test]
    fn append_child_should_add_to_end() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let initial_len = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };

        let new_circle = make_element(ElementType::Circle);
        let circle_id = ast.insert_node(Node::Element(new_circle));

        ast.append_child(svg_id, circle_id);

        let final_len = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };

        assert_eq!(final_len, initial_len + 1);

        let last_child = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[element.children.len() - 1]
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(last_child, circle_id);

        let circle_parent = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            circle_node.parent_id()
        };
        assert_eq!(circle_parent, Some(svg_id));
    }

    #[test]
    fn prepend_child_should_add_to_beginning() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let initial_len = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };

        let new_rect = make_element(ElementType::Rect);
        let rect_id = ast.insert_node(Node::Element(new_rect));

        ast.prepend_child(svg_id, rect_id);

        let first_child = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(first_child, rect_id);

        let rect_parent = {
            let rect_node = ast.nodes.get(rect_id).unwrap();
            rect_node.parent_id()
        };
        assert_eq!(rect_parent, Some(svg_id));
    }

    #[test]
    fn insert_child_at_should_place_at_index() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];

        let new_text = make_text("Middle");
        let text_id = ast.insert_node(Node::Text(new_text));

        ast.insert_child_at(svg_id, text_id, 1);

        let children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.clone()
            } else {
                panic!("not an element");
            }
        };

        assert_eq!(children.len(), 3);

        let node = ast.get_node(children[1]).unwrap();
        assert!(matches!(node, Node::Text(text) if text.content == "Middle"));
    }

    #[test]
    fn insert_child_at_should_clamp_to_end() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];

        let new_rect = make_element(ElementType::Rect);
        let rect_id = ast.insert_node(Node::Element(new_rect));

        ast.insert_child_at(svg_id, rect_id, 999);

        let last_child = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[element.children.len() - 1]
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(last_child, rect_id);
    }

    #[test]
    fn remove_child_at_should_remove_and_update_parent() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        let removed = ast.remove_child_at(svg_id, 0).unwrap();
        assert!(matches!(removed, Node::Element(_)));

        let children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(children, 1);

        assert!(ast.get_node(rect_id).is_none());
    }

    #[test]
    fn remove_child_at_should_return_none_for_out_of_bounds() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let removed = ast.remove_child_at(svg_id, 99);
        assert!(removed.is_none());
    }

    #[test]
    fn replace_child_at_should_swap_node() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        let new_circle = make_element(ElementType::Circle);

        let replaced_id = ast.replace_child_at(svg_id, 0, Node::Element(new_circle));

        let new_child = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(new_child, replaced_id);

        let replaced_node = ast.get_node(replaced_id).unwrap();
        assert!(matches!(replaced_node, Node::Element(e) if e.element_type == ElementType::Circle));

        let child_parent = {
            let node = ast.nodes.get(replaced_id).unwrap();
            node.parent_id()
        };
        assert_eq!(child_parent, Some(svg_id));

        assert!(ast.get_node(rect_id).is_none());
    }

    #[test]
    fn replace_child_at_should_panic_for_out_of_bounds() {
        let mut ast = build_sample_svg();
        let svg_id = ast.children[0];

        let new_rect = make_element(ElementType::Rect);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _ = ast.replace_child_at(svg_id, 99, Node::Element(new_rect));
        }));
        assert!(result.is_err());
    }

    #[test]
    fn reorder_children_should_rearrange_order() {
        let ast = build_sample_svg();

        let svg_id = ast.children[0];
        let children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.clone()
            } else {
                panic!("not an element");
            }
        };

        let mut ast = build_sample_svg();
        let reversed: Vec<_> = children.iter().rev().cloned().collect();
        ast.reorder_children(svg_id, &reversed);

        let new_children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.clone()
            } else {
                panic!("not an element");
            }
        };

        for child_id in new_children.iter() {
            let parent = {
                let node = ast.nodes.get(*child_id).unwrap();
                node.parent_id()
            };
            assert_eq!(parent, Some(svg_id));
        }
    }

    #[test]
    fn move_child_within_same_parent() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        ast.move_child(svg_id, svg_id, 0, Some(1));

        let children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.clone()
            } else {
                panic!("not an element");
            }
        };

        assert_eq!(children[0], circle_id);
        assert_eq!(children[1], rect_id);
    }

    #[test]
    fn move_child_between_parents() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        let moved = ast.move_child(svg_id, circle_id, 0, None);
        assert!(moved.is_some());

        let svg_children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(svg_children, 1);

        let circle_children = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(circle_children, 2);

        let rect_parent = {
            let node = ast.nodes.get(rect_id).unwrap();
            node.parent_id()
        };
        assert_eq!(rect_parent, Some(circle_id));
    }

    #[test]
    fn swap_children_should_exchange_positions() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };

        ast.swap_children(svg_id, 0, 1);

        let children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.clone()
            } else {
                panic!("not an element");
            }
        };

        assert_eq!(children[0], circle_id);
        assert_eq!(children[1], rect_id);
    }

    #[test]
    fn swap_children_should_panic_for_out_of_bounds() {
        let mut ast = build_sample_svg();
        let svg_id = ast.children[0];

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            ast.swap_children(svg_id, 0, 99);
        }));
        assert!(result.is_err());
    }

    #[test]
    fn append_child_should_move_from_different_parent() {
        let mut ast = build_sample_svg();

        let svg_id = ast.children[0];
        let rect_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };
        let circle_id = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children[1]
            } else {
                panic!("not an element");
            }
        };
        let text_id = {
            let circle_node = ast.nodes.get(circle_id).unwrap();
            if let Node::Element(element) = circle_node {
                element.children[0]
            } else {
                panic!("not an element");
            }
        };

        ast.append_child(rect_id, text_id);

        let text_parent = {
            let node = ast.nodes.get(text_id).unwrap();
            node.parent_id()
        };
        assert_eq!(text_parent, Some(rect_id));

        let svg_children = {
            let svg_node = ast.nodes.get(svg_id).unwrap();
            if let Node::Element(element) = svg_node {
                element.children.len()
            } else {
                panic!("not an element");
            }
        };
        assert_eq!(svg_children, 2);

        let rect_node = ast.nodes.get(rect_id).unwrap();
        if let Node::Element(element) = rect_node {
            assert_eq!(element.children.len(), 1);
        }
    }
}
