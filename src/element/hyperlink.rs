use crate::Element;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ReferrerPolicy {
    NoReferrer,
    NoReferrerWhenDowngrade,
    SameOrigin,
    Origin,
    StrictOrigin,
    OriginWhenCrossOrigin,
    StrictOriginWhenCrossOrigin,
    UnsafeUrl,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Target {
    #[default]
    Self_,
    Parent,
    Top,
    Blank,
}

#[derive(Clone, Debug)]
pub struct Hyperlink {
    /// None if not set, Some(None) if empty,
    /// Some(Some(filename)) if set,
    pub download: Option<Option<String>>,
    pub href: String,
    pub hreflang: Option<String>,
    pub interestfor: Option<String>,
    pub ping: Option<String>,
    pub referrerpolicy: Option<ReferrerPolicy>,
    pub rel: Option<String>,
    pub target: Option<Target>,
    pub type_: Option<String>,
    pub xlink_href: Option<String>,
}

#[derive(Clone, Debug)]
pub struct HyperlinkElement {
    pub hyperlink: Hyperlink,
    pub children: Vec<Element>,
}
