use std::{convert::Infallible, fmt, str::FromStr};

use crate::element::{
    lang::LanguageTag,
    types::{
        AbsoluteLength, AbsoluteSize, Color, FontWeight, Length, LengthOrPercentage, Paint,
        Percentage, RelativeSize,
    },
};

use crate::element::types::ColorLiteral;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum AlignmentBaseline {
    #[default]
    Auto,
    Baseline,
    BeforeEdge,
    TextBeforeEdge,
    Middle,
    Central,
    AfterEdge,
    TextAfterEdge,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Top,
    Center,
    Bottom,
}

impl AlignmentBaseline {
    pub fn as_str(&self) -> &str {
        match self {
            AlignmentBaseline::Auto => "auto",
            AlignmentBaseline::Baseline => "baseline",
            AlignmentBaseline::BeforeEdge => "before-edge",
            AlignmentBaseline::TextBeforeEdge => "text-before-edge",
            AlignmentBaseline::Middle => "middle",
            AlignmentBaseline::Central => "central",
            AlignmentBaseline::AfterEdge => "after-edge",
            AlignmentBaseline::TextAfterEdge => "text-after-edge",
            AlignmentBaseline::Ideographic => "ideographic",
            AlignmentBaseline::Alphabetic => "alphabetic",
            AlignmentBaseline::Hanging => "hanging",
            AlignmentBaseline::Mathematical => "mathematical",
            AlignmentBaseline::Top => "top",
            AlignmentBaseline::Center => "center",
            AlignmentBaseline::Bottom => "bottom",
        }
    }
}

impl FromStr for AlignmentBaseline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(AlignmentBaseline::Auto),
            "baseline" => Ok(AlignmentBaseline::Baseline),
            "before-edge" => Ok(AlignmentBaseline::BeforeEdge),
            "text-before-edge" => Ok(AlignmentBaseline::TextBeforeEdge),
            "middle" => Ok(AlignmentBaseline::Middle),
            "central" => Ok(AlignmentBaseline::Central),
            "after-edge" => Ok(AlignmentBaseline::AfterEdge),
            "text-after-edge" => Ok(AlignmentBaseline::TextAfterEdge),
            "ideographic" => Ok(AlignmentBaseline::Ideographic),
            "alphabetic" => Ok(AlignmentBaseline::Alphabetic),
            "hanging" => Ok(AlignmentBaseline::Hanging),
            "mathematical" => Ok(AlignmentBaseline::Mathematical),
            "top" => Ok(AlignmentBaseline::Top),
            "center" => Ok(AlignmentBaseline::Center),
            "bottom" => Ok(AlignmentBaseline::Bottom),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BaselineShift {
    LengthOrPercentage(LengthOrPercentage),
    Sub,
    Super,
}

impl fmt::Display for BaselineShift {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BaselineShift::LengthOrPercentage(length_or_percentage) => {
                write!(f, "{}", length_or_percentage)
            }
            BaselineShift::Sub => write!(f, "sub"),
            BaselineShift::Super => write!(f, "super"),
        }
    }
}

impl Default for BaselineShift {
    fn default() -> Self {
        BaselineShift::LengthOrPercentage(LengthOrPercentage::default())
    }
}

impl FromStr for BaselineShift {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "sub" => Ok(BaselineShift::Sub),
            "super" => Ok(BaselineShift::Super),
            _ => Ok(BaselineShift::LengthOrPercentage(
                LengthOrPercentage::from_str(s)?,
            )),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ClipRule {
    #[default]
    NonZero,
    EvenOdd,
    Inherit,
}

impl ClipRule {
    pub fn as_str(&self) -> &str {
        match self {
            ClipRule::NonZero => "nonzero",
            ClipRule::EvenOdd => "evenodd",
            ClipRule::Inherit => "inherit",
        }
    }
}

impl FromStr for ClipRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nonzero" => Ok(ClipRule::NonZero),
            "evenodd" => Ok(ClipRule::EvenOdd),
            _ => Ok(ClipRule::Inherit),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ColorInterpolation {
    Auto,
    #[default]
    SRGB,
    LinearRGB,
}

impl ColorInterpolation {
    pub fn as_str(&self) -> &str {
        match self {
            ColorInterpolation::Auto => "auto",
            ColorInterpolation::SRGB => "sRGB",
            ColorInterpolation::LinearRGB => "linearRGB",
        }
    }
}

impl FromStr for ColorInterpolation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorInterpolation::Auto),
            "sRGB" => Ok(ColorInterpolation::SRGB),
            "linearRGB" => Ok(ColorInterpolation::LinearRGB),
            _ => Ok(ColorInterpolation::Auto),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum ColorInterpolationFilter {
    Auto,
    SRGB,
    #[default]
    LinearRGB,
}

