use crate::Element;

#[derive(Debug, Default)]
pub struct AST {
    pub children: Vec<Element>,
}

impl AST {
    pub fn to_svg(&self) -> String {
        let mut svg = String::new();

        for element in &self.children {
            svg.push_str(&element.to_svg(0));
        }

        svg
    }
}
