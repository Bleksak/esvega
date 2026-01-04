use std::convert::Infallible;
use std::str::FromStr;

#[derive(Clone, Debug)]
pub enum Element {
    Hyperlink,
    Circle,
    Ellipse,
    Group,
    Image,
    Line,
    Path,
    Polygon,
    PolyLine,
    Rect,
    Text,
    TextPath,
}

impl FromStr for Element {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
