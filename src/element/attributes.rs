use std::{convert::Infallible, fmt, str::FromStr};

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

impl AlignmentBaseline {
    fn as_str(&self) -> &str {
        match self {
            AlignmentBaseline::Auto => "auto",
            AlignmentBaseline::Baseline => "baseline",
            AlignmentBaseline::BeforeEdge => "before-edge",
            AlignmentBaseline::TextBeforeEdge => "text-before-edge",
            AlignmentBaseline::Middle => "middle",
            AlignmentBaseline::Central => "central",
            AlignmentBaseline::AfterEdge => "after-edge",
            AlignmentBaseline::TextAfterEdge => "text-after-edge",
            AlignmentBaseline::Ideographic => "ideographic",
            AlignmentBaseline::Alphabetic => "alphabetic",
            AlignmentBaseline::Hanging => "hanging",
            AlignmentBaseline::Mathematical => "mathematical",
            AlignmentBaseline::Top => "top",
            AlignmentBaseline::Center => "center",
            AlignmentBaseline::Bottom => "bottom",
        }
    }
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

impl fmt::Display for BaselineShift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaselineShift::LengthOrPercentage(length_or_percentage) => {
                write!(f, "{}", length_or_percentage)
            }
            BaselineShift::Sub => write!(f, "sub"),
            BaselineShift::Super => write!(f, "super"),
        }
    }
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

impl ClipRule {
    fn as_str(&self) -> &str {
        match self {
            ClipRule::NonZero => "nonzero",
            ClipRule::EvenOdd => "evenodd",
            ClipRule::Inherit => "inherit",
        }
    }
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

impl ColorInterpolation {
    pub fn as_str(&self) -> &str {
        match self {
            ColorInterpolation::Auto => "auto",
            ColorInterpolation::SRGB => "sRGB",
            ColorInterpolation::LinearRGB => "linearRGB",
        }
    }
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

impl ColorInterpolationFilter {
    pub fn as_str(&self) -> &str {
        match self {
            ColorInterpolationFilter::Auto => "auto",
            ColorInterpolationFilter::SRGB => "sRGB",
            ColorInterpolationFilter::LinearRGB => "linearRGB",
        }
    }
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

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Cursor::Auto => "auto",
            Cursor::Crosshair => "crosshair",
            Cursor::Default => "default",
            Cursor::Pointer => "pointer",
            Cursor::Move => "move",
            Cursor::EResize => "e-resize",
            Cursor::NEResize => "ne-resize",
            Cursor::NWResize => "nw-resize",
            Cursor::NResize => "n-resize",
            Cursor::SEResize => "se-resize",
            Cursor::SWResize => "sw-resize",
            Cursor::SResize => "s-resize",
            Cursor::WResize => "w-resize",
            Cursor::Text => "text",
            Cursor::Wait => "wait",
            Cursor::Help => "help",
            Cursor::Inherit => "inherit",
        })
    }
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
    Absolute((Number, Number)), // M x y
    Relative((Number, Number)), // m dx dy
}

