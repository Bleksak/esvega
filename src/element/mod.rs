use crate::parser::ast::{AST, NodeId};
use std::{fmt::Display, str::FromStr};

use crate::element::attributes::Attribute;

pub mod attributes;
pub mod types;

#[derive(Clone, Debug, PartialEq)]
pub struct Element {
    pub element_type: ElementType,
    pub attributes: Vec<Attribute>,
    pub children: Vec<NodeId>,
}

impl Element {
    pub fn to_svg(&self, ast: &AST, current_indent: usize) -> String {
        let mut svg = String::new();

        for _ in 0..current_indent {
            svg.push_str("  ");
        }

        svg.push_str(&format!("<{}", self.element_type));

        if !self.attributes.is_empty() {
            svg.push(' ');
        }

        let mut attribute_iterator = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_svg())
            .peekable();

        while let Some(attribute) = attribute_iterator.next() {
            svg.push_str(&attribute);

            if let Some(_) = attribute_iterator.peek() {
                svg.push(' ');
            }
        }

        if self.children.is_empty() {
            svg.push_str("/>\n");
            return svg;
        }

        svg.push_str(">\n");

        // NOTE(@bleksak): If this crashes some day, we need to rewrite it without recursion
        let mut children = self.children.iter().peekable();

        while let Some(child_node) = children.next() {
            let Some(child) = ast.nodes.get(*child_node) else {
                continue;
            };

            svg.push_str(&child.to_svg(ast, current_indent + 1));

            if let Some(_) = children.peek() {
                svg.push('\n');
            }
        }

        for _ in 0..current_indent {
            svg.push_str("  ");
        }

        svg.push_str(&format!("</{}>\n", self.element_type));

        svg
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ElementType {
    // Animation Elements
    Animate,
    AnimateMotion,
    AnimateTransform,
    MPath,
    Set,

    // Basic Shapes
    Circle,
    Ellipse,
    Line,
    Polygon,
    PolyLine,
    Rect,

    // Container Elements
    A,
    Defs,
    G,
    Marker,
    Mask,
    Pattern,
    Svg,
    Switch,
    Symbol,

    // Descriptive Elements
    Desc,
    Metadata,
    Title,

    // Filter primitive Elements
    FeBlend,
    FeColorMatrix,
    FeComponentTransfer,
    FeComposite,
    FeConvolveMatrix,
    FeDiffuseLightning,
    FeDisplacementMap,
    FeDropShadow,
    FeFlood,
    FeFuncA,
    FeFuncB,
    FeFuncG,
    FeFuncR,
    FeGaussianBlur,
    FeImage,
    FeMerge,
    FeMergeNode,
    FeMorphology,
    FeOffset,
    FeSpecularLighting,
    FeTile,
    FeTurbulence,

    // Gradient Elements
    LinearGradient,
    RadialGradient,
    Stop,

    // Graphics Elements
    Image,
    Path,
    Text,
    Use,

    // Light Source Elements
    FeDistantLight,
    FePointLight,
    FeSpotLight,

    // Never rendered elements
    ClipPath,
    Script,
    Style,

    // Text Content Elements
    TextPath,
    TSpan,

    // Uncategorized Elements
    Filter,
    ForeignObject,
    View,
}

impl ElementType {
    pub fn is_animation(&self) -> bool {
        matches!(
            self,
            ElementType::Animate
                | ElementType::AnimateMotion
                | ElementType::AnimateTransform
                | ElementType::MPath
                | ElementType::Set
        )
    }

    pub fn is_descriptive(&self) -> bool {
        matches!(
            self,
            ElementType::Desc | ElementType::Metadata | ElementType::Title
        )
    }

