use std::{convert::Infallible, str::FromStr};

use crate::{
    Element,
    element::{
        ElementType,
        types::{
            AbsoluteLength, AbsoluteSize, Color, FontWeight, Length, LengthOrPercentage, Paint,
            Percentage, RelativeSize, Url,
        },
    },
};

use super::types::ColorLiteral;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AlignmentBaseline {
    #[default]
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Top,
    Center,
    Bottom,
}

impl FromStr for AlignmentBaseline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(AlignmentBaseline::Auto),
            "baseline" => Ok(AlignmentBaseline::Baseline),
            "before-edge" => Ok(AlignmentBaseline::BeforeEdge),
            "text-before-edge" => Ok(AlignmentBaseline::TextBeforeEdge),
            "middle" => Ok(AlignmentBaseline::Middle),
            "central" => Ok(AlignmentBaseline::Central),
            "after-edge" => Ok(AlignmentBaseline::AfterEdge),
            "text-after-edge" => Ok(AlignmentBaseline::TextAfterEdge),
            "ideographic" => Ok(AlignmentBaseline::Ideographic),
            "alphabetic" => Ok(AlignmentBaseline::Alphabetic),
            "hanging" => Ok(AlignmentBaseline::Hanging),
            "mathematical" => Ok(AlignmentBaseline::Mathematical),
            "top" => Ok(AlignmentBaseline::Top),
            "center" => Ok(AlignmentBaseline::Center),
            "bottom" => Ok(AlignmentBaseline::Bottom),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BaselineShift {
    LengthOrPercentage(LengthOrPercentage),
    Sub,
    Super,
}

impl Default for BaselineShift {
    fn default() -> Self {
        BaselineShift::LengthOrPercentage(LengthOrPercentage::default())
    }
}

impl FromStr for BaselineShift {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sub" => Ok(BaselineShift::Sub),
            "super" => Ok(BaselineShift::Super),
            _ => Ok(BaselineShift::LengthOrPercentage(
                LengthOrPercentage::from_str(s)?,
            )),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ClipRule {
    #[default]
    NonZero,
    EvenOdd,
    Inherit,
}

impl FromStr for ClipRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nonzero" => Ok(ClipRule::NonZero),
            "evenodd" => Ok(ClipRule::EvenOdd),
            _ => Ok(ClipRule::Inherit),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ColorInterpolation {
    Auto,
    #[default]
    SRGB,
    LinearRGB,
}

impl FromStr for ColorInterpolation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorInterpolation::Auto),
            "sRGB" => Ok(ColorInterpolation::SRGB),
            "linearRGB" => Ok(ColorInterpolation::LinearRGB),
            _ => Ok(ColorInterpolation::Auto),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ColorInterpolationFilter {
    Auto,
    SRGB,
    #[default]
    LinearRGB,
}

impl FromStr for ColorInterpolationFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorInterpolationFilter::Auto),
            "sRGB" => Ok(ColorInterpolationFilter::SRGB),
            "linearRGB" => Ok(ColorInterpolationFilter::LinearRGB),
            _ => Ok(ColorInterpolationFilter::Auto),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Cursor {
    #[default]
    Auto,
    Crosshair,
    Default,
    Pointer,
    Move,
    EResize,
    NEResize,
    NWResize,
    NResize,
    SEResize,
    SWResize,
    SResize,
    WResize,
    Text,
    Wait,
    Help,
    Inherit,
    // TODO: add FuncURI, not sure what that is at the moment
}