impl fmt::Display for MoveTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MoveTo::Absolute((x, y)) => write!(f, "M {},{}", x, y),
            MoveTo::Relative((dx, dy)) => write!(f, "m {},{}", dx, dy),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Number(f64);

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Number {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LineTo {
    XYAbsolute(Vec<Number>),         // L (x y)+
    XYRelative(Vec<Number>),         // l (dx dy)+
    HorizontalAbsolute(Vec<Number>), // H (x)+
    HorizontalRelative(Vec<Number>), // h (dx)+
    VerticalAbsolute(Vec<Number>),   // V (y)+
    VerticalRelative(Vec<Number>),   // v (dy)+
}

impl fmt::Display for LineTo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LineTo::XYAbsolute(v) => write!(
                f,
                "L {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            LineTo::XYRelative(v) => write!(
                f,
                "l {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            LineTo::HorizontalAbsolute(v) => write!(
                f,
                "H {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            LineTo::HorizontalRelative(v) => write!(
                f,
                "h {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            LineTo::VerticalAbsolute(v) => write!(
                f,
                "V {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            LineTo::VerticalRelative(v) => write!(
                f,
                "v {}",
                v.iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join(",")
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CubicBezierCurvePoint {
    pub x1: Number,
    pub y1: Number,
    pub x2: Number,
    pub y2: Number,
    pub x: Number,
    pub y: Number,
}

impl fmt::Display for CubicBezierCurvePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {}",
            self.x1,
            self.y1,
            self.x2,
            self.y2,
            self.x,
            self.y,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SmoothCubicBezierCurvePoint {
    pub x2: Number,
    pub y2: Number,
    pub x: Number,
    pub y: Number,
}

impl fmt::Display for SmoothCubicBezierCurvePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.x2,
            self.y2,
            self.x,
            self.y
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CubicBezierCurve {
    Absolute(Vec<CubicBezierCurvePoint>), // C (x1 y1 x2 y2 x y)+
    Relative(Vec<CubicBezierCurvePoint>), // c (x1 y1 x2 y2 x y)+
    SmoothAbsolute(Vec<SmoothCubicBezierCurvePoint>), // S (x2 y2 x y)+
    SmoothRelative(Vec<SmoothCubicBezierCurvePoint>), // s (x2 y2 x y)+
}

impl fmt::Display for CubicBezierCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubicBezierCurve::Absolute(cubic_bezier_curve_points) => write!(
                f,
                "C {}",
                cubic_bezier_curve_points
                    .iter()
                    .map(|cubic_bezier_curve_point| cubic_bezier_curve_point.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            CubicBezierCurve::Relative(cubic_bezier_curve_points) => write!(
                f,
                "c {}",
                cubic_bezier_curve_points
                    .iter()
                    .map(|cubic_bezier_curve_point| cubic_bezier_curve_point.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            CubicBezierCurve::SmoothAbsolute(smooth_cubic_bezier_curve_points) => {
                write!(
                    f,
                    "S {}",
                    smooth_cubic_bezier_curve_points
                        .iter()
                        .map(|smooth_cubic_bezier_curve_point| {
                            smooth_cubic_bezier_curve_point.to_string()
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            CubicBezierCurve::SmoothRelative(smooth_cubic_bezier_curve_points) => {
                write!(
                    f,
                    "s {}",
                    smooth_cubic_bezier_curve_points
                        .iter()
                        .map(|smooth_cubic_bezier_curve_point| {
                            smooth_cubic_bezier_curve_point.to_string()
                        })
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct QuadraticBezierCurvePoint {
    pub x1: Number,
    pub y1: Number,
    pub x: Number,
    pub y: Number,
}

impl fmt::Display for QuadraticBezierCurvePoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {}",
            self.x1,
            self.y1,
            self.x,
            self.y
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Point {
    pub x: Number,
    pub y: Number,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.x, self.y)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum QuadraticBezierCurve {
    Absolute(Vec<QuadraticBezierCurvePoint>), // Q (x1 y1 x y)+
    Relative(Vec<QuadraticBezierCurvePoint>), // q (x1 y1 x y)+
    SmoothAbsolute(Vec<Point>),               // T (x y)+
    SmoothRelative(Vec<Point>),               // t (x y)+
}

impl fmt::Display for QuadraticBezierCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QuadraticBezierCurve::Absolute(quadratic_bezier_curve_points) => {
                write!(
                    f,
                    "Q {}",
                    quadratic_bezier_curve_points
                        .iter()
                        .map(|point| point.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            QuadraticBezierCurve::Relative(quadratic_bezier_curve_points) => {
                write!(
                    f,
                    "q {}",
                    quadratic_bezier_curve_points
                        .iter()
                        .map(|point| point.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
                )
            }
            QuadraticBezierCurve::SmoothAbsolute(points) => write!(
                f,
                "T {}",
                points
                    .iter()
                    .map(|point| point.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            QuadraticBezierCurve::SmoothRelative(points) => write!(
                f,
                "t {}",
                points
                    .iter()
                    .map(|point| point.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EllipticalArcPoint {
    pub rx: Number,
    pub ry: Number,
    pub angle: Number,
    pub large_arc_flag: bool,
    pub sweep_flag: bool,
    pub x: Number,
    pub y: Number,
}

impl fmt::Display for EllipticalArcPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {}",
            self.rx,
            self.ry,
            self.angle,
            self.large_arc_flag as u8,
            self.sweep_flag as u8,
            self.x,
            self.y,
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum EllipticalArcCurve {
    Absolute(Vec<EllipticalArcPoint>), // A (rx ry angle large-arc-flag sweep-flag x y)+
    Relative(Vec<EllipticalArcPoint>), // a (rx ry angle large-arc-flag sweep-flag dx dy)+
}

impl fmt::Display for EllipticalArcCurve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EllipticalArcCurve::Absolute(elliptical_arc_points) => write!(
                f,
                "A {}",
                elliptical_arc_points
                    .iter()
                    .map(|point| point.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
            EllipticalArcCurve::Relative(elliptical_arc_points) => write!(
                f,
                "a {}",
                elliptical_arc_points
                    .iter()
                    .map(|point| point.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PathType {
    MoveTo(MoveTo),                             // M m
    LineTo(LineTo),                             // L l H h V v
    CubicBezierCurve(CubicBezierCurve),         // C c S s
    QuadraticBezierCurve(QuadraticBezierCurve), // Q q T t
    EllipticalArcCurve(EllipticalArcCurve),     // A a
    ClosePath,                                  // Z z
}

impl fmt::Display for PathType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PathType::MoveTo(move_to) => write!(f, "{}", move_to),
            PathType::LineTo(line_to) => write!(f, "{}", line_to),
            PathType::CubicBezierCurve(cubic_bezier_curve) => write!(f, "{}", cubic_bezier_curve),
            PathType::QuadraticBezierCurve(quadratic_bezier_curve) => {
                write!(f, "{}", quadratic_bezier_curve)
            }
            PathType::EllipticalArcCurve(elliptical_arc_curve) => write!(f, "{}", elliptical_arc_curve),
            PathType::ClosePath => write!(f, "Z"),
        }
    }
}

impl PathType {
    fn split_left_rest(input: &str) -> Result<(&str, &str), ()> {
        input.split_once(',').ok_or(())
    }

    fn split_eos(input: &str) -> (&str, &str) {
        let split_index = input
            .find(|c: char| c.is_ascii_alphabetic() && !matches!(c, '.' | '+' | '-' | 'e' | 'E'))
            .unwrap_or(input.len());
        let (left, rest) = input.split_at(split_index);

        (left, rest)
    }

    fn split_left_right_rest(input: &str) -> Result<(&str, &str, &str), ()> {
        let (left, rest) = Self::split_left_rest(input)?;
        let (right, rest) = Self::split_eos(rest);

        Ok((left, right, rest))
    }

    pub fn parse(s: &str) -> Result<(Self, &str), ()> {
        let Some(first_char) = s.chars().next() else {
            return Err(());
        };

        let rest = &s[1..];

        match first_char {
            'M' => {
                let (left, right, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::MoveTo(MoveTo::Absolute((
                        Number::from_str(left)?,
                        Number::from_str(right)?,
                    ))),
                    rest,
                ))
            }
            'm' => {
                let (left, right, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::MoveTo(MoveTo::Relative((
                        Number::from_str(left)?,
                        Number::from_str(right)?,
                    ))),
                    rest,
                ))
            }
            'L' => {
                let (left, right, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::LineTo(LineTo::XYAbsolute(vec![
                        Number::from_str(left)?,
                        Number::from_str(right)?,
                    ])),
                    rest,
                ))
            }
            'l' => {
                let (left, right, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::LineTo(LineTo::XYRelative(vec![
                        Number::from_str(left)?,
                        Number::from_str(right)?,
                    ])),
                    rest,
                ))
            }
            'H' => {
                let (left, rest) = Self::split_eos(rest);

                Ok((
                    PathType::LineTo(LineTo::HorizontalAbsolute(vec![Number::from_str(left)?])),
                    rest,
                ))
            }
            'h' => {
                let (left, rest) = Self::split_eos(rest);

                Ok((
                    PathType::LineTo(LineTo::HorizontalRelative(vec![Number::from_str(left)?])),
                    rest,
                ))
            }
            'V' => {
                let (left, rest) = Self::split_eos(rest);

                Ok((
                    PathType::LineTo(LineTo::VerticalAbsolute(vec![Number::from_str(left)?])),
                    rest,
                ))
            }
            'v' => {
                let (left, rest) = Self::split_eos(rest);

                Ok((
                    PathType::LineTo(LineTo::VerticalRelative(vec![Number::from_str(left)?])),
                    rest,
                ))
            }
            'C' => {
                let (x1_1, rest) = Self::split_left_rest(rest)?;
                let (y1_1, rest) = Self::split_left_rest(rest)?;
                let (x2_1, rest) = Self::split_left_rest(rest)?;
                let (y2_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x1_2, rest) = Self::split_left_rest(rest)?;
                let (y1_2, rest) = Self::split_left_rest(rest)?;
                let (x2_2, rest) = Self::split_left_rest(rest)?;
                let (y2_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                let (x1_3, rest) = Self::split_left_rest(rest)?;
                let (y1_3, rest) = Self::split_left_rest(rest)?;
                let (x2_3, rest) = Self::split_left_rest(rest)?;
                let (y2_3, rest) = Self::split_left_rest(rest)?;
                let (x_3, rest) = Self::split_left_rest(rest)?;
                let (y_3, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::CubicBezierCurve(CubicBezierCurve::Absolute(vec![
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_1)?,
                            y1: Number::from_str(y1_1)?,
                            x2: Number::from_str(x2_1)?,
                            y2: Number::from_str(y2_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_2)?,
                            y1: Number::from_str(y1_2)?,
                            x2: Number::from_str(x2_2)?,
                            y2: Number::from_str(y2_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_3)?,
                            y1: Number::from_str(y1_3)?,
                            x2: Number::from_str(x2_3)?,
                            y2: Number::from_str(y2_3)?,
                            x: Number::from_str(x_3)?,
                            y: Number::from_str(y_3)?,
                        },
                    ])),
                    rest,
                ))
            }
            'c' => {
                let (x1_1, rest) = Self::split_left_rest(rest)?;
                let (y1_1, rest) = Self::split_left_rest(rest)?;
                let (x2_1, rest) = Self::split_left_rest(rest)?;
                let (y2_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x1_2, rest) = Self::split_left_rest(rest)?;
                let (y1_2, rest) = Self::split_left_rest(rest)?;
                let (x2_2, rest) = Self::split_left_rest(rest)?;
                let (y2_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                let (x1_3, rest) = Self::split_left_rest(rest)?;
                let (y1_3, rest) = Self::split_left_rest(rest)?;
                let (x2_3, rest) = Self::split_left_rest(rest)?;
                let (y2_3, rest) = Self::split_left_rest(rest)?;
                let (x_3, rest) = Self::split_left_rest(rest)?;
                let (y_3, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::CubicBezierCurve(CubicBezierCurve::Relative(vec![
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_1)?,
                            y1: Number::from_str(y1_1)?,
                            x2: Number::from_str(x2_1)?,
                            y2: Number::from_str(y2_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_2)?,
                            y1: Number::from_str(y1_2)?,
                            x2: Number::from_str(x2_2)?,
                            y2: Number::from_str(y2_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                        CubicBezierCurvePoint {
                            x1: Number::from_str(x1_3)?,
                            y1: Number::from_str(y1_3)?,
                            x2: Number::from_str(x2_3)?,
                            y2: Number::from_str(y2_3)?,
                            x: Number::from_str(x_3)?,
                            y: Number::from_str(y_3)?,
                        },
                    ])),
                    rest,
                ))
            }
            'S' => {
                let (x2_1, rest) = Self::split_left_rest(rest)?;
                let (y2_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x2_2, rest) = Self::split_left_rest(rest)?;
                let (y2_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::CubicBezierCurve(CubicBezierCurve::SmoothAbsolute(vec![
                        SmoothCubicBezierCurvePoint {
                            x2: Number::from_str(x2_1)?,
                            y2: Number::from_str(y2_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        SmoothCubicBezierCurvePoint {
                            x2: Number::from_str(x2_2)?,
                            y2: Number::from_str(y2_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            's' => {
                let (x2_1, rest) = Self::split_left_rest(rest)?;
                let (y2_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x2_2, rest) = Self::split_left_rest(rest)?;
                let (y2_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::CubicBezierCurve(CubicBezierCurve::SmoothRelative(vec![
                        SmoothCubicBezierCurvePoint {
                            x2: Number::from_str(x2_1)?,
                            y2: Number::from_str(y2_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        SmoothCubicBezierCurvePoint {
                            x2: Number::from_str(x2_2)?,
                            y2: Number::from_str(y2_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            'Q' => {
                let (x1_1, rest) = Self::split_left_rest(rest)?;
                let (y1_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x1_2, rest) = Self::split_left_rest(rest)?;
                let (y1_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::QuadraticBezierCurve(QuadraticBezierCurve::Absolute(vec![
                        QuadraticBezierCurvePoint {
                            x1: Number::from_str(x1_1)?,
                            y1: Number::from_str(y1_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        QuadraticBezierCurvePoint {
                            x1: Number::from_str(x1_2)?,
                            y1: Number::from_str(y1_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            'q' => {
                let (x1_1, rest) = Self::split_left_rest(rest)?;
                let (y1_1, rest) = Self::split_left_rest(rest)?;
                let (x_1, rest) = Self::split_left_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (x1_2, rest) = Self::split_left_rest(rest)?;
                let (y1_2, rest) = Self::split_left_rest(rest)?;
                let (x_2, rest) = Self::split_left_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::QuadraticBezierCurve(QuadraticBezierCurve::Relative(vec![
                        QuadraticBezierCurvePoint {
                            x1: Number::from_str(x1_1)?,
                            y1: Number::from_str(y1_1)?,
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        QuadraticBezierCurvePoint {
                            x1: Number::from_str(x1_2)?,
                            y1: Number::from_str(y1_2)?,
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            'T' => {
                let (x_1, y_1, rest) = Self::split_left_right_rest(rest)?;
                let (x_2, y_2, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::QuadraticBezierCurve(QuadraticBezierCurve::SmoothAbsolute(vec![
                        Point {
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        Point {
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            't' => {
                let (x_1, y_1, rest) = Self::split_left_right_rest(rest)?;
                let (x_2, y_2, rest) = Self::split_left_right_rest(rest)?;

                Ok((
                    PathType::QuadraticBezierCurve(QuadraticBezierCurve::SmoothRelative(vec![
                        Point {
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        Point {
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                    ])),
                    rest,
                ))
            }
            'A' => {
                let (rx_1, ry_1, rest) = Self::split_left_right_rest(rest)?;
                let (angle_1, large_arc_flag_1, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_1, x_1, rest) = Self::split_left_right_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (rx_2, ry_2, rest) = Self::split_left_right_rest(rest)?;
                let (angle_2, large_arc_flag_2, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_2, x_2, rest) = Self::split_left_right_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                let (rx_3, ry_3, rest) = Self::split_left_right_rest(rest)?;
                let (angle_3, large_arc_flag_3, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_3, x_3, rest) = Self::split_left_right_rest(rest)?;
                let (y_3, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::EllipticalArcCurve(EllipticalArcCurve::Absolute(vec![
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_1)?,
                            ry: Number::from_str(ry_1)?,
                            angle: Number::from_str(angle_1).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_1.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_1.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_2)?,
                            ry: Number::from_str(ry_2)?,
                            angle: Number::from_str(angle_2).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_2.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_2.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_3)?,
                            ry: Number::from_str(ry_3)?,
                            angle: Number::from_str(angle_3).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_3.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_3.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_3)?,
                            y: Number::from_str(y_3)?,
                        },
                    ])),
                    rest,
                ))
            }
            'a' => {
                let (rx_1, ry_1, rest) = Self::split_left_right_rest(rest)?;
                let (angle_1, large_arc_flag_1, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_1, x_1, rest) = Self::split_left_right_rest(rest)?;
                let (y_1, rest) = Self::split_left_rest(rest)?;

                let (rx_2, ry_2, rest) = Self::split_left_right_rest(rest)?;
                let (angle_2, large_arc_flag_2, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_2, x_2, rest) = Self::split_left_right_rest(rest)?;
                let (y_2, rest) = Self::split_left_rest(rest)?;

                let (rx_3, ry_3, rest) = Self::split_left_right_rest(rest)?;
                let (angle_3, large_arc_flag_3, rest) = Self::split_left_right_rest(rest)?;
                let (sweep_flag_3, x_3, rest) = Self::split_left_right_rest(rest)?;
                let (y_3, rest) = Self::split_left_rest(rest)?;

                Ok((
                    PathType::EllipticalArcCurve(EllipticalArcCurve::Relative(vec![
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_1)?,
                            ry: Number::from_str(ry_1)?,
                            angle: Number::from_str(angle_1).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_1.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_1.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_1)?,
                            y: Number::from_str(y_1)?,
                        },
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_2)?,
                            ry: Number::from_str(ry_2)?,
                            angle: Number::from_str(angle_2).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_2.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_2.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_2)?,
                            y: Number::from_str(y_2)?,
                        },
                        EllipticalArcPoint {
                            rx: Number::from_str(rx_3)?,
                            ry: Number::from_str(ry_3)?,
                            angle: Number::from_str(angle_3).map_err(|_| ())?,
                            large_arc_flag: large_arc_flag_3.eq_ignore_ascii_case("1"),
                            sweep_flag: sweep_flag_3.eq_ignore_ascii_case("1"),
                            x: Number::from_str(x_3)?,
                            y: Number::from_str(y_3)?,
                        },
                    ])),
                    rest,
                ))
            }
            'Z' => Ok((PathType::ClosePath, "")),
            'z' => Ok((PathType::ClosePath, "")),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Path(Vec<PathType>);

impl FromStr for Path {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        // M31,3h38l28,28v38l-28,28h-38l-28-28v-38z
        let replaced = input.replace(' ', "");
        let mut input = replaced.as_str();

        let mut result = vec![];

        while !input.is_empty() {
            if let Ok((path_type, new_input)) = PathType::parse(&input) {
                result.push(path_type);
                input = new_input;
            }
        }

        Ok(Self(result))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDirection::Ltr => write!(f, "ltr"),
            TextDirection::Rtl => write!(f, "rtl"),
        }
    }
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

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Display::Inline => "inline",
            Display::Block => "block",
            Display::RunIn => "run-in",
            Display::Flow => "flow",
            Display::FlowRoot => "flow-root",
            Display::Table => "table",
            Display::Flex => "flex",
            Display::Grid => "grid",
            Display::Ruby => "ruby",
            Display::ListItem => "list-item",
            Display::TableRowGroup => "table-row-group",
            Display::TableHeaderGroup => "table-header-group",
            Display::TableFooterGroup => "table-footer-group",
            Display::TableRow => "table-row",
            Display::TableCell => "table-cell",
            Display::TableColumnGroup => "table-column-group",
            Display::TableColumn => "table-column",
            Display::TableCaption => "table-caption",
            Display::RubyBase => "ruby-base",
            Display::RubyText => "ruby-text",
            Display::RubyBaseContainer => "ruby-base-container",
            Display::RubyTextContainer => "ruby-text-container",
            Display::Contents => "contents",
            Display::None => "none",
            Display::InlineBlock => "inline-block",
            Display::InlineTable => "inline-table",
            Display::InlineFlex => "inline-flex",
            Display::InlineGrid => "inline-grid",
        })
    }
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

impl fmt::Display for DominantBaseline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            DominantBaseline::Auto => "auto",
            DominantBaseline::TextBottom => "text-bottom",
            DominantBaseline::Alphabetic => "alphabetic",
            DominantBaseline::Ideographic => "ideographic",
            DominantBaseline::Middle => "middle",
            DominantBaseline::Central => "central",
            DominantBaseline::Mathematical => "mathematical",
            DominantBaseline::Hanging => "hanging",
            DominantBaseline::TextTop => "text-top",
        })
    }
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
    None,
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fill::Paint(paint) => write!(f, "{}", paint),
            Fill::Freeze => write!(f, "freeze"),
            Fill::Remove => write!(f, "remove"),
            Fill::None => write!(f, "none"),
        }
    }
}

impl FromStr for Fill {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "remove" => Ok(Fill::Remove),
            "freeze" => Ok(Fill::Freeze),
            "none" => Ok(Fill::None),
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

impl fmt::Display for FillRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FillRule::NonZero => write!(f, "nonzero"),
            FillRule::EvenOdd => write!(f, "evenodd"),
        }
    }
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

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSize::Absolute(absolute_size) => write!(f, "{}", absolute_size),
            FontSize::Relative(relative_size) => write!(f, "{}", relative_size),
            FontSize::Length(length) => write!(f, "{}", length),
            FontSize::Percentage(percentage) => write!(f, "{}", percentage),
        }
    }
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

impl fmt::Display for FontSizeAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSizeAdjust::None => write!(f, "none"),
            FontSizeAdjust::Number(number) => write!(f, "{}", number),
        }
    }
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

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontStyle::Normal => write!(f, "normal"),
            FontStyle::Italic => write!(f, "italic"),
            FontStyle::Oblique => write!(f, "oblique"),
        }
    }
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

impl fmt::Display for ImageRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ImageRendering::Auto => "auto",
            ImageRendering::OptimizeSpeed => "optimizeSpeed",
            ImageRendering::OptimizeQuality => "optimizeQuality",
        })
    }
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

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LetterSpacing::Normal => write!(f, "normal"),
            LetterSpacing::Length(length) => write!(f, "{}", length),
        }
    }
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

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Marker::None => write!(f, "none"),
            Marker::Url(_) => todo!(),
        }
    }
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

impl fmt::Display for MaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskType::Alpha => write!(f, "alpha"),
            MaskType::Luminance => write!(f, "luminance"),
        }
    }
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

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Overflow::Visible => write!(f, "visible"),
            Overflow::Hidden => write!(f, "hidden"),
            Overflow::Scroll => write!(f, "scroll"),
            Overflow::Auto => write!(f, "auto"),
        }
    }
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

impl fmt::Display for PointerEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointerEvents::BoundingBox => write!(f, "bounding-box"),
            PointerEvents::VisiblePainted => write!(f, "visiblePainted"),
            PointerEvents::VisibleFill => write!(f, "visibleFill"),
            PointerEvents::VisibleStroke => write!(f, "visibleStroke"),
            PointerEvents::Visible => write!(f, "visible"),
            PointerEvents::Painted => write!(f, "painted"),
            PointerEvents::Fill => write!(f, "fill"),
            PointerEvents::Stroke => write!(f, "stroke"),
            PointerEvents::All => write!(f, "all"),
            PointerEvents::None => write!(f, "none"),
        }
    }
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

// NOTE: This is called EllipsisRadius, but it can be applied to both ellipse and a rect
#[derive(Clone, Debug, PartialEq, Default)]
pub enum EllipsisRadius {
    LengthOrPercentage(LengthOrPercentage),
    #[default]
    Auto,
}

impl fmt::Display for EllipsisRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EllipsisRadius::LengthOrPercentage(length_or_percentage) => {
                write!(f, "{}", length_or_percentage)
            }
            EllipsisRadius::Auto => write!(f, "auto"),
        }
    }
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

impl fmt::Display for ShapeRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ShapeRendering::Auto => "auto",
            ShapeRendering::OptimizeSpeed => "optimizeSpeed",
            ShapeRendering::CrispEdges => "crispEdges",
            ShapeRendering::GeometricPrecision => "geometricPrecision",
        })
    }
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

impl fmt::Display for StopColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

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

impl fmt::Display for StrokeLinecap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            StrokeLinecap::Butt => "butt",
            StrokeLinecap::Round => "round",
            StrokeLinecap::Square => "square",
        })
    }
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

impl fmt::Display for StrokeLinejoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            StrokeLinejoin::Arcs => "arcs",
            StrokeLinejoin::Bevel => "bevel",
            StrokeLinejoin::Miter => "miter",
            StrokeLinejoin::MiterClip => "miter-clip",
            StrokeLinejoin::Round => "round",
        })
    }
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

impl fmt::Display for StrokeOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrokeOpacity::Number(number) => write!(f, "{}", number),
            StrokeOpacity::Percentage(percentage) => write!(f, "{}", percentage),
        }
    }
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

impl fmt::Display for VectorEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            VectorEffect::None => "none",
            VectorEffect::NonScalingStroke => "non-scaling-stroke",
            VectorEffect::NonScalingSize => "non-scaling-size",
            VectorEffect::NonRotation => "non-rotation",
            VectorEffect::FixedPosition => "fixeed-position",
        })
    }
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

impl fmt::Display for TextAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TextAnchor::Start => "start",
            TextAnchor::Middle => "middle",
            TextAnchor::End => "end",
        })
    }
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
    Ellipsis,
}

