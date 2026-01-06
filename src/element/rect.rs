use crate::{Element, element::types::LengthOrPercentage};

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: Option<LengthOrPercentage>,
    pub y: Option<LengthOrPercentage>,
    pub width: Option<LengthOrPercentage>,
    pub height: Option<LengthOrPercentage>,
    pub rx: Option<LengthOrPercentage>,
    pub ry: Option<LengthOrPercentage>,
    pub path_length: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct RectElement {
    pub rect: Rect,
    pub children: Vec<Element>,
}
