use crate::{
    Element,
    parser::ast::{AST, CommentNode, Node, NodeId, TextNode},
    token::{Token, TokenKind},
};

pub mod ast;

pub struct Parser<'input> {
    lexer: crate::lexer::Lexer<'input>,
}

#[derive(Debug, Default, PartialEq)]
enum State {
    #[default]
    Text, // outside of any tag
    TagOpen, // after <
    TagName,
    AttributeName,
    AttributeEquals,
    AttributeValueOpeningQuote,
    AttributeValue,
    AttributeValueClosingQuote,
    TagClose, // after </
    TagCloseName,
}

#[derive(Debug, Default)]
struct StateMachine {
    ast: AST,
    current_state: State,
    element_stack: Vec<NodeId>,
    current_attribute: Option<String>,
}

impl StateMachine {
    fn consume(&mut self, token: Token) {
        match (&self.current_state, token.kind) {
            (State::Text, TokenKind::LessThan) => {
                self.current_state = State::TagOpen;
            }
            (State::TagOpen, TokenKind::Identifier) => {
                let Ok(element_type) = token.value.parse() else {
                    panic!(
                        "Parse error, expected a valid element name, got: {}",
                        &token.value
                    );
                };

                let element = Element {
                    element_type,
                    attributes: vec![],
                    children: vec![],
                    parent: None,
                };

                self.current_state = State::TagName;
                let node_id = self.ast.nodes.insert(Node::Element(element));
                self.element_stack.push(node_id);
            }
            (State::TagName | State::AttributeValueClosingQuote, TokenKind::Identifier) => {
                self.current_attribute = Some(token.value);
                self.current_state = State::AttributeName;
            }
            (State::AttributeName, TokenKind::Equals) => {
                self.current_state = State::AttributeEquals;
            }
            (State::AttributeName, TokenKind::Identifier) => {
                let attribute_name = self.current_attribute.take().unwrap();
                let attribute_value = "".to_string();

                let Ok(attribute) = (&attribute_name, &attribute_value).try_into() else {
                    panic!("Invalid attribute, got: {}", attribute_name);
                };

                let node_id = self.element_stack.last().unwrap();

                let Node::Element(en) = self.ast.nodes.get_mut(*node_id).unwrap() else {
                    panic!("Node is supposed to be of type Element at this point");
                };

                en.attributes.push(attribute);

                self.current_state = State::AttributeName;
                self.current_attribute = Some(token.value);
            }
            (State::AttributeName, TokenKind::GreaterThan) => {
                let attribute_name = self.current_attribute.take().unwrap();
                let attribute_value = "".to_string();

                let Ok(attribute) = (&attribute_name, &attribute_value).try_into() else {
                    panic!("Invalid attribute, got: {}", attribute_name);
                };

                let node_id = self.element_stack.last().unwrap();

                let Node::Element(en) = self.ast.nodes.get_mut(*node_id).unwrap() else {
                    panic!("Node is supposed to be of type Element at this point");
                };

                en.attributes.push(attribute);

                self.current_state = State::Text;
            }
            (State::AttributeEquals, TokenKind::Quote) => {
                self.current_state = State::AttributeValueOpeningQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Quote) => {
                self.current_state = State::AttributeValueClosingQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Literal) => {
                let attribute_name = self.current_attribute.take().unwrap();
                let attribute_value = token.value;

                let Ok(attribute) = (&attribute_name, &attribute_value).try_into() else {
                    panic!(
                        "Invalid attribute, got: {}={}",
                        attribute_name, attribute_value
                    );
                };

                let node_id = self.element_stack.last().unwrap();

                let Node::Element(en) = self.ast.nodes.get_mut(*node_id).unwrap() else {
                    panic!("Node is supposed to be of type Element at this point");
                };

                en.attributes.push(attribute);

                self.current_state = State::AttributeValue;
                self.current_attribute = None;
            }
            (State::AttributeValue, TokenKind::Quote) => {
                self.current_state = State::AttributeValueClosingQuote;
            }
            (State::TagName | State::AttributeValueClosingQuote, TokenKind::GreaterThan) => {
                // here we are in a <tagname>
                self.current_state = State::Text;
            }

            (
                State::TagName | State::AttributeValueClosingQuote | State::AttributeName,
                TokenKind::SlashGreaterThan,
            ) => {
                // here we are in a <tagname/>
                if self.current_state == State::AttributeName {
                    let attribute_name = self.current_attribute.take().unwrap();
                    let attribute_value = "".to_string();

                    let Ok(attribute) = (&attribute_name, &attribute_value).try_into() else {
                        panic!("Invalid attribute, got: {}", attribute_name);
                    };

                    let node_id = self.element_stack.last().unwrap();

                    let Node::Element(en) = self.ast.nodes.get_mut(*node_id).unwrap() else {
                        panic!("Node is supposed to be of type Element at this point");
                    };

                    en.attributes.push(attribute);
                }

                self.current_state = State::Text;

                let element_id = self.element_stack.pop().unwrap();
                if let Some(last_element_id) = self.element_stack.last() {
                    let child_type = match self.ast.nodes.get(element_id).unwrap() {
                        Node::Element(e) => e.element_type,
                        _ => panic!("Node is supposed to be of type Element at this point"),
                    };

                    let Node::Element(last_en) = self.ast.nodes.get_mut(*last_element_id).unwrap()
                    else {
                        panic!("Node is supposed to be of type Element at this point");
                    };

                    if !last_en.is_allowed_as_child(&child_type) {
                        panic!(
                            "<{}> is not allowed as a child of <{}>",
                            child_type, last_en.element_type
                        );
                    }

                    let parent_id = *last_element_id;
                    let child_id = element_id;
                    {
                        last_en.children.push(child_id);
                    }
                    if let Some(Node::Element(en)) = self.ast.nodes.get_mut(child_id) {
                        en.parent = Some(parent_id);
                    }
                } else {
                    self.ast.children.push(element_id);
                }
            }
            (State::Text, TokenKind::LessThanSlash) => {
                self.current_state = State::TagClose;
            }
            (State::TagClose, TokenKind::Identifier) => {
                let Some(element_id) = self.element_stack.pop() else {
                    panic!("Element stack is empty");
                };

                let child_type = match self.ast.nodes.get(element_id).unwrap() {
                    Node::Element(e) => {
                        if e.element_type.as_str() != token.value {
                            panic!(
                                "Element stack does not match closing tag {} != {}",
                                e.element_type, token.value
                            );
                        }
                        e.element_type
                    }
                    _ => panic!("Node is supposed to be of type Element at this point"),
                };

                if let Some(last_element_id) = self.element_stack.last_mut() {
                    let Node::Element(last_en) = self.ast.nodes.get_mut(*last_element_id).unwrap()
                    else {
                        panic!("Node is supposed to be of type Element at this point");
                    };

                    if !last_en.is_allowed_as_child(&child_type) {
                        panic!(
                            "<{}> is not allowed as a child of <{}>",
                            child_type, last_en.element_type
                        );
                    }

                    let parent_id = *last_element_id;
                    let child_id = element_id;
                    {
                        last_en.children.push(child_id);
                    }
                    if let Some(Node::Element(en)) = self.ast.nodes.get_mut(child_id) {
                        en.parent = Some(parent_id);
                    }
                } else {
                    self.ast.children.push(element_id);
                }

                self.current_state = State::TagCloseName;
            }
            (State::TagCloseName, TokenKind::GreaterThan) => {
                self.current_state = State::Text;
            }
            (State::TagCloseName, TokenKind::SlashGreaterThan) => {
                // TODO: This is not a self closing element, it has a typo, we should be able to
                // parse this correctly, but we also need to emit a warning in the CLI
                unimplemented!("Tag close name should not be followed by a slash");
            }
            (State::TagCloseName, TokenKind::Identifier) => {
                // TODO: error
                panic!("Unexpected identifier after tag close name");
            }
            (State::Text, TokenKind::Text) => {
                let node_id = self.ast.nodes.insert(Node::Text(TextNode {
                    content: token.value,
                    parent: None,
                }));

                let last_element_id = self.element_stack.last().unwrap();

                let Node::Element(last_en) = self.ast.nodes.get_mut(*last_element_id).unwrap()
                else {
                    panic!("Node is supposed to be of type Element at this point");
                };

                let parent_id = *last_element_id;
                let child_id = node_id;
                {
                    last_en.children.push(child_id);
                }
                if let Some(Node::Text(tn)) = self.ast.nodes.get_mut(child_id) {
                    tn.parent = Some(parent_id);
                }
            }
            (
                State::Text,
                TokenKind::GreaterThan
                | TokenKind::SlashGreaterThan
                | TokenKind::Identifier
                | TokenKind::Literal
                | TokenKind::Quote
                | TokenKind::Equals,
            ) => {
                unreachable!("Unexpected token after tag close name");
            }
            (State::TagCloseName, TokenKind::Literal) => {
                unreachable!("Unexpected literal after tag close name");
            }
            (_, TokenKind::Comment) => {
                let content = &token.value[4..token.value.len() - 3];
                let node_id = self.ast.nodes.insert(Node::Comment(CommentNode {
                    content: content.to_string(),
                    parent: None,
                }));

                if let Some(last_element_id) = self.element_stack.last() {
                    let Node::Element(last_en) = self.ast.nodes.get_mut(*last_element_id).unwrap()
                    else {
                        panic!("Node is supposed to be of type Element at this point");
                    };
                    let parent_id = *last_element_id;
                    let child_id = node_id;
                    {
                        last_en.children.push(child_id);
                    }
                    if let Some(Node::Comment(cn)) = self.ast.nodes.get_mut(child_id) {
                        cn.parent = Some(parent_id);
                    }
                } else {
                    self.ast.children.push(node_id);
                }
            }
            (State::TagOpen, TokenKind::LessThan | TokenKind::LessThanSlash) => {
                panic!("Unexpected < after tag open (sequence <<) ");
            }
            (_, TokenKind::Text) => {
                unreachable!("Unexpected text token");
            }
            (State::TagOpen, TokenKind::GreaterThan | TokenKind::SlashGreaterThan) => {
                panic!("Unexpected > after tag open (sequence <>) ");
            }
            (_, TokenKind::Literal) => {
                unreachable!("Unexpected literal token");
            }
            (_, TokenKind::Quote) => {
                unreachable!("Unexpected quote token");
            }
            (_, TokenKind::LessThan | TokenKind::LessThanSlash) => {
                unreachable!("Unexpected opening tag token");
            }
            (_, TokenKind::Equals) => {
                unreachable!("Unexpected equals token");
            }
            (State::AttributeEquals, _) => {
                unreachable!("Unexpected token after attribute equals");
            }
            (
                State::AttributeValueOpeningQuote | State::AttributeValue,
                TokenKind::GreaterThan | TokenKind::SlashGreaterThan | TokenKind::Identifier,
            ) => {
                unreachable!("Unexpected > after attribute value opening quote");
            }
            (State::TagClose, TokenKind::GreaterThan | TokenKind::SlashGreaterThan) => {
                panic!("unexpected > after a tag close (sequence </> or <> or even <//>)");
            }
        }
    }
}