impl fmt::Display for TextOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TextOverflow::Clip => "clip",
            TextOverflow::Ellipsis => "ellipsis",
        })
    }
}

impl FromStr for TextOverflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clip" => Ok(TextOverflow::Clip),
            "ellipsis" => Ok(TextOverflow::Ellipsis),
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

impl fmt::Display for TextRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            TextRendering::Auto => "auto",
            TextRendering::OptimizeSpeed => "optimizeSpeed",
            TextRendering::OptimizeLegibility => "optimizeLegibility",
            TextRendering::GeometricPrecision => "geometricPrecision",
        })
    }
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

impl fmt::Display for UnicodeBidi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            UnicodeBidi::Normal => "normal",
            UnicodeBidi::Embed => "embed",
            UnicodeBidi::Isolate => "isolate",
            UnicodeBidi::BidiOverride => "bidi-override",
            UnicodeBidi::IsolateOverride => "isolate-override",
            UnicodeBidi::Plaintext => "plaintext",
        })
    }
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

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Visibility::Visible => "visible",
            Visibility::Hidden => "hidden",
            Visibility::Collapse => "collapse",
        })
    }
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

impl fmt::Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            WhiteSpace::Normal => "normal",
            WhiteSpace::Pre => "pre",
            WhiteSpace::Nowrap => "nowrap",
            WhiteSpace::PreWrap => "pre-wrap",
            WhiteSpace::BreakSpace => "break-space",
            WhiteSpace::PreLine => "pre-line",
        })
    }
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