impl ColorInterpolationFilter {
    pub fn as_str(&self) -> &str {
        match self {
            ColorInterpolationFilter::Auto => "auto",
            ColorInterpolationFilter::SRGB => "sRGB",
            ColorInterpolationFilter::LinearRGB => "linearRGB",
        }
    }
}

impl FromStr for ColorInterpolationFilter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ColorInterpolationFilter::Auto),
            "sRGB" => Ok(ColorInterpolationFilter::SRGB),
            "linearRGB" => Ok(ColorInterpolationFilter::LinearRGB),
            _ => Ok(ColorInterpolationFilter::Auto),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Cursor {
    #[default]
    Auto,
    Crosshair,
    Default,
    Pointer,
    Move,
    EResize,
    NEResize,
    NWResize,
    NResize,
    SEResize,
    SWResize,
    SResize,
    WResize,
    Text,
    Wait,
    Help,
    Inherit,
}

impl fmt::Display for Cursor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Cursor::Auto => "auto",
                Cursor::Crosshair => "crosshair",
                Cursor::Default => "default",
                Cursor::Pointer => "pointer",
                Cursor::Move => "move",
                Cursor::EResize => "e-resize",
                Cursor::NEResize => "ne-resize",
                Cursor::NWResize => "nw-resize",
                Cursor::NResize => "n-resize",
                Cursor::SEResize => "se-resize",
                Cursor::SWResize => "sw-resize",
                Cursor::SResize => "s-resize",
                Cursor::WResize => "w-resize",
                Cursor::Text => "text",
                Cursor::Wait => "wait",
                Cursor::Help => "help",
                Cursor::Inherit => "inherit",
            }
        )
    }
}

impl FromStr for Cursor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(Cursor::Auto),
            "crosshair" => Ok(Cursor::Crosshair),
            "default" => Ok(Cursor::Default),
            "pointer" => Ok(Cursor::Pointer),
            "move" => Ok(Cursor::Move),
            "e-resize" => Ok(Cursor::EResize),
            "ne-resize" => Ok(Cursor::NEResize),
            "nw-resize" => Ok(Cursor::NWResize),
            "n-resize" => Ok(Cursor::NResize),
            "se-resize" => Ok(Cursor::SEResize),
            "sw-resize" => Ok(Cursor::SWResize),
            "s-resize" => Ok(Cursor::SResize),
            "w-resize" => Ok(Cursor::WResize),
            "text" => Ok(Cursor::Text),
            "wait" => Ok(Cursor::Wait),
            "help" => Ok(Cursor::Help),
            "inherit" => Ok(Cursor::Inherit),
            _ => Err(()),
        }
    }
}

/// The value of the `cursor` attribute: zero or more `url()` fallbacks
/// followed by a required keyword value.
#[derive(Clone, Debug, PartialEq)]
pub struct CursorValue {
    pub urls: Vec<crate::element::types::Url>,
    pub keyword: Cursor,
}

impl fmt::Display for CursorValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for url in &self.urls {
            write!(f, "url({}), ", url)?;
        }
        write!(f, "{}", self.keyword)
    }
}

impl FromStr for CursorValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        let (url_parts, keyword_part) = parts.split_at(parts.len().saturating_sub(1));
        let keyword = keyword_part.first().ok_or(())?.parse::<Cursor>()?;
        let urls = url_parts
            .iter()
            .map(|p| {
                let inner = p
                    .strip_prefix("url(")
                    .and_then(|s| s.strip_suffix(')'))
                    .ok_or(())?;
                inner.parse::<crate::element::types::Url>()
            })
            .collect::<Result<_, _>>()?;
        Ok(CursorValue { urls, keyword })
    }
}

