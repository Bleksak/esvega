use std::fmt;
use std::str::FromStr;

use crate::element::types::{AbsoluteLength, Length, LengthOrPercentage};

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
    type Err = std::convert::Infallible;

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
        write!(
            f,
            "{}",
            match self {
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
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                Operator::Over => "over",
                Operator::In => "in",
                Operator::Out => "out",
                Operator::Atop => "atop",
                Operator::Xor => "xor",
                Operator::Lighter => "lighter",
                Operator::Arithmetic => "arithmetic",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                EdgeMode::Duplicate => "duplicate",
                EdgeMode::Wrap => "wrap",
                EdgeMode::None => "none",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                ChannelSelector::R => "R",
                ChannelSelector::G => "G",
                ChannelSelector::B => "B",
                ChannelSelector::A => "A",
            }
        )
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
pub enum MaskContentUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl fmt::Display for MaskContentUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MaskContentUnits::UserSpaceOnUse => "userSpaceOnUse",
                MaskContentUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                MaskUnits::UserSpaceOnUse => "userSpaceOnUse",
                MaskUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                PatternContentUnits::UserSpaceOnUse => "userSpaceOnUse",
                PatternContentUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                PatternUnits::UserSpaceOnUse => "userSpaceOnUse",
                PatternUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
pub enum FilterUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl fmt::Display for FilterUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FilterUnits::UserSpaceOnUse => "userSpaceOnUse",
                FilterUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                PrimitiveUnits::UserSpaceOnUse => "userSpaceOnUse",
                PrimitiveUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
