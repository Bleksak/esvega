use crate::svg::types::basic::Number;

pub enum Unit {
    Pixel,
    Point,
    Pica,
    Milimeter,
    Centimeter,
    Inch,
    Rem,
    Em,
    Ex,
    Ch,
    Vw,
    Vh,
    VMin,
    VMax,
}

pub struct Length {
    value: Number,
    unit: Unit,
}

pub struct Percentage(Number);