/// The value of the `clip-path` presentation attribute.
/// `none | url(#id) | <basic-shape> || <geometry-box>` (Raw for complex CSS shapes).
#[derive(Clone, Debug, PartialEq)]
pub enum ClipPathValue {
    None,
    Url(crate::element::types::Url),
    /// CSS basic shape or geometry box (e.g. `circle(50%)`, `inset(10px)`)
    Raw(String),
}

impl fmt::Display for ClipPathValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClipPathValue::None => write!(f, "none"),
            ClipPathValue::Url(url) => write!(f, "url({})", url),
            ClipPathValue::Raw(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for ClipPathValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("none") {
            return Ok(ClipPathValue::None);
        }
        if let Some(inner) = s.strip_prefix("url(").and_then(|s| s.strip_suffix(')')) {
            return Ok(ClipPathValue::Url(inner.parse()?));
        }
        Ok(ClipPathValue::Raw(s.to_string()))
    }
}

/// The value of the `filter` presentation attribute.
/// `none | <url> | <filter-function>+` (Raw for CSS filter functions).
#[derive(Clone, Debug, PartialEq)]
pub enum FilterValue {
    None,
    Url(crate::element::types::Url),
    /// CSS filter functions (e.g. `blur(4px)`, `brightness(50%) contrast(200%)`)
    Raw(String),
}

impl fmt::Display for FilterValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterValue::None => write!(f, "none"),
            FilterValue::Url(url) => write!(f, "url({})", url),
            FilterValue::Raw(s) => write!(f, "{}", s),
        }
    }
}

impl FromStr for FilterValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("none") {
            return Ok(FilterValue::None);
        }
        if let Some(inner) = s.strip_prefix("url(").and_then(|s| s.strip_suffix(')')) {
            return Ok(FilterValue::Url(inner.parse()?));
        }
        Ok(FilterValue::Raw(s.to_string()))
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum TextDirection {
    #[default]
    Ltr,
    Rtl,
}

impl fmt::Display for TextDirection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TextDirection::Ltr => write!(f, "ltr"),
            TextDirection::Rtl => write!(f, "rtl"),
        }
    }
}

impl FromStr for TextDirection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ltr" => Ok(TextDirection::Ltr),
            "rtl" => Ok(TextDirection::Rtl),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Display {
    #[default]
    Inline,
    Block,
    RunIn,
    Flow,
    FlowRoot,
    Table,
    Flex,
    Grid,
    Ruby,
    ListItem,
    TableRowGroup,
    TableHeaderGroup,
    TableFooterGroup,
    TableRow,
    TableCell,
    TableColumnGroup,
    TableColumn,
    TableCaption,
    RubyBase,
    RubyText,
    RubyBaseContainer,
    RubyTextContainer,
    Contents,
    None,
    InlineBlock,
    InlineTable,
    InlineFlex,
    InlineGrid,
}

impl fmt::Display for Display {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Display::Inline => "inline",
                Display::Block => "block",
                Display::RunIn => "run-in",
                Display::Flow => "flow",
                Display::FlowRoot => "flow-root",
                Display::Table => "table",
                Display::Flex => "flex",
                Display::Grid => "grid",
                Display::Ruby => "ruby",
                Display::ListItem => "list-item",
                Display::TableRowGroup => "table-row-group",
                Display::TableHeaderGroup => "table-header-group",
                Display::TableFooterGroup => "table-footer-group",
                Display::TableRow => "table-row",
                Display::TableCell => "table-cell",
                Display::TableColumnGroup => "table-column-group",
                Display::TableColumn => "table-column",
                Display::TableCaption => "table-caption",
                Display::RubyBase => "ruby-base",
                Display::RubyText => "ruby-text",
                Display::RubyBaseContainer => "ruby-base-container",
                Display::RubyTextContainer => "ruby-text-container",
                Display::Contents => "contents",
                Display::None => "none",
                Display::InlineBlock => "inline-block",
                Display::InlineTable => "inline-table",
                Display::InlineFlex => "inline-flex",
                Display::InlineGrid => "inline-grid",
            }
        )
    }
}

