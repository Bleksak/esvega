pub struct Boolean(bool);
pub struct Integer(i64);
pub struct Number(f64);
pub struct SvgString(String);
pub enum Url {
    Target(String),
    Url(String),
}