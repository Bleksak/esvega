use std::collections::HashMap;

use crate::{
    Element,
    element::{
        Circle, CircleElement, Hyperlink, HyperlinkElement, Rect, RectElement, SVGElement, Svg,
    },
    token::{Token, TokenKind},
};

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

    fn construct_element(temporary_element: &TemporaryElement) -> Element {
        match temporary_element.name.as_str() {
            "svg" => Element::Svg(SVGElement {
                svg: Svg {
                    base_profile: temporary_element.attributes.get("baseProfile").cloned(),
                    height: temporary_element
                        .attributes
                        .get("height")
                        .map(|h| h.as_str().try_into().ok())
                        .flatten(),
                    preserve_aspect_ratio: temporary_element
                        .attributes
                        .get("preserveAspectRatio")
                        .map(|aspect_ratio| aspect_ratio.as_str().try_into().ok())
                        .flatten(),
                    version: temporary_element
                        .attributes
                        .get("version")
                        .map(|v| v.as_str().parse().ok())
                        .flatten(),
                    view_box: temporary_element
                        .attributes
                        .get("viewBox")
                        .and_then(|view_box| {
                            let mut iter = view_box
                                .as_str()
                                .split_whitespace()
                                .map(|s| s.parse::<f64>().ok());

                            Some([iter.next()??, iter.next()??, iter.next()??, iter.next()??])
                        }),
                    width: temporary_element
                        .attributes
                        .get("width")
                        .map(|w| w.as_str().try_into().ok())
                        .flatten(),
                    x: temporary_element
                        .attributes
                        .get("x")
                        .map(|x| x.as_str().try_into().ok())
                        .flatten(),
                    y: temporary_element
                        .attributes
                        .get("y")
                        .map(|y| y.as_str().try_into().ok())
                        .flatten(),
                    xmlns: temporary_element
                        .attributes
                        .get("xmlns")
                        .map(|xmlns| xmlns.to_string()),
                },
                children: vec![],
            }),
            "circle" => Element::Circle(CircleElement {
                circle: Circle {
                    cx: temporary_element
                        .attributes
                        .get("cx")
                        .map(|cx| cx.as_str().try_into().ok())
                        .flatten(),
                    cy: temporary_element
                        .attributes
                        .get("cy")
                        .map(|cy| cy.as_str().try_into().ok())
                        .flatten(),
                    r: temporary_element
                        .attributes
                        .get("r")
                        .map(|r| r.as_str().try_into().ok())
                        .flatten(),
                    path_length: temporary_element
                        .attributes
                        .get("pathLength")
                        .map(|path_length| path_length.as_str().parse().ok())
                        .flatten(),
                    stroke: temporary_element
                        .attributes
                        .get("stroke")
                        .map(|stroke| stroke.as_str().parse().ok())
                        .flatten(),
                    stroke_opacity: temporary_element
                        .attributes
                        .get("strokeOpacity")
                        .map(|stroke_opacity| stroke_opacity.as_str().parse().ok())
                        .flatten(),
                    stroke_width: temporary_element
                        .attributes
                        .get("strokeWidth")
                        .map(|stroke_width| stroke_width.as_str().parse().ok())
                        .flatten(),
                    stroke_linecap: temporary_element
                        .attributes
                        .get("strokeLinecap")
                        .map(|stroke_linecap| stroke_linecap.as_str().parse().ok())
                        .flatten(),
                    stroke_linejoin: temporary_element
                        .attributes
                        .get("strokeLinejoin")
                        .map(|stroke_linejoin| stroke_linejoin.as_str().parse().ok())
                        .flatten(),
                    stroke_miterlimit: temporary_element
                        .attributes
                        .get("strokeMiterlimit")
                        .map(|stroke_miterlimit| stroke_miterlimit.as_str().parse().ok())
                        .flatten(),
                    stroke_dasharray: temporary_element
                        .attributes
                        .get("strokeDasharray")
                        .map(|stroke_dasharray| stroke_dasharray.as_str().parse().ok())
                        .flatten(),
                    stroke_dashoffset: temporary_element
                        .attributes
                        .get("strokeDashoffset")
                        .map(|stroke_dashoffset| stroke_dashoffset.as_str().parse().ok())
                        .flatten(),
                    fill: temporary_element
                        .attributes
                        .get("fill")
                        .map(|fill| fill.as_str().parse().ok())
                        .flatten(),
                    fill_opacity: temporary_element
                        .attributes
                        .get("fillOpacity")
                        .map(|fill_opacity| fill_opacity.as_str().parse().ok())
                        .flatten(),
                    fill_rule: temporary_element
                        .attributes
                        .get("fillRule")
                        .map(|fill_rule| fill_rule.as_str().parse().ok())
                        .flatten(),
                    vector_effect: temporary_element
                        .attributes
                        .get("vectorEffect")
                        .map(|vector_effect| vector_effect.as_str().parse().ok())
                        .flatten(),
                },
                children: vec![],
            }),
            "rect" => Element::Rect(RectElement {
                rect: Rect {
                    x: temporary_element
                        .attributes
                        .get("x")
                        .map(|x| x.as_str().try_into().ok())
                        .flatten(),
                    y: temporary_element
                        .attributes
                        .get("y")
                        .map(|y| y.as_str().try_into().ok())
                        .flatten(),
                    width: temporary_element
                        .attributes
                        .get("width")
                        .map(|width| width.as_str().try_into().ok())
                        .flatten(),
                    height: temporary_element
                        .attributes
                        .get("height")
                        .map(|height| height.as_str().try_into().ok())
                        .flatten(),
                    rx: temporary_element
                        .attributes
                        .get("rx")
                        .map(|rx| rx.as_str().try_into().ok())
                        .flatten(),
                    ry: temporary_element
                        .attributes
                        .get("ry")
                        .map(|ry| ry.as_str().try_into().ok())
                        .flatten(),
                    path_length: temporary_element
                        .attributes
                        .get("pathLength")
                        .map(|path_length| path_length.as_str().parse().ok())
                        .flatten(),
                    stroke: temporary_element
                        .attributes
                        .get("stroke")
                        .map(|stroke| stroke.as_str().parse().ok())
                        .flatten(),
                    stroke_opacity: temporary_element
                        .attributes
                        .get("strokeOpacity")
                        .map(|stroke_opacity| stroke_opacity.as_str().parse().ok())
                        .flatten(),
                    stroke_width: temporary_element
                        .attributes
                        .get("strokeWidth")
                        .map(|stroke_width| stroke_width.as_str().parse().ok())
                        .flatten(),
                    stroke_linecap: temporary_element
                        .attributes
                        .get("strokeLinecap")
                        .map(|stroke_linecap| stroke_linecap.as_str().parse().ok())
                        .flatten(),
                    stroke_linejoin: temporary_element
                        .attributes
                        .get("strokeLinejoin")
                        .map(|stroke_linejoin| stroke_linejoin.as_str().parse().ok())
                        .flatten(),
                    stroke_miterlimit: temporary_element
                        .attributes
                        .get("strokeMiterlimit")
                        .map(|stroke_miterlimit| stroke_miterlimit.as_str().parse().ok())
                        .flatten(),
                    stroke_dasharray: temporary_element
                        .attributes
                        .get("strokeDasharray")
                        .map(|stroke_dasharray| stroke_dasharray.as_str().parse().ok())
                        .flatten(),
                    stroke_dashoffset: temporary_element
                        .attributes
                        .get("strokeDashoffset")
                        .map(|stroke_dashoffset| stroke_dashoffset.as_str().parse().ok())
                        .flatten(),
                    fill: temporary_element
                        .attributes
                        .get("fill")
                        .map(|fill| fill.as_str().parse().ok())
                        .flatten(),
                    fill_opacity: temporary_element
                        .attributes
                        .get("fillOpacity")
                        .map(|fill_opacity| fill_opacity.as_str().parse().ok())
                        .flatten(),
                    fill_rule: temporary_element
                        .attributes
                        .get("fillRule")
                        .map(|fill_rule| fill_rule.as_str().parse().ok())
                        .flatten(),
                    vector_effect: temporary_element
                        .attributes
                        .get("vectorEffect")
                        .map(|vector_effect| vector_effect.as_str().parse().ok())
                        .flatten(),
                },
                children: vec![],
            }),
            "a" => Element::A(HyperlinkElement {
                hyperlink: Hyperlink {
                    download: temporary_element
                        .attributes
                        .get("download")
                        .map(|download| {
                            if download.is_empty() {
                                None
                            } else {
                                Some(download.to_string())
                            }
                        }),
                    href: temporary_element
                        .attributes
                        .get("href")
                        .map(|href| href.to_string())
                        .unwrap_or_default(),
                    hreflang: temporary_element
                        .attributes
                        .get("hreflang")
                        .map(|hreflang| hreflang.to_string()),
                    interestfor: temporary_element
                        .attributes
                        .get("interestFor")
                        .map(|interest_for| interest_for.to_string()),
                    ping: temporary_element
                        .attributes
                        .get("ping")
                        .map(|ping| ping.to_string()),
                    referrerpolicy: temporary_element
                        .attributes
                        .get("referrerPolicy")
                        .map(|referrer_policy| referrer_policy.as_str().try_into().ok())
                        .flatten(),
                    rel: temporary_element
                        .attributes
                        .get("rel")
                        .map(|rel| rel.to_string()),
                    target: temporary_element
                        .attributes
                        .get("target")
                        .map(|target| target.as_str().try_into().ok())
                        .flatten(),
                    type_: temporary_element
                        .attributes
                        .get("type")
                        .map(|type_| type_.to_string()),
                    xlink_href: temporary_element
                        .attributes
                        .get("xlink:href")
                        .map(|xlink_href| xlink_href.to_string()),
                },
                children: vec![],
            }),
            _ => todo!(),
        }
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
                parent_children = parent_children[*idx].children_mut();
                parent = parent_children.get(*idx);
            }

            let element = Self::construct_element(temporary_element);

            if let Some(parent) = parent {
                if !element.is_allowed_as_child(&parent) {
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
        dbg!(&ast);

        ast
    }
}