impl<'input> Parser<'input> {
    pub fn new(lexer: crate::lexer::Lexer<'input>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Option<ast::AST> {
        let mut state_machine = StateMachine::default();

        while let Some(token) = self.lexer.advance() {
            state_machine.consume(token);
        }

        // TODO: make this an actual error
        assert_eq!(state_machine.element_stack.len(), 0);

        Some(state_machine.ast)
    }
}

#[cfg(test)]
mod tests {
    use crate::element::attributes::Attribute;
    use crate::element::ElementType;
    use crate::lexer::Input;
    use crate::Parser;

    #[test]
    fn parse_data_attribute() {
        let svg = r#"<rect data-test="value" x="10" y="10"/>"#;
        let lexer = crate::Lexer::new(Input::new(svg.as_bytes()));
        let ast = Parser::new(lexer).parse().unwrap();

        let rects = ast.find_by_type(ElementType::Rect);
        assert_eq!(rects.len(), 1);
        let rect = rects[0];
        let rect_node = ast.nodes.get(rect).unwrap();
        let element = rect_node.as_element().unwrap();

        assert_eq!(element.attributes.len(), 3);

        let data_attr = &element.attributes[0];
        assert!(matches!(data_attr, Attribute::Data(_, _)));

        if let Attribute::Data(name, value) = data_attr {
            assert_eq!(name, "data-test");
            assert_eq!(value, "value");
        } else {
            panic!("Expected Data attribute");
        }
    }

