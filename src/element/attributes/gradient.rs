use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum GradientUnits {
    UserSpaceOnUse,
    #[default]
    ObjectBoundingBox,
}

impl fmt::Display for GradientUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                GradientUnits::UserSpaceOnUse => "userSpaceOnUse",
                GradientUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                SpreadMethod::Pad => "pad",
                SpreadMethod::Reflect => "reflect",
                SpreadMethod::Repeat => "repeat",
            }
        )
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

#[derive(Clone, Debug, PartialEq, Default)]
pub struct ViewBox(pub Vec<f64>);

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
pub enum MarkerUnits {
    UserSpaceOnUse,
    #[default]
    StrokeWidth,
}

impl fmt::Display for MarkerUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MarkerUnits::UserSpaceOnUse => "userSpaceOnUse",
                MarkerUnits::StrokeWidth => "strokeWidth",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
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
            }
        )
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

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum ClipPathUnits {
    #[default]
    UserSpaceOnUse,
    ObjectBoundingBox,
}

impl fmt::Display for ClipPathUnits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ClipPathUnits::UserSpaceOnUse => "userSpaceOnUse",
                ClipPathUnits::ObjectBoundingBox => "objectBoundingBox",
            }
        )
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
pub enum CrossOrigin {
    Anonymous,
    UseCredentials,
    #[default]
    Empty,
}

impl fmt::Display for CrossOrigin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CrossOrigin::Anonymous => "anonymous",
                CrossOrigin::UseCredentials => "use-credentials",
                CrossOrigin::Empty => "",
            }
        )
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
        write!(
            f,
            "{}",
            match self {
                StitchTiles::NoStitch => "noStitch",
                StitchTiles::Stitch => "stitch",
            }
        )
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