impl FromStr for Display {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inline" => Ok(Display::Inline),
            "block" => Ok(Display::Block),
            "run-in" => Ok(Display::RunIn),
            "flow" => Ok(Display::Flow),
            "flow-root" => Ok(Display::FlowRoot),
            "table" => Ok(Display::Table),
            "flex" => Ok(Display::Flex),
            "grid" => Ok(Display::Grid),
            "ruby" => Ok(Display::Ruby),
            "list-item" => Ok(Display::ListItem),
            "table-row-group" => Ok(Display::TableRowGroup),
            "table-header-group" => Ok(Display::TableHeaderGroup),
            "table-footer-group" => Ok(Display::TableFooterGroup),
            "table-row" => Ok(Display::TableRow),
            "table-cell" => Ok(Display::TableCell),
            "table-column-group" => Ok(Display::TableColumnGroup),
            "table-column" => Ok(Display::TableColumn),
            "table-caption" => Ok(Display::TableCaption),
            "ruby-base" => Ok(Display::RubyBase),
            "ruby-text" => Ok(Display::RubyText),
            "ruby-base-container" => Ok(Display::RubyBaseContainer),
            "ruby-text-container" => Ok(Display::RubyTextContainer),
            "contents" => Ok(Display::Contents),
            "none" => Ok(Display::None),
            "inline-block" => Ok(Display::InlineBlock),
            "inline-table" => Ok(Display::InlineTable),
            "inline-flex" => Ok(Display::InlineFlex),
            "inline-grid" => Ok(Display::InlineGrid),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DominantBaseline {
    #[default]
    Auto,
    TextBottom,
    Alphabetic,
    Ideographic,
    Middle,
    Central,
    Mathematical,
    Hanging,
    TextTop,
}

impl fmt::Display for DominantBaseline {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DominantBaseline::Auto => "auto",
                DominantBaseline::TextBottom => "text-bottom",
                DominantBaseline::Alphabetic => "alphabetic",
                DominantBaseline::Ideographic => "ideographic",
                DominantBaseline::Middle => "middle",
                DominantBaseline::Central => "central",
                DominantBaseline::Mathematical => "mathematical",
                DominantBaseline::Hanging => "hanging",
                DominantBaseline::TextTop => "text-top",
            }
        )
    }
}

impl FromStr for DominantBaseline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(DominantBaseline::Auto),
            "text-bottom" => Ok(DominantBaseline::TextBottom),
            "alphabetic" => Ok(DominantBaseline::Alphabetic),
            "ideographic" => Ok(DominantBaseline::Ideographic),
            "middle" => Ok(DominantBaseline::Middle),
            "central" => Ok(DominantBaseline::Central),
            "mathematical" => Ok(DominantBaseline::Mathematical),
            "hanging" => Ok(DominantBaseline::Hanging),
            "text-top" => Ok(DominantBaseline::TextTop),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Fill {
    Paint(Paint),
    Freeze,
    Remove,
    None,
}

impl fmt::Display for Fill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Fill::Paint(paint) => write!(f, "{}", paint),
            Fill::Freeze => write!(f, "freeze"),
            Fill::Remove => write!(f, "remove"),
            Fill::None => write!(f, "none"),
        }
    }
}

impl FromStr for Fill {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "remove" => Ok(Fill::Remove),
            "freeze" => Ok(Fill::Freeze),
            "none" => Ok(Fill::None),
            _ => Ok(Fill::Paint(Paint::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FillRule {
    #[default]
    NonZero,
    EvenOdd,
}

impl fmt::Display for FillRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FillRule::NonZero => write!(f, "nonzero"),
            FillRule::EvenOdd => write!(f, "evenodd"),
        }
    }
}

impl FromStr for FillRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nonzero" => Ok(FillRule::NonZero),
            "evenodd" => Ok(FillRule::EvenOdd),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontSize {
    Absolute(AbsoluteSize),
    Relative(RelativeSize),
    Length(Length),
    Percentage(Percentage),
}

