use crate::{Element, element::types::LengthOrPercentage};

#[derive(Clone, Debug)]
pub struct Circle {
    pub cx: Option<LengthOrPercentage>,
    pub cy: Option<LengthOrPercentage>,
    pub r: Option<LengthOrPercentage>,
    pub path_length: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct CircleElement {
    pub circle: Circle,
    pub children: Vec<Element>,
}
