use std::fmt;
use std::str::FromStr;

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

impl fmt::Display for ReferrerPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReferrerPolicy::NoReferrer => "no-referrer",
                ReferrerPolicy::NoReferrerWhenDowngrade => "no-referrer-when-downgrade",
                ReferrerPolicy::SameOrigin => "same-origin",
                ReferrerPolicy::Origin => "origin",
                ReferrerPolicy::StrictOrigin => "strict-origin",
                ReferrerPolicy::OriginWhenCrossOrigin => "origin-when-cross-origin",
                ReferrerPolicy::StrictOriginWhenCrossOrigin => "strict-origin-when-cross-origin",
                ReferrerPolicy::UnsafeUrl => "unsafe-url",
            }
        )
    }
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RelType {
    Alternate,
    Author,
    Bookmark,
    Canonical,
    CompressionDictionary,
    DnsPrefetch,
    External,
    Expect,
    Help,
    Icon,
    License,
    Manifest,
    Me,
    ModulePreload,
    Next,
    NoFollow,
    NoOpener,
    NoReferrer,
    Opener,
    PingBack,
    PreConnect,
    Prefetch,
    Preload,
    Prev,
    PrivacyPolicy,
    Search,
    StyleSheet,
    Tag,
    TermsOfService,
}

impl fmt::Display for RelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RelType::Alternate => "alternate",
                RelType::Author => "author",
                RelType::Bookmark => "bookmark",
                RelType::Canonical => "canonical",
                RelType::CompressionDictionary => "compression-dictionary",
                RelType::DnsPrefetch => "dns-prefetch",
                RelType::External => "external",
                RelType::Expect => "expect",
                RelType::Help => "help",
                RelType::Icon => "icon",
                RelType::License => "license",
                RelType::Manifest => "manifest",
                RelType::Me => "me",
                RelType::ModulePreload => "module-preload",
                RelType::Next => "next",
                RelType::NoFollow => "nofollow",
                RelType::NoOpener => "noopener",
                RelType::NoReferrer => "noreferrer",
                RelType::Opener => "opener",
                RelType::PingBack => "pingback",
                RelType::PreConnect => "preconnect",
                RelType::Prefetch => "prefetch",
                RelType::Preload => "preload",
                RelType::Prev => "prev",
                RelType::PrivacyPolicy => "privacy-policy",
                RelType::Search => "search",
                RelType::StyleSheet => "stylesheet",
                RelType::Tag => "tag",
                RelType::TermsOfService => "terms-of-service",
            }
        )
    }
}

impl FromStr for RelType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alternate" => Ok(Self::Alternate),
            "author" => Ok(Self::Author),
            "bookmark" => Ok(Self::Bookmark),
            "canonical" => Ok(Self::Canonical),
            "compression-dictionary" => Ok(Self::CompressionDictionary),
            "dns-prefetch" => Ok(Self::DnsPrefetch),
            "external" => Ok(Self::External),
            "expect" => Ok(Self::Expect),
            "help" => Ok(Self::Help),
            "icon" => Ok(Self::Icon),
            "license" => Ok(Self::License),
            "manifest" => Ok(Self::Manifest),
            "me" => Ok(Self::Me),
            "modulepreload" => Ok(Self::ModulePreload),
            "next" => Ok(Self::Next),
            "nofollow" => Ok(Self::NoFollow),
            "noopener" => Ok(Self::NoOpener),
            "noreferrer" => Ok(Self::NoReferrer),
            "opener" => Ok(Self::Opener),
            "pingback" => Ok(Self::PingBack),
            "preconnect" => Ok(Self::PreConnect),
            "prefetch" => Ok(Self::Prefetch),
            "preload" => Ok(Self::Preload),
            "prev" => Ok(Self::Prev),
            "privacy-policy" => Ok(Self::PrivacyPolicy),
            "search" => Ok(Self::Search),
            "stylesheet" => Ok(Self::StyleSheet),
            "tag" => Ok(Self::Tag),
            "terms-of-service" => Ok(Self::TermsOfService),
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

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Target::Self_ => "_self",
                Target::Parent => "parent",
                Target::Top => "top",
                Target::Blank => "blank",
            }
        )
    }
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