impl fmt::Display for WordSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordSpacing::Normal => write!(f, "normal"),
            WordSpacing::Length(length) => write!(f, "{}", length),
        }
    }
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

impl fmt::Display for WritingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            WritingMode::HorizontalTb => "horizontal-tb",
            WritingMode::VerticalRl => "vertical-rl",
            WritingMode::VerticalLr => "vertical-lr",
        })
    }
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

impl fmt::Display for Rotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rotate::Auto => write!(f, "auto"),
            Rotate::AutoReverse => write!(f, "auto-reverse"),
            Rotate::Number(number) => write!(f, "{}", number),
        }
    }
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

impl fmt::Display for LengthOrPercentageOrNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthOrPercentageOrNumber::Length(length) => write!(f, "{}", length),
            LengthOrPercentageOrNumber::Percentage(percentage) => write!(f, "{}", percentage),
            LengthOrPercentageOrNumber::Number(number) => write!(f, "{}", number),
        }
    }
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

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ReferrerPolicy::NoReferrer => "no-referrer",
            ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
            ReferrerPolicy::SameOrigin => "same-origin",
            ReferrerPolicy::Origin => "origin",
            ReferrerPolicy::StrictOrigin => "strict-origin",
            ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
            ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
            ReferrerPolicy::UnsafeUrl => "unsafe-url",
        })
    }
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

impl fmt::Display for RelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            RelType::Alternate => "alternate",
            RelType::Author => "author",
            RelType::Bookmark => "bookmark",
            RelType::Canonical => "canonical",
            RelType::CompressionDictionary => "compression-dictionary",
            RelType::DnsPrefetch => "dns-prefetch",
            RelType::External => "external",
            RelType::Expect => "expect",
            RelType::Help => "help",
            RelType::Icon => "icon",
            RelType::License => "license",
            RelType::Manifest => "manifest",
            RelType::Me => "me",
            RelType::ModulePreload => "module-preload",
            RelType::Next => "next",
            RelType::NoFollow => "nofollow",
            RelType::NoOpener => "noopener",
            RelType::NoReferrer => "noreferrer",
            RelType::Opener => "opener",
            RelType::PingBack => "pingback",
            RelType::PreConnect => "preconnect",
            RelType::Prefetch => "prefetch",
            RelType::Preload => "preload",
            RelType::Prev => "prev",
            RelType::PrivacyPolicy => "privacy-policy",
            RelType::Search => "search",
            RelType::StyleSheet => "stylesheet",
            RelType::Tag => "tag",
            RelType::TermsOfService => "terms-of-service",
        })
    }
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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Target::Self_ => "_self",
            Target::Parent => "parent",
            Target::Top => "top",
            Target::Blank => "blank",
        })
    }
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

impl fmt::Display for MarkerUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MarkerUnits::UserSpaceOnUse => "userSpaceOnUse",
            MarkerUnits::StrokeWidth => "strokeWidth",
        })
    }
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

impl fmt::Display for Orient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Orient::Auto => write!(f, "auto"),
            Orient::AutoStartReverse => write!(f, "auto-start-reverse"),
            Orient::Angle(angle) => write!(f, "{}", angle),
        }
    }
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

impl fmt::Display for PreserveAspectRatio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PreserveAspectRatio::None => "none",
            PreserveAspectRatio::XMinYMinMeet => "xMinYMin meet",
            PreserveAspectRatio::XMidYMinMeet => "xMidYMid meet",
            PreserveAspectRatio::XMaxYMinMeet => "xMaxYMin meet",
            PreserveAspectRatio::XMinYMidMeet => "xMinYMid meet",
            PreserveAspectRatio::XMidYMidMeet => "xMidYMid meet",
            PreserveAspectRatio::XMaxYMidMeet => "xMaxYMid meet",
            PreserveAspectRatio::XMinYMaxMeet => "xMinYMax meet",
            PreserveAspectRatio::XMidYMaxMeet => "xMidYMax meet",
            PreserveAspectRatio::XMaxYMaxMeet => "xMaxYMax meet",
            PreserveAspectRatio::XMinYMinSlice => "xMinYMin slice",
            PreserveAspectRatio::XMidYMinSlice => "xMidYMid slice",
            PreserveAspectRatio::XMaxYMinSlice => "xMaxYMin slice",
            PreserveAspectRatio::XMinYMidSlice => "xMinYMid slice",
            PreserveAspectRatio::XMidYMidSlice => "xMidYMid slice",
            PreserveAspectRatio::XMaxYMidSlice => "xMaxYMid slice",
            PreserveAspectRatio::XMinYMaxSlice => "xMinYMax slice",
            PreserveAspectRatio::XMidYMaxSlice => "xMidYMax slice",
            PreserveAspectRatio::XMaxYMaxSlice => "xMaxYMax slice",
        })
    }
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

