use crate::{Element, element::types::{Color, LengthOrPercentage}};

#[derive(Clone, Debug)]
pub struct Circle {
    pub cx: Option<LengthOrPercentage>,
    pub cy: Option<LengthOrPercentage>,
    pub r: Option<LengthOrPercentage>,
    pub path_length: Option<f64>,

    pub stroke: Option<Color>,
    pub stroke_width: Option<LengthOrPercentage>,
    pub stroke_opacity: Option<f64>,

    pub stroke_dasharray: Option<String>,
    pub stroke_dashoffset: Option<LengthOrPercentage>,

    pub stroke_linecap: Option<String>,
    pub stroke_linejoin: Option<String>,
    pub stroke_miterlimit: Option<f64>,

    pub vector_effect: Option<String>,

    pub fill: Option<Color>,
    pub fill_opacity: Option<f64>,
    pub fill_rule: Option<String>,
}

#[derive(Clone, Debug)]
pub struct CircleElement {
    pub circle: Circle,
    pub children: Vec<Element>,
}