impl FromStr for Cursor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Cursor::Auto),
            "crosshair" => Ok(Cursor::Crosshair),
            "default" => Ok(Cursor::Default),
            "pointer" => Ok(Cursor::Pointer),
            "move" => Ok(Cursor::Move),
            "e-resize" => Ok(Cursor::EResize),
            "ne-resize" => Ok(Cursor::NEResize),
            "nw-resize" => Ok(Cursor::NWResize),
            "n-resize" => Ok(Cursor::NResize),
            "se-resize" => Ok(Cursor::SEResize),
            "sw-resize" => Ok(Cursor::SWResize),
            "s-resize" => Ok(Cursor::SResize),
            "w-resize" => Ok(Cursor::WResize),
            "text" => Ok(Cursor::Text),
            "wait" => Ok(Cursor::Wait),
            "help" => Ok(Cursor::Help),
            "inherit" => Ok(Cursor::Inherit),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MoveTo {
    Absolute((LengthOrPercentage, LengthOrPercentage)), // M x y
    Relative((LengthOrPercentage, LengthOrPercentage)), // m dx dy
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineTo {
    XYAbsolute(Vec<LengthOrPercentage>),         // L (x y)+
    XYRelative(Vec<LengthOrPercentage>),         // l (dx dy)+
    HorizontalAbsolute(Vec<LengthOrPercentage>), // H (x)+
    HorizontalRelative(Vec<LengthOrPercentage>), // h (dx)+
    VerticalAbsolute(Vec<LengthOrPercentage>),   // V (y)+
    VerticalRelative(Vec<LengthOrPercentage>),   // v (dy)+
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubicBezierCurvePoint {
    pub x1: LengthOrPercentage,
    pub y1: LengthOrPercentage,
    pub x2: LengthOrPercentage,
    pub y2: LengthOrPercentage,
    pub x: LengthOrPercentage,
    pub y: LengthOrPercentage,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SmoothCubicBezierCurvePoint {
    pub x2: LengthOrPercentage,
    pub y2: LengthOrPercentage,
    pub x: LengthOrPercentage,
    pub y: LengthOrPercentage,
}

#[derive(Clone, Debug, PartialEq)]
pub enum CubicBezierCurve {
    Absolute(Vec<CubicBezierCurvePoint>), // C (x1 y1 x2 y2 x y)+
    Relative(Vec<CubicBezierCurvePoint>), // c (x1 y1 x2 y2 x y)+
    SmoothAbsolute(Vec<SmoothCubicBezierCurvePoint>), // S (x2 y2 x y)+
    SmoothRelative(Vec<SmoothCubicBezierCurvePoint>), // s (x2 y2 x y)+
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuadraticBezierCurvePoint {
    pub x1: LengthOrPercentage,
    pub y1: LengthOrPercentage,
    pub x: LengthOrPercentage,
    pub y: LengthOrPercentage,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: LengthOrPercentage,
    pub y: LengthOrPercentage,
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuadraticBezierCurve {
    Absolute(Vec<QuadraticBezierCurvePoint>), // Q (x1 y1 x y)+
    Relative(Vec<QuadraticBezierCurvePoint>), // q (x1 y1 x y)+
    SmoothAbsolute(Vec<Point>),               // T (x y)+
    SmoothRelative(Vec<Point>),               // t (x y)+
}

#[derive(Clone, Debug, PartialEq)]
pub struct EllipticalArcPoint {
    pub rx: LengthOrPercentage,
    pub ry: LengthOrPercentage,
    pub angle: f64,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
    pub x: LengthOrPercentage,
    pub y: LengthOrPercentage,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EllipticalArcCurve {
    Absolute(Vec<EllipticalArcPoint>), // A (rx ry angle large-arc-flag sweep-flag x y)+
    Relative(Vec<EllipticalArcPoint>), // a (rx ry angle large-arc-flag sweep-flag dx dy)+
}

#[derive(Clone, Debug, PartialEq)]
pub enum Path {
    MoveTo(MoveTo),                             // M m
    LineTo(LineTo),                             // L l H h V v
    CubicBezierCurve(CubicBezierCurve),         // C c S s
    QuadraticBezierCurve(QuadraticBezierCurve), // Q q T t
    EllipticalArcCurve(EllipticalArcCurve),     // A a
    ClosePath,                                  // Z z
}

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(first_char) = s.chars().next() else {
            return Err(());
        };

        let rest = &s[1..];
        let mut iterator = rest.split_whitespace();

        match first_char {
            'M' => Ok(Path::MoveTo(MoveTo::Absolute((
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            )))),
            'm' => Ok(Path::MoveTo(MoveTo::Relative((
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            )))),
            'L' => Ok(Path::LineTo(LineTo::XYAbsolute(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'l' => Ok(Path::LineTo(LineTo::XYRelative(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'H' => Ok(Path::LineTo(LineTo::HorizontalAbsolute(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'h' => Ok(Path::LineTo(LineTo::HorizontalRelative(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'V' => Ok(Path::LineTo(LineTo::VerticalAbsolute(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'v' => Ok(Path::LineTo(LineTo::VerticalRelative(vec![
                LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
            ]))),
            'C' => Ok(Path::CubicBezierCurve(CubicBezierCurve::Absolute(vec![
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
            ]))),
            'c' => Ok(Path::CubicBezierCurve(CubicBezierCurve::Relative(vec![
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
                CubicBezierCurvePoint {
                    x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                },
            ]))),
            'S' => Ok(Path::CubicBezierCurve(CubicBezierCurve::SmoothAbsolute(
                vec![
                    SmoothCubicBezierCurvePoint {
                        x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    SmoothCubicBezierCurvePoint {
                        x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            's' => Ok(Path::CubicBezierCurve(CubicBezierCurve::SmoothRelative(
                vec![
                    SmoothCubicBezierCurvePoint {
                        x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    SmoothCubicBezierCurvePoint {
                        x2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y2: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            'Q' => Ok(Path::QuadraticBezierCurve(QuadraticBezierCurve::Absolute(
                vec![
                    QuadraticBezierCurvePoint {
                        x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    QuadraticBezierCurvePoint {
                        x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            'q' => Ok(Path::QuadraticBezierCurve(QuadraticBezierCurve::Relative(
                vec![
                    QuadraticBezierCurvePoint {
                        x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    QuadraticBezierCurvePoint {
                        x1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y1: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            'T' => Ok(Path::QuadraticBezierCurve(
                QuadraticBezierCurve::SmoothAbsolute(vec![
                    Point {
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    Point {
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ]),
            )),
            't' => Ok(Path::QuadraticBezierCurve(
                QuadraticBezierCurve::SmoothRelative(vec![
                    Point {
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    Point {
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ]),
            )),
            'A' => Ok(Path::EllipticalArcCurve(EllipticalArcCurve::Absolute(
                vec![
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            'a' => Ok(Path::EllipticalArcCurve(EllipticalArcCurve::Relative(
                vec![
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                    EllipticalArcPoint {
                        rx: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        ry: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        angle: f64::from_str(iterator.next().ok_or(())?).map_err(|_| ())?,
                        large_arc_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        sweep_flag: iterator.next().ok_or(())?.eq_ignore_ascii_case("1"),
                        x: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                        y: LengthOrPercentage::from_str(iterator.next().ok_or(())?)?,
                    },
                ],
            ))),
            'Z' => Ok(Path::ClosePath),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

impl FromStr for TextDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(TextDirection::Ltr),
            "rtl" => Ok(TextDirection::Rtl),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Display {
    #[default]
    Inline,
    Block,
    RunIn,
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,
    ListItem,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
    Contents,
    None,
    InlineBlock,
    InlineTable,
    InlineFlex,
    InlineGrid,
}

impl FromStr for Display {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inline" => Ok(Display::Inline),
            "block" => Ok(Display::Block),
            "run-in" => Ok(Display::RunIn),
            "flow" => Ok(Display::Flow),
            "flow-root" => Ok(Display::FlowRoot),
            "table" => Ok(Display::Table),
            "flex" => Ok(Display::Flex),
            "grid" => Ok(Display::Grid),
            "ruby" => Ok(Display::Ruby),
            "list-item" => Ok(Display::ListItem),
            "table-row-group" => Ok(Display::TableRowGroup),
            "table-header-group" => Ok(Display::TableHeaderGroup),
            "table-footer-group" => Ok(Display::TableFooterGroup),
            "table-row" => Ok(Display::TableRow),
            "table-cell" => Ok(Display::TableCell),
            "table-column-group" => Ok(Display::TableColumnGroup),
            "table-column" => Ok(Display::TableColumn),
            "table-caption" => Ok(Display::TableCaption),
            "ruby-base" => Ok(Display::RubyBase),
            "ruby-text" => Ok(Display::RubyText),
            "ruby-base-container" => Ok(Display::RubyBaseContainer),
            "ruby-text-container" => Ok(Display::RubyTextContainer),
            "contents" => Ok(Display::Contents),
            "none" => Ok(Display::None),
            "inline-block" => Ok(Display::InlineBlock),
            "inline-table" => Ok(Display::InlineTable),
            "inline-flex" => Ok(Display::InlineFlex),
            "inline-grid" => Ok(Display::InlineGrid),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum DominantBaseline {
    #[default]
    Auto,
    TextBottom,
    Alphabetic,
    Ideographic,
    Middle,
    Central,
    Mathematical,
    Hanging,
    TextTop,
}

impl FromStr for DominantBaseline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(DominantBaseline::Auto),
            "text-bottom" => Ok(DominantBaseline::TextBottom),
            "alphabetic" => Ok(DominantBaseline::Alphabetic),
            "ideographic" => Ok(DominantBaseline::Ideographic),
            "middle" => Ok(DominantBaseline::Middle),
            "central" => Ok(DominantBaseline::Central),
            "mathematical" => Ok(DominantBaseline::Mathematical),
            "hanging" => Ok(DominantBaseline::Hanging),
            "text-top" => Ok(DominantBaseline::TextTop),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Paint(Paint),
    Freeze,
    Remove,
}

impl FromStr for Fill {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Fill::Remove),
            "freeze" => Ok(Fill::Freeze),
            _ => Ok(Fill::Paint(Paint::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FillRule {
    #[default]
    NonZero,
    EvenOdd,
}

impl FromStr for FillRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nonzero" => Ok(FillRule::NonZero),
            "evenodd" => Ok(FillRule::EvenOdd),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontSize {
    Absolute(AbsoluteSize),
    Relative(RelativeSize),
    Length(Length),
    Percentage(Percentage),
}

impl FromStr for FontSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(FontSize::Percentage(percentage));
        }

        if let Ok(absolute) = AbsoluteSize::from_str(s) {
            return Ok(FontSize::Absolute(absolute));
        }

        if let Ok(relative) = RelativeSize::from_str(s) {
            return Ok(FontSize::Relative(relative));
        }

        if let Ok(length) = Length::from_str(s) {
            return Ok(FontSize::Length(length));
        }

        return Err(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontSizeAdjust {
    None,
    Number(f64),
}

impl FromStr for FontSizeAdjust {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(FontSizeAdjust::None),
            _ => Ok(FontSizeAdjust::Number(s.parse().map_err(|_| ())?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl FromStr for FontStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(FontStyle::Normal),
            "italic" => Ok(FontStyle::Italic),
            "oblique" => Ok(FontStyle::Oblique),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ImageRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    OptimizeQuality,
}

impl FromStr for ImageRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ImageRendering::Auto),
            "optimizeSpeed" => Ok(ImageRendering::OptimizeSpeed),
            "optimizeQuality" => Ok(ImageRendering::OptimizeQuality),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum LetterSpacing {
    #[default]
    Normal,
    Length(Length),
}

impl FromStr for LetterSpacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(LetterSpacing::Normal),
            _ => Ok(LetterSpacing::Length(Length::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LightingColor(pub Color);

impl FromStr for LightingColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LightingColor(Color::from_str(s)?))
    }
}

impl Default for LightingColor {
    fn default() -> Self {
        LightingColor(Color::Literal(ColorLiteral::Black))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Marker {
    None,
    Url(Url), // TODO: Is this correct?
}

impl FromStr for Marker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("none") {
            return Ok(Marker::None);
        }

        Ok(Marker::Url(s.parse()?))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum MaskType {
    Alpha,
    #[default]
    Luminance,
}

impl FromStr for MaskType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alpha" => Ok(MaskType::Alpha),
            "luminance" => Ok(MaskType::Luminance),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Opacity(pub f64);

impl FromStr for Opacity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Opacity(s.parse().map_err(|_| ())?))
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl FromStr for Overflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "visible" => Ok(Overflow::Visible),
            "hidden" => Ok(Overflow::Hidden),
            "scroll" => Ok(Overflow::Scroll),
            "auto" => Ok(Overflow::Auto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PointerEvents {
    BoundingBox,
    #[default]
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    All,
    None,
}

impl FromStr for PointerEvents {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bounding-box" => Ok(PointerEvents::BoundingBox),
            "visiblePainted" => Ok(PointerEvents::VisiblePainted),
            "visibleFill" => Ok(PointerEvents::VisibleFill),
            "visibleStroke" => Ok(PointerEvents::VisibleStroke),
            "visible" => Ok(PointerEvents::Visible),
            "painted" => Ok(PointerEvents::Painted),
            "fill" => Ok(PointerEvents::Fill),
            "stroke" => Ok(PointerEvents::Stroke),
            "all" => Ok(PointerEvents::All),
            "none" => Ok(PointerEvents::None),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CircleRadius(pub LengthOrPercentage);

impl Default for CircleRadius {
    fn default() -> Self {
        CircleRadius(LengthOrPercentage::Percentage(Percentage(50.0)))
    }
}

// NOTE: This is called EllipsisRadius, but it can be applied to both ellipse and a rect
#[derive(Clone, Debug, PartialEq, Default)]
pub enum EllipsisRadius {
    LengthOrPercentage(LengthOrPercentage),
    #[default]
    Auto,
}

impl FromStr for EllipsisRadius {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(EllipsisRadius::Auto),
            _ => Ok(EllipsisRadius::LengthOrPercentage(
                LengthOrPercentage::from_str(s)?,
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ShapeRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    CrispEdges,
    GeometricPrecision,
}

impl FromStr for ShapeRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ShapeRendering::Auto),
            "optimizeSpeed" => Ok(ShapeRendering::OptimizeSpeed),
            "crispEdges" => Ok(ShapeRendering::CrispEdges),
            "geometricPrecision" => Ok(ShapeRendering::GeometricPrecision),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopColor(pub Color);

impl FromStr for StopColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StopColor(Color::from_str(s)?))
    }
}

impl Default for StopColor {
    fn default() -> Self {
        StopColor(Color::Literal(ColorLiteral::Black))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum StrokeLinecap {
    #[default]
    Butt,
    Round,
    Square,
}

impl FromStr for StrokeLinecap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "butt" => Ok(StrokeLinecap::Butt),
            "round" => Ok(StrokeLinecap::Round),
            "square" => Ok(StrokeLinecap::Square),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum StrokeLinejoin {
    Arcs,
    Bevel,
    #[default]
    Miter,
    MiterClip,
    Round,
}

impl FromStr for StrokeLinejoin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arcs" => Ok(StrokeLinejoin::Arcs),
            "bevel" => Ok(StrokeLinejoin::Bevel),
            "miter" => Ok(StrokeLinejoin::Miter),
            "miter-clip" => Ok(StrokeLinejoin::MiterClip),
            "round" => Ok(StrokeLinejoin::Round),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrokeOpacity {
    Number(f64),
    Percentage(Percentage),
}

impl FromStr for StrokeOpacity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(StrokeOpacity::Percentage(percentage));
        }

        Ok(StrokeOpacity::Number(s.parse().map_err(|_| ())?))
    }
}

impl Default for StrokeOpacity {
    fn default() -> Self {
        StrokeOpacity::Number(1.0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum VectorEffect {
    #[default]
    None,
    NonScalingStroke,
    NonScalingSize,
    NonRotation,
    FixedPosition,
}

impl FromStr for VectorEffect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(VectorEffect::None),
            "non-scaling-stroke" => Ok(VectorEffect::NonScalingStroke),
            "non-scaling-size" => Ok(VectorEffect::NonScalingSize),
            "non-rotation" => Ok(VectorEffect::NonRotation),
            "fixed-position" => Ok(VectorEffect::FixedPosition),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}

impl FromStr for TextAnchor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(TextAnchor::Start),
            "middle" => Ok(TextAnchor::Middle),
            "end" => Ok(TextAnchor::End),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipses,
}

impl FromStr for TextOverflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clip" => Ok(TextOverflow::Clip),
            "ellipsis" => Ok(TextOverflow::Ellipses),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

impl FromStr for TextRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(TextRendering::Auto),
            "optimizeSpeed" => Ok(TextRendering::OptimizeSpeed),
            "optimizeLegibility" => Ok(TextRendering::OptimizeLegibility),
            "geometricPrecision" => Ok(TextRendering::GeometricPrecision),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum UnicodeBidi {
    #[default]
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

impl FromStr for UnicodeBidi {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(UnicodeBidi::Normal),
            "embed" => Ok(UnicodeBidi::Embed),
            "isolate" => Ok(UnicodeBidi::Isolate),
            "bidi-override" => Ok(UnicodeBidi::BidiOverride),
            "isolate-override" => Ok(UnicodeBidi::IsolateOverride),
            "plaintext" => Ok(UnicodeBidi::Plaintext),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
    Collapse,
}

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "visible" => Ok(Visibility::Visible),
            "hidden" => Ok(Visibility::Hidden),
            "collapse" => Ok(Visibility::Collapse),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WhiteSpace {
    #[default]
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpace,
    PreLine,
}

impl FromStr for WhiteSpace {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(WhiteSpace::Normal),
            "pre" => Ok(WhiteSpace::Pre),
            "nowrap" => Ok(WhiteSpace::Nowrap),
            "pre-wrap" => Ok(WhiteSpace::PreWrap),
            "break-space" => Ok(WhiteSpace::BreakSpace),
            "pre-line" => Ok(WhiteSpace::PreLine),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WordSpacing {
    #[default]
    Normal,
    Length(Length),
}

impl FromStr for WordSpacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(WordSpacing::Normal),
            _ => Ok(WordSpacing::Length(Length::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WritingMode {
    #[default]
    HorizontalTb,
    VerticalRl,
    VerticalLr,
}

impl FromStr for WritingMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "horizontal-tb" => Ok(WritingMode::HorizontalTb),
            "vertical-rl" => Ok(WritingMode::VerticalRl),
            "vertical-lr" => Ok(WritingMode::VerticalLr),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rotate {
    Auto,
    AutoReverse,
    Number(f64),
}

impl Default for Rotate {
    fn default() -> Self {
        Rotate::Number(0.0)
    }
}

impl FromStr for Rotate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = f64::from_str(s) {
            return Ok(Rotate::Number(number));
        }

        match s {
            "auto" => Ok(Rotate::Auto),
            "auto-reverse" => Ok(Rotate::AutoReverse),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LengthOrPercentageOrNumber {
    Length(Length),
    Percentage(Percentage),
    Number(f64),
}

impl FromStr for LengthOrPercentageOrNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Percentage(percentage));
        }

        if let Ok(length) = Length::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Length(length));
        }

        if let Ok(number) = f64::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Number(number));
        }

        Err(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

impl TryFrom<&str> for ReferrerPolicy {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "no-referrer" => Ok(Self::NoReferrer),
            "no-referrer-when-downgrade" => Ok(Self::NoReferrerWhenDowngrade),
            "same-origin" => Ok(Self::SameOrigin),
            "origin" => Ok(Self::Origin),
            "strict-origin" => Ok(Self::StrictOrigin),
            "origin-when-cross-origin" => Ok(Self::OriginWhenCrossOrigin),
            "strict-origin-when-cross-origin" => Ok(Self::StrictOriginWhenCrossOrigin),
            "unsafe-url" => Ok(Self::UnsafeUrl),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelType {
    Alternate,
    Author,
    Bookmark,
    Canonical,
    CompressionDictionary,
    DnsPrefetch,
    External,
    Expect,
    Help,
    Icon,
    License,
    Manifest,
    Me,
    ModulePreload,
    Next,
    NoFollow,
    NoOpener,
    NoReferrer,
    Opener,
    PingBack,
    PreConnect,
    Prefetch,
    Preload,
    Prev,
    PrivacyPolicy,
    Search,
    StyleSheet,
    Tag,
    TermsOfService,
}

impl FromStr for RelType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alternate" => Ok(Self::Alternate),
            "author" => Ok(Self::Author),
            "bookmark" => Ok(Self::Bookmark),
            "canonical" => Ok(Self::Canonical),
            "compression-dictionary" => Ok(Self::CompressionDictionary),
            "dns-prefetch" => Ok(Self::DnsPrefetch),
            "external" => Ok(Self::External),
            "expect" => Ok(Self::Expect),
            "help" => Ok(Self::Help),
            "icon" => Ok(Self::Icon),
            "license" => Ok(Self::License),
            "manifest" => Ok(Self::Manifest),
            "me" => Ok(Self::Me),
            "modulepreload" => Ok(Self::ModulePreload),
            "next" => Ok(Self::Next),
            "nofollow" => Ok(Self::NoFollow),
            "noopener" => Ok(Self::NoOpener),
            "noreferrer" => Ok(Self::NoReferrer),
            "opener" => Ok(Self::Opener),
            "pingback" => Ok(Self::PingBack),
            "preconnect" => Ok(Self::PreConnect),
            "prefetch" => Ok(Self::Prefetch),
            "preload" => Ok(Self::Preload),
            "prev" => Ok(Self::Prev),
            "privacy-policy" => Ok(Self::PrivacyPolicy),
            "search" => Ok(Self::Search),
            "stylesheet" => Ok(Self::StyleSheet),
            "tag" => Ok(Self::Tag),
            "terms-of-service" => Ok(Self::TermsOfService),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Target {
    #[default]
    Self_,
    Parent,
    Top,
    Blank,
}

impl TryFrom<&str> for Target {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "_self" => Ok(Self::Self_),
            "_parent" => Ok(Self::Parent),
            "_top" => Ok(Self::Top),
            "_blank" => Ok(Self::Blank),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum MarkerUnits {
    UserSpaceOnUse,
    #[default]
    StrokeWidth,
}

impl FromStr for MarkerUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "strokeWidth" => Ok(Self::StrokeWidth),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Orient {
    Auto,
    AutoStartReverse,
    Angle(f64),
}

impl Default for Orient {
    fn default() -> Self {
        Self::Angle(0.0)
    }
}

impl FromStr for Orient {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(angle) = f64::from_str(s) {
            return Ok(Self::Angle(angle));
        }

        match s {
            "auto" => Ok(Self::Auto),
            "auto-start-reverse" => Ok(Self::AutoStartReverse),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreserveAspectRatio {
    None,
    XMinYMinMeet,
    XMidYMinMeet,
    XMaxYMinMeet,
    XMinYMidMeet,
    XMidYMidMeet,
    XMaxYMidMeet,
    XMinYMaxMeet,
    XMidYMaxMeet,
    XMaxYMaxMeet,

    XMinYMinSlice,
    XMidYMinSlice,
    XMaxYMinSlice,
    XMinYMidSlice,
    XMidYMidSlice,
    XMaxYMidSlice,
    XMinYMaxSlice,
    XMidYMaxSlice,
    XMaxYMaxSlice,
}

impl FromStr for PreserveAspectRatio {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(Self::None),
            "xMinYMin meet" => Ok(Self::XMinYMinMeet),
            "xMidYMin meet" => Ok(Self::XMidYMinMeet),
            "xMaxYMin meet" => Ok(Self::XMaxYMinMeet),
            "xMinYMid meet" => Ok(Self::XMinYMidMeet),
            "xMidYMid meet" => Ok(Self::XMidYMidMeet),
            "xMaxYMid meet" => Ok(Self::XMaxYMidMeet),
            "xMinYMax meet" => Ok(Self::XMinYMaxMeet),
            "xMidYMax meet" => Ok(Self::XMidYMaxMeet),
            "xMaxYMax meet" => Ok(Self::XMaxYMaxMeet),
            "xMinYMin slice" => Ok(Self::XMinYMinSlice),
            "xMidYMin slice" => Ok(Self::XMidYMinSlice),
            "xMaxYMin slice" => Ok(Self::XMaxYMinSlice),
            "xMinYMid slice" => Ok(Self::XMinYMidSlice),
            "xMidYMid slice" => Ok(Self::XMidYMidSlice),
            "xMaxYMid slice" => Ok(Self::XMaxYMidSlice),
            "xMinYMax slice" => Ok(Self::XMinYMaxSlice),
            "xMidYMax slice" => Ok(Self::XMidYMaxSlice),
            "xMaxYMax slice" => Ok(Self::XMaxYMaxSlice),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RefX {
    Left,
    Center,
    Right,
    Coordinate(LengthOrPercentage),
}

impl Default for RefX {
    fn default() -> Self {
        Self::Coordinate(LengthOrPercentage::Length(Length::Absolute(
            AbsoluteLength::Px(0.0),
        )))
    }
}

impl FromStr for RefX {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Self::Left),
            "center" => Ok(Self::Center),
            "right" => Ok(Self::Right),
            _ => Ok(Self::Coordinate(LengthOrPercentage::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RefY {
    Top,
    Center,
    Bottom,
    Coordinate(LengthOrPercentage),
}

impl Default for RefY {
    fn default() -> Self {
        Self::Coordinate(LengthOrPercentage::Length(Length::Absolute(
            AbsoluteLength::Px(0.0),
        )))
    }
}

impl FromStr for RefY {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "top" => Ok(Self::Top),
            "center" => Ok(Self::Center),
            "bottom" => Ok(Self::Bottom),
            _ => Ok(Self::Coordinate(LengthOrPercentage::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ViewBox(Vec<f64>);

impl FromStr for ViewBox {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split_whitespace();

        let x = it.next().ok_or(())?.parse().map_err(|_| ())?;
        let y = it.next().ok_or(())?.parse().map_err(|_| ())?;
        let width = it.next().ok_or(())?.parse().map_err(|_| ())?;
        let height = it.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(Self(vec![x, y, width, height]))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum MaskContentUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl FromStr for MaskContentUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum MaskUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FromStr for MaskUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PatternContentUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FromStr for PatternContentUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PatternUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FromStr for PatternUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum In {
    #[default]
    SourceGraphic,
    SourceAlpha,
    BackgroundImage,
    BackgroundAlpha,
    FillPaint,
    StrokePaint,
    Identifier(String),
}

impl FromStr for In {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "SourceGraphic" => Ok(Self::SourceGraphic),
            "SourceAlpha" => Ok(Self::SourceAlpha),
            "BackgroundImage" => Ok(Self::BackgroundImage),
            "BackgroundAlpha" => Ok(Self::BackgroundAlpha),
            "FillPaint" => Ok(Self::FillPaint),
            "StrokePaint" => Ok(Self::StrokePaint),
            _ => Ok(Self::Identifier(s.to_string())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum BlendMode {
    #[default]
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

impl FromStr for BlendMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(Self::Normal),
            "multiply" => Ok(Self::Multiply),
            "screen" => Ok(Self::Screen),
            "overlay" => Ok(Self::Overlay),
            "darken" => Ok(Self::Darken),
            "lighten" => Ok(Self::Lighten),
            "color-dodge" => Ok(Self::ColorDodge),
            "color-burn" => Ok(Self::ColorBurn),
            "hard-light" => Ok(Self::HardLight),
            "soft-light" => Ok(Self::SoftLight),
            "difference" => Ok(Self::Difference),
            "exclusion" => Ok(Self::Exclusion),
            "hue" => Ok(Self::Hue),
            "saturation" => Ok(Self::Saturation),
            "color" => Ok(Self::Color),
            "luminosity" => Ok(Self::Luminosity),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Operator {
    #[default]
    Over,
    In,
    Out,
    Atop,
    Xor,
    Lighter,
    Arithmetic,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "over" => Ok(Self::Over),
            "in" => Ok(Self::In),
            "out" => Ok(Self::Out),
            "atop" => Ok(Self::Atop),
            "xor" => Ok(Self::Xor),
            "lighter" => Ok(Self::Lighter),
            "arithmetic" => Ok(Self::Arithmetic),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum EdgeMode {
    #[default]
    Duplicate,
    Wrap,
    None,
}

impl FromStr for EdgeMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "duplicate" => Ok(Self::Duplicate),
            "wrap" => Ok(Self::Wrap),
            "none" => Ok(Self::None),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ChannelSelector {
    R,
    G,
    B,
    #[default]
    A,
}

impl FromStr for ChannelSelector {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Self::R),
            "G" => Ok(Self::G),
            "B" => Ok(Self::B),
            "A" => Ok(Self::A),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
    #[default]
    Empty,
}

impl FromStr for CrossOrigin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "anonymous" => Ok(Self::Anonymous),
            "use-credentials" => Ok(Self::UseCredentials),
            "" => Ok(Self::Empty),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum StitchTiles {
    #[default]
    NoStitch,
    Stitch,
}

impl FromStr for StitchTiles {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noStitch" => Ok(Self::NoStitch),
            "stitch" => Ok(Self::Stitch),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum GradientUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl FromStr for GradientUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum SpreadMethod {
    #[default]
    Pad,
    Reflect,
    Repeat,
}

impl FromStr for SpreadMethod {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pad" => Ok(Self::Pad),
            "reflect" => Ok(Self::Reflect),
            "repeat" => Ok(Self::Repeat),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Decoding {
    #[default]
    Auto,
    Synchronous,
    Asynchronous,
}

impl FromStr for Decoding {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "sync" => Ok(Self::Synchronous),
            "async" => Ok(Self::Asynchronous),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum FetchPriority {
    #[default]
    Auto,
    High,
    Low,
}

impl FromStr for FetchPriority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Self::Auto),
            "high" => Ok(Self::High),
            "low" => Ok(Self::Low),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum LengthAdjust {
    #[default]
    Spacing,
    SpacingAndGlyphs,
}

impl FromStr for LengthAdjust {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "spacing" => Ok(Self::Spacing),
            "spacingAndGlyphs" => Ok(Self::SpacingAndGlyphs),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ClipPathUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FromStr for ClipPathUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Method {
    #[default]
    Align,
    Stretch,
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "align" => Ok(Self::Align),
            "stretch" => Ok(Self::Stretch),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Side {
    #[default]
    Left,
    Right,
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Self::Left),
            "right" => Ok(Self::Right),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Spacing {
    #[default]
    Exact,
    Auto,
}

impl FromStr for Spacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "exact" => Ok(Self::Exact),
            "auto" => Ok(Self::Auto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum FilterUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl FromStr for FilterUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum PrimitiveUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl FromStr for PrimitiveUnits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "userSpaceOnUse" => Ok(Self::UserSpaceOnUse),
            "objectBoundingBox" => Ok(Self::ObjectBoundingBox),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    Xmlns(String),
    // Core(Global) Attributes
    Autofocus(bool),
    Id(String),
    Class(Vec<String>),
    Style(String),
    Lang(String), // TODO: this should be a BCP47 language tag as specified in: https://developer.mozilla.org/en-US/docs/Glossary/BCP_47_language_tag
    Tabindex(i64),

    // Conditional Processing Attributes
    RequiredExtensions(Vec<String>), // Value is a space-separated list of URL references
    // identifying the required extensions
    SystemLanguage(String), // TODO: this should be a BCP47 language tag as specified in: https://developer.mozilla.org/en-US/docs/Glossary/BCP_47_language_tag

    // Presentation Attributes
    AlignmentBaseline(AlignmentBaseline),
    BaselineShift(BaselineShift),
    ClipPath(String), // TODO: figure out how this works
    ClipRule(ClipRule),
    Color(Color),
    ColorInterpolation(ColorInterpolation),
    ColorInterpolationFilters(ColorInterpolationFilter),
    Cursor(Cursor),
    Cx(LengthOrPercentage),
    Cy(LengthOrPercentage),
    D(Vec<Path>),
    Direction(TextDirection),
    Display(Display),
    DominantBaseline(DominantBaseline),
    Fill(Fill),
    FillOpacity(Percentage),
    FillRule(FillRule),
    Filter(String), // TODO: implement this
    FloodColor(Color),
    FloodOpacity(f64),  // value between 0 and 1
    FontFamily(String), // TODO: implement proper font family parsing
    FontSize(FontSize),
    FontSizeAdjust(FontSizeAdjust),
    FontStyle(FontStyle),
    FontVariant(String), // TODO: implement proper font variant parsing
    FontWeight(FontWeight),
    Height(LengthOrPercentage), // TODO: here the type depends on the element used
    ImageRendering(ImageRendering),
    LetterSpacing(LetterSpacing),
    LightingColor(LightingColor),
    MarkerEnd(Marker),
    MarkerMid(Marker),
    MarkerStart(Marker),
    Mask(String), // TODO: implement this
    MaskType(MaskType),
    Opacity(Opacity),
    Overflow(Overflow),
    PointerEvents(PointerEvents),
    R(LengthOrPercentage), // TODO: this is either a RectRadius, or CircleRadius? maybe more
    Rx(EllipsisRadius),
    Ry(EllipsisRadius),
    ShapeRendering(ShapeRendering),
    StopColor(StopColor),
    StopOpacity(Opacity),
    Stroke(Paint),
    StrokeDasharray(Vec<i64>), // Space separated list of numbers
    StrokeDashoffset(LengthOrPercentage),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(f64), // TODO: default value = 4.0
    StrokeOpacity(StrokeOpacity),
    StrokeWidth(LengthOrPercentage),
    TextAnchor(TextAnchor),
    TextDecoration(String), // TODO: implement this
    TextOverflow(TextOverflow),
    TextRendering(TextRendering),
    Transform(String),       // TODO: implement this
    TransformOrigin(String), // TODO: implement this
    UnicodeBidi(UnicodeBidi),
    VectorEffect(VectorEffect),
    Visibility(Visibility),
    Width(LengthOrPercentage), // TODO: here the type depends on the element used, for example
    // pattern accepts only Length type
    WhiteSpace(WhiteSpace),
    WordSpacing(WordSpacing),
    WritingMode(WritingMode),
    X(LengthOrPercentage), // TODO: here the type depends on the element used
    Y(LengthOrPercentage), // TODO: here the type depends on the element used

    // Transfer Function Attributes
    // TODO: The rest of the attributes are left out for now
    Type,
    TableValues,
    Slope,
    Intercept,
    Amplitude,
    Exponent,
    Offset, // TODO: This one is maybe deprecated, where to find the documentation for it?

    // Animation Attributes
    Href(String),
    AttributeType,
    AttributeName,
    Begin,
    Dur,
    End,
    Min,
    Max,
    Restart,
    RepeatCount,
    RepeatDur,
    Additive,
    Accumulate,

    // Event Attributes (taken from https://www.w3schools.com/tags/ref_eventattributes.asp)
    OnAfterPrint(String),
    OnBeforePrint(String),
    OnBeforeUnload(String),
    OnError(String),
    OnHashChange(String),
    OnLoad(String),
    OnMessage(String),
    OnOffline(String),
    OnOnline(String),
    OnPageHide(String),
    OnPageShow(String),
    OnPopState(String),
    OnResize(String),
    OnStorage(String),
    OnUnload(String),

    OnBlur(String),
    OnChange(String),
    OnContextMenu(String),
    OnFocus(String),
    OnInput(String),
    OnInvalid(String),
    OnReset(String),
    OnSearch(String),
    OnSelect(String),
    OnSubmit(String),

    OnKeyDown(String),
    OnKeyPress(String),
    OnKeyUp(String),

    OnClick(String),
    OnDoubleClick(String),
    OnMouseDown(String),
    OnMouseMove(String),
    OnMouseOut(String),
    OnMouseOver(String),
    OnMouseUp(String),
    OnWheel(String),

    OnDrag(String),
    OnDragEnd(String),
    OnDragEnter(String),
    OnDragLeave(String),
    OnDragOver(String),
    OnDragStart(String),
    OnDrop(String),
    OnScroll(String),

    OnCopy(String),
    OnCut(String),
    OnPaste(String),

    OnAbort(String),
    OnCanPlay(String),
    OnCanPlayThrough(String),
    OnCueChange(String),
    OnDurationChange(String),
    OnEmptied(String),
    OnEnded(String),
    OnLoadedData(String),
    OnLoadedMetadata(String),
    OnLoadStart(String),
    OnPause(String),
    OnPlay(String),
    OnPlaying(String),
    OnProgress(String),
    OnRateChange(String),
    OnSeeked(String),
    OnSeeking(String),
    OnStalled(String),
    OnSuspend(String),
    OnTimeUpdate(String),
    OnVolumeChange(String),
    OnWaiting(String),

    OnToggle(String),

    // Element Specific
    KeyPoints(String),
    Path(Vec<Path>),
    Rotate(Rotate),

    // Animation Value Attributes
    CalcMode,
    Values,
    KeyTimes,
    KeySplines,
    From,
    To,
    By,
    PathLength(f64),

    X1(LengthOrPercentageOrNumber), // TODO: this can be a LengthOrPercentageOrNumber on <line> or LengthOrPercentage only on <linearGradient>, also the defaults are different (Number(0.0) and Percentage(0.0))
    Y1(LengthOrPercentageOrNumber),
    X2(LengthOrPercentageOrNumber),
    Y2(LengthOrPercentageOrNumber),

    Points(Vec<Point>),

    Download(String),
    HrefLang(String), // TODO: this should be a BCP47 language tag as specified in: https://developer.mozilla.org/en-US/docs/Glossary/BCP_47_language_tag
    InterestFor(String), // TODO: https://developer.mozilla.org/en-US/docs/Web/API/Popover_API/Using_interest_invokers
    Ping(Vec<Url>),
    ReferrerPolicy(ReferrerPolicy),
    Rel(Vec<RelType>),
    Target(Target),

    MarkerHeight(LengthOrPercentage),
    MarkerUnits(MarkerUnits),
    MarkerWidth(LengthOrPercentage),
    Orient(Orient),
    PreserveAspectRatio(PreserveAspectRatio),
    RefX(RefX),
    RefY(RefY),
    ViewBox(ViewBox),

    MaskContentUnits(MaskContentUnits),
    MaskUnits(MaskUnits),

    PatternContentUnits(PatternContentUnits),
    PatternUnits(PatternUnits),
    PatternTransform(String), // TODO: implement this

    Result(String),
    In(In),
    In2(In),
    Mode(BlendMode),

    Operator(Operator),
    K1(f64),
    K2(f64),
    K3(f64),
    K4(f64),

    Order(u64), // TODO: if it's a valid f64 then it must be truncated to u32
    KernelMatrix(Vec<f64>),
    Divisor(f64),
    Bias(f64),
    TargetX(i64),
    TargetY(i64),
    EdgeMode(EdgeMode),
    KernelUnitLength(f64, Option<f64>),
    PreserveAlpha(bool),

    SurfaceScale(f64),
    DiffuseConstant(f64),

    Scale(f64),
    XChannelSelector(ChannelSelector),
    YChannelSelector(ChannelSelector),

    Dx(f64),
    Dy(f64),
    StdDeviation(f64, Option<f64>),

    CrossOrigin(CrossOrigin),

    Radius(f64, Option<f64>),

    SpecularConstant(f64),
    SpecularExponent(f64),

    BaseFrequency(f64, Option<f64>),
    NumOctaves(u64),
    Seed(f64),
    StitchTiles(StitchTiles),

    GradientUnits(GradientUnits),
    GradientTransform(String), // TODO: implement this
    SpreadMethod(SpreadMethod),

    Fx(LengthOrPercentage),
    Fy(LengthOrPercentage),
    Fr(LengthOrPercentage),

    Decoding(Decoding),
    FetchPriority(FetchPriority),

    LengthAdjust(LengthAdjust),
    TextLength(LengthOrPercentage),

    ClipPathUnits(ClipPathUnits),

    Method(Method),
    Side(Side),
    Spacing(Spacing),
    StartOffset(LengthOrPercentageOrNumber),

    FilterUnits(FilterUnits),
    PrimitiveUnits(PrimitiveUnits),
}

impl TryFrom<(&String, &String)> for Attribute {
    type Error = ();

    fn try_from((key, value): (&String, &String)) -> Result<Self, Self::Error> {
        match key.as_str() {
            "xmlns" => Ok(Attribute::Xmlns(value.clone())),
            "autofocus" => {
                if value.is_empty() || value.eq_ignore_ascii_case("autofocus") {
                    Ok(Attribute::Autofocus(true))
                } else {
                    Ok(Attribute::Autofocus(false))
                }
            }
            "id" => {
                if value.is_empty() {
                    // This is invalid, but we need to parse it
                }

                Ok(Attribute::Id(value.clone()))
            }
            "class" => Ok(Attribute::Class(
                value
                    .split_whitespace()
                    .map(|class| class.to_string())
                    .collect(),
            )),
            "style" => Ok(Attribute::Style(value.clone())),
            "lang" => Ok(Attribute::Lang(value.clone())),
            "tabindex" => Ok(Attribute::Tabindex(value.parse().unwrap_or(0))),
            "requiredExtensions" => {
                let extensions = value
                    .split_whitespace()
                    .map(|ext| ext.to_string())
                    .collect();
                Ok(Attribute::RequiredExtensions(extensions))
            }
            "systemLanguage" => Ok(Attribute::SystemLanguage(value.clone())),
            "alignment-baseline" => Ok(Attribute::AlignmentBaseline(value.parse()?)),
            "baseline-shift" => Ok(Attribute::BaselineShift(value.parse()?)),
            "clip-path" => Ok(Attribute::ClipPath(value.clone())),
            "clip-rule" => Ok(Attribute::ClipRule(value.parse()?)),
            "color" => Ok(Attribute::Color(value.parse()?)),
            "color-interpolation" => Ok(Attribute::ColorInterpolation(value.parse()?)),
            "color-interpolation-filters" => {
                Ok(Attribute::ColorInterpolationFilters(value.parse()?))
            }
            "cursor" => Ok(Attribute::Cursor(value.parse()?)),
            "cx" => Ok(Attribute::Cx(value.parse()?)),
            "cy" => Ok(Attribute::Cy(value.parse()?)),
            "d" => Ok(Attribute::D(
                value
                    .split_whitespace()
                    .map(Path::from_str)
                    .collect::<Result<_, _>>()?,
            )),
            "direction" => Ok(Attribute::Direction(value.parse()?)),
            "display" => Ok(Attribute::Display(value.parse()?)),
            "dominant-baseline" => Ok(Attribute::DominantBaseline(value.parse()?)),
            "fill" => Ok(Attribute::Fill(value.parse()?)),
            "fill-opacity" => Ok(Attribute::FillOpacity(value.parse()?)),
            "fill-rule" => Ok(Attribute::FillRule(value.parse()?)),
            "filter" => Ok(Attribute::Filter(value.clone())),
            "flood-color" => Ok(Attribute::FloodColor(value.parse()?)),
            "flood-opacity" => Ok(Attribute::FloodOpacity(value.parse().map_err(|_| ())?)),
            "font-family" => Ok(Attribute::FontFamily(value.clone())),
            "font-size" => Ok(Attribute::FontSize(value.parse().map_err(|_| ())?)),
            "font-size-adjust" => Ok(Attribute::FontSizeAdjust(value.parse()?)),
            "font-style" => Ok(Attribute::FontStyle(value.parse()?)),
            "font-variant" => Ok(Attribute::FontVariant(value.clone())),
            "font-weight" => Ok(Attribute::FontWeight(value.parse()?)),
            "height" => Ok(Attribute::Height(value.parse().unwrap_or(
                LengthOrPercentage::Length(Length::Absolute(AbsoluteLength::Px(1.0))),
            ))),
            "image-rendering" => Ok(Attribute::ImageRendering(value.parse()?)),
            "letter-spacing" => Ok(Attribute::LetterSpacing(value.parse()?)),
            "lighting-color" => Ok(Attribute::LightingColor(value.parse()?)),
            "marker-end" => Ok(Attribute::MarkerEnd(value.parse()?)),
            "marker-mid" => Ok(Attribute::MarkerMid(value.parse()?)),
            "marker-start" => Ok(Attribute::MarkerStart(value.parse()?)),
            "mask" => Ok(Attribute::Mask(value.clone())),
            "mask-type" => Ok(Attribute::MaskType(value.parse()?)),
            "opacity" => Ok(Attribute::Opacity(value.parse()?)),
            "overflow" => Ok(Attribute::Overflow(value.parse()?)),
            "pointer-events" => Ok(Attribute::PointerEvents(value.parse()?)),
            "r" => Ok(Attribute::R(value.parse()?)),
            "rx" => Ok(Attribute::Rx(value.parse()?)),
            "ry" => Ok(Attribute::Ry(value.parse()?)),
            "shape-rendering" => Ok(Attribute::ShapeRendering(value.parse()?)),
            "stop-color" => Ok(Attribute::StopColor(value.parse()?)),
            "stop-opacity" => Ok(Attribute::StopOpacity(value.parse()?)),
            "stroke" => Ok(Attribute::Stroke(value.parse()?)),
            "stroke-dasharray" => Ok(Attribute::StrokeDasharray(
                value
                    .split_whitespace()
                    .map(i64::from_str)
                    .collect::<Result<_, _>>()
                    .map_err(|_| ())?,
            )),
            "stroke-dashoffset" => Ok(Attribute::StrokeDashoffset(value.parse()?)),
            "stroke-linecap" => Ok(Attribute::StrokeLinecap(value.parse()?)),
            "stroke-linejoin" => Ok(Attribute::StrokeLinejoin(value.parse()?)),
            "stroke-miterlimit" => Ok(Attribute::StrokeMiterlimit(value.parse().map_err(|_| ())?)),
            "stroke-opacity" => Ok(Attribute::StrokeOpacity(value.parse()?)),
            "stroke-width" => Ok(Attribute::StrokeWidth(value.parse()?)),
            "text-anchor" => Ok(Attribute::TextAnchor(value.parse()?)),
            "text-decoration" => Ok(Attribute::TextDecoration(value.clone())),
            "text-overflow" => Ok(Attribute::TextOverflow(value.parse()?)),
            "text-rendering" => Ok(Attribute::TextRendering(value.parse()?)),
            "transform" => Ok(Attribute::Transform(value.clone())),
            "transform-origin" => Ok(Attribute::TransformOrigin(value.clone())),
            "unicode-bidi" => Ok(Attribute::UnicodeBidi(value.parse()?)),
            "vector-effect" => Ok(Attribute::VectorEffect(value.parse()?)),
            "visibility" => Ok(Attribute::Visibility(value.parse()?)),
            "width" => Ok(Attribute::Width(value.parse().unwrap_or(
                LengthOrPercentage::Length(Length::Absolute(AbsoluteLength::Px(1.0))),
            ))),
            "white-space" => Ok(Attribute::WhiteSpace(value.parse()?)),
            "word-spacing" => Ok(Attribute::WordSpacing(value.parse()?)),
            "writing-mode" => Ok(Attribute::WritingMode(value.parse()?)),
            "x" => Ok(Attribute::X(value.parse().unwrap_or(
                LengthOrPercentage::Length(Length::Absolute(AbsoluteLength::Px(0.0))),
            ))),
            "y" => Ok(Attribute::Y(value.parse().unwrap_or(
                LengthOrPercentage::Length(Length::Absolute(AbsoluteLength::Px(0.0))),
            ))),
            "type" => todo!(),
            "tableValues" => todo!(),
            "slope" => todo!(),
            "intercept" => todo!(),
            "amplitude" => todo!(),
            "exponent" => todo!(),
            "offset" => todo!(),
            "href" => Ok(Attribute::Href(value.clone())),
            "attributeType" => todo!(),
            "attributeName" => todo!(),
            "begin" => todo!(),
            "dur" => todo!(),
            "end" => todo!(),
            "min" => todo!(),
            "max" => todo!(),
            "restart" => todo!(),
            "repeatCount" => todo!(),
            "repeatDur" => todo!(),
            "additive" => todo!(),
            "accumulate" => todo!(),
            "onAfterPrint" => todo!(),
            "onBeforePrint" => todo!(),
            "onBeforeUnload" => todo!(),
            "onError" => todo!(),
            "onHashChange" => todo!(),
            "onLoad" => todo!(),
            "onMessage" => todo!(),
            "onOffline" => todo!(),
            "onOnline" => todo!(),
            "onPageHide" => todo!(),
            "onPageShow" => todo!(),
            "onPopState" => todo!(),
            "onResize" => todo!(),
            "onStorage" => todo!(),
            "onUnload" => todo!(),
            "onBlur" => todo!(),
            "onChange" => todo!(),
            "onContextMenu" => todo!(),
            "onFocus" => todo!(),
            "onInput" => todo!(),
            "onInvalid" => todo!(),
            "onReset" => todo!(),
            "onSearch" => todo!(),
            "onSelect" => todo!(),
            "onSubmit" => todo!(),
            "onKeyDown" => todo!(),
            "onKeyPress" => todo!(),
            "onKeyUp" => todo!(),
            "onClick" => todo!(),
            "onDoubleClick" => todo!(),
            "onMouseDown" => todo!(),
            "onMouseMove" => todo!(),
            "onMouseOut" => todo!(),
            "onMouseOver" => todo!(),
            "onMouseUp" => todo!(),
            "onWheel" => todo!(),
            "onDrag" => todo!(),
            "onDragEnd" => todo!(),
            "onDragEnter" => todo!(),
            "onDragLeave" => todo!(),
            "onDragOver" => todo!(),
            "onDragStart" => todo!(),
            "onDrop" => todo!(),
            "onScroll" => todo!(),
            "onCopy" => todo!(),
            "onCut" => todo!(),
            "onPaste" => todo!(),
            "onAbort" => todo!(),
            "onCanPlay" => todo!(),
            "onCanPlayThrough" => todo!(),
            "onCueChange" => todo!(),
            "onDurationChange" => todo!(),
            "onEmptied" => todo!(),
            "onEnded" => todo!(),
            "onLoadedData" => todo!(),
            "onLoadedMetadata" => todo!(),
            "onLoadStart" => todo!(),
            "onPause" => todo!(),
            "onPlay" => todo!(),
            "onPlaying" => todo!(),
            "onProgress" => todo!(),
            "onRateChange" => todo!(),
            "onSeeked" => todo!(),
            "onSeeking" => todo!(),
            "onStalled" => todo!(),
            "onSuspend" => todo!(),
            "onTimeUpdate" => todo!(),
            "onVolumeChange" => todo!(),
            "onWaiting" => todo!(),
            "onToggle" => todo!(),
            _ => {
                return Err(());
            }
        }
    }
}

impl Attribute {
    #[inline]
    pub fn is_global(&self) -> bool {
        matches!(
            self,
            Attribute::Autofocus(_)
                | Attribute::Id(_)
                | Attribute::Class(_)
                | Attribute::Style(_)
                | Attribute::Lang(_)
                | Attribute::Tabindex(_)
        )
    }

    #[inline]
    pub fn is_core(&self) -> bool {
        self.is_global()
    }

    #[inline]
    pub fn is_conditional_processing(&self) -> bool {
        matches!(
            self,
            Attribute::RequiredExtensions(_) | Attribute::SystemLanguage(_)
        )
    }

    #[inline]
    pub fn is_presentation(&self) -> bool {
        matches!(
            self,
            Attribute::AlignmentBaseline(_)
                | Attribute::BaselineShift(_)
                | Attribute::ClipPath(_)
                | Attribute::ClipRule(_)
                | Attribute::Color(_)
                | Attribute::ColorInterpolation(_)
                | Attribute::ColorInterpolationFilters(_)
                | Attribute::Cursor(_)
                | Attribute::Cx(_)
                | Attribute::Cy(_)
                | Attribute::D(_)
                | Attribute::Direction(_)
                | Attribute::Display(_)
                | Attribute::DominantBaseline(_)
                | Attribute::Fill(_)
                | Attribute::FillOpacity(_)
                | Attribute::FillRule(_)
                | Attribute::Filter(_)
                | Attribute::FloodColor(_)
                | Attribute::FloodOpacity(_)
                | Attribute::FontFamily(_)
                | Attribute::FontSize(_)
                | Attribute::FontSizeAdjust(_)
                | Attribute::FontStyle(_)
                | Attribute::FontVariant(_)
                | Attribute::FontWeight(_)
                | Attribute::Height(_)
                | Attribute::ImageRendering(_)
                | Attribute::LetterSpacing(_)
                | Attribute::LightingColor(_)
                | Attribute::MarkerEnd(_)
                | Attribute::MarkerMid(_)
                | Attribute::MarkerStart(_)
                | Attribute::Mask(_)
                | Attribute::MaskType(_)
                | Attribute::Opacity(_)
                | Attribute::Overflow(_)
                | Attribute::PointerEvents(_)
                | Attribute::R(_)
                | Attribute::Rx(_)
                | Attribute::Ry(_)
                | Attribute::ShapeRendering(_)
                | Attribute::StopColor(_)
                | Attribute::StopOpacity(_)
                | Attribute::Stroke(_)
                | Attribute::StrokeDasharray(_)
                | Attribute::StrokeDashoffset(_)
                | Attribute::StrokeLinecap(_)
                | Attribute::StrokeLinejoin(_)
                | Attribute::StrokeMiterlimit(_)
                | Attribute::StrokeOpacity(_)
                | Attribute::StrokeWidth(_)
                | Attribute::TextAnchor(_)
                | Attribute::TextDecoration(_)
                | Attribute::TextOverflow(_)
                | Attribute::TextRendering(_)
                | Attribute::Transform(_)
                | Attribute::TransformOrigin(_)
                | Attribute::UnicodeBidi(_)
                | Attribute::VectorEffect(_)
                | Attribute::Visibility(_)
                | Attribute::Width(_)
                | Attribute::WhiteSpace(_)
                | Attribute::WordSpacing(_)
                | Attribute::WritingMode(_)
                | Attribute::X(_)
                | Attribute::Y(_)
        )
    }

    #[inline]
    pub fn is_animation_timing(&self) -> bool {
        matches!(
            self,
            Attribute::Begin
                | Attribute::Dur
                | Attribute::End
                | Attribute::Min
                | Attribute::Max
                | Attribute::Restart
                | Attribute::RepeatCount
                | Attribute::RepeatDur
                | Attribute::Fill(_)
        )
    }

    #[inline]
    pub fn is_animation_value(&self) -> bool {
        matches!(
            self,
            Attribute::CalcMode
                | Attribute::Values
                | Attribute::KeyTimes
                | Attribute::KeySplines
                | Attribute::From
                | Attribute::To
                | Attribute::By
        )
    }

    #[inline]
    pub fn is_animation_addition(&self) -> bool {
        matches!(self, Attribute::Accumulate | Attribute::Additive)
    }

    #[inline]
    pub fn applies_to_shape(&self) -> bool {
        self.is_fill() || self.is_stroke()
    }

    #[inline]
    pub fn is_fill(&self) -> bool {
        matches!(
            self,
            Attribute::Fill(_) | Attribute::FillOpacity(_) | Attribute::FillRule(_)
        )
    }

    #[inline]
    pub fn is_stroke(&self) -> bool {
        matches!(
            self,
            Attribute::Stroke(_)
                | Attribute::StrokeDasharray(_)
                | Attribute::StrokeDashoffset(_)
                | Attribute::StrokeLinecap(_)
                | Attribute::StrokeLinejoin(_)
                | Attribute::StrokeMiterlimit(_)
                | Attribute::StrokeOpacity(_)
                | Attribute::StrokeWidth(_)
        )
    }

    pub fn is_filter_primitive(&self) -> bool {
        matches!(
            self,
            Attribute::Height(_)
                | Attribute::Result(_)
                | Attribute::Width(_)
                | Attribute::X(_)
                | Attribute::Y(_)
        )
    }

    pub fn allowed_in_element(&self, element_type: ElementType, element: &Element) -> bool {
        match element_type {
            ElementType::Animate => self.is_global(),
            ElementType::AnimateMotion => {
                self.is_global()
                    || self.is_animation_timing()
                    || self.is_animation_value()
                    || self.is_animation_addition()
                    || matches!(
                        self,
                        Attribute::KeyPoints(_)
                            | Attribute::Path(_)
                            | Attribute::Rotate(_)
                            | Attribute::AttributeName // TODO: OnBegin, OnEnd, OnRepeat events
                    )
            }
            ElementType::AnimateTransform => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::By | Attribute::From | Attribute::To | Attribute::Type
                    )
            }
            ElementType::MPath => self.is_global() || matches!(self, Attribute::Href(_)),
            ElementType::Set => matches!(self, Attribute::To),
            ElementType::Circle => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::R(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::Ellipse => {
                self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::Rx(_)
                            | Attribute::Ry(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::Line => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X1(_) | Attribute::Y1(_) | Attribute::X2(_) | Attribute::Y2(_)
                    )
            }
            ElementType::Polygon => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::Points(_) | Attribute::PathLength(_))
            }
            ElementType::PolyLine => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::Points(_) | Attribute::PathLength(_))
            }
            ElementType::Rect => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::Rx(_)
                            | Attribute::Ry(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::A => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::Download(_)
                            | Attribute::HrefLang(_)
                            | Attribute::InterestFor(_)
                            | Attribute::Ping(_)
                            | Attribute::ReferrerPolicy(_)
                            | Attribute::Rel(_)
                            | Attribute::Target(_)
                            | Attribute::Type
                    )
            }
            ElementType::Defs => self.is_global(),
            ElementType::G => self.is_global(),
            ElementType::Marker => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::MarkerHeight(_)
                            | Attribute::MarkerUnits(_)
                            | Attribute::MarkerWidth(_)
                            | Attribute::Orient(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::RefX(_)
                            | Attribute::RefY(_)
                            | Attribute::ViewBox(_)
                    )
            }
            ElementType::Mask => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::MaskUnits(_)
                            | Attribute::MaskContentUnits(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                    )
            }
            ElementType::Pattern => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::Href(_)
                            | Attribute::PatternContentUnits(_)
                            | Attribute::PatternUnits(_)
                            | Attribute::PatternTransform(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                    )
            }
            ElementType::Svg => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Xmlns(_)
                    )
            }
            ElementType::Switch => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::RequiredExtensions(_)
                            | Attribute::SystemLanguage(_)
                    )
            }
            ElementType::Symbol => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::RefX(_)
                            | Attribute::RefY(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                    )
            }
            ElementType::Desc => self.is_global(),
            ElementType::Metadata => self.is_global(),
            ElementType::Title => self.is_global(),
            ElementType::FeBlend => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::In2(_) | Attribute::Mode(_)
                    )
            }
            ElementType::FeColorMatrix => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(self, Attribute::In(_) | Attribute::Type | Attribute::Values)
            }
            ElementType::FeComponentTransfer => {
                self.is_global() || self.is_filter_primitive() || matches!(self, Attribute::In(_))
            }
            ElementType::FeComposite => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::In2(_)
                            | Attribute::Operator(_)
                            | Attribute::K1(_)
                            | Attribute::K2(_)
                            | Attribute::K3(_)
                            | Attribute::K4(_)
                    )
            }
            ElementType::FeConvolveMatrix => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::Order(_)
                            | Attribute::KernelMatrix(_)
                            | Attribute::Divisor(_)
                            | Attribute::Bias(_)
                            | Attribute::TargetX(_)
                            | Attribute::TargetY(_)
                            | Attribute::EdgeMode(_)
                            | Attribute::KernelUnitLength(_, _)
                            | Attribute::PreserveAlpha(_)
                    )
            }
            ElementType::FeDiffuseLightning => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::SurfaceScale(_)
                            | Attribute::DiffuseConstant(_)
                            | Attribute::KernelUnitLength(_, _)
                    )
            }
            ElementType::FeDisplacementMap => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::In2(_)
                            | Attribute::Scale(_)
                            | Attribute::XChannelSelector(_)
                            | Attribute::YChannelSelector(_)
                    )
            }
            ElementType::FeDropShadow => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::StdDeviation(_, _)
                    )
            }
            ElementType::FeFlood => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(self, Attribute::FloodColor(_) | Attribute::FloodOpacity(_))
            }
            ElementType::FeFuncA => self.is_global(),
            ElementType::FeFuncB => self.is_global(),
            ElementType::FeFuncG => self.is_global(),
            ElementType::FeFuncR => self.is_global(),
            ElementType::FeGaussianBlur => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::StdDeviation(_, _) | Attribute::EdgeMode(_)
                    )
            }
            ElementType::FeImage => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::CrossOrigin(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::Href(_)
                    )
            }
            ElementType::FeMerge => self.is_global() || self.is_filter_primitive(),
            ElementType::FeMergeNode => self.is_global() || matches!(self, Attribute::In(_)),
            ElementType::FeMorphology => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::Operator(_) | Attribute::Radius(_, _)
                    )
            }
            ElementType::FeOffset => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(self, Attribute::In(_) | Attribute::Dx(_) | Attribute::Dy(_))
            }
            ElementType::FeSpecularLighting => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::SurfaceScale(_)
                            | Attribute::SpecularConstant(_)
                            | Attribute::SpecularExponent(_)
                            | Attribute::KernelUnitLength(_, _)
                    )
            }
            ElementType::FeTile => {
                self.is_global() || self.is_filter_primitive() || matches!(self, Attribute::In(_))
            }
            ElementType::FeTurbulence => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::BaseFrequency(_, _)
                            | Attribute::NumOctaves(_)
                            | Attribute::Seed(_)
                            | Attribute::StitchTiles(_)
                            | Attribute::Type
                    )
            }
            ElementType::LinearGradient => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::GradientUnits(_)
                            | Attribute::GradientTransform(_)
                            | Attribute::Href(_)
                            | Attribute::SpreadMethod(_)
                            | Attribute::X1(_)
                            | Attribute::Y1(_)
                            | Attribute::X2(_)
                            | Attribute::Y2(_)
                    )
            }
            ElementType::RadialGradient => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::Fr(_)
                            | Attribute::Fx(_)
                            | Attribute::Fy(_)
                            | Attribute::GradientUnits(_)
                            | Attribute::GradientTransform(_)
                            | Attribute::Href(_)
                            | Attribute::R(_)
                            | Attribute::SpreadMethod(_)
                    )
            }
            ElementType::Stop => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Offset | Attribute::StopColor(_) | Attribute::StopOpacity(_)
                    )
            }
            ElementType::Image => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::Href(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::CrossOrigin(_)
                            | Attribute::Decoding(_)
                            | Attribute::FetchPriority(_)
                    )
            }
            ElementType::Path => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::D(_) | Attribute::PathLength(_))
            }
            ElementType::Text => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::Rotate(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::Use => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                    )
            }
            ElementType::FeDistantLight => todo!(),
            ElementType::FePointLight => todo!(),
            ElementType::FeSpotLight => todo!(),
            ElementType::ClipPath => {
                self.is_global() || matches!(self, Attribute::ClipPathUnits(_))
            }
            ElementType::Script => todo!(),
            ElementType::Style => todo!(),
            ElementType::TextPath => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::Method(_)
                            | Attribute::Path(_)
                            | Attribute::Side(_)
                            | Attribute::Spacing(_)
                            | Attribute::StartOffset(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::TSpan => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::Rotate(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::Filter => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::FilterUnits(_)
                            | Attribute::PrimitiveUnits(_)
                    )
            }
            ElementType::ForeignObject => todo!(),
            ElementType::View => todo!(),
        }
    }
}
