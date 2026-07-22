use crate::{
    Element,
    parser::ast::{
        AST, CDataNode, CommentNode, Node, NodeId, ProcessingInstructionNode, TextNode,
        XmlDeclaration,
    },
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
                    last_en.children.push(child_id);
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
                    last_en.children.push(child_id);
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
                last_en.children.push(child_id);
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
                    last_en.children.push(child_id);
                    if let Some(Node::Comment(cn)) = self.ast.nodes.get_mut(child_id) {
                        cn.parent = Some(parent_id);
                    }
                } else {
                    self.ast.children.push(node_id);
                }
            }
            (_, TokenKind::CData) => {
                // strip the `<![CDATA[` (9 bytes) prefix and `]]>` (3 bytes) suffix
                let content = &token.value[9..token.value.len() - 3];
                let node_id = self.ast.nodes.insert(Node::CData(CDataNode {
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
                    last_en.children.push(child_id);
                    if let Some(Node::CData(cn)) = self.ast.nodes.get_mut(child_id) {
                        cn.parent = Some(parent_id);
                    }
                } else {
                    self.ast.children.push(node_id);
                }
            }
            (_, TokenKind::ProcessingInstruction) => {
                let content = &token.value[2..token.value.len() - 2];

                // The `<?xml ...?>` prolog is promoted to a structured declaration
                // on the AST; every other processing instruction becomes a node.
                let target = content
                    .trim_start()
                    .split(|c: char| c.is_ascii_whitespace())
                    .next()
                    .unwrap_or("");

                if target.eq_ignore_ascii_case("xml") {
                    let Ok(declaration) = XmlDeclaration::parse(content) else {
                        panic!("Parse error, malformed XML declaration: <?{}?>", content);
                    };
                    self.ast.declaration = declaration;
                    return;
                }

                let node_id =
                    self.ast
                        .nodes
                        .insert(Node::ProcessingInstruction(ProcessingInstructionNode {
                            content: content.to_string(),
                            parent: None,
                        }));

                if let Some(last_element_id) = self.element_stack.last() {
                    let parent_id = *last_element_id;
                    let Node::Element(last_en) = self.ast.nodes.get_mut(parent_id).unwrap() else {
                        panic!("Node is supposed to be of type Element at this point");
                    };
                    last_en.children.push(node_id);
                    if let Some(Node::ProcessingInstruction(pn)) = self.ast.nodes.get_mut(node_id) {
                        pn.parent = Some(parent_id);
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

    fn parse(svg: &str) -> crate::AST {
        let lexer = crate::Lexer::new(Input::new(svg.as_bytes()));
        Parser::new(lexer).parse().unwrap()
    }

    #[test]
    fn xml_declaration_defaults_missing_fields() {
        let ast = parse(r#"<?xml version="1.1"?><svg xmlns="http://www.w3.org/2000/svg"/>"#);

        assert_eq!(ast.declaration.version, "1.1");
        assert_eq!(ast.declaration.encoding, "UTF-8"); // defaulted
        assert_eq!(ast.declaration.standalone, None); // defaulted
    }

    #[test]
    fn xml_declaration_parses_standalone() {
        let ast = parse(
            r#"<?xml version="1.0" encoding="UTF-8" standalone="no"?><svg xmlns="http://www.w3.org/2000/svg"/>"#,
        );

        assert_eq!(ast.declaration.standalone, Some(false));
    }

    #[test]
    fn absent_xml_declaration_uses_defaults() {
        let ast = parse(r#"<svg xmlns="http://www.w3.org/2000/svg"/>"#);
        assert_eq!(ast.declaration, crate::parser::ast::XmlDeclaration::default());
    }

    #[test]
    #[should_panic(expected = "malformed XML declaration")]
    fn malformed_xml_declaration_panics() {
        parse(r#"<?xml version=?><svg xmlns="http://www.w3.org/2000/svg"/>"#);
    }

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

    fn data_attr<'a>(ast: &'a crate::AST, name_expected: &str) -> &'a str {
        let rects = ast.find_by_type(ElementType::Rect);
        let element = ast.nodes.get(rects[0]).unwrap().as_element().unwrap();

        match &element.attributes[0] {
            Attribute::Data(name, value) => {
                assert_eq!(name, name_expected);
                value
            }
            _ => panic!("Expected Data attribute"),
        }
    }

    #[test]
    fn predefined_entities_in_attribute_value_are_decoded() {
        let ast = parse(r#"<rect data-label="a &amp; b &lt; c &gt; d &quot;e&quot; &apos;f&apos;"/>"#);
        assert_eq!(data_attr(&ast, "data-label"), r#"a & b < c > d "e" 'f'"#);
    }

    #[test]
    fn numeric_character_references_are_decoded() {
        // &#169; is ©, &#xA9; is © as well, &#960; is π.
        let ast = parse(r#"<rect data-glyph="&#169; &#xA9; &#960;"/>"#);
        assert_eq!(data_attr(&ast, "data-glyph"), "\u{A9} \u{A9} \u{3C0}");
    }

    #[test]
    fn single_quoted_attribute_values_are_supported() {
        let ast = parse(r#"<rect data-label='hello &amp; "world"'/>"#);
        assert_eq!(data_attr(&ast, "data-label"), r#"hello & "world""#);
    }

    #[test]
    fn unrecognized_entities_pass_through_verbatim() {
        let ast = parse(r#"<rect data-raw="a &bogus; b & c"/>"#);
        assert_eq!(data_attr(&ast, "data-raw"), "a &bogus; b & c");
    }

    #[test]
    fn entities_in_text_content_are_decoded() {
        let ast = parse(r#"<text>5 &lt; 10 &amp;&amp; 10 &gt; 5</text>"#);
        let texts = ast.find_by_type(ElementType::Text);
        let element = ast.nodes.get(texts[0]).unwrap().as_element().unwrap();
        let child = ast.nodes.get(element.children[0]).unwrap();
        assert_eq!(child.as_text().unwrap(), "5 < 10 && 10 > 5");
    }

    #[test]
    fn reserved_characters_are_re_encoded_on_serialize() {
        // Parse decodes the entities; to_svg must escape them back so the output
        // is valid XML and round-trips.
        let ast = parse(r#"<text data-note="a &amp; b &lt; &quot;c&quot;">5 &lt; 10 &amp; ok</text>"#);
        let svg = ast.to_svg();

        assert!(
            svg.contains(r#"data-note="a &amp; b &lt; &quot;c&quot;""#),
            "attribute value not re-encoded: {svg}"
        );
        assert!(svg.contains("5 &lt; 10 &amp; ok"), "text not re-encoded: {svg}");
    }

    #[test]
    fn bogus_entity_becomes_valid_xml_on_serialize() {
        // An unrecognized `&bogus;` decodes verbatim, then serializes as a valid,
        // escaped `&amp;bogus;` — so the previously-lenient pass-through is safe.
        let ast = parse(r#"<rect data-raw="a &bogus; b"/>"#);
        let svg = ast.to_svg();
        assert!(svg.contains(r#"data-raw="a &amp;bogus; b""#), "got: {svg}");
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
