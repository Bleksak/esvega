use crate::{Element, element::types::LengthOrPercentage};

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

impl TryFrom<&str> for PreserveAspectRatio {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "none" {
            Ok(PreserveAspectRatio::None)
        } else if value == "xMinYMin meet" {
            Ok(PreserveAspectRatio::XMinYMinMeet)
        } else if value == "xMidYMin meet" {
            Ok(PreserveAspectRatio::XMidYMinMeet)
        } else if value == "xMaxYMin meet" {
            Ok(PreserveAspectRatio::XMaxYMinMeet)
        } else if value == "xMinYMid meet" {
            Ok(PreserveAspectRatio::XMinYMidMeet)
        } else if value == "xMidYMid meet" {
            Ok(PreserveAspectRatio::XMidYMidMeet)
        } else if value == "xMaxYMid meet" {
            Ok(PreserveAspectRatio::XMaxYMidMeet)
        } else if value == "xMinYMax meet" {
            Ok(PreserveAspectRatio::XMinYMaxMeet)
        } else if value == "xMidYMax meet" {
            Ok(PreserveAspectRatio::XMidYMaxMeet)
        } else if value == "xMaxYMax meet" {
            Ok(PreserveAspectRatio::XMaxYMaxMeet)
        } else if value == "xMinYMin slice" {
            Ok(PreserveAspectRatio::XMinYMinSlice)
        } else if value == "xMidYMin slice" {
            Ok(PreserveAspectRatio::XMidYMinSlice)
        } else if value == "xMaxYMin slice" {
            Ok(PreserveAspectRatio::XMaxYMinSlice)
        } else if value == "xMinYMid slice" {
            Ok(PreserveAspectRatio::XMinYMidSlice)
        } else if value == "xMidYMid slice" {
            Ok(PreserveAspectRatio::XMidYMidSlice)
        } else if value == "xMaxYMid slice" {
            Ok(PreserveAspectRatio::XMaxYMidSlice)
        } else if value == "xMinYMax slice" {
            Ok(PreserveAspectRatio::XMinYMaxSlice)
        } else if value == "xMidYMax slice" {
            Ok(PreserveAspectRatio::XMidYMaxSlice)
        } else if value == "xMaxYMax slice" {
            Ok(PreserveAspectRatio::XMaxYMaxSlice)
        } else {
            Err(())
        }
    }
}

#[derive(Clone, Debug)]
pub struct Svg {
    pub base_profile: Option<String>,
    pub height: Option<LengthOrPercentage>,
    pub preserve_aspect_ratio: Option<String>,
    pub version: Option<f64>,
    pub view_box: Option<[f64; 4]>,
    pub width: Option<LengthOrPercentage>,
    pub x: Option<LengthOrPercentage>,
    pub y: Option<LengthOrPercentage>,
    pub xmlns: Option<String>,
}

#[derive(Clone, Debug)]
pub struct SVGElement {
    pub svg: Svg,
    pub children: Vec<Element>,
}