impl fmt::Display for FontSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSize::Absolute(absolute_size) => write!(f, "{}", absolute_size),
            FontSize::Relative(relative_size) => write!(f, "{}", relative_size),
            FontSize::Length(length) => write!(f, "{}", length),
            FontSize::Percentage(percentage) => write!(f, "{}", percentage),
        }
    }
}

impl FromStr for FontSize {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(FontSize::Percentage(percentage));
        }

        if let Ok(absolute) = AbsoluteSize::from_str(s) {
            return Ok(FontSize::Absolute(absolute));
        }

        if let Ok(relative) = RelativeSize::from_str(s) {
            return Ok(FontSize::Relative(relative));
        }

        if let Ok(length) = Length::from_str(s) {
            return Ok(FontSize::Length(length));
        }

        return Err(());
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum FontSizeAdjust {
    None,
    Number(f64),
}

impl fmt::Display for FontSizeAdjust {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontSizeAdjust::None => write!(f, "none"),
            FontSizeAdjust::Number(number) => write!(f, "{}", number),
        }
    }
}

impl FromStr for FontSizeAdjust {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(FontSizeAdjust::None),
            _ => Ok(FontSizeAdjust::Number(s.parse().map_err(|_| ())?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum FontStyle {
    #[default]
    Normal,
    Italic,
    Oblique,
}

impl fmt::Display for FontStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FontStyle::Normal => write!(f, "normal"),
            FontStyle::Italic => write!(f, "italic"),
            FontStyle::Oblique => write!(f, "oblique"),
        }
    }
}

impl FromStr for FontStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(FontStyle::Normal),
            "italic" => Ok(FontStyle::Italic),
            "oblique" => Ok(FontStyle::Oblique),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ImageRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    OptimizeQuality,
}

impl fmt::Display for ImageRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageRendering::Auto => "auto",
                ImageRendering::OptimizeSpeed => "optimizeSpeed",
                ImageRendering::OptimizeQuality => "optimizeQuality",
            }
        )
    }
}

impl FromStr for ImageRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ImageRendering::Auto),
            "optimizeSpeed" => Ok(ImageRendering::OptimizeSpeed),
            "optimizeQuality" => Ok(ImageRendering::OptimizeQuality),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum LetterSpacing {
    #[default]
    Normal,
    Length(Length),
}

impl fmt::Display for LetterSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LetterSpacing::Normal => write!(f, "normal"),
            LetterSpacing::Length(length) => write!(f, "{}", length),
        }
    }
}

impl FromStr for LetterSpacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(LetterSpacing::Normal),
            _ => Ok(LetterSpacing::Length(Length::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LightingColor(pub Color);

impl FromStr for LightingColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LightingColor(Color::from_str(s)?))
    }
}

impl Default for LightingColor {
    fn default() -> Self {
        LightingColor(Color::Literal(ColorLiteral::Black))
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Marker {
    None,
    Url(crate::element::types::Url),
}

impl fmt::Display for Marker {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Marker::None => write!(f, "none"),
            Marker::Url(url) => write!(f, "url({})", url),
        }
    }
}

impl FromStr for Marker {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.eq_ignore_ascii_case("none") {
            return Ok(Marker::None);
        }

        Ok(Marker::Url(s.parse()?))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum MaskType {
    Alpha,
    #[default]
    Luminance,
}

impl fmt::Display for MaskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MaskType::Alpha => write!(f, "alpha"),
            MaskType::Luminance => write!(f, "luminance"),
        }
    }
}

impl FromStr for MaskType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "alpha" => Ok(MaskType::Alpha),
            "luminance" => Ok(MaskType::Luminance),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Opacity(pub f64);

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Opacity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Opacity(s.parse().map_err(|_| ())?))
    }
}

impl Default for Opacity {
    fn default() -> Self {
        Opacity(1.0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Overflow {
    #[default]
    Visible,
    Hidden,
    Scroll,
    Auto,
}

impl fmt::Display for Overflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Overflow::Visible => write!(f, "visible"),
            Overflow::Hidden => write!(f, "hidden"),
            Overflow::Scroll => write!(f, "scroll"),
            Overflow::Auto => write!(f, "auto"),
        }
    }
}

