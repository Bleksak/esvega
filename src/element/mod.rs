pub use svg::SVGElement;
pub use svg::Svg;

pub use rect::Rect;
pub use rect::RectElement;

pub use circle::Circle;
pub use circle::CircleElement;

pub use hyperlink::Hyperlink;
pub use hyperlink::HyperlinkElement;

pub mod circle;
pub mod hyperlink;
pub mod rect;
pub mod svg;
pub mod types;

#[derive(Clone, Debug)]
pub enum Element {
    // Animation Elements
    Animate,
    AnimateMotion,
    AnimateTransform,
    MPath,
    Set,

    // Basic Shapes
    Circle(CircleElement),
    Ellipse,
    Line,
    Polygon,
    PolyLine,
    Rect(RectElement),

    // Container Elements
    A(HyperlinkElement),
    Defs,
    G,
    Marker,
    Mask,
    Pattern,
    Svg(SVGElement),
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

impl Element {
    pub fn is_animation(&self) -> bool {
        matches!(
            self,
            Element::Animate
                | Element::AnimateMotion
                | Element::AnimateTransform
                | Element::MPath
                | Element::Set
        )
    }

    pub fn is_descriptive(&self) -> bool {
        matches!(self, Element::Desc | Element::Metadata | Element::Title)
    }

