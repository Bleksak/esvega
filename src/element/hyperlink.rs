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

impl TryFrom<&str> for ReferrerPolicy {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "no-referrer" => Ok(Self::NoReferrer),
            "no-referrer-when-downgrade" => Ok(Self::NoReferrerWhenDowngrade),
            "same-origin" => Ok(Self::SameOrigin),
            "origin" => Ok(Self::Origin),
            "strict-origin" => Ok(Self::StrictOrigin),
            "origin-when-cross-origin" => Ok(Self::OriginWhenCrossOrigin),
            "strict-origin-when-cross-origin" => Ok(Self::StrictOriginWhenCrossOrigin),
            "unsafe-url" => Ok(Self::UnsafeUrl),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Target {
    #[default]
    Self_,
    Parent,
    Top,
    Blank,
}

impl TryFrom<&str> for Target {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "_self" => Ok(Self::Self_),
            "_parent" => Ok(Self::Parent),
            "_top" => Ok(Self::Top),
            "_blank" => Ok(Self::Blank),
            _ => Err(()),
        }
    }
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