impl fmt::Display for RefX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefX::Left => write!(f, "left"),
            RefX::Center => write!(f, "center"),
            RefX::Right => write!(f, "right"),
            RefX::Coordinate(length_or_percentage) => write!(f, "{}", length_or_percentage),
        }
    }
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

impl fmt::Display for RefY {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RefY::Top => write!(f, "top"),
            RefY::Center => write!(f, "center"),
            RefY::Bottom => write!(f, "bottom"),
            RefY::Coordinate(length_or_percentage) => write!(f, "{}", length_or_percentage),
        }
    }
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

impl fmt::Display for MaskContentUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MaskContentUnits::UserSpaceOnUse => "userSpaceOnUse",
            MaskContentUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for MaskUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            MaskUnits::UserSpaceOnUse => "userSpaceOnUse",
            MaskUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for PatternContentUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PatternContentUnits::UserSpaceOnUse => "userSpaceOnUse",
            PatternContentUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for PatternUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PatternUnits::UserSpaceOnUse => "userSpaceOnUse",
            PatternUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for In {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            In::SourceGraphic => write!(f, "SourceGraphic"),
            In::SourceAlpha => write!(f, "SourceAlpha"),
            In::BackgroundImage => write!(f, "BackgroundImage"),
            In::BackgroundAlpha => write!(f, "BackgroundAlpha"),
            In::FillPaint => write!(f, "FillPaint"),
            In::StrokePaint => write!(f, "StrokePaint"),
            In::Identifier(identifier) => write!(f, "{}", identifier),
        }
    }
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

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            BlendMode::Normal => "normal",
            BlendMode::Multiply => "multiply",
            BlendMode::Screen => "screen",
            BlendMode::Overlay => "overlay",
            BlendMode::Darken => "darken",
            BlendMode::Lighten => "lighten",
            BlendMode::ColorDodge => "color-dodge",
            BlendMode::ColorBurn => "color-burn",
            BlendMode::HardLight => "hard-light",
            BlendMode::SoftLight => "soft-light",
            BlendMode::Difference => "difference",
            BlendMode::Exclusion => "exclusion",
            BlendMode::Hue => "hue",
            BlendMode::Saturation => "saturation",
            BlendMode::Color => "color",
            BlendMode::Luminosity => "luminosity",
        })
    }
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

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Operator::Over => "over",
            Operator::In => "in",
            Operator::Out => "out",
            Operator::Atop => "atop",
            Operator::Xor => "xor",
            Operator::Lighter => "lighter",
            Operator::Arithmetic => "arithmetic",
        })
    }
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

impl fmt::Display for EdgeMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            EdgeMode::Duplicate => "duplicate",
            EdgeMode::Wrap => "wrap",
            EdgeMode::None => "none",
        })
    }
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

impl fmt::Display for ChannelSelector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ChannelSelector::R => "R",
            ChannelSelector::G => "G",
            ChannelSelector::B => "B",
            ChannelSelector::A => "A",
        })
    }
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

impl fmt::Display for CrossOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            CrossOrigin::Anonymous => "anonymous",
            CrossOrigin::UseCredentials => "use-credentials",
            CrossOrigin::Empty => "",
        })
    }
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

impl fmt::Display for StitchTiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            StitchTiles::NoStitch => "noStitch",
            StitchTiles::Stitch => "stitch",
        })
    }
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

impl fmt::Display for GradientUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            GradientUnits::UserSpaceOnUse => "userSpaceOnUse",
            GradientUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for SpreadMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            SpreadMethod::Pad => "pad",
            SpreadMethod::Reflect => "reflect",
            SpreadMethod::Repeat => "repeat",
        })
    }
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

impl fmt::Display for Decoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Decoding::Auto => "auto",
            Decoding::Synchronous => "sync",
            Decoding::Asynchronous => "async",
        })
    }
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

impl fmt::Display for FetchPriority {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            FetchPriority::Auto => "auto",
            FetchPriority::High => "high",
            FetchPriority::Low => "low",
        })
    }
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

impl fmt::Display for LengthAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            LengthAdjust::Spacing => "spacing",
            LengthAdjust::SpacingAndGlyphs => "spacingAndGlyphs",
        })
    }
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

impl fmt::Display for ClipPathUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            ClipPathUnits::UserSpaceOnUse => "userSpaceOnUse",
            ClipPathUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Method::Align => "align",
            Method::Stretch => "stretch",
        })
    }
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

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Side::Left => "left",
            Side::Right => "right",
        })
    }
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

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Spacing::Exact => "exact",
            Spacing::Auto => "auto",
        })
    }
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

impl fmt::Display for FilterUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            FilterUnits::UserSpaceOnUse => "userSpaceOnUse",
            FilterUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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

impl fmt::Display for PrimitiveUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            PrimitiveUnits::UserSpaceOnUse => "userSpaceOnUse",
            PrimitiveUnits::ObjectBoundingBox => "objectBoundingBox",
        })
    }
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
    D(Path),
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
    Path(Path),
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
    Version(f64),
}

impl TryFrom<(&String, &String)> for Attribute {
    type Error = ();