    pub fn is_shape(&self) -> bool {
        matches!(
            self,
            Element::Circle(_)
                | Element::Ellipse
                | Element::Line
                | Element::Path
                | Element::Polygon
                | Element::PolyLine
                | Element::Rect(_)
        )
    }

    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            Element::Defs | Element::G | Element::Svg(_) | Element::Symbol | Element::Use
        )
    }

    pub fn is_gradient(&self) -> bool {
        matches!(
            self,
            Element::LinearGradient | Element::RadialGradient | Element::Stop
        )
    }

    pub fn is_light_source(&self) -> bool {
        matches!(
            self,
            Element::FeDistantLight | Element::FePointLight | Element::FeSpotLight
        )
    }

    pub fn is_text_content_child(&self) -> bool {
        matches!(self, Element::TextPath | Element::TSpan)
    }

    pub fn is_filter_primitive(&self) -> bool {
        matches!(
            self,
            Element::FeBlend
                | Element::FeColorMatrix
                | Element::FeComponentTransfer
                | Element::FeComposite
                | Element::FeConvolveMatrix
                | Element::FeDiffuseLightning
                | Element::FeDisplacementMap
                | Element::FeDropShadow
                | Element::FeFlood
                | Element::FeFuncA
                | Element::FeFuncB
                | Element::FeFuncG
                | Element::FeFuncR
                | Element::FeGaussianBlur
                | Element::FeImage
                | Element::FeMerge
                | Element::FeMergeNode
                | Element::FeMorphology
                | Element::FeOffset
                | Element::FeSpecularLighting
                | Element::FeTile
                | Element::FeTurbulence
        )
    }

    pub fn is_allowed_as_child(&self, element: &Element) -> bool {
        match self {
            Element::Animate => element.is_descriptive(),
            Element::AnimateMotion => element.is_descriptive() || matches!(element, Element::MPath),
            Element::AnimateTransform => element.is_descriptive(),
            Element::MPath => element.is_descriptive(),
            Element::Set => element.is_descriptive(),
            Element::Circle(_) => element.is_animation() || element.is_descriptive(),
            Element::Ellipse => element.is_animation() || element.is_descriptive(),
            Element::Line => element.is_animation() || element.is_descriptive(),
            Element::Polygon => element.is_animation() || element.is_descriptive(),
            Element::PolyLine => element.is_animation() || element.is_descriptive(),
            Element::Rect(_) => element.is_animation() || element.is_descriptive(),
            Element::A(_) => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Defs => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::G => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Marker => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Mask => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Pattern => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Svg(_) => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Switch => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ForeignObject
                            | Element::G
                            | Element::Image
                            | Element::Svg(_)
                            | Element::Switch
                            | Element::Text
                            | Element::Use
                    )
            }
            Element::Symbol => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || element.is_structural()
                    || element.is_gradient()
                    || matches!(
                        element,
                        Element::A(_)
                            | Element::ClipPath
                            | Element::Filter
                            | Element::ForeignObject
                            | Element::Image
                            | Element::Marker
                            | Element::Mask
                            | Element::Pattern
                            | Element::Script
                            | Element::Style
                            | Element::Switch
                            | Element::Text
                            | Element::View
                    )
            }
            Element::Desc => true,
            Element::Metadata => true,
            Element::Title => true,
            Element::FeBlend => {
                // NOTE: We do not allow Discard element, they were deprecated
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeColorMatrix => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeComponentTransfer => {
                matches!(
                    element,
                    Element::FeFuncA | Element::FeFuncB | Element::FeFuncG | Element::FeFuncR
                )
            }
            Element::FeComposite => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeConvolveMatrix => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeDiffuseLightning => {
                // TODO: FeDiffuseLightning allows only one child element of light source type
                element.is_descriptive() || element.is_light_source()
            }
            Element::FeDisplacementMap => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeDropShadow => {
                matches!(element, Element::Animate | Element::Set | Element::Script)
            }
            Element::FeFlood => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeFuncA => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeFuncB => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeFuncG => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeFuncR => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeGaussianBlur => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeImage => {
                matches!(
                    element,
                    Element::Animate | Element::AnimateTransform | Element::Set
                )
            }
            Element::FeMerge => {
                matches!(element, Element::FeMergeNode)
            }
            Element::FeMergeNode => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeMorphology => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeOffset => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeSpecularLighting => {
                // TODO: The light source element must be first child
                element.is_light_source() || element.is_descriptive()
            }
            Element::FeTile => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeTurbulence => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::LinearGradient => {
                element.is_descriptive()
                    || matches!(
                        element,
                        Element::Animate
                            | Element::AnimateTransform
                            | Element::Script
                            | Element::Set
                            | Element::Stop
                            | Element::Style
                    )
            }
            Element::RadialGradient => {
                element.is_descriptive()
                    || matches!(
                        element,
                        Element::Animate
                            | Element::AnimateTransform
                            | Element::Script
                            | Element::Set
                            | Element::Stop
                            | Element::Style
                    )
            }
            Element::Stop => {
                matches!(
                    element,
                    Element::Animate | Element::Script | Element::Set | Element::Style
                )
            }
            Element::Image => {
                element.is_animation()
                    || element.is_descriptive()
                    || matches!(
                        element,
                        Element::Animate
                            | Element::AnimateMotion
                            | Element::AnimateTransform
                            | Element::Script
                            | Element::Style
                    )
            }
            Element::Path => element.is_animation() || element.is_descriptive(),
            Element::Text => {
                element.is_animation() | element.is_descriptive()
                    || element.is_text_content_child()
                    || matches!(element, Element::A(_))
            }
            Element::Use => element.is_animation() || element.is_descriptive(),
            Element::FeDistantLight => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FePointLight => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::FeSpotLight => {
                matches!(element, Element::Animate | Element::Set)
            }
            Element::ClipPath => {
                element.is_animation()
                    || element.is_descriptive()
                    || element.is_shape()
                    || matches!(element, Element::Text | Element::Use)
            }
            Element::Script => true,
            Element::Style => true,
            Element::TextPath => {
                element.is_descriptive()
                    || matches!(
                        element,
                        Element::A(_) | Element::Animate | Element::Set | Element::TSpan
                    )
            }
            Element::TSpan => {
                element.is_descriptive()
                    || matches!(
                        element,
                        Element::A(_) | Element::Animate | Element::Set | Element::TSpan
                    )
            }
            Element::Filter => {
                element.is_descriptive()
                    || element.is_filter_primitive()
                    || matches!(element, Element::Animate | Element::Set)
            }
            Element::ForeignObject => true,
            Element::View => element.is_descriptive(),
        }
    }

    pub fn children_mut(&mut self) -> &mut Vec<Element> {
        match self {
            Element::Animate => todo!(),
            Element::AnimateMotion => todo!(),
            Element::AnimateTransform => todo!(),
            Element::MPath => todo!(),
            Element::Set => todo!(),
            Element::Circle(circle_element) => &mut circle_element.children,
            Element::Ellipse => todo!(),
            Element::Line => todo!(),
            Element::Polygon => todo!(),
            Element::PolyLine => todo!(),
            Element::Rect(rect_element) => rect_element.children.as_mut(),
            Element::A(hyperlink_element) => hyperlink_element.children.as_mut(),
            Element::Defs => todo!(),
            Element::G => todo!(),
            Element::Marker => todo!(),
            Element::Mask => todo!(),
            Element::Pattern => todo!(),
            Element::Svg(svg_element) => svg_element.children.as_mut(),
            Element::Switch => todo!(),
            Element::Symbol => todo!(),
            Element::Desc => todo!(),
            Element::Metadata => todo!(),
            Element::Title => todo!(),
            Element::FeBlend => todo!(),
            Element::FeColorMatrix => todo!(),
            Element::FeComponentTransfer => todo!(),
            Element::FeComposite => todo!(),
            Element::FeConvolveMatrix => todo!(),
            Element::FeDiffuseLightning => todo!(),
            Element::FeDisplacementMap => todo!(),
            Element::FeDropShadow => todo!(),
            Element::FeFlood => todo!(),
            Element::FeFuncA => todo!(),
            Element::FeFuncB => todo!(),
            Element::FeFuncG => todo!(),
            Element::FeFuncR => todo!(),
            Element::FeGaussianBlur => todo!(),
            Element::FeImage => todo!(),
            Element::FeMerge => todo!(),
            Element::FeMergeNode => todo!(),
            Element::FeMorphology => todo!(),
            Element::FeOffset => todo!(),
            Element::FeSpecularLighting => todo!(),
            Element::FeTile => todo!(),
            Element::FeTurbulence => todo!(),
            Element::LinearGradient => todo!(),
            Element::RadialGradient => todo!(),
            Element::Stop => todo!(),
            Element::Image => todo!(),
            Element::Path => todo!(),
            Element::Text => todo!(),
            Element::Use => todo!(),
            Element::FeDistantLight => todo!(),
            Element::FePointLight => todo!(),
            Element::FeSpotLight => todo!(),
            Element::ClipPath => todo!(),
            Element::Script => todo!(),
            Element::Style => todo!(),
            Element::TextPath => todo!(),
            Element::TSpan => todo!(),
            Element::Filter => todo!(),
            Element::ForeignObject => todo!(),
            Element::View => todo!(),
        }
    }
}