impl FromStr for Overflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "visible" => Ok(Overflow::Visible),
            "hidden" => Ok(Overflow::Hidden),
            "scroll" => Ok(Overflow::Scroll),
            "auto" => Ok(Overflow::Auto),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum PointerEvents {
    BoundingBox,
    #[default]
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    All,
    None,
}

impl fmt::Display for PointerEvents {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointerEvents::BoundingBox => write!(f, "bounding-box"),
            PointerEvents::VisiblePainted => write!(f, "visiblePainted"),
            PointerEvents::VisibleFill => write!(f, "visibleFill"),
            PointerEvents::VisibleStroke => write!(f, "visibleStroke"),
            PointerEvents::Visible => write!(f, "visible"),
            PointerEvents::Painted => write!(f, "painted"),
            PointerEvents::Fill => write!(f, "fill"),
            PointerEvents::Stroke => write!(f, "stroke"),
            PointerEvents::All => write!(f, "all"),
            PointerEvents::None => write!(f, "none"),
        }
    }
}

impl FromStr for PointerEvents {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bounding-box" => Ok(PointerEvents::BoundingBox),
            "visiblePainted" => Ok(PointerEvents::VisiblePainted),
            "visibleFill" => Ok(PointerEvents::VisibleFill),
            "visibleStroke" => Ok(PointerEvents::VisibleStroke),
            "visible" => Ok(PointerEvents::Visible),
            "painted" => Ok(PointerEvents::Painted),
            "fill" => Ok(PointerEvents::Fill),
            "stroke" => Ok(PointerEvents::Stroke),
            "all" => Ok(PointerEvents::All),
            "none" => Ok(PointerEvents::None),
            _ => Err(()),
        }
    }
}

/// NOTE: This is called EllipsisRadius, but it can be applied to both ellipse and a rect
#[derive(Clone, Debug, PartialEq, Default)]
pub enum EllipsisRadius {
    LengthOrPercentage(LengthOrPercentage),
    #[default]
    Auto,
}

impl fmt::Display for EllipsisRadius {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EllipsisRadius::LengthOrPercentage(length_or_percentage) => {
                write!(f, "{}", length_or_percentage)
            }
            EllipsisRadius::Auto => write!(f, "auto"),
        }
    }
}

impl FromStr for EllipsisRadius {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(EllipsisRadius::Auto),
            _ => Ok(EllipsisRadius::LengthOrPercentage(
                LengthOrPercentage::from_str(s)?,
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum ShapeRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    CrispEdges,
    GeometricPrecision,
}

impl fmt::Display for ShapeRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ShapeRendering::Auto => "auto",
                ShapeRendering::OptimizeSpeed => "optimizeSpeed",
                ShapeRendering::CrispEdges => "crispEdges",
                ShapeRendering::GeometricPrecision => "geometricPrecision",
            }
        )
    }
}

impl FromStr for ShapeRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(ShapeRendering::Auto),
            "optimizeSpeed" => Ok(ShapeRendering::OptimizeSpeed),
            "crispEdges" => Ok(ShapeRendering::CrispEdges),
            "geometricPrecision" => Ok(ShapeRendering::GeometricPrecision),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StopColor(pub Color);

impl fmt::Display for StopColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for StopColor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(StopColor(Color::from_str(s)?))
    }
}

impl Default for StopColor {
    fn default() -> Self {
        StopColor(Color::Literal(ColorLiteral::Black))
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum StrokeLinecap {
    #[default]
    Butt,
    Round,
    Square,
}

impl fmt::Display for StrokeLinecap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StrokeLinecap::Butt => "butt",
                StrokeLinecap::Round => "round",
                StrokeLinecap::Square => "square",
            }
        )
    }
}

impl FromStr for StrokeLinecap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "butt" => Ok(StrokeLinecap::Butt),
            "round" => Ok(StrokeLinecap::Round),
            "square" => Ok(StrokeLinecap::Square),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum StrokeLinejoin {
    Arcs,
    Bevel,
    #[default]
    Miter,
    MiterClip,
    Round,
}