    #[test]
    fn parse_multiple_data_attributes() {
        let svg = r#"<rect data-foo="bar" data-baz="qux" data-foo-bar="baz-qux"/>"#;
        let lexer = crate::Lexer::new(Input::new(svg.as_bytes()));
        let ast = Parser::new(lexer).parse().unwrap();

      let rects = ast.find_by_type(ElementType::Rect);
        assert_eq!(rects.len(), 1);
        let rect = rects[0];
        let rect_node = ast.nodes.get(rect).unwrap();
        let element = rect_node.as_element().unwrap();

        assert_eq!(element.attributes.len(), 3);

        if let Attribute::Data(name, value) = &element.attributes[0] {
            assert_eq!(name, "data-foo");
            assert_eq!(value, "bar");
        } else {
            panic!("Expected Data attribute at index 0");
        }

        if let Attribute::Data(name, value) = &element.attributes[1] {
            assert_eq!(name, "data-baz");
            assert_eq!(value, "qux");
        } else {
            panic!("Expected Data attribute at index 1");
        }

        if let Attribute::Data(name, value) = &element.attributes[2] {
            assert_eq!(name, "data-foo-bar");
            assert_eq!(value, "baz-qux");
        } else {
            panic!("Expected Data attribute at index 2");
        }
    }
}
