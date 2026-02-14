use crate::{
    Element,
    parser::ast::{AST, NodeId},
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
                };

                self.current_state = State::TagName;
                let node_id = self.ast.nodes.insert(element);
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

                self.ast
                    .nodes
                    .get_mut(*node_id)
                    .unwrap()
                    .attributes
                    .push(attribute);

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

                self.ast
                    .nodes
                    .get_mut(*node_id)
                    .unwrap()
                    .attributes
                    .push(attribute);

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

                self.ast
                    .nodes
                    .get_mut(*node_id)
                    .unwrap()
                    .attributes
                    .push(attribute);

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

                    self.ast
                        .nodes
                        .get_mut(*node_id)
                        .unwrap()
                        .attributes
                        .push(attribute);
                }

                self.current_state = State::Text;

                let element_id = self.element_stack.pop().unwrap();
                if let Some(last_element_id) = self.element_stack.last() {
                    let last_element = self.ast.nodes.get_mut(*last_element_id).unwrap();
                    last_element.children.push(element_id);
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

                let element = self.ast.nodes.get(element_id).unwrap();

                if element.element_type.as_str() != token.value {
                    panic!(
                        "Element stack does not match closing tag {} != {}",
                        element.element_type, token.value
                    );
                }

                if let Some(last_element_id) = self.element_stack.last_mut() {
                    let last_element = self.ast.nodes.get_mut(*last_element_id).unwrap();
                    last_element.children.push(element_id);
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
                // TODO: handle inner text where aplicable
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
            (_, TokenKind::Comment) => {}
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