impl fmt::Display for StrokeLinejoin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                StrokeLinejoin::Arcs => "arcs",
                StrokeLinejoin::Bevel => "bevel",
                StrokeLinejoin::Miter => "miter",
                StrokeLinejoin::MiterClip => "miter-clip",
                StrokeLinejoin::Round => "round",
            }
        )
    }
}

impl FromStr for StrokeLinejoin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arcs" => Ok(StrokeLinejoin::Arcs),
            "bevel" => Ok(StrokeLinejoin::Bevel),
            "miter" => Ok(StrokeLinejoin::Miter),
            "miter-clip" => Ok(StrokeLinejoin::MiterClip),
            "round" => Ok(StrokeLinejoin::Round),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum StrokeOpacity {
    Number(f64),
    Percentage(Percentage),
}

impl fmt::Display for StrokeOpacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StrokeOpacity::Number(number) => write!(f, "{}", number),
            StrokeOpacity::Percentage(percentage) => write!(f, "{}", percentage),
        }
    }
}

impl FromStr for StrokeOpacity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(StrokeOpacity::Percentage(percentage));
        }

        Ok(StrokeOpacity::Number(s.parse().map_err(|_| ())?))
    }
}

impl Default for StrokeOpacity {
    fn default() -> Self {
        StrokeOpacity::Number(1.0)
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum VectorEffect {
    #[default]
    None,
    NonScalingStroke,
    NonScalingSize,
    NonRotation,
    FixedPosition,
}

impl fmt::Display for VectorEffect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VectorEffect::None => "none",
                VectorEffect::NonScalingStroke => "non-scaling-stroke",
                VectorEffect::NonScalingSize => "non-scaling-size",
                VectorEffect::NonRotation => "non-rotation",
                VectorEffect::FixedPosition => "fixeed-position",
            }
        )
    }
}

impl FromStr for VectorEffect {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(VectorEffect::None),
            "non-scaling-stroke" => Ok(VectorEffect::NonScalingStroke),
            "non-scaling-size" => Ok(VectorEffect::NonScalingSize),
            "non-rotation" => Ok(VectorEffect::NonRotation),
            "fixed-position" => Ok(VectorEffect::FixedPosition),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}

impl fmt::Display for TextAnchor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextAnchor::Start => "start",
                TextAnchor::Middle => "middle",
                TextAnchor::End => "end",
            }
        )
    }
}

impl FromStr for TextAnchor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "start" => Ok(TextAnchor::Start),
            "middle" => Ok(TextAnchor::Middle),
            "end" => Ok(TextAnchor::End),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextOverflow {
    #[default]
    Clip,
    Ellipsis,
}

impl fmt::Display for TextOverflow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextOverflow::Clip => "clip",
                TextOverflow::Ellipsis => "ellipsis",
            }
        )
    }
}

impl FromStr for TextOverflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "clip" => Ok(TextOverflow::Clip),
            "ellipsis" => Ok(TextOverflow::Ellipsis),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum TextRendering {
    #[default]
    Auto,
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

impl fmt::Display for TextRendering {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                TextRendering::Auto => "auto",
                TextRendering::OptimizeSpeed => "optimizeSpeed",
                TextRendering::OptimizeLegibility => "optimizeLegibility",
                TextRendering::GeometricPrecision => "geometricPrecision",
            }
        )
    }
}

impl FromStr for TextRendering {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auto" => Ok(TextRendering::Auto),
            "optimizeSpeed" => Ok(TextRendering::OptimizeSpeed),
            "optimizeLegibility" => Ok(TextRendering::OptimizeLegibility),
            "geometricPrecision" => Ok(TextRendering::GeometricPrecision),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum UnicodeBidi {
    #[default]
    Normal,
    Embed,
    Isolate,
    BidiOverride,
    IsolateOverride,
    Plaintext,
}

impl fmt::Display for UnicodeBidi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                UnicodeBidi::Normal => "normal",
                UnicodeBidi::Embed => "embed",
                UnicodeBidi::Isolate => "isolate",
                UnicodeBidi::BidiOverride => "bidi-override",
                UnicodeBidi::IsolateOverride => "isolate-override",
                UnicodeBidi::Plaintext => "plaintext",
            }
        )
    }
}

