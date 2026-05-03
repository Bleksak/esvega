use crate::svg::types::basic::Number;

pub enum Unit {
    Degree,
    Radian,
    Gradian,
    Turn,
}

pub struct Angle {
    value: Number,
    unit: Unit,
}
