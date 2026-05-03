use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Decoding {
    #[default]
    Auto,
    Synchronous,
    Asynchronous,
}

impl fmt::Display for Decoding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Decoding::Auto => "auto",
                Decoding::Synchronous => "sync",
                Decoding::Asynchronous => "async",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                FetchPriority::Auto => "auto",
                FetchPriority::High => "high",
                FetchPriority::Low => "low",
            }
        )
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
    Unknown,
    Spatial,
    Text,
}

impl fmt::Display for LengthAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LengthAdjust::Unknown => "unknown",
                LengthAdjust::Spatial => "spatial",
                LengthAdjust::Text => "text",
            }
        )
    }
}

impl FromStr for LengthAdjust {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "spatial" => Ok(Self::Spatial),
            "text" => Ok(Self::Text),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Method {
    Async,
    Auto,
    Bezier,
    discrete,
    linear,
    slope,
    spline,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Method::Async => "async",
                Method::Auto => "auto",
                Method::Bezier => "bezier",
                Method::discrete => "discrete",
                Method::linear => "linear",
                Method::slope => "slope",
                Method::spline => "spline",
            }
        )
    }
}

impl FromStr for Method {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "async" => Ok(Self::Async),
            "auto" => Ok(Self::Auto),
            "bezier" => Ok(Self::Bezier),
            "discrete" => Ok(Self::discrete),
            "linear" => Ok(Self::linear),
            "slope" => Ok(Self::slope),
            "spline" => Ok(Self::spline),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Side {
    #[default]
    Unknown,
    Front,
    Back,
}

impl fmt::Display for Side {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Side::Unknown => "unknown",
                Side::Front => "front",
                Side::Back => "back",
            }
        )
    }
}

impl FromStr for Side {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "front" => Ok(Self::Front),
            "back" => Ok(Self::Back),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Spacing {
    #[default]
    Unknown,
    Auto,
    Exact,
}

impl fmt::Display for Spacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Spacing::Unknown => "unknown",
                Spacing::Auto => "auto",
                Spacing::Exact => "exact",
            }
        )
    }
}

impl FromStr for Spacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "auto" => Ok(Self::Auto),
            "exact" => Ok(Self::Exact),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AnimationAttributeType {
    #[default]
    Xml,
    Csstype,
}

impl fmt::Display for AnimationAttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnimationAttributeType::Xml => "xml",
                AnimationAttributeType::Csstype => "css",
            }
        )
    }
}

impl FromStr for AnimationAttributeType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "xml" => Ok(Self::Xml),
            "css" => Ok(Self::Csstype),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct KeyPoint(pub f64);

impl fmt::Display for KeyPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for KeyPoint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct KeySpline {
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
}

impl fmt::Display for KeySpline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{},{},{}", self.x1, self.y1, self.x2, self.y2)
    }
}

impl FromStr for KeySpline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 4 {
            return Err(());
        }
        Ok(Self {
            x1: parts[0].parse().map_err(|_| ())?,
            y1: parts[1].parse().map_err(|_| ())?,
            x2: parts[2].parse().map_err(|_| ())?,
            y2: parts[3].parse().map_err(|_| ())?,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum FeFuncType {
    #[default]
    Linear,
    Discrete,
    Table,
    Spline,
}

impl fmt::Display for FeFuncType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FeFuncType::Linear => "linear",
                FeFuncType::Discrete => "discrete",
                FeFuncType::Table => "table",
                FeFuncType::Spline => "spline",
            }
        )
    }
}

impl FromStr for FeFuncType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linear" => Ok(Self::Linear),
            "discrete" => Ok(Self::Discrete),
            "table" => Ok(Self::Table),
            "spline" => Ok(Self::Spline),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum CalcMode {
    #[default]
    Linear,
    Discrete,
    Spline,
    Paced,
}

impl fmt::Display for CalcMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CalcMode::Linear => "linear",
                CalcMode::Discrete => "discrete",
                CalcMode::Spline => "spline",
                CalcMode::Paced => "paced",
            }
        )
    }
}

impl FromStr for CalcMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "linear" => Ok(Self::Linear),
            "discrete" => Ok(Self::Discrete),
            "spline" => Ok(Self::Spline),
            "paced" => Ok(Self::Paced),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClockValue(pub f64);