    fn try_from((key, value): (&String, &String)) -> Result<Self, Self::Error> {
        match key.as_str() {
            "xmlns" => Ok(Attribute::Xmlns(value.clone())),
            "version" => Ok(Attribute::Version(value.parse().map_err(|_| ())?)),
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
            "d" => Ok(Attribute::D(value.parse()?)),
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
            "viewBox" => Ok(Attribute::ViewBox(value.parse()?)),
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

fn write_space_separated<W, I>(f: &mut W, iter: I) -> fmt::Result
where
    W: fmt::Write,
    I: Iterator,
    I::Item: fmt::Display,
{
    write!(f, "=\"")?;
    for (i, item) in iter.enumerate() {
        if i > 0 {
            write!(f, " ")?;
        }
        write!(f, "{}", item)?;
    }
    write!(f, "\"")
}

impl Attribute {
    pub fn name(&self) -> &str {
        match self {
            Attribute::Xmlns(_) => "xmlns",
            Attribute::Autofocus(_) => "autofocus",
            Attribute::Id(_) => "id",
            Attribute::Class(_) => "class",
            Attribute::Style(_) => "style",
            Attribute::Lang(_) => "lang",
            Attribute::Tabindex(_) => "tabindex",
            Attribute::RequiredExtensions(_) => "requiredExtensions",
            Attribute::SystemLanguage(_) => "systemLanguage",
            Attribute::AlignmentBaseline(_) => "alignment-baseline",
            Attribute::BaselineShift(_) => "baseline-shift",
            Attribute::ClipPath(_) => "clip-path",
            Attribute::ClipRule(_) => "clip-rule",
            Attribute::Color(_) => "color",
            Attribute::ColorInterpolation(_) => "color-interpolation",
            Attribute::ColorInterpolationFilters(_) => "color-interpolation-filters",
            Attribute::Cursor(_) => "cursor",
            Attribute::Cx(_) => "cx",
            Attribute::Cy(_) => "cy",
            Attribute::D(_) => "d",
            Attribute::Direction(_) => "direction",
            Attribute::Display(_) => "display",
            Attribute::DominantBaseline(_) => "dominant-baseline",
            Attribute::Fill(_) => "fill",
            Attribute::FillOpacity(_) => "fill-opacity",
            Attribute::FillRule(_) => "fill-rule",
            Attribute::Filter(_) => "filter",
            Attribute::FloodColor(_) => "flood-color",
            Attribute::FloodOpacity(_) => "flood-opacity",
            Attribute::FontFamily(_) => "font-family",
            Attribute::FontSize(_) => "font-size",
            Attribute::FontSizeAdjust(_) => "font-size-adjust",
            Attribute::FontStyle(_) => "font-style",
            Attribute::FontVariant(_) => "font-variant",
            Attribute::FontWeight(_) => "font-weight",
            Attribute::Height(_) => "height",
            Attribute::ImageRendering(_) => "image-rendering",
            Attribute::LetterSpacing(_) => "letter-spacing",
            Attribute::LightingColor(_) => "lighting-color",
            Attribute::MarkerEnd(_) => "marker-end",
            Attribute::MarkerMid(_) => "marker-mid",
            Attribute::MarkerStart(_) => "marker-start",
            Attribute::Mask(_) => "mask",
            Attribute::MaskType(_) => "mask-type",
            Attribute::Opacity(_) => "opacity",
            Attribute::Overflow(_) => "overflow",
            Attribute::PointerEvents(_) => "pointer-events",
            Attribute::R(_) => "r",
            Attribute::Rx(_) => "rx",
            Attribute::Ry(_) => "ry",
            Attribute::ShapeRendering(_) => "shape-rendering",
            Attribute::StopColor(_) => "stop-color",
            Attribute::StopOpacity(_) => "stop-opacity",
            Attribute::Stroke(_) => "stroke",
            Attribute::StrokeDasharray(_) => "stroke-dasharray",
            Attribute::StrokeDashoffset(_) => "stroke-dashoffset",
            Attribute::StrokeLinecap(_) => "stroke-linecap",
            Attribute::StrokeLinejoin(_) => "stroke-linejoin",
            Attribute::StrokeMiterlimit(_) => "stroke-miterlimit",
            Attribute::StrokeOpacity(_) => "stroke-opacity",
            Attribute::StrokeWidth(_) => "stroke-width",
            Attribute::TextAnchor(_) => "text-anchor",
            Attribute::TextDecoration(_) => "text-decoration",
            Attribute::TextOverflow(_) => "text-overflow",
            Attribute::TextRendering(_) => "text-rendering",
            Attribute::Transform(_) => "transform",
            Attribute::TransformOrigin(_) => "transform-origin",
            Attribute::UnicodeBidi(_) => "unicode-bidi",
            Attribute::VectorEffect(_) => "vector-effect",
            Attribute::Visibility(_) => "visibility",
            Attribute::Width(_) => "width",
            Attribute::WhiteSpace(_) => "white-space",
            Attribute::WordSpacing(_) => "word-spacing",
            Attribute::WritingMode(_) => "writing-mode",
            Attribute::X(_) => "x",
            Attribute::Y(_) => "y",
            Attribute::Type => "type",
            Attribute::TableValues => "tableValues",
            Attribute::Slope => "slope",
            Attribute::Intercept => "intercept",
            Attribute::Amplitude => "amplitude",
            Attribute::Exponent => "exponent",
            Attribute::Offset => "offset",
            Attribute::Href(_) => "href",
            Attribute::AttributeType => "attributeType",
            Attribute::AttributeName => "attributeName",
            Attribute::Begin => "begin",
            Attribute::Dur => "dur",
            Attribute::End => "end",
            Attribute::Min => "min",
            Attribute::Max => "max",
            Attribute::Restart => "restart",
            Attribute::RepeatCount => "repeatCount",
            Attribute::RepeatDur => "repeatDur",
            Attribute::Additive => "additive",
            Attribute::Accumulate => "accumulate",
            Attribute::OnAfterPrint(_) => "onAfterPrint",
            Attribute::OnBeforePrint(_) => "onBeforePrint",
            Attribute::OnBeforeUnload(_) => "onBeforeUnload",
            Attribute::OnError(_) => "onError",
            Attribute::OnHashChange(_) => "onHashChange",
            Attribute::OnLoad(_) => "onLoad",
            Attribute::OnMessage(_) => "onMessage",
            Attribute::OnOffline(_) => "onOffline",
            Attribute::OnOnline(_) => "onOnline",
            Attribute::OnPageHide(_) => "onPageHide",
            Attribute::OnPageShow(_) => "onPageShow",
            Attribute::OnPopState(_) => "onPopState",
            Attribute::OnResize(_) => "onResize",
            Attribute::OnStorage(_) => "onStorage",
            Attribute::OnUnload(_) => "onUnload",
            Attribute::OnBlur(_) => "onBlur",
            Attribute::OnChange(_) => "onChange",
            Attribute::OnContextMenu(_) => "onContextMenu",
            Attribute::OnFocus(_) => "onFocus",
            Attribute::OnInput(_) => "onInput",
            Attribute::OnInvalid(_) => "onInvalid",
            Attribute::OnReset(_) => "onReset",
            Attribute::OnSearch(_) => "onSearch",
            Attribute::OnSelect(_) => "onSelect",
            Attribute::OnSubmit(_) => "onSubmit",
            Attribute::OnKeyDown(_) => "onKeyDown",
            Attribute::OnKeyPress(_) => "onKeyPress",
            Attribute::OnKeyUp(_) => "onKeyUp",
            Attribute::OnClick(_) => "onClick",
            Attribute::OnDoubleClick(_) => "onDoubleClick",
            Attribute::OnMouseDown(_) => "onMouseDown",
            Attribute::OnMouseMove(_) => "onMouseMove",
            Attribute::OnMouseOut(_) => "onMouseOut",
            Attribute::OnMouseOver(_) => "onMouseOver",
            Attribute::OnMouseUp(_) => "onMouseUp",
            Attribute::OnWheel(_) => "onWheel",
            Attribute::OnDrag(_) => "onDrag",
            Attribute::OnDragEnd(_) => "onDragEnd",
            Attribute::OnDragEnter(_) => "onDragEnter",
            Attribute::OnDragLeave(_) => "onDragLeave",
            Attribute::OnDragOver(_) => "onDragOver",
            Attribute::OnDragStart(_) => "onDragStart",
            Attribute::OnDrop(_) => "onDrop",
            Attribute::OnScroll(_) => "onScroll",
            Attribute::OnCopy(_) => "onCopy",
            Attribute::OnCut(_) => "onCut",
            Attribute::OnPaste(_) => "onPaste",
            Attribute::OnAbort(_) => "onAbort",
            Attribute::OnCanPlay(_) => "onCanPlay",
            Attribute::OnCanPlayThrough(_) => "onCanPlayThrough",
            Attribute::OnCueChange(_) => "onCueChange",
            Attribute::OnDurationChange(_) => "onDurationChange",
            Attribute::OnEmptied(_) => "onEmptied",
            Attribute::OnEnded(_) => "onEnded",
            Attribute::OnLoadedData(_) => "onLoadedData",
            Attribute::OnLoadedMetadata(_) => "onLoadedMetadata",
            Attribute::OnLoadStart(_) => "onLoadStart",
            Attribute::OnPause(_) => "onPause",
            Attribute::OnPlay(_) => "onPlay",
            Attribute::OnPlaying(_) => "onPlaying",
            Attribute::OnProgress(_) => "onProgress",
            Attribute::OnRateChange(_) => "onRateChange",
            Attribute::OnSeeked(_) => "onSeeked",
            Attribute::OnSeeking(_) => "onSeeking",
            Attribute::OnStalled(_) => "onStalled",
            Attribute::OnSuspend(_) => "onSuspend",
            Attribute::OnTimeUpdate(_) => "onTimeUpdate",
            Attribute::OnVolumeChange(_) => "onVolumeChange",
            Attribute::OnWaiting(_) => "onWaiting",
            Attribute::OnToggle(_) => "onToggle",
            Attribute::KeyPoints(_) => "keyPoints",
            Attribute::Path(_) => "path",
            Attribute::Rotate(_) => "rotate",
            Attribute::CalcMode => "calcMode",
            Attribute::Values => "values",
            Attribute::KeyTimes => "keyTimes",
            Attribute::KeySplines => "keySplines",
            Attribute::From => "from",
            Attribute::To => "to",
            Attribute::By => "by",
            Attribute::PathLength(_) => "pathLength",
            Attribute::X1(_) => "x1",
            Attribute::Y1(_) => "y1",
            Attribute::X2(_) => "x2",
            Attribute::Y2(_) => "y2",
            Attribute::Points(_) => "points",
            Attribute::Download(_) => "download",
            Attribute::HrefLang(_) => "hrefLang",
            Attribute::InterestFor(_) => "interestFor",
            Attribute::Ping(_) => "ping",
            Attribute::ReferrerPolicy(_) => "referrerPolicy",
            Attribute::Rel(_) => "rel",
            Attribute::Target(_) => "target",
            Attribute::MarkerHeight(_) => "markerHeight",
            Attribute::MarkerUnits(_) => "markerUnits",
            Attribute::MarkerWidth(_) => "markerWidth",
            Attribute::Orient(_) => "orient",
            Attribute::PreserveAspectRatio(_) => "preserveAspectRatio",
            Attribute::RefX(_) => "refX",
            Attribute::RefY(_) => "refY",
            Attribute::ViewBox(_) => "viewBox",
            Attribute::MaskContentUnits(_) => "maskContentUnits",
            Attribute::MaskUnits(_) => "maskUnits",
            Attribute::PatternContentUnits(_) => "patternContentUnits",
            Attribute::PatternUnits(_) => "patternUnits",
            Attribute::PatternTransform(_) => "patternTransform",
            Attribute::Result(_) => "result",
            Attribute::In(_) => "in",
            Attribute::In2(_) => "in2",
            Attribute::Mode(_) => "mode",
            Attribute::Operator(_) => "operator",
            Attribute::K1(_) => "k1",
            Attribute::K2(_) => "k2",
            Attribute::K3(_) => "k3",
            Attribute::K4(_) => "k4",
            Attribute::Order(_) => "order",
            Attribute::KernelMatrix(_) => "kernelMatrix",
            Attribute::Divisor(_) => "divisor",
            Attribute::Bias(_) => "bias",
            Attribute::TargetX(_) => "targetX",
            Attribute::TargetY(_) => "targetY",
            Attribute::EdgeMode(_) => "edgeMode",
            Attribute::KernelUnitLength(_, _) => "kernelUnitLength",
            Attribute::PreserveAlpha(_) => "preserveAlpha",
            Attribute::SurfaceScale(_) => "surfaceScale",
            Attribute::DiffuseConstant(_) => "diffuseConstant",
            Attribute::Scale(_) => "scale",
            Attribute::XChannelSelector(_) => "xChannelSelector",
            Attribute::YChannelSelector(_) => "yChannelSelector",
            Attribute::Dx(_) => "dx",
            Attribute::Dy(_) => "dy",
            Attribute::StdDeviation(_, _) => "stdDeviation",
            Attribute::CrossOrigin(_) => "crossOrigin",
            Attribute::Radius(_, _) => "radius",
            Attribute::SpecularConstant(_) => "specularConstant",
            Attribute::SpecularExponent(_) => "specularExponent",
            Attribute::BaseFrequency(_, _) => "baseFrequency",
            Attribute::NumOctaves(_) => "numOctaves",
            Attribute::Seed(_) => "seed",
            Attribute::StitchTiles(_) => "stitchTiles",
            Attribute::GradientUnits(_) => "gradientUnits",
            Attribute::GradientTransform(_) => "gradientTransform",
            Attribute::SpreadMethod(_) => "spreadMethod",
            Attribute::Fx(_) => "fx",
            Attribute::Fy(_) => "fy",
            Attribute::Fr(_) => "fr",
            Attribute::Decoding(_) => "decoding",
            Attribute::FetchPriority(_) => "fetchPriority",
            Attribute::LengthAdjust(_) => "lengthAdjust",
            Attribute::TextLength(_) => "textLength",
            Attribute::ClipPathUnits(_) => "clipPathUnits",
            Attribute::Method(_) => "method",
            Attribute::Side(_) => "side",
            Attribute::Spacing(_) => "spacing",
            Attribute::StartOffset(_) => "startOffset",
            Attribute::FilterUnits(_) => "filterUnits",
            Attribute::PrimitiveUnits(_) => "primitiveUnits",
            Attribute::Version(_) => "version",
        }
    }

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

    pub fn allowed_in_element(&self, element_type: ElementType, _element: &Element) -> bool {
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

    fn write_value(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Attribute::Xmlns(s) => write!(f, "=\"{}\"", s),
            Attribute::Autofocus(v) => write!(f, "=\"{}\"", if *v { "1" } else { "0" }),
            Attribute::Id(s) => write!(f, "=\"{}\"", s),
            Attribute::Class(items) => write_space_separated(f, items.iter()),
            Attribute::Style(s) => write!(f, "=\"{}\"", s),
            Attribute::Lang(s) => write!(f, "=\"{}\"", s),
            Attribute::Tabindex(n) => write!(f, "=\"{}\"", n),
            Attribute::RequiredExtensions(items) => write_space_separated(f, items.iter()),
            Attribute::SystemLanguage(s) => write!(f, "=\"{}\"", s),
            Attribute::AlignmentBaseline(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::BaselineShift(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipPath(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipRule(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::Color(v) => write!(f, "=\"{}\"", v),
            Attribute::ColorInterpolation(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::ColorInterpolationFilters(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::Cursor(v) => write!(f, "=\"{}\"", v),
            Attribute::Cx(v) => write!(f, "=\"{}\"", v),
            Attribute::Cy(v) => write!(f, "=\"{}\"", v),
            Attribute::D(paths) => write_space_separated(f, paths.0.iter()),
            Attribute::Direction(v) => write!(f, "=\"{}\"", v),
            Attribute::Display(v) => write!(f, "=\"{}\"", v),
            Attribute::DominantBaseline(v) => write!(f, "=\"{}\"", v),
            Attribute::Fill(v) => write!(f, "=\"{}\"", v),
            Attribute::FillOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::FillRule(v) => write!(f, "=\"{}\"", v),
            Attribute::Filter(s) => write!(f, "=\"{}\"", s),
            Attribute::FloodColor(v) => write!(f, "=\"{}\"", v),
            Attribute::FloodOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::FontFamily(v) => write!(f, "=\"{}\"", v),
            Attribute::FontSize(v) => write!(f, "=\"{}\"", v),
            Attribute::FontSizeAdjust(v) => write!(f, "=\"{}\"", v),
            Attribute::FontStyle(v) => write!(f, "=\"{}\"", v),
            Attribute::FontVariant(v) => write!(f, "=\"{}\"", v),
            Attribute::FontWeight(v) => write!(f, "=\"{}\"", v),
            Attribute::Height(v) => write!(f, "=\"{}\"", v),
            Attribute::ImageRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::LetterSpacing(v) => write!(f, "=\"{}\"", v),
            Attribute::LightingColor(v) => write!(f, "=\"{}\"", v.0),
            Attribute::MarkerEnd(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerMid(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerStart(v) => write!(f, "=\"{}\"", v),
            Attribute::Mask(v) => write!(f, "=\"{}\"", v),
            Attribute::MaskType(v) => write!(f, "=\"{}\"", v),
            Attribute::Opacity(v) => write!(f, "=\"{}\"", v),
            Attribute::Overflow(v) => write!(f, "=\"{}\"", v),
            Attribute::PointerEvents(v) => write!(f, "=\"{}\"", v),
            Attribute::R(v) => write!(f, "=\"{}\"", v),
            Attribute::Rx(v) => write!(f, "=\"{}\"", v),
            Attribute::Ry(v) => write!(f, "=\"{}\"", v),
            Attribute::ShapeRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::StopColor(v) => write!(f, "=\"{}\"", v),
            Attribute::StopOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::Stroke(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeDasharray(items) => write_space_separated(f, items.iter()),
            Attribute::StrokeDashoffset(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeLinecap(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeLinejoin(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeMiterlimit(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeWidth(v) => write!(f, "=\"{}\"", v),
            Attribute::TextAnchor(v) => write!(f, "=\"{}\"", v),
            Attribute::TextDecoration(v) => write!(f, "=\"{}\"", v),
            Attribute::TextOverflow(v) => write!(f, "=\"{}\"", v),
            Attribute::TextRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::Transform(v) => write!(f, "=\"{}\"", v),
            Attribute::TransformOrigin(v) => write!(f, "=\"{}\"", v),
            Attribute::UnicodeBidi(v) => write!(f, "=\"{}\"", v),
            Attribute::VectorEffect(v) => write!(f, "=\"{}\"", v),
            Attribute::Visibility(v) => write!(f, "=\"{}\"", v),
            Attribute::Width(v) => write!(f, "=\"{}\"", v),
            Attribute::WhiteSpace(v) => write!(f, "=\"{}\"", v),
            Attribute::WordSpacing(v) => write!(f, "=\"{}\"", v),
            Attribute::WritingMode(v) => write!(f, "=\"{}\"", v),
            Attribute::X(v) => write!(f, "=\"{}\"", v),
            Attribute::Y(v) => write!(f, "=\"{}\"", v),
            Attribute::Type => todo!(),
            Attribute::TableValues => todo!(),
            Attribute::Slope => todo!(),
            Attribute::Intercept => todo!(),
            Attribute::Amplitude => todo!(),
            Attribute::Exponent => todo!(),
            Attribute::Offset => todo!(),
            Attribute::Href(v) => write!(f, "=\"{}\"", v),
            Attribute::AttributeType => todo!(),
            Attribute::AttributeName => todo!(),
            Attribute::Begin => todo!(),
            Attribute::Dur => todo!(),
            Attribute::End => todo!(),
            Attribute::Min => todo!(),
            Attribute::Max => todo!(),
            Attribute::Restart => todo!(),
            Attribute::RepeatCount => todo!(),
            Attribute::RepeatDur => todo!(),
            Attribute::Additive => todo!(),
            Attribute::Accumulate => todo!(),
            Attribute::OnAfterPrint(_) => todo!(),
            Attribute::OnBeforePrint(_) => todo!(),
            Attribute::OnBeforeUnload(_) => todo!(),
            Attribute::OnError(_) => todo!(),
            Attribute::OnHashChange(_) => todo!(),
            Attribute::OnLoad(_) => todo!(),
            Attribute::OnMessage(_) => todo!(),
            Attribute::OnOffline(_) => todo!(),
            Attribute::OnOnline(_) => todo!(),
            Attribute::OnPageHide(_) => todo!(),
            Attribute::OnPageShow(_) => todo!(),
            Attribute::OnPopState(_) => todo!(),
            Attribute::OnResize(_) => todo!(),
            Attribute::OnStorage(_) => todo!(),
            Attribute::OnUnload(_) => todo!(),
            Attribute::OnBlur(_) => todo!(),
            Attribute::OnChange(_) => todo!(),
            Attribute::OnContextMenu(_) => todo!(),
            Attribute::OnFocus(_) => todo!(),
            Attribute::OnInput(_) => todo!(),
            Attribute::OnInvalid(_) => todo!(),
            Attribute::OnReset(_) => todo!(),
            Attribute::OnSearch(_) => todo!(),
            Attribute::OnSelect(_) => todo!(),
            Attribute::OnSubmit(_) => todo!(),
            Attribute::OnKeyDown(_) => todo!(),
            Attribute::OnKeyPress(_) => todo!(),
            Attribute::OnKeyUp(_) => todo!(),
            Attribute::OnClick(_) => todo!(),
            Attribute::OnDoubleClick(_) => todo!(),
            Attribute::OnMouseDown(_) => todo!(),
            Attribute::OnMouseMove(_) => todo!(),
            Attribute::OnMouseOut(_) => todo!(),
            Attribute::OnMouseOver(_) => todo!(),
            Attribute::OnMouseUp(_) => todo!(),
            Attribute::OnWheel(_) => todo!(),
            Attribute::OnDrag(_) => todo!(),
            Attribute::OnDragEnd(_) => todo!(),
            Attribute::OnDragEnter(_) => todo!(),
            Attribute::OnDragLeave(_) => todo!(),
            Attribute::OnDragOver(_) => todo!(),
            Attribute::OnDragStart(_) => todo!(),
            Attribute::OnDrop(_) => todo!(),
            Attribute::OnScroll(_) => todo!(),
            Attribute::OnCopy(_) => todo!(),
            Attribute::OnCut(_) => todo!(),
            Attribute::OnPaste(_) => todo!(),
            Attribute::OnAbort(_) => todo!(),
            Attribute::OnCanPlay(_) => todo!(),
            Attribute::OnCanPlayThrough(_) => todo!(),
            Attribute::OnCueChange(_) => todo!(),
            Attribute::OnDurationChange(_) => todo!(),
            Attribute::OnEmptied(_) => todo!(),
            Attribute::OnEnded(_) => todo!(),
            Attribute::OnLoadedData(_) => todo!(),
            Attribute::OnLoadedMetadata(_) => todo!(),
            Attribute::OnLoadStart(_) => todo!(),
            Attribute::OnPause(_) => todo!(),
            Attribute::OnPlay(_) => todo!(),
            Attribute::OnPlaying(_) => todo!(),
            Attribute::OnProgress(_) => todo!(),
            Attribute::OnRateChange(_) => todo!(),
            Attribute::OnSeeked(_) => todo!(),
            Attribute::OnSeeking(_) => todo!(),
            Attribute::OnStalled(_) => todo!(),
            Attribute::OnSuspend(_) => todo!(),
            Attribute::OnTimeUpdate(_) => todo!(),
            Attribute::OnVolumeChange(_) => todo!(),
            Attribute::OnWaiting(_) => todo!(),
            Attribute::OnToggle(_) => todo!(),
            Attribute::KeyPoints(_) => todo!(),
            Attribute::Path(paths) => write_space_separated(f, paths.0.iter()),
            Attribute::Rotate(v) => write!(f, "=\"{}\"", v),
            Attribute::CalcMode => todo!(),
            Attribute::Values => todo!(),
            Attribute::KeyTimes => todo!(),
            Attribute::KeySplines => todo!(),
            Attribute::From => todo!(),
            Attribute::To => todo!(),
            Attribute::By => todo!(),
            Attribute::PathLength(v) => write!(f, "=\"{}\"", v),
            Attribute::X1(v) => write!(f, "=\"{}\"", v),
            Attribute::Y1(v) => write!(f, "=\"{}\"", v),
            Attribute::X2(v) => write!(f, "=\"{}\"", v),
            Attribute::Y2(v) => write!(f, "=\"{}\"", v),
            Attribute::Points(points) => write_space_separated(f, points.iter()),
            Attribute::Download(v) => write!(f, "=\"{}\"", v),
            Attribute::HrefLang(v) => write!(f, "=\"{}\"", v),
            Attribute::InterestFor(v) => write!(f, "=\"{}\"", v),
            Attribute::Ping(urls) => write_space_separated(f, urls.iter()),
            Attribute::ReferrerPolicy(v) => write!(f, "=\"{}\"", v),
            Attribute::Rel(items) => write_space_separated(f, items.iter()),
            Attribute::Target(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerHeight(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerWidth(v) => write!(f, "=\"{}\"", v),
            Attribute::Orient(v) => write!(f, "=\"{}\"", v),
            Attribute::PreserveAspectRatio(v) => write!(f, "=\"{}\"", v),
            Attribute::RefX(v) => write!(f, "=\"{}\"", v),
            Attribute::RefY(v) => write!(f, "=\"{}\"", v),
            Attribute::ViewBox(view_box) => write_space_separated(f, view_box.0.iter()),
            Attribute::MaskContentUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::MaskUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternContentUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternTransform(v) => write!(f, "=\"{}\"", v),
            Attribute::Result(v) => write!(f, "=\"{}\"", v),
            Attribute::In(v) => write!(f, "=\"{}\"", v),
            Attribute::In2(v) => write!(f, "=\"{}\"", v),
            Attribute::Mode(v) => write!(f, "=\"{}\"", v),
            Attribute::Operator(v) => write!(f, "=\"{}\"", v),
            Attribute::K1(v) => write!(f, "=\"{}\"", v),
            Attribute::K2(v) => write!(f, "=\"{}\"", v),
            Attribute::K3(v) => write!(f, "=\"{}\"", v),
            Attribute::K4(v) => write!(f, "=\"{}\"", v),
            Attribute::Order(v) => write!(f, "=\"{}\"", v),
            Attribute::KernelMatrix(items) => write_space_separated(f, items.iter()),
            Attribute::Divisor(v) => write!(f, "=\"{}\"", v),
            Attribute::Bias(v) => write!(f, "=\"{}\"", v),
            Attribute::TargetX(v) => write!(f, "=\"{}\"", v),
            Attribute::TargetY(v) => write!(f, "=\"{}\"", v),
            Attribute::EdgeMode(v) => write!(f, "=\"{}\"", v),
            Attribute::KernelUnitLength(a, None) => write!(f, "=\"{}\"", a),
            Attribute::KernelUnitLength(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::PreserveAlpha(v) => write!(f, "=\"{}\"", if *v { "1" } else { "0" }),
            Attribute::SurfaceScale(v) => write!(f, "=\"{}\"", v),
            Attribute::DiffuseConstant(v) => write!(f, "=\"{}\"", v),
            Attribute::Scale(v) => write!(f, "=\"{}\"", v),
            Attribute::XChannelSelector(v) => write!(f, "=\"{}\"", v),
            Attribute::YChannelSelector(v) => write!(f, "=\"{}\"", v),
            Attribute::Dx(v) => write!(f, "=\"{}\"", v),
            Attribute::Dy(v) => write!(f, "=\"{}\"", v),
            Attribute::StdDeviation(a, None) => write!(f, "=\"{}\"", a),
            Attribute::StdDeviation(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::CrossOrigin(v) => write!(f, "=\"{}\"", v),
            Attribute::Radius(a, None) => write!(f, "=\"{}\"", a),
            Attribute::Radius(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::SpecularConstant(v) => write!(f, "=\"{}\"", v),
            Attribute::SpecularExponent(v) => write!(f, "=\"{}\"", v),
            Attribute::BaseFrequency(a, None) => write!(f, "=\"{}\"", a),
            Attribute::BaseFrequency(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::NumOctaves(v) => write!(f, "=\"{}\"", v),
            Attribute::Seed(v) => write!(f, "=\"{}\"", v),
            Attribute::StitchTiles(v) => write!(f, "=\"{}\"", v),
            Attribute::GradientUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::GradientTransform(v) => write!(f, "=\"{}\"", v),
            Attribute::SpreadMethod(v) => write!(f, "=\"{}\"", v),
            Attribute::Fx(v) => write!(f, "=\"{}\"", v),
            Attribute::Fy(v) => write!(f, "=\"{}\"", v),
            Attribute::Fr(v) => write!(f, "=\"{}\"", v),
            Attribute::Decoding(v) => write!(f, "=\"{}\"", v),
            Attribute::FetchPriority(v) => write!(f, "=\"{}\"", v),
            Attribute::LengthAdjust(v) => write!(f, "=\"{}\"", v),
            Attribute::TextLength(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipPathUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::Method(v) => write!(f, "=\"{}\"", v),
            Attribute::Side(v) => write!(f, "=\"{}\"", v),
            Attribute::Spacing(v) => write!(f, "=\"{}\"", v),
            Attribute::StartOffset(v) => write!(f, "=\"{}\"", v),
            Attribute::FilterUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PrimitiveUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::Version(v) => write!(f, "=\"{}\"", v),
        }
    }

    pub fn write_svg(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, "{}", self.name())?;
        self.write_value(f)
    }
}