    pub fn is_shape(&self) -> bool {
        matches!(
            self,
            ElementType::Circle
                | ElementType::Ellipse
                | ElementType::Line
                | ElementType::Path
                | ElementType::Polygon
                | ElementType::PolyLine
                | ElementType::Rect
        )
    }

    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            ElementType::Defs
                | ElementType::G
                | ElementType::Svg
                | ElementType::Symbol
                | ElementType::Use
        )
    }

    pub fn is_gradient(&self) -> bool {
        matches!(
            self,
            ElementType::LinearGradient | ElementType::RadialGradient | ElementType::Stop
        )
    }

    pub fn is_light_source(&self) -> bool {
        matches!(
            self,
            ElementType::FeDistantLight | ElementType::FePointLight | ElementType::FeSpotLight
        )
    }

    pub fn is_text_content_child(&self) -> bool {
        matches!(self, ElementType::TextPath | ElementType::TSpan)
    }

    pub fn is_filter_primitive(&self) -> bool {
        matches!(
            self,
            ElementType::FeBlend
                | ElementType::FeColorMatrix
                | ElementType::FeComponentTransfer
                | ElementType::FeComposite
                | ElementType::FeConvolveMatrix
                | ElementType::FeDiffuseLightning
                | ElementType::FeDisplacementMap
                | ElementType::FeDropShadow
                | ElementType::FeFlood
                | ElementType::FeFuncA
                | ElementType::FeFuncB
                | ElementType::FeFuncG
                | ElementType::FeFuncR
                | ElementType::FeGaussianBlur
                | ElementType::FeImage
                | ElementType::FeMerge
                | ElementType::FeMergeNode
                | ElementType::FeMorphology
                | ElementType::FeOffset
                | ElementType::FeSpecularLighting
                | ElementType::FeTile
                | ElementType::FeTurbulence
        )
    }

    // TODO: This should probably accept a reference to the parent element and also be a method on
    // Element
    pub fn is_allowed_as_child(&self, element: &ElementType) -> bool {
        match self {
            ElementType::Animate => element.is_descriptive(),
            ElementType::AnimateMotion => {
                element.is_descriptive() || matches!(element, ElementType::MPath)
            }
            ElementType::AnimateTransform => element.is_descriptive(),
            ElementType::MPath => element.is_descriptive(),
            ElementType::Set => element.is_descriptive(),
            ElementType::Circle => element.is_animation() || element.is_descriptive(),
            ElementType::Ellipse => element.is_animation() || element.is_descriptive(),
            ElementType::Line => element.is_animation() || element.is_descriptive(),
            ElementType::Polygon => element.is_animation() || element.is_descriptive(),
            ElementType::PolyLine => element.is_animation() || element.is_descriptive(),
            ElementType::Rect => element.is_animation() || element.is_descriptive(),
            ElementType::A => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Defs => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::G => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Marker => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Mask => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Pattern => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Svg => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Switch => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ForeignObject
                            | ElementType::G
                            | ElementType::Image
                            | ElementType::Svg
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::Use
                    )
            }
            ElementType::Symbol => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::ClipPath
                            | ElementType::Filter
                            | ElementType::ForeignObject
                            | ElementType::Image
                            | ElementType::Marker
                            | ElementType::Mask
                            | ElementType::Pattern
                            | ElementType::Script
                            | ElementType::Style
                            | ElementType::Switch
                            | ElementType::Text
                            | ElementType::View
                    )
            }
            ElementType::Desc => true,
            ElementType::Metadata => true,
            ElementType::Title => true,
            ElementType::FeBlend => {
                // NOTE: We do not allow Discard element, they were deprecated
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeColorMatrix => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeComponentTransfer => {
                matches!(
                    element,
                    ElementType::FeFuncA
                        | ElementType::FeFuncB
                        | ElementType::FeFuncG
                        | ElementType::FeFuncR
                )
            }
            ElementType::FeComposite => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeConvolveMatrix => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeDiffuseLightning => {
                // TODO: FeDiffuseLightning allows only one child element of light source type
                element.is_descriptive() || element.is_light_source()
            }
            ElementType::FeDisplacementMap => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeDropShadow => {
                matches!(
                    element,
                    ElementType::Animate | ElementType::Set | ElementType::Script
                )
            }
            ElementType::FeFlood => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeFuncA => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeFuncB => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeFuncG => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeFuncR => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeGaussianBlur => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeImage => {
                matches!(
                    element,
                    ElementType::Animate | ElementType::AnimateTransform | ElementType::Set
                )
            }
            ElementType::FeMerge => {
                matches!(element, ElementType::FeMergeNode)
            }
            ElementType::FeMergeNode => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeMorphology => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeOffset => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeSpecularLighting => {
                // TODO: The light source element must be first child
                element.is_light_source() || element.is_descriptive()
            }
            ElementType::FeTile => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeTurbulence => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::LinearGradient => {
                element.is_descriptive()
                    || matches!(
                        element,
                        ElementType::Animate
                            | ElementType::AnimateTransform
                            | ElementType::Script
                            | ElementType::Set
                            | ElementType::Stop
                            | ElementType::Style
                    )
            }
            ElementType::RadialGradient => {
                element.is_descriptive()
                    || matches!(
                        element,
                        ElementType::Animate
                            | ElementType::AnimateTransform
                            | ElementType::Script
                            | ElementType::Set
                            | ElementType::Stop
                            | ElementType::Style
                    )
            }
            ElementType::Stop => {
                matches!(
                    element,
                    ElementType::Animate
                        | ElementType::Script
                        | ElementType::Set
                        | ElementType::Style
                )
            }
            ElementType::Image => {
                element.is_animation()
                    || element.is_descriptive()
                    || matches!(
                        element,
                        ElementType::Animate
                            | ElementType::AnimateMotion
                            | ElementType::AnimateTransform
                            | ElementType::Script
                            | ElementType::Style
                    )
            }
            ElementType::Path => element.is_animation() || element.is_descriptive(),
            ElementType::Text => {
                element.is_animation() | element.is_descriptive()
                    || element.is_text_content_child()
                    || matches!(element, ElementType::A)
            }
            ElementType::Use => element.is_animation() || element.is_descriptive(),
            ElementType::FeDistantLight => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FePointLight => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::FeSpotLight => {
                matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::ClipPath => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || matches!(element, ElementType::Text | ElementType::Use)
            }
            ElementType::Script => true,
            ElementType::Style => true,
            ElementType::TextPath => {
                element.is_descriptive()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::Animate
                            | ElementType::Set
                            | ElementType::TSpan
                    )
            }
            ElementType::TSpan => {
                element.is_descriptive()
                    || matches!(
                        element,
                        ElementType::A
                            | ElementType::Animate
                            | ElementType::Set
                            | ElementType::TSpan
                    )
            }
            ElementType::Filter => {
                element.is_descriptive()
                    || element.is_filter_primitive()
                    || matches!(element, ElementType::Animate | ElementType::Set)
            }
            ElementType::ForeignObject => true,
            ElementType::View => element.is_descriptive(),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Animate => "animate",
            Self::AnimateMotion => "animatemotion",
            Self::AnimateTransform => "animatetransform",
            Self::MPath => "mpath",
            Self::Set => "set",
            Self::Circle => "circle",
            Self::Ellipse => "ellipse",
            Self::Line => "line",
            Self::Polygon => "polygon",
            Self::PolyLine => "polyline",
            Self::Rect => "rect",
            Self::A => "a",
            Self::Defs => "defs",
            Self::G => "g",
            Self::Marker => "marker",
            Self::Mask => "mask",
            Self::Pattern => "pattern",
            Self::Svg => "svg",
            Self::Switch => "switch",
            Self::Symbol => "symbol",
            Self::Desc => "desc",
            Self::Metadata => "metadata",
            Self::Title => "title",
            Self::FeBlend => "feBlend",
            Self::FeColorMatrix => "feColorMatrix",
            Self::FeComponentTransfer => "feComponentTransfer",
            Self::FeComposite => "feComposite",
            Self::FeConvolveMatrix => "feConvolveMatrix",
            Self::FeDiffuseLightning => "feDiffuseLightning",
            Self::FeDisplacementMap => "feDisplacementMap",
            Self::FeDropShadow => "feDropShadow",
            Self::FeFlood => "feFlood",
            Self::FeFuncA => "feFuncA",
            Self::FeFuncB => "feFuncB",
            Self::FeFuncG => "feFuncG",
            Self::FeFuncR => "feFuncR",
            Self::FeGaussianBlur => "feGaussianBlur",
            Self::FeImage => "feImage",
            Self::FeMerge => "feMerge",
            Self::FeMergeNode => "feMergeNode",
            Self::FeMorphology => "feMorphology",
            Self::FeOffset => "feOffset",
            Self::FeSpecularLighting => "feSpecularLighting",
            Self::FeTile => "feTile",
            Self::FeTurbulence => "feTurbulence",
            Self::LinearGradient => "linearGradient",
            Self::RadialGradient => "radialGradient",
            Self::Stop => "stop",
            Self::Image => "image",
            Self::Path => "path",
            Self::Text => "text",
            Self::Use => "use",
            Self::FeDistantLight => "feDistantLight",
            Self::FePointLight => "fePointLight",
            Self::FeSpotLight => "feSpotLight",
            Self::ClipPath => "clipPath",
            Self::Script => "script",
            Self::Style => "style",
            Self::TextPath => "textPath",
            Self::TSpan => "tspan",
            Self::Filter => "filter",
            Self::ForeignObject => "foreignObject",
            Self::View => "view",
        }
    }
}

