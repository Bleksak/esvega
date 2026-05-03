use std::fmt;
use std::str::FromStr;

use crate::element::types::{AbsoluteLength, Length};

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
pub enum MoveTo {
    Absolute((Number, Number)),
    Relative((Number, Number)),
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
pub enum LineTo {
    XYAbsolute(Vec<Number>),
    XYRelative(Vec<Number>),
    HorizontalAbsolute(Vec<Number>),
    HorizontalRelative(Vec<Number>),
    VerticalAbsolute(Vec<Number>),
    VerticalRelative(Vec<Number>),
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
            self.x1, self.y1, self.x2, self.y2, self.x, self.y,
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
        write!(f, "{} {} {} {}", self.x2, self.y2, self.x, self.y)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CubicBezierCurve {
    Absolute(Vec<CubicBezierCurvePoint>),
    Relative(Vec<CubicBezierCurvePoint>),
    SmoothAbsolute(Vec<SmoothCubicBezierCurvePoint>),
    SmoothRelative(Vec<SmoothCubicBezierCurvePoint>),
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
        write!(f, "{} {} {} {}", self.x1, self.y1, self.x, self.y)
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
    Absolute(Vec<QuadraticBezierCurvePoint>),
    Relative(Vec<QuadraticBezierCurvePoint>),
    SmoothAbsolute(Vec<Point>),
    SmoothRelative(Vec<Point>),
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
    Absolute(Vec<EllipticalArcPoint>),
    Relative(Vec<EllipticalArcPoint>),
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
    MoveTo(MoveTo),
    LineTo(LineTo),
    CubicBezierCurve(CubicBezierCurve),
    QuadraticBezierCurve(QuadraticBezierCurve),
    EllipticalArcCurve(EllipticalArcCurve),
    ClosePath,
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
            PathType::EllipticalArcCurve(elliptical_arc_curve) => {
                write!(f, "{}", elliptical_arc_curve)
            }
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
pub struct Path(pub Vec<PathType>);

impl FromStr for Path {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
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
