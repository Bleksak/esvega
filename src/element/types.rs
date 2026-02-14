// TODO: implement PartialOrd and PartialEq for all types

use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum RelativeLength {
    Cap(f64),
    Em(f64),
    Ex(f64),
    Ic(f64),
    Lh(f64),
}

impl ToString for RelativeLength {
    fn to_string(&self) -> String {
        match self {
            RelativeLength::Cap(c) => format!("{}cap", c),
            RelativeLength::Em(e) => format!("{}em", e),
            RelativeLength::Ex(e) => format!("{}ex", e),
            RelativeLength::Ic(i) => format!("{}ic", i),
            RelativeLength::Lh(l) => format!("{}lh", l),
        }
    }
}

impl FromStr for RelativeLength {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("cap") => Ok(Self::Cap(1.0)),
            _ if s.eq_ignore_ascii_case("em") => Ok(Self::Em(1.0)),
            _ if s.eq_ignore_ascii_case("ex") => Ok(Self::Ex(1.0)),
            _ if s.eq_ignore_ascii_case("ic") => Ok(Self::Ic(1.0)),
            _ if s.eq_ignore_ascii_case("lh") => Ok(Self::Lh(1.0)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelativeLengthBasedOnRoot {
    Rcap(f64),
    Rch(f64),
    Rem(f64),
    Rex(f64),
    Ric(f64),
    Rlh(f64),
}

impl ToString for RelativeLengthBasedOnRoot {
    fn to_string(&self) -> String {
        match self {
            RelativeLengthBasedOnRoot::Rcap(c) => format!("{}rcap", c),
            RelativeLengthBasedOnRoot::Rch(c) => format!("{}rch", c),
            RelativeLengthBasedOnRoot::Rem(r) => format!("{}rem", r),
            RelativeLengthBasedOnRoot::Rex(r) => format!("{}rex", r),
            RelativeLengthBasedOnRoot::Ric(r) => format!("{}ric", r),
            RelativeLengthBasedOnRoot::Rlh(r) => format!("{}rlh", r),
        }
    }
}

impl FromStr for RelativeLengthBasedOnRoot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("rcap") => Ok(Self::Rcap(1.0)),
            _ if s.eq_ignore_ascii_case("rch") => Ok(Self::Rch(1.0)),
            _ if s.eq_ignore_ascii_case("rem") => Ok(Self::Rem(1.0)),
            _ if s.eq_ignore_ascii_case("rex") => Ok(Self::Rex(1.0)),
            _ if s.eq_ignore_ascii_case("ric") => Ok(Self::Ric(1.0)),
            _ if s.eq_ignore_ascii_case("rlh") => Ok(Self::Rlh(1.0)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AbsoluteLength {
    Px(f64), // for print 1px = 1/96th of an inch
    Cm(f64), // 1cm = 96px / 2.54
    Q(f64),  // 1q = 1cm / 40
    In(f64), // 1in = 96px
    Mm(f64), // 1mm = 1cm / 10
    Pt(f64), // 1pt = 1/72th of an inch
    Pc(f64), // 1pc = 1/6th of an inch
}

impl ToString for AbsoluteLength {
    fn to_string(&self) -> String {
        match self {
            AbsoluteLength::Px(p) => format!("{}px", p),
            AbsoluteLength::Cm(c) => format!("{}cm", c),
            AbsoluteLength::Q(q) => format!("{}q", q),
            AbsoluteLength::In(i) => format!("{}in", i),
            AbsoluteLength::Mm(m) => format!("{}mm", m),
            AbsoluteLength::Pt(p) => format!("{}pt", p),
            AbsoluteLength::Pc(p) => format!("{}pc", p),
        }
    }
}

impl FromStr for AbsoluteLength {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("px") => Ok(Self::Px(1.0)),
            _ if s.eq_ignore_ascii_case("cm") => Ok(Self::Cm(1.0)),
            _ if s.eq_ignore_ascii_case("q") => Ok(Self::Q(1.0)),
            _ if s.eq_ignore_ascii_case("in") => Ok(Self::In(1.0)),
            _ if s.eq_ignore_ascii_case("mm") => Ok(Self::Mm(1.0)),
            _ if s.eq_ignore_ascii_case("pt") => Ok(Self::Pt(1.0)),
            _ if s.eq_ignore_ascii_case("pc") => Ok(Self::Pc(1.0)),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelativeLengths {
    RelativeLength(RelativeLength),
    RelativeLengthBasedOnRoot(RelativeLengthBasedOnRoot),
}

impl ToString for RelativeLengths {
    fn to_string(&self) -> String {
        match self {
            RelativeLengths::RelativeLength(r) => r.to_string(),
            RelativeLengths::RelativeLengthBasedOnRoot(r) => r.to_string(),
        }
    }
}

impl FromStr for RelativeLengths {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(relative) = RelativeLength::from_str(s) {
            return Ok(RelativeLengths::RelativeLength(relative));
        }

        if let Ok(relative) = RelativeLengthBasedOnRoot::from_str(s) {
            return Ok(RelativeLengths::RelativeLengthBasedOnRoot(relative));
        }

        return Err(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Length {
    Absolute(AbsoluteLength),
    Relative(RelativeLengths),
}

impl ToString for Length {
    fn to_string(&self) -> String {
        match self {
            Length::Absolute(a) => a.to_string(),
            Length::Relative(r) => r.to_string(),
        }
    }
}

impl FromStr for Length {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(absolute) = AbsoluteLength::from_str(s) {
            return Ok(Length::Absolute(absolute));
        }

        if let Ok(relative) = RelativeLengths::from_str(s) {
            return Ok(Length::Relative(relative));
        }

        return Err(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Percentage(pub f64);

impl ToString for Percentage {
    fn to_string(&self) -> String {
        format!("{}%", self.0)
    }
}

impl FromStr for Percentage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.ends_with('%') {
            let (value, rhs) = s.split_once('%').ok_or(())?;

            if rhs.is_empty() {
                return Ok(Percentage(value.parse::<f64>().map_err(|_| ())?));
            }
        }

        Err(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LengthOrPercentage {
    Length(Length),
    Percentage(Percentage),
}

impl ToString for LengthOrPercentage {
    fn to_string(&self) -> String {
        match self {
            LengthOrPercentage::Length(l) => l.to_string(),
            LengthOrPercentage::Percentage(p) => p.to_string(),
        }
    }
}

impl Default for LengthOrPercentage {
    fn default() -> Self {
        LengthOrPercentage::Length(Length::Absolute(AbsoluteLength::Px(0.0)))
    }
}

impl FromStr for LengthOrPercentage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

impl TryFrom<&str> for RelativeLength {
    type Error = ();

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        if let Some(num_str) = input.strip_suffix("cap") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLength::Cap)
        } else if let Some(num_str) = input.strip_suffix("em") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLength::Em)
        } else if let Some(num_str) = input.strip_suffix("ex") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLength::Ex)
        } else if let Some(num_str) = input.strip_suffix("ic") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLength::Ic)
        } else if let Some(num_str) = input.strip_suffix("lh") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLength::Lh)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for RelativeLengthBasedOnRoot {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(num_str) = value.strip_suffix("rcap") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Rcap)
        } else if let Some(num_str) = value.strip_suffix("rch") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Rch)
        } else if let Some(num_str) = value.strip_suffix("rem") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Rem)
        } else if let Some(num_str) = value.strip_suffix("rex") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Rex)
        } else if let Some(num_str) = value.strip_suffix("ric") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Ric)
        } else if let Some(num_str) = value.strip_suffix("rlh") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(RelativeLengthBasedOnRoot::Rlh)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for AbsoluteLength {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(num_str) = value.strip_suffix("px") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Px)
        } else if let Some(num_str) = value.strip_suffix("cm") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Cm)
        } else if let Some(num_str) = value.strip_suffix("q") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Q)
        } else if let Some(num_str) = value.strip_suffix("in") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::In)
        } else if let Some(num_str) = value.strip_suffix("mm") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Mm)
        } else if let Some(num_str) = value.strip_suffix("pt") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Pt)
        } else if let Some(num_str) = value.strip_suffix("pc") {
            num_str
                .trim()
                .parse::<f64>()
                .map_err(|_| ())
                .map(AbsoluteLength::Pc)
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for RelativeLengths {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(relative_length) = RelativeLength::try_from(value) {
            Ok(RelativeLengths::RelativeLength(relative_length))
        } else if let Ok(relative_length) = RelativeLengthBasedOnRoot::try_from(value) {
            Ok(RelativeLengths::RelativeLengthBasedOnRoot(relative_length))
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for Length {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(absolute_length) = AbsoluteLength::try_from(value) {
            Ok(Length::Absolute(absolute_length))
        } else if let Ok(relative_length) = RelativeLengths::try_from(value) {
            Ok(Length::Relative(relative_length))
        } else {
            Err(())
        }
    }
}

impl TryFrom<&str> for LengthOrPercentage {
    type Error = ();

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Ok(length) = Length::try_from(s) {
            return Ok(LengthOrPercentage::Length(length));
        }

        if let Ok(percentage) = s.parse() {
            return Ok(LengthOrPercentage::Percentage(percentage));
        }

        if s.chars().all(|c| c.is_ascii_digit()) {
            return Ok(LengthOrPercentage::Length(Length::Absolute(
                AbsoluteLength::Px(s.parse::<f64>().map_err(|_| ())?),
            )));
        }

        return Err(());
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColorLiteral {
    Maroon,
    DarkRed,
    Brown,
    FireBrick,
    Crimson,
    Red,
    IndianRed,
    SaddleBrown,
    Sienna,
    RosyBrown,
    Chocolate,
    Peru,
    DarkGoldenRod,
    DarkSalmon,
    LightCoral,
    GoldenRod,
    Tan,
    DarkKhaki,
    BurlyWood,
    PaleGoldenRod,
    SandyBrown,
    Salmon,
    OrangeRed,
    Tomato,
    Coral,
    DarkOrange,
    LightSalmon,
    Orange,
    Gold,
    Yellow,
    Khaki,
    Wheat,
    Beige,
    AntiqueWhite,
    Linen,
    LightGoldenRodYellow,
    OldLace,
    PeachPuff,
    NavajoWhite,
    Moccasin,
    Bisque,
    BlanchedAlmond,
    PapayaWhip,
    LemonChiffon,
    LightYellow,
    Cornsilk,
    MistyRose,
    FloralWhite,
    SeaShell,
    Snow,
    Ivory,
    White,
    Black,
    DimGray,
    DarkGray,
    Gray,
    Silver,
    LightGray,
    Gainsboro,
    WhiteSmoke,
    DarkSlateGray,
    SlateGray,
    LightSlateGray,
    DarkGreen,
    Green,
    ForestGreen,
    DarkSeaGreen,
    SeaGreen,
    MediumSeaGreen,
    LightSeaGreen,
    LimeGreen,
    Lime,
    Chartreuse,
    LawnGreen,
    SpringGreen,
    MediumSpringGreen,
    DarkOliveGreen,
    OliveDrab,
    Olive,
    YellowGreen,
    GreenYellow,
    PaleGreen,
    LightGreen,
    HoneyDew,
    Teal,
    DarkCyan,
    CadetBlue,
    DarkTurquoise,
    Turquoise,
    MediumTurquoise,
    MediumAquaMarine,
    Aquamarine,
    MidnightBlue,
    Navy,
    DarkBlue,
    MediumBlue,
    RoyalBlue,
    CornflowerBlue,
    SteelBlue,
    Blue,
    DodgerBlue,
    PaleTurquoise,
    LightSteelBlue,
    PowderBlue,
    DeepSkyBlue,
    SkyBlue,
    LightSkyBlue,
    LightBlue,
    Aqua,
    Cyan,
    LightCyan,
    Azure,
    AliceBlue,
    MintCream,
    GhostWhite,
}

impl ColorLiteral {
    pub fn as_str(&self) -> &str {
        match self {
            ColorLiteral::Maroon => "maroon",
            ColorLiteral::DarkRed => "darkred",
            ColorLiteral::Brown => "brown",
            ColorLiteral::FireBrick => "firebrick",
            ColorLiteral::Crimson => "crimson",
            ColorLiteral::Red => "red",
            ColorLiteral::IndianRed => "indianred",
            ColorLiteral::SaddleBrown => "saddlebrown",
            ColorLiteral::Sienna => "sienna",
            ColorLiteral::RosyBrown => "rosybrown",
            ColorLiteral::Chocolate => "chocolate",
            ColorLiteral::Peru => "peru",
            ColorLiteral::DarkGoldenRod => "darkgoldenrod",
            ColorLiteral::DarkSalmon => "darksalmon",
            ColorLiteral::LightCoral => "lightcoral",
            ColorLiteral::GoldenRod => "goldenrod",
            ColorLiteral::Tan => "tan",
            ColorLiteral::DarkKhaki => "darkkhaki",
            ColorLiteral::BurlyWood => "burlywood",
            ColorLiteral::PaleGoldenRod => "palegoldenrod",
            ColorLiteral::SandyBrown => "sandybrown",
            ColorLiteral::Salmon => "salmon",
            ColorLiteral::OrangeRed => "orangered",
            ColorLiteral::Tomato => "tomato",
            ColorLiteral::Coral => "coral",
            ColorLiteral::DarkOrange => "darkorange",
            ColorLiteral::LightSalmon => "lightsalmon",
            ColorLiteral::Orange => "orange",
            ColorLiteral::Gold => "gold",
            ColorLiteral::Yellow => "yellow",
            ColorLiteral::Khaki => "khaki",
            ColorLiteral::Wheat => "wheat",
            ColorLiteral::Beige => "beige",
            ColorLiteral::AntiqueWhite => "antiquewhite",
            ColorLiteral::Linen => "linen",
            ColorLiteral::LightGoldenRodYellow => "lightgoldenrodyellow",
            ColorLiteral::OldLace => "oldlace",
            ColorLiteral::PeachPuff => "peachpuff",
            ColorLiteral::NavajoWhite => "navajowhite",
            ColorLiteral::Moccasin => "moccasin",
            ColorLiteral::Bisque => "bisque",
            ColorLiteral::BlanchedAlmond => "blanchedalmond",
            ColorLiteral::PapayaWhip => "papayawhip",
            ColorLiteral::LemonChiffon => "lemonchiffon",
            ColorLiteral::LightYellow => "lightyellow",
            ColorLiteral::Cornsilk => "cornsilk",
            ColorLiteral::MistyRose => "mistyrose",
            ColorLiteral::FloralWhite => "floralwhite",
            ColorLiteral::SeaShell => "seashell",
            ColorLiteral::Snow => "snow",
            ColorLiteral::Ivory => "ivory",
            ColorLiteral::White => "white",
            ColorLiteral::Black => "black",
            ColorLiteral::DimGray => "dimgray",
            ColorLiteral::DarkGray => "darkgray",
            ColorLiteral::Gray => "gray",
            ColorLiteral::Silver => "silver",
            ColorLiteral::LightGray => "lightgray",
            ColorLiteral::Gainsboro => "gainsboro",
            ColorLiteral::WhiteSmoke => "whitesmoke",
            ColorLiteral::DarkSlateGray => "darkslategray",
            ColorLiteral::SlateGray => "slategray",
            ColorLiteral::LightSlateGray => "lightslategray",
            ColorLiteral::DarkGreen => "darkgreen",
            ColorLiteral::Green => "green",
            ColorLiteral::ForestGreen => "forestgreen",
            ColorLiteral::DarkSeaGreen => "darkseagreen",
            ColorLiteral::SeaGreen => "seagreen",
            ColorLiteral::MediumSeaGreen => "mediumseagreen",
            ColorLiteral::LightSeaGreen => "lightseagreen",
            ColorLiteral::LimeGreen => "limegreen",
            ColorLiteral::Lime => "lime",
            ColorLiteral::Chartreuse => "chartreuse",
            ColorLiteral::LawnGreen => "lawngreen",
            ColorLiteral::SpringGreen => "springgreen",
            ColorLiteral::MediumSpringGreen => "mediumspringgreen",
            ColorLiteral::DarkOliveGreen => "darkolivegreen",
            ColorLiteral::OliveDrab => "olivedrab",
            ColorLiteral::Olive => "olive",
            ColorLiteral::YellowGreen => "yellowgreen",
            ColorLiteral::GreenYellow => "greenyellow",
            ColorLiteral::PaleGreen => "palegreen",
            ColorLiteral::LightGreen => "lightgreen",
            ColorLiteral::HoneyDew => "honeydew",
            ColorLiteral::Teal => "teal",
            ColorLiteral::DarkCyan => "darkcyan",
            ColorLiteral::CadetBlue => "cadetblue",
            ColorLiteral::DarkTurquoise => "darkturquoise",
            ColorLiteral::Turquoise => "turquoise",
            ColorLiteral::MediumTurquoise => "mediumturquoise",
            ColorLiteral::MediumAquaMarine => "mediumaquamarine",
            ColorLiteral::Aquamarine => "aquamarine",
            ColorLiteral::MidnightBlue => "midnightblue",
            ColorLiteral::Navy => "navy",
            ColorLiteral::DarkBlue => "darkblue",
            ColorLiteral::MediumBlue => "mediumblue",
            ColorLiteral::RoyalBlue => "royalblue",
            ColorLiteral::CornflowerBlue => "cornflowerblue",
            ColorLiteral::SteelBlue => "steelblue",
            ColorLiteral::Blue => "blue",
            ColorLiteral::DodgerBlue => "dodgerblue",
            ColorLiteral::PaleTurquoise => "paleturquoise",
            ColorLiteral::LightSteelBlue => "lightsteelblue",
            ColorLiteral::PowderBlue => "powderblue",
            ColorLiteral::DeepSkyBlue => "deepskyblue",
            ColorLiteral::SkyBlue => "skyblue",
            ColorLiteral::LightSkyBlue => "lightskyblue",
            ColorLiteral::LightBlue => "lightblue",
            ColorLiteral::Aqua => "aqua",
            ColorLiteral::Cyan => "cyan",
            ColorLiteral::LightCyan => "lightcyan",
            ColorLiteral::Azure => "azure",
            ColorLiteral::AliceBlue => "aliceblue",
            ColorLiteral::MintCream => "mintcream",
            ColorLiteral::GhostWhite => "ghostwhite",
        }
    }
}

impl FromStr for ColorLiteral {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("maroon") => Ok(Self::Maroon),
            _ if s.eq_ignore_ascii_case("darkred") => Ok(Self::DarkRed),
            _ if s.eq_ignore_ascii_case("brown") => Ok(Self::Brown),
            _ if s.eq_ignore_ascii_case("firebrick") => Ok(Self::FireBrick),
            _ if s.eq_ignore_ascii_case("crimson") => Ok(Self::Crimson),
            _ if s.eq_ignore_ascii_case("red") => Ok(Self::Red),
            _ if s.eq_ignore_ascii_case("indianred") => Ok(Self::IndianRed),
            _ if s.eq_ignore_ascii_case("salmon") => Ok(Self::Salmon),
            _ if s.eq_ignore_ascii_case("orangered") => Ok(Self::OrangeRed),
            _ if s.eq_ignore_ascii_case("tomato") => Ok(Self::Tomato),
            _ if s.eq_ignore_ascii_case("coral") => Ok(Self::Coral),
            _ if s.eq_ignore_ascii_case("darkorange") => Ok(Self::DarkOrange),
            _ if s.eq_ignore_ascii_case("lightsalmon") => Ok(Self::LightSalmon),
            _ if s.eq_ignore_ascii_case("orange") => Ok(Self::Orange),
            _ if s.eq_ignore_ascii_case("gold") => Ok(Self::Gold),
            _ if s.eq_ignore_ascii_case("yellow") => Ok(Self::Yellow),
            _ if s.eq_ignore_ascii_case("khaki") => Ok(Self::Khaki),
            _ if s.eq_ignore_ascii_case("wheat") => Ok(Self::Wheat),
            _ if s.eq_ignore_ascii_case("beige") => Ok(Self::Beige),
            _ if s.eq_ignore_ascii_case("antiquewhite") => Ok(Self::AntiqueWhite),
            _ if s.eq_ignore_ascii_case("linen") => Ok(Self::Linen),
            _ if s.eq_ignore_ascii_case("lightgoldenrodyellow") => Ok(Self::LightGoldenRodYellow),
            _ if s.eq_ignore_ascii_case("oldlace") => Ok(Self::OldLace),
            _ if s.eq_ignore_ascii_case("peachpuff") => Ok(Self::PeachPuff),
            _ if s.eq_ignore_ascii_case("navajowhite") => Ok(Self::NavajoWhite),
            _ if s.eq_ignore_ascii_case("moccasin") => Ok(Self::Moccasin),
            _ if s.eq_ignore_ascii_case("bisque") => Ok(Self::Bisque),
            _ if s.eq_ignore_ascii_case("blanchedalmond") => Ok(Self::BlanchedAlmond),
            _ if s.eq_ignore_ascii_case("papayawhip") => Ok(Self::PapayaWhip),
            _ if s.eq_ignore_ascii_case("lemonchiffon") => Ok(Self::LemonChiffon),
            _ if s.eq_ignore_ascii_case("lightyellow") => Ok(Self::LightYellow),
            _ if s.eq_ignore_ascii_case("cornsilk") => Ok(Self::Cornsilk),
            _ if s.eq_ignore_ascii_case("mistyrose") => Ok(Self::MistyRose),
            _ if s.eq_ignore_ascii_case("floralwhite") => Ok(Self::FloralWhite),
            _ if s.eq_ignore_ascii_case("seashell") => Ok(Self::SeaShell),
            _ if s.eq_ignore_ascii_case("snow") => Ok(Self::Snow),
            _ if s.eq_ignore_ascii_case("ivory") => Ok(Self::Ivory),
            _ if s.eq_ignore_ascii_case("white") => Ok(Self::White),
            _ if s.eq_ignore_ascii_case("black") => Ok(Self::Black),
            _ if s.eq_ignore_ascii_case("dimgray") => Ok(Self::DimGray),
            _ if s.eq_ignore_ascii_case("darkgray") => Ok(Self::DarkGray),
            _ if s.eq_ignore_ascii_case("gray") => Ok(Self::Gray),
            _ if s.eq_ignore_ascii_case("silver") => Ok(Self::Silver),
            _ if s.eq_ignore_ascii_case("lightgray") => Ok(Self::LightGray),
            _ if s.eq_ignore_ascii_case("gainsboro") => Ok(Self::Gainsboro),
            _ if s.eq_ignore_ascii_case("whitesmoke") => Ok(Self::WhiteSmoke),
            _ if s.eq_ignore_ascii_case("darkslategray") => Ok(Self::DarkSlateGray),
            _ if s.eq_ignore_ascii_case("slategray") => Ok(Self::SlateGray),
            _ if s.eq_ignore_ascii_case("lightslategray") => Ok(Self::LightSlateGray),
            _ if s.eq_ignore_ascii_case("darkgreen") => Ok(Self::DarkGreen),
            _ if s.eq_ignore_ascii_case("green") => Ok(Self::Green),
            _ if s.eq_ignore_ascii_case("forestgreen") => Ok(Self::ForestGreen),
            _ if s.eq_ignore_ascii_case("darkseagreen") => Ok(Self::DarkSeaGreen),
            _ if s.eq_ignore_ascii_case("seagreen") => Ok(Self::SeaGreen),
            _ if s.eq_ignore_ascii_case("mediumseagreen") => Ok(Self::MediumSeaGreen),
            _ if s.eq_ignore_ascii_case("lightseagreen") => Ok(Self::LightSeaGreen),
            _ if s.eq_ignore_ascii_case("limegreen") => Ok(Self::LimeGreen),
            _ if s.eq_ignore_ascii_case("lime") => Ok(Self::Lime),
            _ if s.eq_ignore_ascii_case("chartreuse") => Ok(Self::Chartreuse),
            _ if s.eq_ignore_ascii_case("lawngreen") => Ok(Self::LawnGreen),
            _ if s.eq_ignore_ascii_case("springgreen") => Ok(Self::SpringGreen),
            _ if s.eq_ignore_ascii_case("mediumspringgreen") => Ok(Self::MediumSpringGreen),
            _ if s.eq_ignore_ascii_case("darkolivegreen") => Ok(Self::DarkOliveGreen),
            _ if s.eq_ignore_ascii_case("olivedrab") => Ok(Self::OliveDrab),
            _ if s.eq_ignore_ascii_case("olive") => Ok(Self::Olive),
            _ if s.eq_ignore_ascii_case("yellowgreen") => Ok(Self::YellowGreen),
            _ if s.eq_ignore_ascii_case("greenyellow") => Ok(Self::GreenYellow),
            _ if s.eq_ignore_ascii_case("palegreen") => Ok(Self::PaleGreen),
            _ if s.eq_ignore_ascii_case("lightgreen") => Ok(Self::LightGreen),
            _ if s.eq_ignore_ascii_case("honeydew") => Ok(Self::HoneyDew),
            _ if s.eq_ignore_ascii_case("teal") => Ok(Self::Teal),
            _ if s.eq_ignore_ascii_case("darkcyan") => Ok(Self::DarkCyan),
            _ if s.eq_ignore_ascii_case("cadetblue") => Ok(Self::CadetBlue),
            _ if s.eq_ignore_ascii_case("darkturquoise") => Ok(Self::DarkTurquoise),
            _ if s.eq_ignore_ascii_case("turquoise") => Ok(Self::Turquoise),
            _ if s.eq_ignore_ascii_case("mediumturquoise") => Ok(Self::MediumTurquoise),
            _ if s.eq_ignore_ascii_case("mediumaquamarine") => Ok(Self::MediumAquaMarine),
            _ if s.eq_ignore_ascii_case("aquamarine") => Ok(Self::Aquamarine),
            _ if s.eq_ignore_ascii_case("midnightblue") => Ok(Self::MidnightBlue),
            _ if s.eq_ignore_ascii_case("navy") => Ok(Self::Navy),
            _ if s.eq_ignore_ascii_case("darkblue") => Ok(Self::DarkBlue),
            _ if s.eq_ignore_ascii_case("mediumblue") => Ok(Self::MediumBlue),
            _ if s.eq_ignore_ascii_case("royalblue") => Ok(Self::RoyalBlue),
            _ if s.eq_ignore_ascii_case("cornflowerblue") => Ok(Self::CornflowerBlue),
            _ if s.eq_ignore_ascii_case("steelblue") => Ok(Self::SteelBlue),
            _ if s.eq_ignore_ascii_case("blue") => Ok(Self::Blue),
            _ if s.eq_ignore_ascii_case("dodgerblue") => Ok(Self::DodgerBlue),
            _ if s.eq_ignore_ascii_case("paleturquoise") => Ok(Self::PaleTurquoise),
            _ if s.eq_ignore_ascii_case("lightsteelblue") => Ok(Self::LightSteelBlue),
            _ if s.eq_ignore_ascii_case("powderblue") => Ok(Self::PowderBlue),
            _ if s.eq_ignore_ascii_case("deepskyblue") => Ok(Self::DeepSkyBlue),
            _ if s.eq_ignore_ascii_case("skyblue") => Ok(Self::SkyBlue),
            _ if s.eq_ignore_ascii_case("lightskyblue") => Ok(Self::LightSkyBlue),
            _ if s.eq_ignore_ascii_case("lightblue") => Ok(Self::LightBlue),
            _ if s.eq_ignore_ascii_case("aqua") => Ok(Self::Aqua),
            _ if s.eq_ignore_ascii_case("cyan") => Ok(Self::Cyan),
            _ if s.eq_ignore_ascii_case("lightcyan") => Ok(Self::LightCyan),
            _ if s.eq_ignore_ascii_case("azure") => Ok(Self::Azure),
            _ if s.eq_ignore_ascii_case("aliceblue") => Ok(Self::AliceBlue),
            _ if s.eq_ignore_ascii_case("mintcream") => Ok(Self::MintCream),
            _ if s.eq_ignore_ascii_case("ghostwhite") => Ok(Self::GhostWhite),
            _ => Err(()),
        }
    }
}

impl ColorLiteral {
    pub fn to_rgba(&self) -> Color {
        match self {
            ColorLiteral::Maroon => Color::Rgba(128, 0, 0, 255),
            ColorLiteral::DarkRed => Color::Rgba(139, 0, 0, 255),
            ColorLiteral::Brown => Color::Rgba(165, 42, 42, 255),
            ColorLiteral::FireBrick => Color::Rgba(178, 34, 34, 255),
            ColorLiteral::Crimson => Color::Rgba(220, 20, 60, 255),
            ColorLiteral::Red => Color::Rgba(255, 0, 0, 255),
            ColorLiteral::IndianRed => Color::Rgba(205, 92, 92, 255),
            ColorLiteral::SaddleBrown => Color::Rgba(139, 69, 19, 255),
            ColorLiteral::Sienna => Color::Rgba(160, 82, 45, 255),
            ColorLiteral::RosyBrown => Color::Rgba(188, 143, 143, 255),
            ColorLiteral::Chocolate => Color::Rgba(210, 105, 30, 255),
            ColorLiteral::Peru => Color::Rgba(205, 133, 63, 255),
            ColorLiteral::DarkGoldenRod => Color::Rgba(184, 134, 11, 255),
            ColorLiteral::DarkSalmon => Color::Rgba(233, 150, 122, 255),
            ColorLiteral::LightCoral => Color::Rgba(240, 128, 128, 255),
            ColorLiteral::GoldenRod => Color::Rgba(218, 165, 32, 255),
            ColorLiteral::Tan => Color::Rgba(210, 180, 140, 255),
            ColorLiteral::DarkKhaki => Color::Rgba(189, 183, 107, 255),
            ColorLiteral::BurlyWood => Color::Rgba(222, 184, 135, 255),
            ColorLiteral::PaleGoldenRod => Color::Rgba(238, 232, 170, 255),
            ColorLiteral::SandyBrown => Color::Rgba(244, 164, 96, 255),
            ColorLiteral::Salmon => Color::Rgba(250, 128, 114, 255),
            ColorLiteral::OrangeRed => Color::Rgba(255, 69, 0, 255),
            ColorLiteral::Tomato => Color::Rgba(255, 99, 71, 255),
            ColorLiteral::Coral => Color::Rgba(255, 127, 80, 255),
            ColorLiteral::DarkOrange => Color::Rgba(255, 140, 0, 255),
            ColorLiteral::LightSalmon => Color::Rgba(255, 160, 122, 255),
            ColorLiteral::Orange => Color::Rgba(255, 165, 0, 255),
            ColorLiteral::Gold => Color::Rgba(255, 215, 0, 255),
            ColorLiteral::Yellow => Color::Rgba(255, 255, 0, 255),
            ColorLiteral::Khaki => Color::Rgba(240, 230, 140, 255),
            ColorLiteral::Wheat => Color::Rgba(245, 222, 179, 255),
            ColorLiteral::Beige => Color::Rgba(245, 245, 220, 255),
            ColorLiteral::AntiqueWhite => Color::Rgba(250, 235, 215, 255),
            ColorLiteral::Linen => Color::Rgba(250, 240, 230, 255),
            ColorLiteral::LightGoldenRodYellow => Color::Rgba(250, 250, 210, 255),
            ColorLiteral::OldLace => Color::Rgba(253, 245, 230, 255),
            ColorLiteral::PeachPuff => Color::Rgba(255, 218, 185, 255),
            ColorLiteral::NavajoWhite => Color::Rgba(255, 222, 173, 255),
            ColorLiteral::Moccasin => Color::Rgba(255, 228, 181, 255),
            ColorLiteral::Bisque => Color::Rgba(255, 228, 196, 255),
            ColorLiteral::BlanchedAlmond => Color::Rgba(255, 235, 205, 255),
            ColorLiteral::PapayaWhip => Color::Rgba(255, 239, 213, 255),
            ColorLiteral::LemonChiffon => Color::Rgba(255, 250, 205, 255),
            ColorLiteral::LightYellow => Color::Rgba(255, 255, 224, 255),
            ColorLiteral::Cornsilk => Color::Rgba(255, 248, 220, 255),
            ColorLiteral::MistyRose => Color::Rgba(255, 228, 225, 255),
            ColorLiteral::FloralWhite => Color::Rgba(255, 250, 240, 255),
            ColorLiteral::SeaShell => Color::Rgba(255, 245, 238, 255),
            ColorLiteral::Snow => Color::Rgba(255, 250, 250, 255),
            ColorLiteral::Ivory => Color::Rgba(255, 255, 240, 255),
            ColorLiteral::White => Color::Rgba(255, 255, 255, 255),
            ColorLiteral::Black => Color::Rgba(0, 0, 0, 255),
            ColorLiteral::DimGray => Color::Rgba(105, 105, 105, 255),
            ColorLiteral::DarkGray => Color::Rgba(169, 169, 169, 255),
            ColorLiteral::Gray => Color::Rgba(128, 128, 128, 255),
            ColorLiteral::Silver => Color::Rgba(192, 192, 192, 255),
            ColorLiteral::LightGray => Color::Rgba(211, 211, 211, 255),
            ColorLiteral::Gainsboro => Color::Rgba(220, 220, 220, 255),
            ColorLiteral::WhiteSmoke => Color::Rgba(245, 245, 245, 255),
            ColorLiteral::DarkSlateGray => Color::Rgba(47, 79, 79, 255),
            ColorLiteral::SlateGray => Color::Rgba(112, 128, 144, 255),
            ColorLiteral::LightSlateGray => Color::Rgba(119, 136, 153, 255),
            ColorLiteral::DarkGreen => Color::Rgba(0, 100, 0, 255),
            ColorLiteral::Green => Color::Rgba(0, 128, 0, 255),
            ColorLiteral::ForestGreen => Color::Rgba(34, 139, 34, 255),
            ColorLiteral::DarkSeaGreen => Color::Rgba(143, 188, 143, 255),
            ColorLiteral::SeaGreen => Color::Rgba(46, 139, 87, 255),
            ColorLiteral::MediumSeaGreen => Color::Rgba(60, 179, 113, 255),
            ColorLiteral::LightSeaGreen => Color::Rgba(32, 178, 170, 255),
            ColorLiteral::LimeGreen => Color::Rgba(50, 205, 50, 255),
            ColorLiteral::Lime => Color::Rgba(0, 255, 0, 255),
            ColorLiteral::Chartreuse => Color::Rgba(127, 255, 0, 255),
            ColorLiteral::LawnGreen => Color::Rgba(124, 252, 0, 255),
            ColorLiteral::SpringGreen => Color::Rgba(0, 255, 127, 255),
            ColorLiteral::MediumSpringGreen => Color::Rgba(0, 250, 154, 255),
            ColorLiteral::DarkOliveGreen => Color::Rgba(85, 107, 47, 255),
            ColorLiteral::OliveDrab => Color::Rgba(107, 142, 35, 255),
            ColorLiteral::Olive => Color::Rgba(128, 128, 0, 255),
            ColorLiteral::YellowGreen => Color::Rgba(154, 205, 50, 255),
            ColorLiteral::GreenYellow => Color::Rgba(173, 255, 47, 255),
            ColorLiteral::PaleGreen => Color::Rgba(152, 251, 152, 255),
            ColorLiteral::LightGreen => Color::Rgba(144, 238, 144, 255),
            ColorLiteral::HoneyDew => Color::Rgba(240, 255, 240, 255),
            ColorLiteral::Teal => Color::Rgba(0, 128, 128, 255),
            ColorLiteral::DarkCyan => Color::Rgba(0, 139, 139, 255),
            ColorLiteral::CadetBlue => Color::Rgba(95, 158, 160, 255),
            ColorLiteral::DarkTurquoise => Color::Rgba(0, 206, 209, 255),
            ColorLiteral::Turquoise => Color::Rgba(64, 224, 208, 255),
            ColorLiteral::MediumTurquoise => Color::Rgba(72, 209, 204, 255),
            ColorLiteral::MediumAquaMarine => Color::Rgba(102, 205, 170, 255),
            ColorLiteral::Aquamarine => Color::Rgba(127, 255, 212, 255),
            ColorLiteral::MidnightBlue => Color::Rgba(25, 25, 112, 255),
            ColorLiteral::Navy => Color::Rgba(0, 0, 128, 255),
            ColorLiteral::DarkBlue => Color::Rgba(0, 0, 139, 255),
            ColorLiteral::MediumBlue => Color::Rgba(0, 0, 205, 255),
            ColorLiteral::RoyalBlue => Color::Rgba(65, 105, 225, 255),
            ColorLiteral::CornflowerBlue => Color::Rgba(100, 149, 237, 255),
            ColorLiteral::SteelBlue => Color::Rgba(70, 130, 180, 255),
            ColorLiteral::Blue => Color::Rgba(0, 0, 255, 255),
            ColorLiteral::DodgerBlue => Color::Rgba(30, 144, 255, 255),
            ColorLiteral::PaleTurquoise => Color::Rgba(175, 238, 238, 255),
            ColorLiteral::LightSteelBlue => Color::Rgba(176, 196, 222, 255),
            ColorLiteral::PowderBlue => Color::Rgba(176, 224, 230, 255),
            ColorLiteral::DeepSkyBlue => Color::Rgba(0, 191, 255, 255),
            ColorLiteral::SkyBlue => Color::Rgba(135, 206, 235, 255),
            ColorLiteral::LightSkyBlue => Color::Rgba(135, 206, 250, 255),
            ColorLiteral::LightBlue => Color::Rgba(173, 216, 230, 255),
            ColorLiteral::Aqua => Color::Rgba(0, 255, 255, 255),
            ColorLiteral::Cyan => Color::Rgba(0, 255, 255, 255),
            ColorLiteral::LightCyan => Color::Rgba(224, 255, 255, 255),
            ColorLiteral::Azure => Color::Rgba(240, 255, 255, 255),
            ColorLiteral::AliceBlue => Color::Rgba(240, 248, 255, 255),
            ColorLiteral::MintCream => Color::Rgba(245, 255, 250, 255),
            ColorLiteral::GhostWhite => Color::Rgba(248, 248, 255, 255),
        }
    }

    pub fn to_hex(&self) -> Color {
        match self {
            ColorLiteral::Maroon => Color::Hex("#800000".into()),
            ColorLiteral::DarkRed => Color::Hex("#8b0000".into()),
            ColorLiteral::Brown => Color::Hex("#a52a2a".into()),
            ColorLiteral::FireBrick => Color::Hex("#b22222".into()),
            ColorLiteral::Crimson => Color::Hex("#dc143c".into()),
            ColorLiteral::Red => Color::Hex("#ff0000".into()),
            ColorLiteral::IndianRed => Color::Hex("#cd5c5c".into()),
            ColorLiteral::SaddleBrown => Color::Hex("#8b4513".into()),
            ColorLiteral::Sienna => Color::Hex("#a0522d".into()),
            ColorLiteral::RosyBrown => Color::Hex("#bc8f8f".into()),
            ColorLiteral::Chocolate => Color::Hex("#d2691e".into()),
            ColorLiteral::Peru => Color::Hex("#cd853f".into()),
            ColorLiteral::DarkGoldenRod => Color::Hex("#b8860b".into()),
            ColorLiteral::DarkSalmon => Color::Hex("#e9967a".into()),
            ColorLiteral::LightCoral => Color::Hex("#f08080".into()),
            ColorLiteral::GoldenRod => Color::Hex("#daa520".into()),
            ColorLiteral::Tan => Color::Hex("#d2b48c".into()),
            ColorLiteral::DarkKhaki => Color::Hex("#bdb76b".into()),
            ColorLiteral::BurlyWood => Color::Hex("#deb887".into()),
            ColorLiteral::PaleGoldenRod => Color::Hex("#eee8aa".into()),
            ColorLiteral::SandyBrown => Color::Hex("#f4a460".into()),
            ColorLiteral::Salmon => Color::Hex("#fa8072".into()),
            ColorLiteral::OrangeRed => Color::Hex("#ff4500".into()),
            ColorLiteral::Tomato => Color::Hex("#ff6347".into()),
            ColorLiteral::Coral => Color::Hex("#ff7f50".into()),
            ColorLiteral::DarkOrange => Color::Hex("#ff8c00".into()),
            ColorLiteral::LightSalmon => Color::Hex("#ffa07a".into()),
            ColorLiteral::Orange => Color::Hex("#ffa500".into()),
            ColorLiteral::Gold => Color::Hex("#ffd700".into()),
            ColorLiteral::Yellow => Color::Hex("#ffff00".into()),
            ColorLiteral::Khaki => Color::Hex("#f0e68c".into()),
            ColorLiteral::Wheat => Color::Hex("#f5deb3".into()),
            ColorLiteral::Beige => Color::Hex("#f5f5dc".into()),
            ColorLiteral::AntiqueWhite => Color::Hex("#faebd7".into()),
            ColorLiteral::Linen => Color::Hex("#faf0e6".into()),
            ColorLiteral::LightGoldenRodYellow => Color::Hex("#fafad2".into()),
            ColorLiteral::OldLace => Color::Hex("#fdf5e6".into()),
            ColorLiteral::PeachPuff => Color::Hex("#ffdab9".into()),
            ColorLiteral::NavajoWhite => Color::Hex("#ffdead".into()),
            ColorLiteral::Moccasin => Color::Hex("#ffe4b5".into()),
            ColorLiteral::Bisque => Color::Hex("#ffe4c4".into()),
            ColorLiteral::BlanchedAlmond => Color::Hex("#ffebcd".into()),
            ColorLiteral::PapayaWhip => Color::Hex("#ffefd5".into()),
            ColorLiteral::LemonChiffon => Color::Hex("#fffacd".into()),
            ColorLiteral::LightYellow => Color::Hex("#ffffe0".into()),
            ColorLiteral::Cornsilk => Color::Hex("#fff8dc".into()),
            ColorLiteral::MistyRose => Color::Hex("#ffe4e1".into()),
            ColorLiteral::FloralWhite => Color::Hex("#fffaf0".into()),
            ColorLiteral::SeaShell => Color::Hex("#fff5ee".into()),
            ColorLiteral::Snow => Color::Hex("#fffafa".into()),
            ColorLiteral::Ivory => Color::Hex("#fffff0".into()),
            ColorLiteral::White => Color::Hex("#ffffff".into()),
            ColorLiteral::Black => Color::Hex("#000000".into()),
            ColorLiteral::DimGray => Color::Hex("#696969".into()),
            ColorLiteral::DarkGray => Color::Hex("#a9a9a9".into()),
            ColorLiteral::Gray => Color::Hex("#808080".into()),
            ColorLiteral::Silver => Color::Hex("#c0c0c0".into()),
            ColorLiteral::LightGray => Color::Hex("#d3d3d3".into()),
            ColorLiteral::Gainsboro => Color::Hex("#dcdcdc".into()),
            ColorLiteral::WhiteSmoke => Color::Hex("#f5f5f5".into()),
            ColorLiteral::DarkSlateGray => Color::Hex("#2f4f4f".into()),
            ColorLiteral::SlateGray => Color::Hex("#708090".into()),
            ColorLiteral::LightSlateGray => Color::Hex("#778899".into()),
            ColorLiteral::DarkGreen => Color::Hex("#006400".into()),
            ColorLiteral::Green => Color::Hex("#008000".into()),
            ColorLiteral::ForestGreen => Color::Hex("#228b22".into()),
            ColorLiteral::DarkSeaGreen => Color::Hex("#8fbc8f".into()),
            ColorLiteral::SeaGreen => Color::Hex("#2e8b57".into()),
            ColorLiteral::MediumSeaGreen => Color::Hex("#3cb371".into()),
            ColorLiteral::LightSeaGreen => Color::Hex("#20b2aa".into()),
            ColorLiteral::LimeGreen => Color::Hex("#32cd32".into()),
            ColorLiteral::Lime => Color::Hex("#00ff00".into()),
            ColorLiteral::Chartreuse => Color::Hex("#7fff00".into()),
            ColorLiteral::LawnGreen => Color::Hex("#7cfc00".into()),
            ColorLiteral::SpringGreen => Color::Hex("#00ff7f".into()),
            ColorLiteral::MediumSpringGreen => Color::Hex("#00fa9a".into()),
            ColorLiteral::DarkOliveGreen => Color::Hex("#556b2f".into()),
            ColorLiteral::OliveDrab => Color::Hex("#6b8e23".into()),
            ColorLiteral::Olive => Color::Hex("#808000".into()),
            ColorLiteral::YellowGreen => Color::Hex("#9acd32".into()),
            ColorLiteral::GreenYellow => Color::Hex("#adff2f".into()),
            ColorLiteral::PaleGreen => Color::Hex("#98fb98".into()),
            ColorLiteral::LightGreen => Color::Hex("#90ee90".into()),
            ColorLiteral::HoneyDew => Color::Hex("#f0fff0".into()),
            ColorLiteral::Teal => Color::Hex("#008080".into()),
            ColorLiteral::DarkCyan => Color::Hex("#008b8b".into()),
            ColorLiteral::CadetBlue => Color::Hex("#5f9ea0".into()),
            ColorLiteral::DarkTurquoise => Color::Hex("#00ced1".into()),
            ColorLiteral::Turquoise => Color::Hex("#40e0d0".into()),
            ColorLiteral::MediumTurquoise => Color::Hex("#48d1cc".into()),
            ColorLiteral::MediumAquaMarine => Color::Hex("#66cdaa".into()),
            ColorLiteral::Aquamarine => Color::Hex("#7fffd4".into()),
            ColorLiteral::MidnightBlue => Color::Hex("#191970".into()),
            ColorLiteral::Navy => Color::Hex("#000080".into()),
            ColorLiteral::DarkBlue => Color::Hex("#00008b".into()),
            ColorLiteral::MediumBlue => Color::Hex("#0000cd".into()),
            ColorLiteral::RoyalBlue => Color::Hex("#4169e1".into()),
            ColorLiteral::CornflowerBlue => Color::Hex("#6495ed".into()),
            ColorLiteral::SteelBlue => Color::Hex("#4682b4".into()),
            ColorLiteral::Blue => Color::Hex("#0000ff".into()),
            ColorLiteral::DodgerBlue => Color::Hex("#1e90ff".into()),
            ColorLiteral::PaleTurquoise => Color::Hex("#afeeee".into()),
            ColorLiteral::LightSteelBlue => Color::Hex("#b0c4de".into()),
            ColorLiteral::PowderBlue => Color::Hex("#b0e0e6".into()),
            ColorLiteral::DeepSkyBlue => Color::Hex("#00bfff".into()),
            ColorLiteral::SkyBlue => Color::Hex("#87ceeb".into()),
            ColorLiteral::LightSkyBlue => Color::Hex("#87cefa".into()),
            ColorLiteral::LightBlue => Color::Hex("#add8e6".into()),
            ColorLiteral::Aqua => Color::Hex("#00ffff".into()),
            ColorLiteral::Cyan => Color::Hex("#00ffff".into()),
            ColorLiteral::LightCyan => Color::Hex("#e0ffff".into()),
            ColorLiteral::Azure => Color::Hex("#f0ffff".into()),
            ColorLiteral::AliceBlue => Color::Hex("#f0f8ff".into()),
            ColorLiteral::MintCream => Color::Hex("#f5fffa".into()),
            ColorLiteral::GhostWhite => Color::Hex("#f8f8ff".into()),
        }
    }

    pub fn to_hsl(&self) -> Color {
        match self {
            ColorLiteral::Maroon => Color::Hsl(0.0, 100.0, 25.0),
            ColorLiteral::DarkRed => Color::Hsl(0.0, 100.0, 27.0),
            ColorLiteral::Brown => Color::Hsl(0.0, 59.0, 40.0),
            ColorLiteral::FireBrick => Color::Hsl(0.0, 67.0, 41.0),
            ColorLiteral::Crimson => Color::Hsl(348.0, 83.0, 47.0),
            ColorLiteral::Red => Color::Hsl(0.0, 100.0, 50.0),
            ColorLiteral::IndianRed => Color::Hsl(0.0, 53.0, 58.0),
            ColorLiteral::SaddleBrown => Color::Hsl(24.0, 75.0, 30.0),
            ColorLiteral::Sienna => Color::Hsl(19.0, 56.0, 40.0),
            ColorLiteral::RosyBrown => Color::Hsl(0.0, 25.0, 64.0),
            ColorLiteral::Chocolate => Color::Hsl(24.0, 74.0, 47.0),
            ColorLiteral::Peru => Color::Hsl(29.0, 58.0, 52.0),
            ColorLiteral::DarkGoldenRod => Color::Hsl(42.0, 88.0, 38.0),
            ColorLiteral::DarkSalmon => Color::Hsl(15.0, 71.0, 69.0),
            ColorLiteral::LightCoral => Color::Hsl(0.0, 78.0, 72.0),
            ColorLiteral::GoldenRod => Color::Hsl(42.0, 88.0, 38.0),
            ColorLiteral::Tan => Color::Hsl(34.0, 43.0, 68.0),
            ColorLiteral::DarkKhaki => Color::Hsl(55.0, 38.0, 58.0),
            ColorLiteral::BurlyWood => Color::Hsl(33.0, 56.0, 70.0),
            ColorLiteral::PaleGoldenRod => Color::Hsl(54.0, 66.0, 80.0),
            ColorLiteral::SandyBrown => Color::Hsl(27.0, 87.0, 66.0),
            ColorLiteral::Salmon => Color::Hsl(6.0, 93.0, 71.0),
            ColorLiteral::OrangeRed => Color::Hsl(16.0, 100.0, 50.0),
            ColorLiteral::Tomato => Color::Hsl(9.0, 100.0, 63.0),
            ColorLiteral::Coral => Color::Hsl(16.0, 100.0, 65.0),
            ColorLiteral::DarkOrange => Color::Hsl(32.0, 100.0, 50.0),
            ColorLiteral::LightSalmon => Color::Hsl(17.0, 100.0, 73.0),
            ColorLiteral::Orange => Color::Hsl(38.0, 100.0, 50.0),
            ColorLiteral::Gold => Color::Hsl(50.0, 100.0, 50.0),
            ColorLiteral::Yellow => Color::Hsl(60.0, 100.0, 50.0),
            ColorLiteral::Khaki => Color::Hsl(54.0, 76.0, 74.0),
            ColorLiteral::Wheat => Color::Hsl(39.0, 76.0, 83.0),
            ColorLiteral::Beige => Color::Hsl(60.0, 55.0, 91.0),
            ColorLiteral::AntiqueWhite => Color::Hsl(34.0, 77.0, 91.0),
            ColorLiteral::Linen => Color::Hsl(30.0, 66.0, 94.0),
            ColorLiteral::LightGoldenRodYellow => Color::Hsl(60.0, 80.0, 90.0),
            ColorLiteral::OldLace => Color::Hsl(39.0, 85.0, 94.0),
            ColorLiteral::PeachPuff => Color::Hsl(28.0, 100.0, 86.0),
            ColorLiteral::NavajoWhite => Color::Hsl(35.0, 100.0, 83.0),
            ColorLiteral::Moccasin => Color::Hsl(38.0, 100.0, 85.0),
            ColorLiteral::Bisque => Color::Hsl(32.0, 100.0, 88.0),
            ColorLiteral::BlanchedAlmond => Color::Hsl(35.0, 100.0, 90.0),
            ColorLiteral::PapayaWhip => Color::Hsl(37.0, 100.0, 91.0),
            ColorLiteral::LemonChiffon => Color::Hsl(53.0, 100.0, 90.0),
            ColorLiteral::LightYellow => Color::Hsl(60.0, 100.0, 93.0),
            ColorLiteral::Cornsilk => Color::Hsl(47.0, 100.0, 93.0),
            ColorLiteral::MistyRose => Color::Hsl(6.0, 100.0, 94.0),
            ColorLiteral::FloralWhite => Color::Hsl(39.0, 100.0, 97.0),
            ColorLiteral::SeaShell => Color::Hsl(24.0, 100.0, 96.0),
            ColorLiteral::Snow => Color::Hsl(0.0, 100.0, 99.0),
            ColorLiteral::Ivory => Color::Hsl(60.0, 100.0, 97.0),
            ColorLiteral::White => Color::Hsl(0.0, 0.0, 100.0),
            ColorLiteral::Black => Color::Hsl(0.0, 0.0, 0.0),
            ColorLiteral::DimGray => Color::Hsl(0.0, 0.0, 41.0),
            ColorLiteral::DarkGray => Color::Hsl(0.0, 0.0, 66.0),
            ColorLiteral::Gray => Color::Hsl(0.0, 0.0, 50.0),
            ColorLiteral::Silver => Color::Hsl(0.0, 0.0, 75.0),
            ColorLiteral::LightGray => Color::Hsl(0.0, 0.0, 82.0),
            ColorLiteral::Gainsboro => Color::Hsl(0.0, 0.0, 86.0),
            ColorLiteral::WhiteSmoke => Color::Hsl(0.0, 0.0, 96.0),
            ColorLiteral::DarkSlateGray => Color::Hsl(180.0, 25.0, 24.0),
            ColorLiteral::SlateGray => Color::Hsl(210.0, 12.0, 50.0),
            ColorLiteral::LightSlateGray => Color::Hsl(210.0, 14.0, 53.0),
            ColorLiteral::DarkGreen => Color::Hsl(120.0, 100.0, 19.0),
            ColorLiteral::Green => Color::Hsl(120.0, 100.0, 25.0),
            ColorLiteral::ForestGreen => Color::Hsl(120.0, 60.0, 33.0),
            ColorLiteral::DarkSeaGreen => Color::Hsl(120.0, 25.0, 64.0),
            ColorLiteral::SeaGreen => Color::Hsl(146.0, 50.0, 36.0),
            ColorLiteral::MediumSeaGreen => Color::Hsl(146.0, 49.0, 46.0),
            ColorLiteral::LightSeaGreen => Color::Hsl(176.0, 69.0, 41.0),
            ColorLiteral::LimeGreen => Color::Hsl(120.0, 60.0, 50.0),
            ColorLiteral::Lime => Color::Hsl(120.0, 100.0, 50.0),
            ColorLiteral::Chartreuse => Color::Hsl(90.0, 100.0, 50.0),
            ColorLiteral::LawnGreen => Color::Hsl(90.0, 100.0, 49.0),
            ColorLiteral::SpringGreen => Color::Hsl(149.0, 100.0, 50.0),
            ColorLiteral::MediumSpringGreen => Color::Hsl(156.0, 100.0, 49.0),
            ColorLiteral::DarkOliveGreen => Color::Hsl(82.0, 38.0, 30.0),
            ColorLiteral::OliveDrab => Color::Hsl(79.0, 60.0, 34.0),
            ColorLiteral::Olive => Color::Hsl(60.0, 100.0, 25.0),
            ColorLiteral::YellowGreen => Color::Hsl(83.0, 100.0, 59.0),
            ColorLiteral::GreenYellow => Color::Hsl(79.0, 60.0, 50.0),
            ColorLiteral::PaleGreen => Color::Hsl(120.0, 92.0, 79.0),
            ColorLiteral::LightGreen => Color::Hsl(120.0, 73.0, 74.0),
            ColorLiteral::HoneyDew => Color::Hsl(120.0, 100.0, 97.0),
            ColorLiteral::Teal => Color::Hsl(180.0, 100.0, 25.0),
            ColorLiteral::DarkCyan => Color::Hsl(180.0, 100.0, 27.0),
            ColorLiteral::CadetBlue => Color::Hsl(181.0, 25.0, 50.0),
            ColorLiteral::DarkTurquoise => Color::Hsl(174.0, 72.0, 56.0),
            ColorLiteral::Turquoise => Color::Hsl(177.0, 59.0, 55.0),
            ColorLiteral::MediumTurquoise => Color::Hsl(159.0, 50.0, 60.0),
            ColorLiteral::MediumAquaMarine => Color::Hsl(159.0, 50.0, 60.0),
            ColorLiteral::Aquamarine => Color::Hsl(159.0, 50.0, 60.0),
            ColorLiteral::MidnightBlue => Color::Hsl(240.0, 63.0, 26.0),
            ColorLiteral::Navy => Color::Hsl(240.0, 100.0, 25.0),
            ColorLiteral::DarkBlue => Color::Hsl(240.0, 100.0, 27.0),
            ColorLiteral::MediumBlue => Color::Hsl(240.0, 100.0, 40.0),
            ColorLiteral::RoyalBlue => Color::Hsl(225.0, 72.0, 56.0),
            ColorLiteral::CornflowerBlue => Color::Hsl(218.0, 79.0, 66.0),
            ColorLiteral::SteelBlue => Color::Hsl(207.0, 44.0, 49.0),
            ColorLiteral::Blue => Color::Hsl(240.0, 100.0, 50.0),
            ColorLiteral::DodgerBlue => Color::Hsl(209.0, 100.0, 55.0),
            ColorLiteral::PaleTurquoise => Color::Hsl(180.0, 64.0, 80.0),
            ColorLiteral::LightSteelBlue => Color::Hsl(213.0, 41.0, 78.0),
            ColorLiteral::PowderBlue => Color::Hsl(186.0, 51.0, 79.0),
            ColorLiteral::DeepSkyBlue => Color::Hsl(195.0, 100.0, 50.0),
            ColorLiteral::SkyBlue => Color::Hsl(197.0, 71.0, 72.0),
            ColorLiteral::LightSkyBlue => Color::Hsl(202.0, 92.0, 75.0),
            ColorLiteral::LightBlue => Color::Hsl(194.0, 53.0, 79.0),
            ColorLiteral::Aqua => Color::Hsl(180.0, 100.0, 50.0),
            ColorLiteral::Cyan => Color::Hsl(180.0, 100.0, 50.0),
            ColorLiteral::LightCyan => Color::Hsl(180.0, 100.0, 93.0),
            ColorLiteral::Azure => Color::Hsl(180.0, 100.0, 30.0),
            ColorLiteral::AliceBlue => Color::Hsl(208.0, 100.0, 97.0),
            ColorLiteral::MintCream => Color::Hsl(149.0, 100.0, 98.0),
            ColorLiteral::GhostWhite => Color::Hsl(240.0, 100.0, 98.0),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Url {
    Url(String),
    Id(String),
}

impl ToString for Url {
    fn to_string(&self) -> String {
        match self {
            Url::Url(url) => url.to_string(),
            Url::Id(id) => format!("#{}", id),
        }
    }
}

impl FromStr for Url {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: The following is incorrect if the url is in any other casing than "url"
        if s.starts_with("url(") && s.ends_with(")") {
            return Ok(Self::Url(s[4..s.len() - 1].to_string()));
        }

        if s.starts_with("url(#") && s.ends_with(")") {
            return Ok(Self::Id(s[5..s.len() - 1].to_string()));
        }

        Ok(Self::Id(s.to_string()))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Color {
    Hex(String),
    Rgb(u8, u8, u8),
    Rgba(u8, u8, u8, u8),
    Hsl(f64, f64, f64),
    Hsla(f64, f64, f64, f64),
    Literal(ColorLiteral),
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self {
            Color::Hex(hex) => hex.clone(),
            Color::Rgb(r, g, b) => format!("rgb({}, {}, {})", r, g, b),
            Color::Rgba(r, g, b, a) => format!("rgba({}, {}, {}, {})", r, g, b, a),
            Color::Hsl(h, s, l) => format!("hsl({}, {}, {})", h, s, l),
            Color::Hsla(h, s, l, a) => format!("hsla({}, {}, {}, {})", h, s, l, a),
            Color::Literal(literal) => literal.as_str().to_string(),
        }
    }
}

impl FromStr for Color {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("#") {
            return Ok(Self::Hex(s[1..].to_string()));
        }

        if s.starts_with("rgb(") && s.ends_with(")") {
            let mut iter = s[4..s.len() - 1].split(',');

            let r = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
            let g = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
            let b = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;

            return Ok(Self::Rgb(r, g, b));
        }

        if s.starts_with("rgba(") && s.ends_with(")") {
            let mut iter = s[5..s.len() - 1].split(',');

            let r = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
            let g = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
            let b = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;
            let a = iter.next().ok_or(())?.parse::<u8>().map_err(|_| ())?;

            return Ok(Self::Rgba(r, g, b, a));
        }

        if s.starts_with("hsl(") && s.ends_with(")") {
            let mut iter = s[4..s.len() - 1].split(',');

            let h = iter.next().ok_or(())?.parse::<f64>().map_err(|_| ())?;
            let s = iter.next().ok_or(())?.parse::<f64>().map_err(|_| ())?;
            let l = iter.next().ok_or(())?.parse::<f64>().map_err(|_| ())?;

            return Ok(Self::Hsl(h, s, l));
        }

        if s.starts_with("hsla(") && s.ends_with(")") {
            let mut iter = s[5..s.len() - 1].split(',');

            let h = iter
                .next()
                .ok_or(())?
                .trim()
                .parse::<f64>()
                .map_err(|_| ())?;

            let s = iter
                .next()
                .ok_or(())?
                .trim()
                .parse::<f64>()
                .map_err(|_| ())?;

            let l = iter
                .next()
                .ok_or(())?
                .trim()
                .parse::<f64>()
                .map_err(|_| ())?;

            let a = iter
                .next()
                .ok_or(())?
                .trim()
                .parse::<f64>()
                .map_err(|_| ())?;

            return Ok(Self::Hsla(h, s, l, a));
        }

        Ok(Self::Literal(s.parse()?))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Paint {
    #[default]
    None,
    Color(Color),
    Url(Url),
    ContextFill,
    ContextStroke,
}

impl ToString for Paint {
    fn to_string(&self) -> String {
        match self {
            Paint::None => "none".to_string(),
            Paint::Color(color) => color.to_string(),
            Paint::Url(url) => url.to_string(),
            Paint::ContextFill => "context-fill".to_string(),
            Paint::ContextStroke => "context-stroke".to_string(),
        }
    }
}

impl FromStr for Paint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("none") => Ok(Self::None),
            _ if s.eq_ignore_ascii_case("context-fill") => Ok(Self::ContextFill),
            _ if s.eq_ignore_ascii_case("context-stroke") => Ok(Self::ContextStroke),
            _ => {
                if let Ok(url) = s.parse::<Url>() {
                    return Ok(Self::Url(url));
                }

                Ok(Self::Color(s.parse()?))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AbsoluteSize {
    XXSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
}

impl ToString for AbsoluteSize {
    fn to_string(&self) -> String {
        match self {
            AbsoluteSize::XXSmall => "xx-small",
            AbsoluteSize::XSmall => "x-small",
            AbsoluteSize::Small => "small",
            AbsoluteSize::Medium => "medium",
            AbsoluteSize::Large => "large",
            AbsoluteSize::XLarge => "x-large",
            AbsoluteSize::XXLarge => "xx-large",
            AbsoluteSize::XXXLarge => "xxx-large",
        }
        .to_string()
    }
}

impl FromStr for AbsoluteSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("xx-small") => Ok(Self::XXSmall),
            _ if s.eq_ignore_ascii_case("x-small") => Ok(Self::XSmall),
            _ if s.eq_ignore_ascii_case("small") => Ok(Self::Small),
            _ if s.eq_ignore_ascii_case("medium") => Ok(Self::Medium),
            _ if s.eq_ignore_ascii_case("large") => Ok(Self::Large),
            _ if s.eq_ignore_ascii_case("x-large") => Ok(Self::XLarge),
            _ if s.eq_ignore_ascii_case("xx-large") => Ok(Self::XXLarge),
            _ if s.eq_ignore_ascii_case("xxx-large") => Ok(Self::XXXLarge),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum RelativeSize {
    Larger,
    Smaller,
}

impl ToString for RelativeSize {
    fn to_string(&self) -> String {
        match self {
            RelativeSize::Larger => "larger".to_string(),
            RelativeSize::Smaller => "smaller".to_string(),
        }
    }
}

impl FromStr for RelativeSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("larger") => Ok(Self::Larger),
            _ if s.eq_ignore_ascii_case("smaller") => Ok(Self::Smaller),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontWeight {
    Normal,
    Bold,
    Bolder,
    Lighter,
    Number(f64),
}

impl ToString for FontWeight {
    fn to_string(&self) -> String {
        match self {
            FontWeight::Normal => "normal".to_string(),
            FontWeight::Bold => "bold".to_string(),
            FontWeight::Bolder => "bolder".to_string(),
            FontWeight::Lighter => "lighter".to_string(),
            FontWeight::Number(number) => number.to_string(),
        }
    }
}

impl FromStr for FontWeight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.eq_ignore_ascii_case("normal") => Ok(Self::Normal),
            _ if s.eq_ignore_ascii_case("bold") => Ok(Self::Bold),
            _ if s.eq_ignore_ascii_case("bolder") => Ok(Self::Bolder),
            _ if s.eq_ignore_ascii_case("lighter") => Ok(Self::Lighter),
            _ => Ok(Self::Number(s.parse().map_err(|_| ())?)),
        }
    }
}
