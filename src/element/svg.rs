use crate::{Element, element::types::LengthOrPercentage};

#[derive(Clone, Debug)]
pub struct Svg {
    pub base_profile: Option<String>,
    pub height: Option<LengthOrPercentage>,
    pub preserve_aspect_ratio: Option<String>,
    pub version: Option<f64>,
    pub view_box: Option<Vec<f64>>,
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
