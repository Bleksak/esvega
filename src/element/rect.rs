use crate::{
    Element,
    element::types::{Color, LengthOrPercentage},
};

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: Option<LengthOrPercentage>,
    pub y: Option<LengthOrPercentage>,
    pub width: Option<LengthOrPercentage>,
    pub height: Option<LengthOrPercentage>,
    pub rx: Option<LengthOrPercentage>,
    pub ry: Option<LengthOrPercentage>,
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
pub struct RectElement {
    pub rect: Rect,
    pub children: Vec<Element>,
}
