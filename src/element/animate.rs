#[derive(Clone, Debug)]
pub struct AnimateElement {
    pub autofocus: Option<bool>,
    pub id: Option<String>,
    pub class: Option<String>,
    pub style: Option<String>,
    pub lang: Option<String>,
    pub tabindex: Option<String>,
    pub xml_lang: Option<String>,
    pub xml_space: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AnimateMotionElement {}

#[derive(Clone, Debug)]
pub struct AnimateTransformElement {}

#[derive(Clone, Debug)]
pub struct MPathElement {}

#[derive(Clone, Debug)]
pub struct SetElement {}
