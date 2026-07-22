use std::fmt;

use crate::{
    Element, ElementType,
    parser::ast::{
        AST, CDataNode, CommentNode, Node, NodeId, ProcessingInstructionNode, TextNode,
        XmlDeclaration,
    },
    token::{Token, TokenKind},
};

pub mod ast;

/// An error produced while parsing an SVG document.
#[derive(Debug)]
pub enum ParseError {
    /// A tag name that is not a recognized element.
    InvalidElementName(String),
    /// An attribute the current element does not accept.
    InvalidAttribute(String),
    /// A malformed `<?xml ... ?>` declaration.
    MalformedXmlDeclaration(String),
    /// End of input reached with elements still open.
    UnexpectedEndOfInput(Vec<ElementType>),
    /// A closing tag that does not match the innermost open element.
    MismatchedClosingTag { expected: ElementType, found: String },
    /// A closing tag with no matching open element.
    UnexpectedClosingTag(String),
    /// An element that is not permitted inside its parent.
    DisallowedChild { child: ElementType, parent: ElementType },
    /// Any other token/state combination that cannot form valid markup.
    Unexpected(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidElementName(name) => write!(f, "invalid element name: {name}"),
            ParseError::InvalidAttribute(attr) => write!(f, "invalid attribute: {attr}"),
            ParseError::MalformedXmlDeclaration(decl) => {
                write!(f, "malformed XML declaration: <?{decl}?>")
            }
            ParseError::UnexpectedEndOfInput(unclosed) => {
                let tags = unclosed
                    .iter()
                    .map(|t| format!("<{t}>"))
                    .collect::<Vec<_>>()
                    .join(", ");
                write!(f, "unexpected end of input: {tags} not closed")
            }
            ParseError::MismatchedClosingTag { expected, found } => {
                write!(f, "mismatched closing tag: expected </{expected}>, found </{found}>")
            }
            ParseError::UnexpectedClosingTag(name) => {
                write!(f, "unexpected closing tag </{name}>: no open element")
            }
            ParseError::DisallowedChild { child, parent } => {
                write!(f, "<{child}> is not allowed as a child of <{parent}>")
            }
            ParseError::Unexpected(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for ParseError {}

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
    fn consume(&mut self, token: Token) -> Result<(), ParseError> {
        match (&self.current_state, token.kind) {
            (State::Text, TokenKind::LessThan) => {
                self.current_state = State::TagOpen;
            }
            (State::TagOpen, TokenKind::Identifier) => {
                let Ok(element_type) = token.value.parse() else {
                    return Err(ParseError::InvalidElementName(token.value));
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
                    return Err(ParseError::InvalidAttribute(attribute_name));
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
                    return Err(ParseError::InvalidAttribute(attribute_name));
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
                    return Err(ParseError::InvalidAttribute(format!(
                        "{attribute_name}={attribute_value}"
                    )));
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
                        return Err(ParseError::InvalidAttribute(attribute_name));
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
                        return Err(ParseError::DisallowedChild {
                            child: child_type,
                            parent: last_en.element_type,
                        });
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
                    return Err(ParseError::UnexpectedClosingTag(token.value));
                };

                let child_type = match self.ast.nodes.get(element_id).unwrap() {
                    Node::Element(e) => {
                        if e.element_type.as_str() != token.value {
                            return Err(ParseError::MismatchedClosingTag {
                                expected: e.element_type,
                                found: token.value,
                            });
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
                        return Err(ParseError::DisallowedChild {
                            child: child_type,
                            parent: last_en.element_type,
                        });
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
                return Err(ParseError::Unexpected(
                    "closing tag must not end with '/>'".to_string(),
                ));
            }
            (State::TagCloseName, TokenKind::Identifier) => {
                return Err(ParseError::Unexpected(
                    "unexpected identifier after closing tag name".to_string(),
                ));
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
                return Err(ParseError::Unexpected(
                    "unexpected token in text content".to_string(),
                ));
            }
            (State::TagCloseName, TokenKind::Literal) => {
                return Err(ParseError::Unexpected(
                    "unexpected literal after closing tag name".to_string(),
                ));
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
                        return Err(ParseError::MalformedXmlDeclaration(content.to_string()));
                    };
                    self.ast.declaration = declaration;
                    return Ok(());
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
                return Err(ParseError::Unexpected(
                    "unexpected '<' after tag open (sequence '<<')".to_string(),
                ));
            }
            (_, TokenKind::Text) => {
                return Err(ParseError::Unexpected("unexpected text token".to_string()));
            }
            (State::TagOpen, TokenKind::GreaterThan | TokenKind::SlashGreaterThan) => {
                return Err(ParseError::Unexpected(
                    "unexpected '>' after tag open (sequence '<>')".to_string(),
                ));
            }
            (_, TokenKind::Literal) => {
                return Err(ParseError::Unexpected("unexpected literal token".to_string()));
            }
            (_, TokenKind::Quote) => {
                return Err(ParseError::Unexpected("unexpected quote token".to_string()));
            }
            (_, TokenKind::LessThan | TokenKind::LessThanSlash) => {
                return Err(ParseError::Unexpected(
                    "unexpected opening tag token".to_string(),
                ));
            }
            (_, TokenKind::Equals) => {
                return Err(ParseError::Unexpected("unexpected equals token".to_string()));
            }
            (State::AttributeEquals, _) => {
                return Err(ParseError::Unexpected(
                    "unexpected token after attribute '='".to_string(),
                ));
            }
            (
                State::AttributeValueOpeningQuote | State::AttributeValue,
                TokenKind::GreaterThan | TokenKind::SlashGreaterThan | TokenKind::Identifier,
            ) => {
                return Err(ParseError::Unexpected(
                    "unexpected token inside attribute value".to_string(),
                ));
            }
            (State::TagClose, TokenKind::GreaterThan | TokenKind::SlashGreaterThan) => {
                return Err(ParseError::Unexpected(
                    "unexpected '>' after tag close (sequence '</>' or '<//>')".to_string(),
                ));
            }
        }

        Ok(())
    }
}

impl<'input> Parser<'input> {
    pub fn new(lexer: crate::lexer::Lexer<'input>) -> Self {
        Self { lexer }
    }

    pub fn parse(&mut self) -> Result<ast::AST, ParseError> {
        let mut state_machine = StateMachine::default();

        while let Some(token) = self.lexer.advance() {
            state_machine.consume(token)?;
        }

        if !state_machine.element_stack.is_empty() {
            let unclosed = state_machine
                .element_stack
                .iter()
                .filter_map(|id| match state_machine.ast.nodes.get(*id) {
                    Some(Node::Element(e)) => Some(e.element_type),
                    _ => None,
                })
                .collect();
            return Err(ParseError::UnexpectedEndOfInput(unclosed));
        }

        Ok(state_machine.ast)
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

    fn parse_err(svg: &str) -> crate::parser::ParseError {
        let lexer = crate::Lexer::new(Input::new(svg.as_bytes()));
        Parser::new(lexer).parse().unwrap_err()
    }

    #[test]
    fn malformed_xml_declaration_is_rejected() {
        let err = parse_err(r#"<?xml version=?><svg xmlns="http://www.w3.org/2000/svg"/>"#);
        assert!(matches!(err, crate::parser::ParseError::MalformedXmlDeclaration(_)));
    }

    #[test]
    fn unclosed_tag_is_rejected() {
        let err = parse_err(r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="1" y="1""#);
        assert!(matches!(err, crate::parser::ParseError::UnexpectedEndOfInput(_)));
    }

    #[test]
    fn mismatched_closing_tag_is_rejected() {
        let err = parse_err(r#"<svg xmlns="http://www.w3.org/2000/svg"><rect x="1"></circle></svg>"#);
        assert!(matches!(err, crate::parser::ParseError::MismatchedClosingTag { .. }));
    }

    #[test]
    fn stray_closing_tag_is_rejected() {
        let err = parse_err(r#"</svg>"#);
        assert!(matches!(err, crate::parser::ParseError::UnexpectedClosingTag(_)));
    }

    #[test]
    fn garbage_input_is_rejected() {
        let err = parse_err(r#"<<<>>> definitely not svg &&& %%%"#);
        assert!(matches!(err, crate::parser::ParseError::InvalidElementName(_)));
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

    #[test]
    fn parse_namespaced_attribute() {
        let ast = parse(
            r#"<svg xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns="http://www.w3.org/2000/svg"/>"#,
        );

        let svgs = ast.find_by_type(ElementType::Svg);
        assert_eq!(svgs.len(), 1);
        let element = ast.nodes.get(svgs[0]).unwrap().as_element().unwrap();

        let ns_attr = &element.attributes[0];
        assert!(matches!(ns_attr, Attribute::Namespaced(_, _)));

        if let Attribute::Namespaced(name, value) = ns_attr {
            assert_eq!(name, "xmlns:dc");
            assert_eq!(value, "http://purl.org/dc/elements/1.1/");
        } else {
            panic!("Expected Namespaced attribute");
        }
    }

    #[test]
    fn namespaced_attribute_round_trips() {
        let svg =
            r#"<svg xmlns:dc="http://purl.org/dc/elements/1.1/" xmlns="http://www.w3.org/2000/svg"/>"#;
        let ast = parse(svg);
        assert!(ast.to_svg().contains(r#"xmlns:dc="http://purl.org/dc/elements/1.1/""#));
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
