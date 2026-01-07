use std::collections::HashMap;

use crate::token::{Token, TokenKind};

mod ast;

pub struct Parser<'input> {
    lexer: crate::lexer::Lexer<'input>,
}

#[derive(Debug, Default)]
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

#[derive(Debug)]
struct TemporaryElement {
    name: String,
    attributes: HashMap<String, String>,
    children: Vec<TemporaryElement>,
}

impl TemporaryElement {
    fn new(name: String) -> Self {
        Self {
            name,
            attributes: HashMap::new(),
            children: Vec::new(),
        }
    }
}

#[derive(Debug, Default)]
struct TemporaryDocument {
    pub children: Vec<TemporaryElement>,
}

#[derive(Debug, Default)]
struct StateMachine {
    current_state: State,
    element_stack: Vec<TemporaryElement>,
    current_attribute: Option<String>,
    pub temp_ast: TemporaryDocument,
}

impl StateMachine {
    fn consume(&mut self, token: Token) {
        match (&self.current_state, token.kind) {
            (State::Text, TokenKind::LessThan) => {
                self.current_state = State::TagOpen;
            }
            (State::TagOpen, TokenKind::Identifier) => {
                self.current_state = State::TagName;
                self.element_stack.push(TemporaryElement::new(token.value));
            }
            (
                State::TagName | State::AttributeValueClosingQuote | State::AttributeName,
                TokenKind::Identifier,
            ) => {
                self.current_attribute = Some(token.value);
                self.current_state = State::AttributeName;
            }
            (State::AttributeName, TokenKind::Equals) => {
                self.current_state = State::AttributeEquals;
            }
            (State::AttributeEquals, TokenKind::Quote) => {
                self.current_state = State::AttributeValueOpeningQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Quote) => {
                // TODO: set attribute value to empty string
                self.current_state = State::AttributeValueClosingQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Literal) => {
                // TODO: set attribute value to token.value
                self.current_state = State::AttributeValue;

                self.element_stack
                    .last_mut()
                    .unwrap()
                    .attributes
                    .insert(self.current_attribute.take().unwrap(), token.value);

                self.current_attribute = None;
            }
            (State::AttributeValue, TokenKind::Quote) => {
                self.current_state = State::AttributeValueClosingQuote;
            }
            (
                State::TagName | State::AttributeValueClosingQuote | State::AttributeName,
                TokenKind::GreaterThan,
            ) => {
                // here we are in a <tagname>
                self.current_state = State::Text;
            }

            (
                State::TagName | State::AttributeValueClosingQuote | State::AttributeName,
                TokenKind::SlashGreaterThan,
            ) => {
                // here we are in a <tagname/>
                // TODO: we can produce a self closing element
                self.current_state = State::Text;

                let element = self.element_stack.pop().unwrap();
                if let Some(last_element) = self.element_stack.last_mut() {
                    last_element.children.push(element);
                } else {
                    self.temp_ast.children.push(element);
                }
            }
            (State::Text, TokenKind::LessThanSlash) => {
                self.current_state = State::TagClose;
            }
            (State::TagClose, TokenKind::Identifier) => {
                let Some(element) = self.element_stack.pop() else {
                    // TODO: error
                    panic!("Element stack is empty");
                };

                if element.name != token.value {
                    // TODO: error
                    panic!(
                        "Element stack does not match closing tag {} != {}",
                        element.name, token.value
                    );
                }

                if let Some(last_element) = self.element_stack.last_mut() {
                    last_element.children.push(element);
                } else {
                    self.temp_ast.children.push(element);
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

        dbg!(&state_machine.temp_ast);

        None
    }
}