impl FromStr for UnicodeBidi {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(UnicodeBidi::Normal),
            "embed" => Ok(UnicodeBidi::Embed),
            "isolate" => Ok(UnicodeBidi::Isolate),
            "bidi-override" => Ok(UnicodeBidi::BidiOverride),
            "isolate-override" => Ok(UnicodeBidi::IsolateOverride),
            "plaintext" => Ok(UnicodeBidi::Plaintext),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
    Collapse,
}

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Visibility::Visible => "visible",
                Visibility::Hidden => "hidden",
                Visibility::Collapse => "collapse",
            }
        )
    }
}

impl FromStr for Visibility {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "visible" => Ok(Visibility::Visible),
            "hidden" => Ok(Visibility::Hidden),
            "collapse" => Ok(Visibility::Collapse),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WhiteSpace {
    #[default]
    Normal,
    Pre,
    Nowrap,
    PreWrap,
    BreakSpace,
    PreLine,
}

impl fmt::Display for WhiteSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WhiteSpace::Normal => "normal",
                WhiteSpace::Pre => "pre",
                WhiteSpace::Nowrap => "nowrap",
                WhiteSpace::PreWrap => "pre-wrap",
                WhiteSpace::BreakSpace => "break-space",
                WhiteSpace::PreLine => "pre-line",
            }
        )
    }
}

impl FromStr for WhiteSpace {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(WhiteSpace::Normal),
            "pre" => Ok(WhiteSpace::Pre),
            "nowrap" => Ok(WhiteSpace::Nowrap),
            "pre-wrap" => Ok(WhiteSpace::PreWrap),
            "break-space" => Ok(WhiteSpace::BreakSpace),
            "pre-line" => Ok(WhiteSpace::PreLine),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WordSpacing {
    #[default]
    Normal,
    Length(Length),
}

impl fmt::Display for WordSpacing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WordSpacing::Normal => write!(f, "normal"),
            WordSpacing::Length(length) => write!(f, "{}", length),
        }
    }
}

impl FromStr for WordSpacing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "normal" => Ok(WordSpacing::Normal),
            _ => Ok(WordSpacing::Length(Length::from_str(s)?)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum WritingMode {
    #[default]
    HorizontalTb,
    VerticalRl,
    VerticalLr,
}

impl fmt::Display for WritingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WritingMode::HorizontalTb => "horizontal-tb",
                WritingMode::VerticalRl => "vertical-rl",
                WritingMode::VerticalLr => "vertical-lr",
            }
        )
    }
}

impl FromStr for WritingMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "horizontal-tb" => Ok(WritingMode::HorizontalTb),
            "vertical-rl" => Ok(WritingMode::VerticalRl),
            "vertical-lr" => Ok(WritingMode::VerticalLr),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Rotate {
    Auto,
    AutoReverse,
    Number(f64),
}

impl fmt::Display for Rotate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rotate::Auto => write!(f, "auto"),
            Rotate::AutoReverse => write!(f, "auto-reverse"),
            Rotate::Number(number) => write!(f, "{}", number),
        }
    }
}

impl Default for Rotate {
    fn default() -> Self {
        Rotate::Number(0.0)
    }
}

impl FromStr for Rotate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = f64::from_str(s) {
            return Ok(Rotate::Number(number));
        }

        match s {
            "auto" => Ok(Rotate::Auto),
            "auto-reverse" => Ok(Rotate::AutoReverse),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum LengthOrPercentageOrNumber {
    Length(Length),
    Percentage(Percentage),
    Number(f64),
}

impl fmt::Display for LengthOrPercentageOrNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LengthOrPercentageOrNumber::Length(length) => write!(f, "{}", length),
            LengthOrPercentageOrNumber::Percentage(percentage) => write!(f, "{}", percentage),
            LengthOrPercentageOrNumber::Number(number) => write!(f, "{}", number),
        }
    }
}

impl FromStr for LengthOrPercentageOrNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(percentage) = Percentage::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Percentage(percentage));
        }

        if let Ok(length) = Length::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Length(length));
        }

        if let Ok(number) = f64::from_str(s) {
            return Ok(LengthOrPercentageOrNumber::Number(number));
        }

        Err(())
    }
}