impl fmt::Display for ClockValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for ClockValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BeginEndValue {
    Clock(ClockValue),
    Event(String),
    AccessKey(String),
    Dur(ClockValue),
}

impl fmt::Display for BeginEndValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BeginEndValue::Clock(clock) => write!(f, "{}", clock),
            BeginEndValue::Event(s) => write!(f, "{}", s),
            BeginEndValue::AccessKey(s) => write!(f, "{}", s),
            BeginEndValue::Dur(dur) => write!(f, "{}", dur),
        }
    }
}

impl FromStr for BeginEndValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("event:") {
            return Ok(BeginEndValue::Event(stripped.to_string()));
        }
        if let Some(stripped) = s.strip_prefix("accessKey:") {
            return Ok(BeginEndValue::AccessKey(stripped.to_string()));
        }
        if let Ok(clock) = ClockValue::from_str(s) {
            return Ok(BeginEndValue::Clock(clock));
        }
        if let Ok(dur) = ClockValue::from_str(s) {
            return Ok(BeginEndValue::Dur(dur));
        }
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DurValue {
    Clock(ClockValue),
    Min(ClockValue),
    Max(ClockValue),
    Begin(BeginEndValue),
}

impl fmt::Display for DurValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DurValue::Clock(clock) => write!(f, "{}", clock),
            DurValue::Min(min) => write!(f, "min({})", min),
            DurValue::Max(max) => write!(f, "max({})", max),
            DurValue::Begin(begin) => write!(f, "begin({})", begin),
        }
    }
}

impl FromStr for DurValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(stripped) = s.strip_prefix("min(").and_then(|s| s.strip_suffix(')')) {
            return Ok(DurValue::Min(ClockValue::from_str(stripped)?));
        }
        if let Some(stripped) = s.strip_prefix("max(").and_then(|s| s.strip_suffix(')')) {
            return Ok(DurValue::Max(ClockValue::from_str(stripped)?));
        }
        if let Some(stripped) = s.strip_prefix("begin(").and_then(|s| s.strip_suffix(')')) {
            return Ok(DurValue::Begin(BeginEndValue::from_str(stripped)?));
        }
        if let Ok(clock) = ClockValue::from_str(s) {
            return Ok(DurValue::Clock(clock));
        }
        Err(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RepeatCount {
    Count(f64),
    Infinite,
}

impl fmt::Display for RepeatCount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepeatCount::Count(count) => write!(f, "{}", count),
            RepeatCount::Infinite => write!(f, "indefinite"),
        }
    }
}

impl FromStr for RepeatCount {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "indefinite" {
            return Ok(RepeatCount::Infinite);
        }
        Ok(RepeatCount::Count(s.parse().map_err(|_| ())?))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AnimationRestart {
    #[default]
    Unknown,
    Never,
    Once,
    Always,
}

impl fmt::Display for AnimationRestart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnimationRestart::Unknown => "unknown",
                AnimationRestart::Never => "never",
                AnimationRestart::Once => "once",
                AnimationRestart::Always => "always",
            }
        )
    }
}

impl FromStr for AnimationRestart {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "never" => Ok(Self::Never),
            "once" => Ok(Self::Once),
            "always" => Ok(Self::Always),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AnimationAdditive {
    #[default]
    Unknown,
    Replace,
    Sum,
}

impl fmt::Display for AnimationAdditive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnimationAdditive::Unknown => "unknown",
                AnimationAdditive::Replace => "replace",
                AnimationAdditive::Sum => "sum",
            }
        )
    }
}

impl FromStr for AnimationAdditive {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "replace" => Ok(Self::Replace),
            "sum" => Ok(Self::Sum),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum AnimationAccumulate {
    #[default]
    Unknown,
    None,
    Sum,
}

impl fmt::Display for AnimationAccumulate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AnimationAccumulate::Unknown => "unknown",
                AnimationAccumulate::None => "none",
                AnimationAccumulate::Sum => "sum",
            }
        )
    }
}

impl FromStr for AnimationAccumulate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "unknown" => Ok(Self::Unknown),
            "none" => Ok(Self::None),
            "sum" => Ok(Self::Sum),
            _ => Err(()),
        }
    }
}
