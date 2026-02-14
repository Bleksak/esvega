use std::collections::HashMap;

use crate::{
    Element,
    element::{ElementType, attributes::Attribute},
    token::{Token, TokenKind},
};

mod ast;

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
            (State::TagName | State::AttributeValueClosingQuote, TokenKind::Identifier) => {
                self.current_attribute = Some(token.value);
                self.current_state = State::AttributeName;
            }
            (State::AttributeName, TokenKind::Equals) => {
                self.current_state = State::AttributeEquals;
            }
            (State::AttributeName, TokenKind::Identifier) => {
                self.element_stack
                    .last_mut()
                    .unwrap()
                    .attributes
                    .insert(self.current_attribute.take().unwrap(), "".to_string());

                self.current_state = State::AttributeName;
                self.current_attribute = Some(token.value);
            }
            (State::AttributeName, TokenKind::GreaterThan) => {
                self.element_stack
                    .last_mut()
                    .unwrap()
                    .attributes
                    .insert(self.current_attribute.take().unwrap(), "".to_string());
                self.current_state = State::Text;
            }
            (State::AttributeEquals, TokenKind::Quote) => {
                self.current_state = State::AttributeValueOpeningQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Quote) => {
                // TODO: set attribute value to empty string
                // not sure if this is still relevant?
                self.current_state = State::AttributeValueClosingQuote;
            }
            (State::AttributeValueOpeningQuote, TokenKind::Literal) => {
                // TODO: set attribute value to token.value
                // not sure if this is still relevant?
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
                    self.element_stack
                        .last_mut()
                        .unwrap()
                        .attributes
                        .insert(self.current_attribute.take().unwrap(), "".to_string());
                }

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

    fn construct_element(temporary_element: &TemporaryElement) -> Option<Element> {
        let element_type: ElementType = temporary_element.name.as_str().parse().ok()?;

        let mut element = Element {
            element_type,
            attributes: vec![],
            children: vec![],
        };

        for (key, value) in &temporary_element.attributes {
            let Some(attribute): Option<Attribute> = (key, value).try_into().ok() else {
                continue;
            };

            if !attribute.allowed_in_element(element.element_type, &element) {
                // TODO: make this an actual error
                panic!(
                    "Attribute {:#?} is not allowed in element {:#?}",
                    attribute, element
                );
            }

            element.attributes.push(attribute);
        }

        Some(element)
    }

    fn construct_ast(temp_ast: TemporaryDocument) -> Option<ast::AST> {
        let mut ast = ast::AST::default();
        let mut stack: Vec<(Vec<usize>, &TemporaryElement)> = vec![];

        for element in &temp_ast.children {
            stack.push((vec![], element));
        }

        while let Some((path, temporary_element)) = stack.pop() {
            let mut parent_children = &mut ast.children;
            let mut parent = None;

            for idx in path.iter() {
                parent_children = &mut parent_children[*idx].children;
                parent = parent_children.get(*idx);
            }

            let element = Self::construct_element(temporary_element)?;

            if let Some(parent) = parent {
                if !element
                    .element_type
                    .is_allowed_as_child(&parent.element_type)
                {
                    println!(
                        "Element {:#?} is not allowed as child of {:#?}",
                        element, parent
                    );
                    return None;
                }
            }

            parent_children.push(element);

            let new_idx = parent_children.len() - 1;

            for child in &temporary_element.children {
                let mut new_path = path.clone();
                new_path.push(new_idx);
                stack.push((new_path, child));
            }
        }

        Some(ast)
    }

    pub fn parse(&mut self) -> Option<ast::AST> {
        let mut state_machine = StateMachine::default();

        while let Some(token) = self.lexer.advance() {
            state_machine.consume(token);
        }

        // TODO: make this an actual error
        assert_eq!(state_machine.element_stack.len(), 0);

        let ast = Self::construct_ast(state_machine.temp_ast);

        ast
    }
}