impl FromStr for ElementType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "animate" => Ok(Self::Animate),
            "animatemotion" => Ok(Self::AnimateMotion),
            "animatetransform" => Ok(Self::AnimateTransform),
            "mpath" => Ok(Self::MPath),
            "set" => Ok(Self::Set),
            "circle" => Ok(Self::Circle),
            "ellipse" => Ok(Self::Ellipse),
            "line" => Ok(Self::Line),
            "polygon" => Ok(Self::Polygon),
            "polyline" => Ok(Self::PolyLine),
            "rect" => Ok(Self::Rect),
            "a" => Ok(Self::A),
            "defs" => Ok(Self::Defs),
            "g" => Ok(Self::G),
            "marker" => Ok(Self::Marker),
            "mask" => Ok(Self::Mask),
            "pattern" => Ok(Self::Pattern),
            "svg" => Ok(Self::Svg),
            "switch" => Ok(Self::Switch),
            "symbol" => Ok(Self::Symbol),
            "desc" => Ok(Self::Desc),
            "metadata" => Ok(Self::Metadata),
            "title" => Ok(Self::Title),
            "feBlend" => Ok(Self::FeBlend),
            "feColorMatrix" => Ok(Self::FeColorMatrix),
            "feComponentTransfer" => Ok(Self::FeComponentTransfer),
            "feComposite" => Ok(Self::FeComposite),
            "feConvolveMatrix" => Ok(Self::FeConvolveMatrix),
            "feDiffuseLightning" => Ok(Self::FeDiffuseLightning),
            "feDisplacementMap" => Ok(Self::FeDisplacementMap),
            "feDropShadow" => Ok(Self::FeDropShadow),
            "feFlood" => Ok(Self::FeFlood),
            "feFuncA" => Ok(Self::FeFuncA),
            "feFuncB" => Ok(Self::FeFuncB),
            "feFuncG" => Ok(Self::FeFuncG),
            "feFuncR" => Ok(Self::FeFuncR),
            "feGaussianBlur" => Ok(Self::FeGaussianBlur),
            "feImage" => Ok(Self::FeImage),
            "feMerge" => Ok(Self::FeMerge),
            "feMergeNode" => Ok(Self::FeMergeNode),
            "feMorphology" => Ok(Self::FeMorphology),
            "feOffset" => Ok(Self::FeOffset),
            "feSpecularLighting" => Ok(Self::FeSpecularLighting),
            "feTile" => Ok(Self::FeTile),
            "feTurbulence" => Ok(Self::FeTurbulence),
            "linearGradient" => Ok(Self::LinearGradient),
            "radialGradient" => Ok(Self::RadialGradient),
            "stop" => Ok(Self::Stop),
            "image" => Ok(Self::Image),
            "path" => Ok(Self::Path),
            "text" => Ok(Self::Text),
            "use" => Ok(Self::Use),
            "feDistantLight" => Ok(Self::FeDistantLight),
            "fePointLight" => Ok(Self::FePointLight),
            "feSpotLight" => Ok(Self::FeSpotLight),
            "clipPath" => Ok(Self::ClipPath),
            "script" => Ok(Self::Script),
            "style" => Ok(Self::Style),
            "textPath" => Ok(Self::TextPath),
            "tspan" => Ok(Self::TSpan),
            "filter" => Ok(Self::Filter),
            "foreignObject" => Ok(Self::ForeignObject),
            "view" => Ok(Self::View),
            _ => Err(()),
        }
    }
}

impl Display for ElementType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
