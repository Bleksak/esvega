use std::fmt;
use std::str::FromStr;

use super::shared::{
    write_comma_separated, write_space_separated, write_semicolon_separated,
};
use super::{
    animation::{
        AnimationAccumulate, AnimationAdditive, AnimationAttributeType, AnimationRestart,
        BeginEndValue, CalcMode, ClockValue, Decoding, DurValue, FeFuncType, KeyPoint, KeySpline,
        LengthAdjust, Method, RepeatCount, Side, Spacing, FetchPriority,
    },
    filter::{
        BlendMode, ChannelSelector, EdgeMode, FilterUnits, In, MaskUnits, Operator, PrimitiveUnits,
        RefX, RefY,
    },
    gradient::{
        ClipPathUnits, CrossOrigin, GradientUnits, MarkerUnits, Orient, PreserveAspectRatio,
        SpreadMethod, StitchTiles, ViewBox,
    },
    presentation::{
        AlignmentBaseline, BaselineShift, ClipPathValue, ClipRule, ColorInterpolation,
        ColorInterpolationFilter, CursorValue, Display, DominantBaseline, EllipsisRadius, Fill,
        FillRule, FilterValue, FontSize, FontSizeAdjust, FontStyle, ImageRendering, LetterSpacing,
        LightingColor, Marker, MaskType, Opacity, Overflow, PointerEvents, Rotate, ShapeRendering,
        StopColor, StrokeLinecap, StrokeLinejoin, StrokeOpacity, TextAnchor, TextDirection,
        TextOverflow, TextRendering, UnicodeBidi, Visibility, VectorEffect, WhiteSpace,
        WordSpacing, WritingMode, LengthOrPercentageOrNumber,
    },
    path::{Path, Point},
    link_media::{ReferrerPolicy, RelType, Target},
};
use crate::element::{
    ElementType,
    lang::LanguageTag,
    types::{AbsoluteLength, Color, Length, LengthOrPercentage, Paint, Percentage, Url},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Attribute {
    // Core(Global) Attributes
    Xmlns(String),
    Autofocus(bool),
    Id(String),
    Class(Vec<String>),
    Style(String),
    Lang(LanguageTag),
    Tabindex(i64),

    // Conditional Processing Attributes
    RequiredExtensions(Vec<String>),
    SystemLanguage(Vec<LanguageTag>),

    // Presentation Attributes
    AlignmentBaseline(AlignmentBaseline),
    BaselineShift(BaselineShift),
    ClipPath(ClipPathValue),
    ClipRule(ClipRule),
    Color(Color),
    ColorInterpolation(ColorInterpolation),
    ColorInterpolationFilters(ColorInterpolationFilter),
    Cursor(CursorValue),
    Cx(LengthOrPercentage),
    Cy(LengthOrPercentage),
    D(Path),
    Direction(TextDirection),
    Display(Display),
    DominantBaseline(DominantBaseline),
    Fill(Fill),
    FillOpacity(Percentage),
    FillRule(FillRule),
    Filter(FilterValue),
    FloodColor(Color),
    FloodOpacity(f64),
    FontFamily(String),
    FontSize(FontSize),
    FontSizeAdjust(FontSizeAdjust),
    FontStyle(FontStyle),
    FontVariant(String),
    FontWeight(crate::element::types::FontWeight),
    Height(LengthOrPercentageOrNumber),
    ImageRendering(ImageRendering),
    LetterSpacing(LetterSpacing),
    LightingColor(LightingColor),
    MarkerEnd(Marker),
    MarkerMid(Marker),
    MarkerStart(Marker),
    Mask(String),
    MaskType(MaskType),
    Opacity(Opacity),
    Overflow(Overflow),
    PointerEvents(PointerEvents),
    R(LengthOrPercentageOrNumber),
    Rx(EllipsisRadius),
    Ry(EllipsisRadius),
    ShapeRendering(ShapeRendering),
    StopColor(StopColor),
    StopOpacity(Opacity),
    Stroke(Paint),
    StrokeDasharray(Vec<i64>),
    StrokeDashoffset(LengthOrPercentage),
    StrokeLinecap(StrokeLinecap),
    StrokeLinejoin(StrokeLinejoin),
    StrokeMiterlimit(f64),
    StrokeOpacity(StrokeOpacity),
    StrokeWidth(LengthOrPercentage),
    TextAnchor(TextAnchor),
    TextDecoration(String),
    TextOverflow(TextOverflow),
    TextRendering(TextRendering),
    Transform(String),
    TransformOrigin(String),
    UnicodeBidi(UnicodeBidi),
    VectorEffect(VectorEffect),
    Visibility(Visibility),
    Width(LengthOrPercentageOrNumber),
    WhiteSpace(WhiteSpace),
    WordSpacing(WordSpacing),
    WritingMode(WritingMode),
    X(LengthOrPercentageOrNumber),
    Y(LengthOrPercentageOrNumber),

    // Transfer Function Attributes
    Type(FeFuncType),
    TableValues(Vec<f64>),
    Slope(f64),
    Intercept(f64),
    Amplitude(f64),
    Exponent(f64),
    Offset(f64),

    // Animation Attributes
    Href(String),
    AttributeType(AnimationAttributeType),
    AttributeName(String),
    Begin(Vec<BeginEndValue>),
    Dur(DurValue),
    End(Vec<BeginEndValue>),
    Min(ClockValue),
    Max(DurValue),
    Restart(AnimationRestart),
    RepeatCount(RepeatCount),
    RepeatDur(DurValue),
    Additive(AnimationAdditive),
    Accumulate(AnimationAccumulate),

    // Event Attributes
    OnAfterPrint(String),
    OnBeforePrint(String),
    OnBeforeUnload(String),
    OnError(String),
    OnHashChange(String),
    OnLoad(String),
    OnMessage(String),
    OnOffline(String),
    OnOnline(String),
    OnPageHide(String),
    OnPageShow(String),
    OnPopState(String),
    OnResize(String),
    OnStorage(String),
    OnUnload(String),
    OnBlur(String),
    OnChange(String),
    OnContextMenu(String),
    OnFocus(String),
    OnInput(String),
    OnInvalid(String),
    OnReset(String),
    OnSearch(String),
    OnSelect(String),
    OnSubmit(String),
    OnKeyDown(String),
    OnKeyPress(String),
    OnKeyUp(String),
    OnClick(String),
    OnDoubleClick(String),
    OnMouseDown(String),
    OnMouseMove(String),
    OnMouseOut(String),
    OnMouseOver(String),
    OnMouseUp(String),
    OnWheel(String),
    OnDrag(String),
    OnDragEnd(String),
    OnDragEnter(String),
    OnDragLeave(String),
    OnDragOver(String),
    OnDragStart(String),
    OnDrop(String),
    OnScroll(String),
    OnCopy(String),
    OnCut(String),
    OnPaste(String),
    OnAbort(String),
    OnCanPlay(String),
    OnCanPlayThrough(String),
    OnCueChange(String),
    OnDurationChange(String),
    OnEmptied(String),
    OnEnded(String),
    OnLoadedData(String),
    OnLoadedMetadata(String),
    OnLoadStart(String),
    OnPause(String),
    OnPlay(String),
    OnPlaying(String),
    OnProgress(String),
    OnRateChange(String),
    OnSeeked(String),
    OnSeeking(String),
    OnStalled(String),
    OnSuspend(String),
    OnTimeUpdate(String),
    OnVolumeChange(String),
    OnWaiting(String),
    OnToggle(String),

    // Animation Event Attributes
    OnBegin(String),
    OnEnd(String),
    OnRepeat(String),

    // Element Specific
    KeyPoints(Vec<KeyPoint>),
    Path(Path),
    Rotate(Rotate),

    // Animation Value Attributes
    CalcMode(CalcMode),
    Values(Vec<String>),
    KeyTimes(Vec<f64>),
    KeySplines(Vec<KeySpline>),
    From(String),
    To(String),
    By(String),
    PathLength(f64),

    // Gradient/Shape coordinates
    X1(LengthOrPercentageOrNumber),
    Y1(LengthOrPercentageOrNumber),
    X2(LengthOrPercentageOrNumber),
    Y2(LengthOrPercentageOrNumber),
    Points(Vec<Point>),

    // Link attributes
    Download(String),
    HrefLang(LanguageTag),
    InterestFor(String),
    Ping(Vec<Url>),
  ReferrerPolicy(ReferrerPolicy),
   Rel(Vec<RelType>),
   Target(Target),

    // Marker attributes
    MarkerHeight(LengthOrPercentage),
    MarkerUnits(MarkerUnits),
    MarkerWidth(LengthOrPercentage),
    Orient(Orient),
    PreserveAspectRatio(PreserveAspectRatio),
    RefX(RefX),
    RefY(RefY),
    ViewBox(ViewBox),

    // Mask attributes
    MaskContentUnits(MaskContentUnits),
    MaskUnits(MaskUnits),

    // Pattern attributes
    PatternContentUnits(PatternContentUnits),
    PatternUnits(PatternUnits),
    PatternTransform(String),

    // Filter attributes
    Result(String),
    In(In),
    In2(In),
    Mode(BlendMode),
    Operator(Operator),
    K1(f64),
    K2(f64),
    K3(f64),
    K4(f64),
    Order(u32),
    KernelMatrix(Vec<f64>),
    Divisor(f64),
    Bias(f64),
    TargetX(i64),
    TargetY(i64),
    EdgeMode(EdgeMode),
    KernelUnitLength(f64, Option<f64>),
    PreserveAlpha(bool),
    SurfaceScale(f64),
    DiffuseConstant(f64),
    Scale(f64),
    XChannelSelector(ChannelSelector),
    YChannelSelector(ChannelSelector),
    Dx(f64),
    Dy(f64),
    StdDeviation(f64, Option<f64>),
    CrossOrigin(CrossOrigin),
    Radius(f64, Option<f64>),
    SpecularConstant(f64),
    SpecularExponent(f64),
    Azimuth(f64),
    Elevation(f64),
    Z(f64),
    PointsAtX(f64),
    PointsAtY(f64),
    PointsAtZ(f64),
    LimitingConeAngle(f64),
    BaseFrequency(f64, Option<f64>),
    NumOctaves(u64),
    Seed(f64),
    StitchTiles(StitchTiles),

    // Gradient attributes
    GradientUnits(GradientUnits),
    GradientTransform(String),
    SpreadMethod(SpreadMethod),

    // Radial gradient attributes
    Fx(LengthOrPercentage),
    Fy(LengthOrPercentage),
    Fr(LengthOrPercentage),

    // Text path attributes
    LengthAdjust(LengthAdjust),
    TextLength(LengthOrPercentage),
    ClipPathUnits(ClipPathUnits),
    Method(Method),
    Side(Side),
    Spacing(Spacing),
    StartOffset(LengthOrPercentageOrNumber),

    // SVG attributes
    FilterUnits(FilterUnits),
    PrimitiveUnits(PrimitiveUnits),
    Version(f64),

    // Media attributes
    Decoding(Decoding),
    FetchPriority(FetchPriority),
}

use super::filter::{MaskContentUnits, PatternContentUnits, PatternUnits};

impl TryFrom<(&String, &String)> for Attribute {
    type Error = ();

    fn try_from((key, value): (&String, &String)) -> Result<Self, Self::Error> {
        match key.as_str() {
            "xmlns" => Ok(Attribute::Xmlns(value.clone())),
            "version" => Ok(Attribute::Version(value.parse().map_err(|_| ())?)),
            "autofocus" => {
                if value.is_empty() || value.eq_ignore_ascii_case("autofocus") {
                    Ok(Attribute::Autofocus(true))
                } else {
                    Ok(Attribute::Autofocus(false))
                }
            }
            "id" => Ok(Attribute::Id(value.clone())),
            "class" => Ok(Attribute::Class(
                value.split_whitespace().map(|class| class.to_string()).collect(),
            )),
            "style" => Ok(Attribute::Style(value.clone())),
            "lang" => Ok(Attribute::Lang(value.parse()?)),
            "tabindex" => Ok(Attribute::Tabindex(value.parse().unwrap_or(0))),
            "requiredExtensions" => {
                let extensions = value
                    .split_whitespace()
                    .map(|ext| ext.to_string())
                    .collect();
                Ok(Attribute::RequiredExtensions(extensions))
            }
            "systemLanguage" => Ok(Attribute::SystemLanguage(
                value.split(',').map(|s| s.trim().parse()).collect::<Result<_, _>>()?,
            )),
            "alignment-baseline" => Ok(Attribute::AlignmentBaseline(value.parse()?)),
            "baseline-shift" => Ok(Attribute::BaselineShift(value.parse()?)),
            "clip-path" => Ok(Attribute::ClipPath(value.parse()?)),
            "clip-rule" => Ok(Attribute::ClipRule(value.parse()?)),
            "color" => Ok(Attribute::Color(value.parse()?)),
            "color-interpolation" => Ok(Attribute::ColorInterpolation(value.parse()?)),
            "color-interpolation-filters" => {
                Ok(Attribute::ColorInterpolationFilters(value.parse()?))
            }
            "cursor" => Ok(Attribute::Cursor(value.parse::<CursorValue>()?)),
            "cx" => Ok(Attribute::Cx(value.parse()?)),
            "cy" => Ok(Attribute::Cy(value.parse()?)),
            "d" => Ok(Attribute::D(value.parse()?)),
            "direction" => Ok(Attribute::Direction(value.parse()?)),
            "display" => Ok(Attribute::Display(value.parse()?)),
            "dominant-baseline" => Ok(Attribute::DominantBaseline(value.parse()?)),
            "fill" => Ok(Attribute::Fill(value.parse()?)),
            "fill-opacity" => Ok(Attribute::FillOpacity(value.parse()?)),
            "fill-rule" => Ok(Attribute::FillRule(value.parse()?)),
            "filter" => Ok(Attribute::Filter(value.parse()?)),
            "flood-color" => Ok(Attribute::FloodColor(value.parse()?)),
            "flood-opacity" => Ok(Attribute::FloodOpacity(value.parse().map_err(|_| ())?)),
            "font-family" => Ok(Attribute::FontFamily(value.clone())),
            "font-size" => Ok(Attribute::FontSize(value.parse().map_err(|_| ())?)),
            "font-size-adjust" => Ok(Attribute::FontSizeAdjust(value.parse()?)),
            "font-style" => Ok(Attribute::FontStyle(value.parse()?)),
            "font-variant" => Ok(Attribute::FontVariant(value.clone())),
            "font-weight" => Ok(Attribute::FontWeight(value.parse()?)),
            "height" => Ok(Attribute::Height(value.parse().unwrap_or(
                LengthOrPercentageOrNumber::Length(Length::Absolute(AbsoluteLength::Px(1.0))),
            ))),
            "image-rendering" => Ok(Attribute::ImageRendering(value.parse()?)),
            "letter-spacing" => Ok(Attribute::LetterSpacing(value.parse()?)),
            "lighting-color" => Ok(Attribute::LightingColor(value.parse()?)),
            "marker-end" => Ok(Attribute::MarkerEnd(value.parse()?)),
            "marker-mid" => Ok(Attribute::MarkerMid(value.parse()?)),
            "marker-start" => Ok(Attribute::MarkerStart(value.parse()?)),
            "mask" => Ok(Attribute::Mask(value.clone())),
            "mask-type" => Ok(Attribute::MaskType(value.parse()?)),
            "opacity" => Ok(Attribute::Opacity(value.parse()?)),
            "overflow" => Ok(Attribute::Overflow(value.parse()?)),
            "pointer-events" => Ok(Attribute::PointerEvents(value.parse()?)),
            "r" => Ok(Attribute::R(value.parse()?)),
            "order" => Ok(Attribute::Order(
                value.parse::<f64>().map(|v| v as u32).map_err(|_| ())?,
            )),
            "rx" => Ok(Attribute::Rx(value.parse()?)),
            "ry" => Ok(Attribute::Ry(value.parse()?)),
            "shape-rendering" => Ok(Attribute::ShapeRendering(value.parse()?)),
            "stop-color" => Ok(Attribute::StopColor(value.parse()?)),
            "stop-opacity" => Ok(Attribute::StopOpacity(value.parse()?)),
            "stroke" => Ok(Attribute::Stroke(value.parse()?)),
            "stroke-dasharray" => Ok(Attribute::StrokeDasharray(
                value
                    .split_whitespace()
                    .map(i64::from_str)
                    .collect::<Result<_, _>>()
                    .map_err(|_| ())?,
            )),
            "stroke-dashoffset" => Ok(Attribute::StrokeDashoffset(value.parse()?)),
            "stroke-linecap" => Ok(Attribute::StrokeLinecap(value.parse()?)),
            "stroke-linejoin" => Ok(Attribute::StrokeLinejoin(value.parse()?)),
            "stroke-miterlimit" => Ok(Attribute::StrokeMiterlimit(value.parse().unwrap_or(4.0))),
            "stroke-opacity" => Ok(Attribute::StrokeOpacity(value.parse()?)),
            "stroke-width" => Ok(Attribute::StrokeWidth(value.parse()?)),
            "text-anchor" => Ok(Attribute::TextAnchor(value.parse()?)),
            "text-decoration" => Ok(Attribute::TextDecoration(value.clone())),
            "text-overflow" => Ok(Attribute::TextOverflow(value.parse()?)),
            "text-rendering" => Ok(Attribute::TextRendering(value.parse()?)),
            "transform" => Ok(Attribute::Transform(value.clone())),
            "transform-origin" => Ok(Attribute::TransformOrigin(value.clone())),
            "unicode-bidi" => Ok(Attribute::UnicodeBidi(value.parse()?)),
            "vector-effect" => Ok(Attribute::VectorEffect(value.parse()?)),
            "viewBox" => Ok(Attribute::ViewBox(value.parse()?)),
            "visibility" => Ok(Attribute::Visibility(value.parse()?)),
            "width" => Ok(Attribute::Width(value.parse().unwrap_or(
                LengthOrPercentageOrNumber::Length(Length::Absolute(AbsoluteLength::Px(1.0))),
            ))),
            "white-space" => Ok(Attribute::WhiteSpace(value.parse()?)),
            "word-spacing" => Ok(Attribute::WordSpacing(value.parse()?)),
            "writing-mode" => Ok(Attribute::WritingMode(value.parse()?)),
            "x" => Ok(Attribute::X(value.parse().unwrap_or(
                LengthOrPercentageOrNumber::Length(Length::Absolute(AbsoluteLength::Px(0.0))),
            ))),
            "y" => Ok(Attribute::Y(value.parse().unwrap_or(
                LengthOrPercentageOrNumber::Length(Length::Absolute(AbsoluteLength::Px(0.0))),
            ))),
            "type" => Ok(Attribute::Type(value.parse()?)),
            "tableValues" => Ok(Attribute::TableValues(
                value
                    .split_whitespace()
                    .map(|s| s.parse::<f64>().map_err(|_| ()))
                    .collect::<Result<_, _>>()?,
            )),
            "slope" => Ok(Attribute::Slope(value.parse::<f64>().map_err(|_| ())?)),
            "intercept" => Ok(Attribute::Intercept(value.parse::<f64>().map_err(|_| ())?)),
            "amplitude" => Ok(Attribute::Amplitude(value.parse::<f64>().map_err(|_| ())?)),
            "exponent" => Ok(Attribute::Exponent(value.parse::<f64>().map_err(|_| ())?)),
            "offset" => Ok(Attribute::Offset(value.parse::<f64>().map_err(|_| ())?)),
            "href" => Ok(Attribute::Href(value.clone())),
            "hreflang" => Ok(Attribute::HrefLang(value.parse()?)),
            "attributeType" => Ok(Attribute::AttributeType(value.parse()?)),
            "attributeName" => Ok(Attribute::AttributeName(value.clone())),
            "begin" => Ok(Attribute::Begin(
                value.split(';').map(|s| s.parse()).collect::<Result<_, _>>()?,
            )),
            "dur" => Ok(Attribute::Dur(value.parse()?)),
            "end" => Ok(Attribute::End(
                value.split(';').map(|s| s.parse()).collect::<Result<_, _>>()?,
            )),
            "min" => Ok(Attribute::Min(value.parse()?)),
            "max" => Ok(Attribute::Max(value.parse()?)),
            "restart" => Ok(Attribute::Restart(value.parse()?)),
            "repeatCount" => Ok(Attribute::RepeatCount(value.parse()?)),
            "repeatDur" => Ok(Attribute::RepeatDur(value.parse()?)),
            "additive" => Ok(Attribute::Additive(value.parse()?)),
            "accumulate" => Ok(Attribute::Accumulate(value.parse()?)),
            "onAfterPrint" => Ok(Attribute::OnAfterPrint(value.clone())),
            "onBeforePrint" => Ok(Attribute::OnBeforePrint(value.clone())),
            "onBeforeUnload" => Ok(Attribute::OnBeforeUnload(value.clone())),
            "onError" => Ok(Attribute::OnError(value.clone())),
            "onHashChange" => Ok(Attribute::OnHashChange(value.clone())),
            "onLoad" => Ok(Attribute::OnLoad(value.clone())),
            "onMessage" => Ok(Attribute::OnMessage(value.clone())),
            "onOffline" => Ok(Attribute::OnOffline(value.clone())),
            "onOnline" => Ok(Attribute::OnOnline(value.clone())),
            "onPageHide" => Ok(Attribute::OnPageHide(value.clone())),
            "onPageShow" => Ok(Attribute::OnPageShow(value.clone())),
            "onPopState" => Ok(Attribute::OnPopState(value.clone())),
            "onResize" => Ok(Attribute::OnResize(value.clone())),
            "onStorage" => Ok(Attribute::OnStorage(value.clone())),
            "onUnload" => Ok(Attribute::OnUnload(value.clone())),
            "onChange" => Ok(Attribute::OnChange(value.clone())),
            "onContextMenu" => Ok(Attribute::OnContextMenu(value.clone())),
            "onFocus" => Ok(Attribute::OnFocus(value.clone())),
            "onInput" => Ok(Attribute::OnInput(value.clone())),
            "onInvalid" => Ok(Attribute::OnInvalid(value.clone())),
            "onReset" => Ok(Attribute::OnReset(value.clone())),
            "onSearch" => Ok(Attribute::OnSearch(value.clone())),
            "onSelect" => Ok(Attribute::OnSelect(value.clone())),
            "onSubmit" => Ok(Attribute::OnSubmit(value.clone())),
            "onKeyDown" => Ok(Attribute::OnKeyDown(value.clone())),
            "onKeyPress" => Ok(Attribute::OnKeyPress(value.clone())),
            "onKeyUp" => Ok(Attribute::OnKeyUp(value.clone())),
            "onClick" => Ok(Attribute::OnClick(value.clone())),
            "onDoubleClick" => Ok(Attribute::OnDoubleClick(value.clone())),
            "onMouseDown" => Ok(Attribute::OnMouseDown(value.clone())),
            "onMouseMove" => Ok(Attribute::OnMouseMove(value.clone())),
            "onMouseOut" => Ok(Attribute::OnMouseOut(value.clone())),
            "onMouseOver" => Ok(Attribute::OnMouseOver(value.clone())),
            "onMouseUp" => Ok(Attribute::OnMouseUp(value.clone())),
            "onWheel" => Ok(Attribute::OnWheel(value.clone())),
            "onDrag" => Ok(Attribute::OnDrag(value.clone())),
            "onDragEnd" => Ok(Attribute::OnDragEnd(value.clone())),
            "onDragEnter" => Ok(Attribute::OnDragEnter(value.clone())),
            "onDragLeave" => Ok(Attribute::OnDragLeave(value.clone())),
            "onDragOver" => Ok(Attribute::OnDragOver(value.clone())),
            "onDragStart" => Ok(Attribute::OnDragStart(value.clone())),
            "onDrop" => Ok(Attribute::OnDrop(value.clone())),
            "onScroll" => Ok(Attribute::OnScroll(value.clone())),
            "onCopy" => Ok(Attribute::OnCopy(value.clone())),
            "onCut" => Ok(Attribute::OnCut(value.clone())),
            "onPaste" => Ok(Attribute::OnPaste(value.clone())),
            "onAbort" => Ok(Attribute::OnAbort(value.clone())),
            "onCanPlay" => Ok(Attribute::OnCanPlay(value.clone())),
            "onCanPlayThrough" => Ok(Attribute::OnCanPlayThrough(value.clone())),
            "onCueChange" => Ok(Attribute::OnCueChange(value.clone())),
            "onDurationChange" => Ok(Attribute::OnDurationChange(value.clone())),
            "onEmptied" => Ok(Attribute::OnEmptied(value.clone())),
            "onEnded" => Ok(Attribute::OnEnded(value.clone())),
            "onLoadedData" => Ok(Attribute::OnLoadedData(value.clone())),
            "onLoadedMetadata" => Ok(Attribute::OnLoadedMetadata(value.clone())),
            "onLoadStart" => Ok(Attribute::OnLoadStart(value.clone())),
            "onPause" => Ok(Attribute::OnPause(value.clone())),
            "onPlay" => Ok(Attribute::OnPlay(value.clone())),
            "onPlaying" => Ok(Attribute::OnPlaying(value.clone())),
            "onProgress" => Ok(Attribute::OnProgress(value.clone())),
            "onRateChange" => Ok(Attribute::OnRateChange(value.clone())),
            "onSeeked" => Ok(Attribute::OnSeeked(value.clone())),
            "onSeeking" => Ok(Attribute::OnSeeking(value.clone())),
            "onStalled" => Ok(Attribute::OnStalled(value.clone())),
            "onSuspend" => Ok(Attribute::OnSuspend(value.clone())),
            "onTimeUpdate" => Ok(Attribute::OnTimeUpdate(value.clone())),
            "onVolumeChange" => Ok(Attribute::OnVolumeChange(value.clone())),
            "onWaiting" => Ok(Attribute::OnWaiting(value.clone())),
            "onToggle" => Ok(Attribute::OnToggle(value.clone())),
            "onbegin" => Ok(Attribute::OnBegin(value.clone())),
            "onend" => Ok(Attribute::OnEnd(value.clone())),
            "onrepeat" => Ok(Attribute::OnRepeat(value.clone())),
            "keyPoints" => Ok(Attribute::KeyPoints(
                value.split(';').map(|s| s.parse()).collect::<Result<_, _>>()?,
            )),
            "calcMode" => Ok(Attribute::CalcMode(value.parse()?)),
            "values" => Ok(Attribute::Values(
                value.split(';').map(|s| s.trim().to_string()).collect(),
            )),
            "keyTimes" => Ok(Attribute::KeyTimes(
                value
                    .split(';')
                    .map(|s| s.trim().parse::<f64>().map_err(|_| ()))
                    .collect::<Result<_, _>>()?,
            )),
            "keySplines" => Ok(Attribute::KeySplines(
                value.split(';').map(|s| s.parse()).collect::<Result<_, _>>()?,
            )),
            "from" => Ok(Attribute::From(value.clone())),
            "to" => Ok(Attribute::To(value.clone())),
            "by" => Ok(Attribute::By(value.clone())),
            "blur" => Ok(Attribute::FloodColor(value.parse()?)),
            "interestFor" => Ok(Attribute::InterestFor(value.clone())),
            "ping" => Ok(Attribute::Ping(
                value.split_whitespace().map(|s| s.parse()).collect::<Result<_, _>>()?,
            )),
            "referrerPolicy" => Ok(Attribute::ReferrerPolicy(
                ReferrerPolicy::try_from(value.as_str()).map_err(|_| ())?,
            )),
            "rel" => Ok(Attribute::Rel(
                value.split_whitespace()
                    .map(|s| s.parse::<RelType>().map_err(|_| ()))
                    .collect::<Result<_, _>>()?,
            )),
            "target" => Ok(Attribute::Target(
                Target::try_from(value.as_str()).map_err(|_| ())?,
            )),
            "markerHeight" => Ok(Attribute::MarkerHeight(value.parse()?)),
            "markerUnits" => Ok(Attribute::MarkerUnits(value.parse()?)),
            "markerWidth" => Ok(Attribute::MarkerWidth(value.parse()?)),
            "orient" => Ok(Attribute::Orient(value.parse()?)),
            "preserveAspectRatio" => Ok(Attribute::PreserveAspectRatio(value.parse()?)),
            "refX" => Ok(Attribute::RefX(value.parse()?)),
            "refY" => Ok(Attribute::RefY(value.parse()?)),
            "maskContentUnits" => Ok(Attribute::MaskContentUnits(value.parse()?)),
            "maskUnits" => Ok(Attribute::MaskUnits(value.parse()?)),
            "patternContentUnits" => Ok(Attribute::PatternContentUnits(value.parse()?)),
            "patternUnits" => Ok(Attribute::PatternUnits(value.parse()?)),
            "patternTransform" => Ok(Attribute::PatternTransform(value.clone())),
            "result" => Ok(Attribute::Result(value.clone())),
            "in" => Ok(Attribute::In(value.parse().map_err(|_| ())?)),
            "in2" => Ok(Attribute::In2(value.parse().map_err(|_| ())?)),
            "mode" => Ok(Attribute::Mode(value.parse()?)),
            "k1" => Ok(Attribute::K1(value.parse().unwrap_or(1.0))),
            "k2" => Ok(Attribute::K2(value.parse().unwrap_or(1.0))),
            "k3" => Ok(Attribute::K3(value.parse().unwrap_or(1.0))),
            "k4" => Ok(Attribute::K4(value.parse().unwrap_or(1.0))),
            "kernelMatrix" => Ok(Attribute::KernelMatrix(
                value
                    .split_whitespace()
                    .map(|s| s.parse::<f64>().map_err(|_| ()))
                    .collect::<Result<_, _>>()?,
            )),
            "divisor" => Ok(Attribute::Divisor(value.parse().unwrap_or(1.0))),
            "bias" => Ok(Attribute::Bias(value.parse().unwrap_or(0.0))),
            "targetX" => Ok(Attribute::TargetX(value.parse().unwrap_or(0))),
            "targetY" => Ok(Attribute::TargetY(value.parse().unwrap_or(0))),
            "edgeMode" => Ok(Attribute::EdgeMode(value.parse()?)),
            "kernelUnitLength" => {
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 1 {
                    Ok(Attribute::KernelUnitLength(parts[0].parse().map_err(|_| ())?, None))
                } else if parts.len() == 2 {
                    Ok(Attribute::KernelUnitLength(
                        parts[0].parse().map_err(|_| ())?,
                        Some(parts[1].parse().map_err(|_| ())?),
                    ))
                } else {
                    Err(())
                }
            }
            "preserveAlpha" => Ok(Attribute::PreserveAlpha(value.parse().unwrap_or(false))),
            "surfaceScale" => Ok(Attribute::SurfaceScale(value.parse().unwrap_or(1.0))),
            "diffuseConstant" => Ok(Attribute::DiffuseConstant(value.parse().unwrap_or(1.0))),
            "scale" => Ok(Attribute::Scale(value.parse().unwrap_or(1.0))),
            "xChannelSelector" => Ok(Attribute::XChannelSelector(value.parse()?)),
            "yChannelSelector" => Ok(Attribute::YChannelSelector(value.parse()?)),
            "dx" => Ok(Attribute::Dx(value.parse().unwrap_or(0.0))),
            "dy" => Ok(Attribute::Dy(value.parse().unwrap_or(0.0))),
            "stdDeviation" => {
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 1 {
                    Ok(Attribute::StdDeviation(parts[0].parse().map_err(|_| ())?, None))
                } else if parts.len() == 2 {
                    Ok(Attribute::StdDeviation(
                        parts[0].parse().map_err(|_| ())?,
                        Some(parts[1].parse().map_err(|_| ())?),
                    ))
                } else {
                    Err(())
                }
            }
            "crossOrigin" => Ok(Attribute::CrossOrigin(value.parse()?)),
            "radius" => {
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 1 {
                    Ok(Attribute::Radius(parts[0].parse().map_err(|_| ())?, None))
                } else if parts.len() == 2 {
                    Ok(Attribute::Radius(
                        parts[0].parse().map_err(|_| ())?,
                        Some(parts[1].parse().map_err(|_| ())?),
                    ))
                } else {
                    Err(())
                }
            }
            "specularConstant" => Ok(Attribute::SpecularConstant(value.parse().unwrap_or(1.0))),
            "specularExponent" => Ok(Attribute::SpecularExponent(value.parse().unwrap_or(1.0))),
            "azimuth" => Ok(Attribute::Azimuth(value.parse().unwrap_or(0.0))),
            "elevation" => Ok(Attribute::Elevation(value.parse().unwrap_or(0.0))),
            "z" => Ok(Attribute::Z(value.parse().unwrap_or(0.0))),
            "pointsAtX" => Ok(Attribute::PointsAtX(value.parse().unwrap_or(0.0))),
            "pointsAtY" => Ok(Attribute::PointsAtY(value.parse().unwrap_or(0.0))),
            "pointsAtZ" => Ok(Attribute::PointsAtZ(value.parse().unwrap_or(0.0))),
            "limitingConeAngle" => Ok(Attribute::LimitingConeAngle(
                value.parse().unwrap_or(0.0),
            )),
            "baseFrequency" => {
                let parts: Vec<&str> = value.split_whitespace().collect();
                if parts.len() == 1 {
                    Ok(Attribute::BaseFrequency(parts[0].parse().map_err(|_| ())?, None))
                } else if parts.len() == 2 {
                    Ok(Attribute::BaseFrequency(
                        parts[0].parse().map_err(|_| ())?,
                        Some(parts[1].parse().map_err(|_| ())?),
                    ))
                } else {
                    Err(())
                }
            }
            "numOctaves" => Ok(Attribute::NumOctaves(
                value.parse::<f64>().map(|v| v as u64).map_err(|_| ())?,
            )),
            "seed" => Ok(Attribute::Seed(value.parse().unwrap_or(0.0))),
            "stitchTiles" => Ok(Attribute::StitchTiles(value.parse()?)),
            "gradientUnits" => Ok(Attribute::GradientUnits(value.parse()?)),
            "gradientTransform" => Ok(Attribute::GradientTransform(value.clone())),
            "spreadMethod" => Ok(Attribute::SpreadMethod(value.parse()?)),
            "fx" => Ok(Attribute::Fx(value.parse()?)),
            "fy" => Ok(Attribute::Fy(value.parse()?)),
            "fr" => Ok(Attribute::Fr(value.parse()?)),
            "decoding" => Ok(Attribute::Decoding(value.parse()?)),
            "fetchPriority" => Ok(Attribute::FetchPriority(value.parse()?)),
            "lengthAdjust" => Ok(Attribute::LengthAdjust(value.parse()?)),
            "textLength" => Ok(Attribute::TextLength(value.parse()?)),
            "clipPathUnits" => Ok(Attribute::ClipPathUnits(value.parse()?)),
            "method" => Ok(Attribute::Method(value.parse()?)),
            "side" => Ok(Attribute::Side(value.parse()?)),
            "spacing" => Ok(Attribute::Spacing(value.parse()?)),
            "startOffset" => Ok(Attribute::StartOffset(value.parse()?)),
            "filterUnits" => Ok(Attribute::FilterUnits(value.parse()?)),
            "primitiveUnits" => Ok(Attribute::PrimitiveUnits(value.parse()?)),
            _ => {
                return Err(());
            }
        }
    }
}

impl Attribute {
    pub fn name(&self) -> &str {
        match self {
            Attribute::Xmlns(_) => "xmlns",
            Attribute::Autofocus(_) => "autofocus",
            Attribute::Id(_) => "id",
            Attribute::Class(_) => "class",
            Attribute::Style(_) => "style",
            Attribute::Lang(_) => "lang",
            Attribute::Tabindex(_) => "tabindex",
            Attribute::RequiredExtensions(_) => "requiredExtensions",
            Attribute::SystemLanguage(_) => "systemLanguage",
            Attribute::AlignmentBaseline(_) => "alignment-baseline",
            Attribute::BaselineShift(_) => "baseline-shift",
            Attribute::ClipPath(_) => "clip-path",
            Attribute::ClipRule(_) => "clip-rule",
            Attribute::Color(_) => "color",
            Attribute::ColorInterpolation(_) => "color-interpolation",
            Attribute::ColorInterpolationFilters(_) => "color-interpolation-filters",
            Attribute::Cursor(_) => "cursor",
            Attribute::Cx(_) => "cx",
            Attribute::Cy(_) => "cy",
            Attribute::D(_) => "d",
            Attribute::Direction(_) => "direction",
            Attribute::Display(_) => "display",
            Attribute::DominantBaseline(_) => "dominant-baseline",
            Attribute::Fill(_) => "fill",
            Attribute::FillOpacity(_) => "fill-opacity",
            Attribute::FillRule(_) => "fill-rule",
            Attribute::Filter(_) => "filter",
            Attribute::FloodColor(_) => "flood-color",
            Attribute::FloodOpacity(_) => "flood-opacity",
            Attribute::FontFamily(_) => "font-family",
            Attribute::FontSize(_) => "font-size",
            Attribute::FontSizeAdjust(_) => "font-size-adjust",
            Attribute::FontStyle(_) => "font-style",
            Attribute::FontVariant(_) => "font-variant",
            Attribute::FontWeight(_) => "font-weight",
            Attribute::Height(_) => "height",
            Attribute::ImageRendering(_) => "image-rendering",
            Attribute::LetterSpacing(_) => "letter-spacing",
            Attribute::LightingColor(_) => "lighting-color",
            Attribute::MarkerEnd(_) => "marker-end",
            Attribute::MarkerMid(_) => "marker-mid",
            Attribute::MarkerStart(_) => "marker-start",
            Attribute::Mask(_) => "mask",
            Attribute::MaskType(_) => "mask-type",
            Attribute::Opacity(_) => "opacity",
            Attribute::Overflow(_) => "overflow",
            Attribute::PointerEvents(_) => "pointer-events",
            Attribute::R(_) => "r",
            Attribute::Rx(_) => "rx",
            Attribute::Ry(_) => "ry",
            Attribute::ShapeRendering(_) => "shape-rendering",
            Attribute::StopColor(_) => "stop-color",
            Attribute::StopOpacity(_) => "stop-opacity",
            Attribute::Stroke(_) => "stroke",
            Attribute::StrokeDasharray(_) => "stroke-dasharray",
            Attribute::StrokeDashoffset(_) => "stroke-dashoffset",
            Attribute::StrokeLinecap(_) => "stroke-linecap",
            Attribute::StrokeLinejoin(_) => "stroke-linejoin",
            Attribute::StrokeMiterlimit(_) => "stroke-miterlimit",
            Attribute::StrokeOpacity(_) => "stroke-opacity",
            Attribute::StrokeWidth(_) => "stroke-width",
            Attribute::TextAnchor(_) => "text-anchor",
            Attribute::TextDecoration(_) => "text-decoration",
            Attribute::TextOverflow(_) => "text-overflow",
            Attribute::TextRendering(_) => "text-rendering",
            Attribute::Transform(_) => "transform",
            Attribute::TransformOrigin(_) => "transform-origin",
            Attribute::UnicodeBidi(_) => "unicode-bidi",
            Attribute::VectorEffect(_) => "vector-effect",
            Attribute::Visibility(_) => "visibility",
            Attribute::Width(_) => "width",
            Attribute::WhiteSpace(_) => "white-space",
            Attribute::WordSpacing(_) => "word-spacing",
            Attribute::WritingMode(_) => "writing-mode",
            Attribute::X(_) => "x",
            Attribute::Y(_) => "y",
            Attribute::Type(_) => "type",
            Attribute::TableValues(_) => "tableValues",
            Attribute::Slope(_) => "slope",
            Attribute::Intercept(_) => "intercept",
            Attribute::Amplitude(_) => "amplitude",
            Attribute::Exponent(_) => "exponent",
            Attribute::Offset(_) => "offset",
            Attribute::Href(_) => "href",
            Attribute::AttributeType(_) => "attributeType",
            Attribute::AttributeName(_) => "attributeName",
            Attribute::Begin(_) => "begin",
            Attribute::Dur(_) => "dur",
            Attribute::End(_) => "end",
            Attribute::Min(_) => "min",
            Attribute::Max(_) => "max",
            Attribute::Restart(_) => "restart",
            Attribute::RepeatCount(_) => "repeatCount",
            Attribute::RepeatDur(_) => "repeatDur",
            Attribute::Additive(_) => "additive",
            Attribute::Accumulate(_) => "accumulate",
            Attribute::OnAfterPrint(_) => "onAfterPrint",
            Attribute::OnBeforePrint(_) => "onBeforePrint",
            Attribute::OnBeforeUnload(_) => "onBeforeUnload",
            Attribute::OnError(_) => "onError",
            Attribute::OnHashChange(_) => "onHashChange",
            Attribute::OnLoad(_) => "onLoad",
            Attribute::OnMessage(_) => "onMessage",
            Attribute::OnOffline(_) => "onOffline",
            Attribute::OnOnline(_) => "onOnline",
            Attribute::OnPageHide(_) => "onPageHide",
            Attribute::OnPageShow(_) => "onPageShow",
            Attribute::OnPopState(_) => "onPopState",
            Attribute::OnResize(_) => "onResize",
            Attribute::OnStorage(_) => "onStorage",
            Attribute::OnUnload(_) => "onUnload",
            Attribute::OnBlur(_) => "onBlur",
            Attribute::OnChange(_) => "onChange",
            Attribute::OnContextMenu(_) => "onContextMenu",
            Attribute::OnFocus(_) => "onFocus",
            Attribute::OnInput(_) => "onInput",
            Attribute::OnInvalid(_) => "onInvalid",
            Attribute::OnReset(_) => "onReset",
            Attribute::OnSearch(_) => "onSearch",
            Attribute::OnSelect(_) => "onSelect",
            Attribute::OnSubmit(_) => "onSubmit",
            Attribute::OnKeyDown(_) => "onKeyDown",
            Attribute::OnKeyPress(_) => "onKeyPress",
            Attribute::OnKeyUp(_) => "onKeyUp",
            Attribute::OnClick(_) => "onClick",
            Attribute::OnDoubleClick(_) => "onDoubleClick",
            Attribute::OnMouseDown(_) => "onMouseDown",
            Attribute::OnMouseMove(_) => "onMouseMove",
            Attribute::OnMouseOut(_) => "onMouseOut",
            Attribute::OnMouseOver(_) => "onMouseOver",
            Attribute::OnMouseUp(_) => "onMouseUp",
            Attribute::OnWheel(_) => "onWheel",
            Attribute::OnDrag(_) => "onDrag",
            Attribute::OnDragEnd(_) => "onDragEnd",
            Attribute::OnDragEnter(_) => "onDragEnter",
            Attribute::OnDragLeave(_) => "onDragLeave",
            Attribute::OnDragOver(_) => "onDragOver",
            Attribute::OnDragStart(_) => "onDragStart",
            Attribute::OnDrop(_) => "onDrop",
            Attribute::OnScroll(_) => "onScroll",
            Attribute::OnCopy(_) => "onCopy",
            Attribute::OnCut(_) => "onCut",
            Attribute::OnPaste(_) => "onPaste",
            Attribute::OnAbort(_) => "onAbort",
            Attribute::OnCanPlay(_) => "onCanPlay",
            Attribute::OnCanPlayThrough(_) => "onCanPlayThrough",
            Attribute::OnCueChange(_) => "onCueChange",
            Attribute::OnDurationChange(_) => "onDurationChange",
            Attribute::OnEmptied(_) => "onEmptied",
            Attribute::OnEnded(_) => "onEnded",
            Attribute::OnLoadedData(_) => "onLoadedData",
            Attribute::OnLoadedMetadata(_) => "onLoadedMetadata",
            Attribute::OnLoadStart(_) => "onLoadStart",
            Attribute::OnPause(_) => "onPause",
            Attribute::OnPlay(_) => "onPlay",
            Attribute::OnPlaying(_) => "onPlaying",
            Attribute::OnProgress(_) => "onProgress",
            Attribute::OnRateChange(_) => "onRateChange",
            Attribute::OnSeeked(_) => "onSeeked",
            Attribute::OnSeeking(_) => "onSeeking",
            Attribute::OnStalled(_) => "onStalled",
            Attribute::OnSuspend(_) => "onSuspend",
            Attribute::OnTimeUpdate(_) => "onTimeUpdate",
            Attribute::OnVolumeChange(_) => "onVolumeChange",
            Attribute::OnWaiting(_) => "onWaiting",
            Attribute::OnToggle(_) => "onToggle",
            Attribute::OnBegin(_) => "onbegin",
            Attribute::OnEnd(_) => "onend",
            Attribute::OnRepeat(_) => "onrepeat",
            Attribute::KeyPoints(_) => "keyPoints",
            Attribute::Path(_) => "path",
            Attribute::Rotate(_) => "rotate",
            Attribute::CalcMode(_) => "calcMode",
            Attribute::Values(_) => "values",
            Attribute::KeyTimes(_) => "keyTimes",
            Attribute::KeySplines(_) => "keySplines",
            Attribute::From(_) => "from",
            Attribute::To(_) => "to",
            Attribute::By(_) => "by",
            Attribute::PathLength(_) => "pathLength",
            Attribute::X1(_) => "x1",
            Attribute::Y1(_) => "y1",
            Attribute::X2(_) => "x2",
            Attribute::Y2(_) => "y2",
            Attribute::Points(_) => "points",
            Attribute::Download(_) => "download",
            Attribute::HrefLang(_) => "hrefLang",
            Attribute::InterestFor(_) => "interestFor",
            Attribute::Ping(_) => "ping",
            Attribute::ReferrerPolicy(_) => "referrerPolicy",
            Attribute::Rel(_) => "rel",
            Attribute::Target(_) => "target",
            Attribute::MarkerHeight(_) => "markerHeight",
            Attribute::MarkerUnits(_) => "markerUnits",
            Attribute::MarkerWidth(_) => "markerWidth",
            Attribute::Orient(_) => "orient",
            Attribute::PreserveAspectRatio(_) => "preserveAspectRatio",
            Attribute::RefX(_) => "refX",
            Attribute::RefY(_) => "refY",
            Attribute::ViewBox(_) => "viewBox",
            Attribute::MaskContentUnits(_) => "maskContentUnits",
            Attribute::MaskUnits(_) => "maskUnits",
            Attribute::PatternContentUnits(_) => "patternContentUnits",
            Attribute::PatternUnits(_) => "patternUnits",
            Attribute::PatternTransform(_) => "patternTransform",
            Attribute::Result(_) => "result",
            Attribute::In(_) => "in",
            Attribute::In2(_) => "in2",
            Attribute::Mode(_) => "mode",
            Attribute::Operator(_) => "operator",
            Attribute::K1(_) => "k1",
            Attribute::K2(_) => "k2",
            Attribute::K3(_) => "k3",
            Attribute::K4(_) => "k4",
            Attribute::Order(_) => "order",
            Attribute::KernelMatrix(_) => "kernelMatrix",
            Attribute::Divisor(_) => "divisor",
            Attribute::Bias(_) => "bias",
            Attribute::TargetX(_) => "targetX",
            Attribute::TargetY(_) => "targetY",
            Attribute::EdgeMode(_) => "edgeMode",
            Attribute::KernelUnitLength(_, _) => "kernelUnitLength",
            Attribute::PreserveAlpha(_) => "preserveAlpha",
            Attribute::SurfaceScale(_) => "surfaceScale",
            Attribute::DiffuseConstant(_) => "diffuseConstant",
            Attribute::Scale(_) => "scale",
            Attribute::XChannelSelector(_) => "xChannelSelector",
            Attribute::YChannelSelector(_) => "yChannelSelector",
            Attribute::Dx(_) => "dx",
            Attribute::Dy(_) => "dy",
            Attribute::StdDeviation(_, _) => "stdDeviation",
            Attribute::CrossOrigin(_) => "crossOrigin",
            Attribute::Radius(_, _) => "radius",
            Attribute::SpecularConstant(_) => "specularConstant",
            Attribute::SpecularExponent(_) => "specularExponent",
            Attribute::Azimuth(_) => "azimuth",
            Attribute::Elevation(_) => "elevation",
            Attribute::Z(_) => "z",
            Attribute::PointsAtX(_) => "pointsAtX",
            Attribute::PointsAtY(_) => "pointsAtY",
            Attribute::PointsAtZ(_) => "pointsAtZ",
            Attribute::LimitingConeAngle(_) => "limitingConeAngle",
            Attribute::BaseFrequency(_, _) => "baseFrequency",
            Attribute::NumOctaves(_) => "numOctaves",
            Attribute::Seed(_) => "seed",
            Attribute::StitchTiles(_) => "stitchTiles",
            Attribute::GradientUnits(_) => "gradientUnits",
            Attribute::GradientTransform(_) => "gradientTransform",
            Attribute::SpreadMethod(_) => "spreadMethod",
            Attribute::Fx(_) => "fx",
            Attribute::Fy(_) => "fy",
            Attribute::Fr(_) => "fr",
            Attribute::Decoding(_) => "decoding",
            Attribute::FetchPriority(_) => "fetchPriority",
            Attribute::LengthAdjust(_) => "lengthAdjust",
            Attribute::TextLength(_) => "textLength",
            Attribute::ClipPathUnits(_) => "clipPathUnits",
            Attribute::Method(_) => "method",
            Attribute::Side(_) => "side",
            Attribute::Spacing(_) => "spacing",
            Attribute::StartOffset(_) => "startOffset",
            Attribute::FilterUnits(_) => "filterUnits",
            Attribute::PrimitiveUnits(_) => "primitiveUnits",
            Attribute::Version(_) => "version",
        }
    }

    #[inline]
    pub fn is_global(&self) -> bool {
        matches!(
            self,
            Attribute::Autofocus(_)
                | Attribute::Id(_)
                | Attribute::Class(_)
                | Attribute::Style(_)
                | Attribute::Lang(_)
                | Attribute::Tabindex(_)
        )
    }

    #[inline]
    pub fn is_core(&self) -> bool {
        self.is_global()
    }

    #[inline]
    pub fn is_conditional_processing(&self) -> bool {
        matches!(
            self,
            Attribute::RequiredExtensions(_) | Attribute::SystemLanguage(_)
        )
    }

    #[inline]
    pub fn is_presentation(&self) -> bool {
        matches!(
            self,
            Attribute::AlignmentBaseline(_)
                | Attribute::BaselineShift(_)
                | Attribute::ClipPath(_)
                | Attribute::ClipRule(_)
                | Attribute::Color(_)
                | Attribute::ColorInterpolation(_)
                | Attribute::ColorInterpolationFilters(_)
                | Attribute::Cursor(_)
                | Attribute::Cx(_)
                | Attribute::Cy(_)
                | Attribute::D(_)
                | Attribute::Direction(_)
                | Attribute::Display(_)
                | Attribute::DominantBaseline(_)
                | Attribute::Fill(_)
                | Attribute::FillOpacity(_)
                | Attribute::FillRule(_)
                | Attribute::Filter(_)
                | Attribute::FloodColor(_)
                | Attribute::FloodOpacity(_)
                | Attribute::FontFamily(_)
                | Attribute::FontSize(_)
                | Attribute::FontSizeAdjust(_)
                | Attribute::FontStyle(_)
                | Attribute::FontVariant(_)
                | Attribute::FontWeight(_)
                | Attribute::Height(_)
                | Attribute::ImageRendering(_)
                | Attribute::LetterSpacing(_)
                | Attribute::LightingColor(_)
                | Attribute::MarkerEnd(_)
                | Attribute::MarkerMid(_)
                | Attribute::MarkerStart(_)
                | Attribute::Mask(_)
                | Attribute::MaskType(_)
                | Attribute::Opacity(_)
                | Attribute::Overflow(_)
                | Attribute::PointerEvents(_)
                | Attribute::R(_)
                | Attribute::Rx(_)
                | Attribute::Ry(_)
                | Attribute::ShapeRendering(_)
                | Attribute::StopColor(_)
                | Attribute::StopOpacity(_)
                | Attribute::Stroke(_)
                | Attribute::StrokeDasharray(_)
                | Attribute::StrokeDashoffset(_)
                | Attribute::StrokeLinecap(_)
                | Attribute::StrokeLinejoin(_)
                | Attribute::StrokeMiterlimit(_)
                | Attribute::StrokeOpacity(_)
                | Attribute::StrokeWidth(_)
                | Attribute::TextAnchor(_)
                | Attribute::TextDecoration(_)
                | Attribute::TextOverflow(_)
                | Attribute::TextRendering(_)
                | Attribute::Transform(_)
                | Attribute::TransformOrigin(_)
                | Attribute::UnicodeBidi(_)
                | Attribute::VectorEffect(_)
                | Attribute::Visibility(_)
                | Attribute::Width(_)
                | Attribute::WhiteSpace(_)
                | Attribute::WordSpacing(_)
                | Attribute::WritingMode(_)
                | Attribute::X(_)
                | Attribute::Y(_)
        )
    }

    #[inline]
    pub fn is_animation_timing(&self) -> bool {
        matches!(
            self,
            Attribute::Begin(_)
                | Attribute::Dur(_)
                | Attribute::End(_)
                | Attribute::Min(_)
                | Attribute::Max(_)
                | Attribute::Restart(_)
                | Attribute::RepeatCount(_)
                | Attribute::RepeatDur(_)
                | Attribute::Fill(_)
        )
    }

    #[inline]
    pub fn is_animation_value(&self) -> bool {
        matches!(
            self,
            Attribute::CalcMode(_)
                | Attribute::Values(_)
                | Attribute::KeyTimes(_)
                | Attribute::KeySplines(_)
                | Attribute::From(_)
                | Attribute::To(_)
                | Attribute::By(_)
        )
    }

    #[inline]
    pub fn is_animation_addition(&self) -> bool {
        matches!(self, Attribute::Accumulate(_) | Attribute::Additive(_))
    }

    #[inline]
    pub fn applies_to_shape(&self) -> bool {
        self.is_fill() || self.is_stroke()
    }

    #[inline]
    pub fn is_fill(&self) -> bool {
        matches!(
            self,
            Attribute::Fill(_) | Attribute::FillOpacity(_) | Attribute::FillRule(_)
        )
    }

    #[inline]
    pub fn is_stroke(&self) -> bool {
        matches!(
            self,
            Attribute::Stroke(_)
                | Attribute::StrokeDasharray(_)
                | Attribute::StrokeDashoffset(_)
                | Attribute::StrokeLinecap(_)
                | Attribute::StrokeLinejoin(_)
                | Attribute::StrokeMiterlimit(_)
                | Attribute::StrokeOpacity(_)
                | Attribute::StrokeWidth(_)
        )
    }

    pub fn is_filter_primitive(&self) -> bool {
        matches!(
            self,
            Attribute::Height(_)
                | Attribute::Result(_)
                | Attribute::Width(_)
                | Attribute::X(_)
                | Attribute::Y(_)
        )
    }

    pub fn allowed_in_element(&self, element_type: ElementType, _element: &crate::element::Element) -> bool {
        match element_type {
            ElementType::Animate => self.is_global(),
            ElementType::AnimateMotion => {
                self.is_global()
                    || self.is_animation_timing()
                    || self.is_animation_value()
                    || self.is_animation_addition()
                    || matches!(
                        self,
                        Attribute::KeyPoints(_)
                            | Attribute::Path(_)
                            | Attribute::Rotate(_)
                            | Attribute::AttributeName(_)
                            | Attribute::OnBegin(_)
                            | Attribute::OnEnd(_)
                            | Attribute::OnRepeat(_)
                    )
            }
            ElementType::AnimateTransform => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::By(_)
                            | Attribute::From(_)
                            | Attribute::To(_)
                            | Attribute::Type(_)
                    )
            }
            ElementType::MPath => self.is_global() || matches!(self, Attribute::Href(_)),
            ElementType::Set => matches!(self, Attribute::To(_)),
            ElementType::Circle => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::R(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::Ellipse => {
                self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::Rx(_)
                            | Attribute::Ry(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::Line => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X1(_) | Attribute::Y1(_) | Attribute::X2(_) | Attribute::Y2(_)
                    )
            }
            ElementType::Polygon => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::Points(_) | Attribute::PathLength(_))
            }
            ElementType::PolyLine => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::Points(_) | Attribute::PathLength(_))
            }
            ElementType::Rect => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::Rx(_)
                            | Attribute::Ry(_)
                            | Attribute::PathLength(_)
                    )
            }
            ElementType::A => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::Download(_)
                            | Attribute::HrefLang(_)
                            | Attribute::InterestFor(_)
                            | Attribute::Ping(_)
                            | Attribute::ReferrerPolicy(_)
                            | Attribute::Rel(_)
                            | Attribute::Target(_)
                            | Attribute::Type(_)
                    )
            }
            ElementType::Defs => self.is_global(),
            ElementType::G => self.is_global(),
            ElementType::Marker => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::MarkerHeight(_)
                            | Attribute::MarkerUnits(_)
                            | Attribute::MarkerWidth(_)
                            | Attribute::Orient(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::RefX(_)
                            | Attribute::RefY(_)
                            | Attribute::ViewBox(_)
                    )
            }
            ElementType::Mask => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::MaskUnits(_)
                            | Attribute::MaskContentUnits(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                    )
            }
            ElementType::Pattern => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::Href(_)
                            | Attribute::PatternContentUnits(_)
                            | Attribute::PatternUnits(_)
                            | Attribute::PatternTransform(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                    )
            }
            ElementType::Svg => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Xmlns(_)
                    )
            }
            ElementType::Switch => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::RequiredExtensions(_)
                            | Attribute::SystemLanguage(_)
                    )
            }
            ElementType::Symbol => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Height(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::RefX(_)
                            | Attribute::RefY(_)
                            | Attribute::ViewBox(_)
                            | Attribute::Width(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                    )
            }
            ElementType::Desc => self.is_global(),
            ElementType::Metadata => self.is_global(),
            ElementType::Title => self.is_global(),
            ElementType::FeBlend => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::In2(_) | Attribute::Mode(_)
                    )
            }
            ElementType::FeColorMatrix => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::Type(_) | Attribute::Values(_)
                    )
            }
            ElementType::FeComponentTransfer => {
                self.is_global() || self.is_filter_primitive() || matches!(self, Attribute::In(_))
            }
            ElementType::FeComposite => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::In2(_)
                            | Attribute::Operator(_)
                            | Attribute::K1(_)
                            | Attribute::K2(_)
                            | Attribute::K3(_)
                            | Attribute::K4(_)
                    )
            }
            ElementType::FeConvolveMatrix => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::Order(_)
                            | Attribute::KernelMatrix(_)
                            | Attribute::Divisor(_)
                            | Attribute::Bias(_)
                            | Attribute::TargetX(_)
                            | Attribute::TargetY(_)
                            | Attribute::EdgeMode(_)
                            | Attribute::KernelUnitLength(_, _)
                            | Attribute::PreserveAlpha(_)
                    )
            }
            ElementType::FeDiffuseLightning => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::SurfaceScale(_)
                            | Attribute::DiffuseConstant(_)
                            | Attribute::KernelUnitLength(_, _)
                    )
            }
            ElementType::FeDisplacementMap => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::In2(_)
                            | Attribute::Scale(_)
                            | Attribute::XChannelSelector(_)
                            | Attribute::YChannelSelector(_)
                    )
            }
            ElementType::FeDropShadow => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::StdDeviation(_, _)
                    )
            }
            ElementType::FeFlood => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(self, Attribute::FloodColor(_) | Attribute::FloodOpacity(_))
            }
            ElementType::FeFuncA => self.is_global(),
            ElementType::FeFuncB => self.is_global(),
            ElementType::FeFuncG => self.is_global(),
            ElementType::FeFuncR => self.is_global(),
            ElementType::FeGaussianBlur => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::StdDeviation(_, _) | Attribute::EdgeMode(_)
                    )
            }
            ElementType::FeImage => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::CrossOrigin(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::Href(_)
                    )
            }
            ElementType::FeMerge => self.is_global() || self.is_filter_primitive(),
            ElementType::FeMergeNode => self.is_global() || matches!(self, Attribute::In(_)),
            ElementType::FeMorphology => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_) | Attribute::Operator(_) | Attribute::Radius(_, _)
                    )
            }
            ElementType::FeOffset => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(self, Attribute::In(_) | Attribute::Dx(_) | Attribute::Dy(_))
            }
            ElementType::FeSpecularLighting => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::In(_)
                            | Attribute::SurfaceScale(_)
                            | Attribute::SpecularConstant(_)
                            | Attribute::SpecularExponent(_)
                            | Attribute::KernelUnitLength(_, _)
                    )
            }
            ElementType::FeTile => {
                self.is_global() || self.is_filter_primitive() || matches!(self, Attribute::In(_))
            }
            ElementType::FeTurbulence => {
                self.is_global()
                    || self.is_filter_primitive()
                    || matches!(
                        self,
                        Attribute::BaseFrequency(_, _)
                            | Attribute::NumOctaves(_)
                            | Attribute::Seed(_)
                            | Attribute::StitchTiles(_)
                            | Attribute::Type(_)
                    )
            }
            ElementType::LinearGradient => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::GradientUnits(_)
                            | Attribute::GradientTransform(_)
                            | Attribute::Href(_)
                            | Attribute::SpreadMethod(_)
                            | Attribute::X1(_)
                            | Attribute::Y1(_)
                            | Attribute::X2(_)
                            | Attribute::Y2(_)
                    )
            }
            ElementType::RadialGradient => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Cx(_)
                            | Attribute::Cy(_)
                            | Attribute::Fr(_)
                            | Attribute::Fx(_)
                            | Attribute::Fy(_)
                            | Attribute::GradientUnits(_)
                            | Attribute::GradientTransform(_)
                            | Attribute::Href(_)
                            | Attribute::R(_)
                            | Attribute::SpreadMethod(_)
                    )
            }
            ElementType::Stop => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Offset(_) | Attribute::StopColor(_) | Attribute::StopOpacity(_)
                    )
            }
            ElementType::Image => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::Href(_)
                            | Attribute::PreserveAspectRatio(_)
                            | Attribute::CrossOrigin(_)
                            | Attribute::Decoding(_)
                            | Attribute::FetchPriority(_)
                    )
            }
            ElementType::Path => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(self, Attribute::D(_) | Attribute::PathLength(_))
            }
            ElementType::Text => {
                self.is_global()
                    || self.applies_to_shape()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::Rotate(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::Use => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                    )
            }
            ElementType::FeDistantLight => {
                self.is_global() || matches!(self, Attribute::Azimuth(_) | Attribute::Elevation(_))
            }
            ElementType::FePointLight => {
                self.is_global()
                    || matches!(self, Attribute::X(_) | Attribute::Y(_) | Attribute::Z(_))
            }
            ElementType::FeSpotLight => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Z(_)
                            | Attribute::PointsAtX(_)
                            | Attribute::PointsAtY(_)
                            | Attribute::PointsAtZ(_)
                            | Attribute::SpecularExponent(_)
                            | Attribute::LimitingConeAngle(_)
                    )
            }
            ElementType::ClipPath => {
                self.is_global() || matches!(self, Attribute::ClipPathUnits(_))
            }
            ElementType::Script => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::Type(_)
                            | Attribute::CrossOrigin(_)
                            | Attribute::FetchPriority(_)
                    )
            }
            ElementType::Style => self.is_global() || matches!(self, Attribute::Type(_)),
            ElementType::TextPath => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::Href(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::Method(_)
                            | Attribute::Path(_)
                            | Attribute::Side(_)
                            | Attribute::Spacing(_)
                            | Attribute::StartOffset(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::TSpan => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Dx(_)
                            | Attribute::Dy(_)
                            | Attribute::Rotate(_)
                            | Attribute::LengthAdjust(_)
                            | Attribute::TextLength(_)
                    )
            }
            ElementType::Filter => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                            | Attribute::FilterUnits(_)
                            | Attribute::PrimitiveUnits(_)
                    )
            }
            ElementType::ForeignObject => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::X(_)
                            | Attribute::Y(_)
                            | Attribute::Width(_)
                            | Attribute::Height(_)
                    )
            }
            ElementType::View => {
                self.is_global()
                    || matches!(
                        self,
                        Attribute::ViewBox(_) | Attribute::PreserveAspectRatio(_)
                    )
            }
        }
    }

    fn write_value(&self, f: &mut impl fmt::Write) -> fmt::Result {
        match self {
            Attribute::Xmlns(s) => write!(f, "=\"{}\"", s),
            Attribute::Autofocus(v) => write!(f, "=\"{}\"", if *v { "1" } else { "0" }),
            Attribute::Id(s) => write!(f, "=\"{}\"", s),
            Attribute::Class(items) => write_space_separated(f, items.iter()),
            Attribute::Style(s) => write!(f, "=\"{}\"", s),
            Attribute::Lang(s) => write!(f, "=\"{}\"", s),
            Attribute::Tabindex(n) => write!(f, "=\"{}\"", n),
            Attribute::RequiredExtensions(items) => write_space_separated(f, items.iter()),
            Attribute::SystemLanguage(v) => write_comma_separated(f, v.iter()),
            Attribute::AlignmentBaseline(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::BaselineShift(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipPath(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipRule(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::Color(v) => write!(f, "=\"{}\"", v),
            Attribute::ColorInterpolation(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::ColorInterpolationFilters(v) => write!(f, "=\"{}\"", v.as_str()),
            Attribute::Cursor(v) => write!(f, "=\"{}\"", v),
            Attribute::Cx(v) => write!(f, "=\"{}\"", v),
            Attribute::Cy(v) => write!(f, "=\"{}\"", v),
            Attribute::D(paths) => write_space_separated(f, paths.0.iter()),
            Attribute::Direction(v) => write!(f, "=\"{}\"", v),
            Attribute::Display(v) => write!(f, "=\"{}\"", v),
            Attribute::DominantBaseline(v) => write!(f, "=\"{}\"", v),
            Attribute::Fill(v) => write!(f, "=\"{}\"", v),
            Attribute::FillOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::FillRule(v) => write!(f, "=\"{}\"", v),
            Attribute::Filter(s) => write!(f, "=\"{}\"", s),
            Attribute::FloodColor(v) => write!(f, "=\"{}\"", v),
            Attribute::FloodOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::FontFamily(v) => write!(f, "=\"{}\"", v),
            Attribute::FontSize(v) => write!(f, "=\"{}\"", v),
            Attribute::FontSizeAdjust(v) => write!(f, "=\"{}\"", v),
            Attribute::FontStyle(v) => write!(f, "=\"{}\"", v),
            Attribute::FontVariant(v) => write!(f, "=\"{}\"", v),
            Attribute::FontWeight(v) => write!(f, "=\"{}\"", v),
            Attribute::Height(v) => write!(f, "=\"{}\"", v),
            Attribute::ImageRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::LetterSpacing(v) => write!(f, "=\"{}\"", v),
            Attribute::LightingColor(v) => write!(f, "=\"{}\"", v.0),
            Attribute::MarkerEnd(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerMid(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerStart(v) => write!(f, "=\"{}\"", v),
            Attribute::Mask(v) => write!(f, "=\"{}\"", v),
            Attribute::MaskType(v) => write!(f, "=\"{}\"", v),
            Attribute::Opacity(v) => write!(f, "=\"{}\"", v),
            Attribute::Overflow(v) => write!(f, "=\"{}\"", v),
            Attribute::PointerEvents(v) => write!(f, "=\"{}\"", v),
            Attribute::R(v) => write!(f, "=\"{}\"", v),
            Attribute::Rx(v) => write!(f, "=\"{}\"", v),
            Attribute::Ry(v) => write!(f, "=\"{}\"", v),
            Attribute::ShapeRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::StopColor(v) => write!(f, "=\"{}\"", v),
            Attribute::StopOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::Stroke(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeDasharray(items) => write_space_separated(f, items.iter()),
            Attribute::StrokeDashoffset(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeLinecap(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeLinejoin(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeMiterlimit(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeOpacity(v) => write!(f, "=\"{}\"", v),
            Attribute::StrokeWidth(v) => write!(f, "=\"{}\"", v),
            Attribute::TextAnchor(v) => write!(f, "=\"{}\"", v),
            Attribute::TextDecoration(v) => write!(f, "=\"{}\"", v),
            Attribute::TextOverflow(v) => write!(f, "=\"{}\"", v),
            Attribute::TextRendering(v) => write!(f, "=\"{}\"", v),
            Attribute::Transform(v) => write!(f, "=\"{}\"", v),
            Attribute::TransformOrigin(v) => write!(f, "=\"{}\"", v),
            Attribute::UnicodeBidi(v) => write!(f, "=\"{}\"", v),
            Attribute::VectorEffect(v) => write!(f, "=\"{}\"", v),
            Attribute::Visibility(v) => write!(f, "=\"{}\"", v),
            Attribute::Width(v) => write!(f, "=\"{}\"", v),
            Attribute::WhiteSpace(v) => write!(f, "=\"{}\"", v),
            Attribute::WordSpacing(v) => write!(f, "=\"{}\"", v),
            Attribute::WritingMode(v) => write!(f, "=\"{}\"", v),
            Attribute::X(v) => write!(f, "=\"{}\"", v),
            Attribute::Y(v) => write!(f, "=\"{}\"", v),
            Attribute::Type(v) => write!(f, "=\"{}\"", v),
            Attribute::TableValues(v) => write_space_separated(f, v.iter()),
            Attribute::Slope(v) => write!(f, "=\"{}\"", v),
            Attribute::Intercept(v) => write!(f, "=\"{}\"", v),
            Attribute::Amplitude(v) => write!(f, "=\"{}\"", v),
            Attribute::Exponent(v) => write!(f, "=\"{}\"", v),
            Attribute::Offset(v) => write!(f, "=\"{}\"", v),
            Attribute::Href(v) => write!(f, "=\"{}\"", v),
            Attribute::AttributeType(v) => write!(f, "=\"{}\"", v),
            Attribute::AttributeName(v) => write!(f, "=\"{}\"", v),
            Attribute::Begin(v) => write_semicolon_separated(f, v.iter()),
            Attribute::Dur(v) => write!(f, "=\"{}\"", v),
            Attribute::End(v) => write_semicolon_separated(f, v.iter()),
            Attribute::Min(v) => write!(f, "=\"{}\"", v),
            Attribute::Max(v) => write!(f, "=\"{}\"", v),
            Attribute::Restart(v) => write!(f, "=\"{}\"", v),
            Attribute::RepeatCount(v) => write!(f, "=\"{}\"", v),
            Attribute::RepeatDur(v) => write!(f, "=\"{}\"", v),
            Attribute::Additive(v) => write!(f, "=\"{}\"", v),
            Attribute::Accumulate(v) => write!(f, "=\"{}\"", v),
            Attribute::OnAfterPrint(v)
            | Attribute::OnBeforePrint(v)
            | Attribute::OnBeforeUnload(v)
            | Attribute::OnError(v)
            | Attribute::OnHashChange(v)
            | Attribute::OnLoad(v)
            | Attribute::OnMessage(v)
            | Attribute::OnOffline(v)
            | Attribute::OnOnline(v)
            | Attribute::OnPageHide(v)
            | Attribute::OnPageShow(v)
            | Attribute::OnPopState(v)
            | Attribute::OnResize(v)
            | Attribute::OnStorage(v)
            | Attribute::OnUnload(v)
            | Attribute::OnBlur(v)
            | Attribute::OnChange(v)
            | Attribute::OnContextMenu(v)
            | Attribute::OnFocus(v)
            | Attribute::OnInput(v)
            | Attribute::OnInvalid(v)
            | Attribute::OnReset(v)
            | Attribute::OnSearch(v)
            | Attribute::OnSelect(v)
            | Attribute::OnSubmit(v)
            | Attribute::OnKeyDown(v)
            | Attribute::OnKeyPress(v)
            | Attribute::OnKeyUp(v)
            | Attribute::OnClick(v)
            | Attribute::OnDoubleClick(v)
            | Attribute::OnMouseDown(v)
            | Attribute::OnMouseMove(v)
            | Attribute::OnMouseOut(v)
            | Attribute::OnMouseOver(v)
            | Attribute::OnMouseUp(v)
            | Attribute::OnWheel(v)
            | Attribute::OnDrag(v)
            | Attribute::OnDragEnd(v)
            | Attribute::OnDragEnter(v)
            | Attribute::OnDragLeave(v)
            | Attribute::OnDragOver(v)
            | Attribute::OnDragStart(v)
            | Attribute::OnDrop(v)
            | Attribute::OnScroll(v)
            | Attribute::OnCopy(v)
            | Attribute::OnCut(v)
            | Attribute::OnPaste(v)
            | Attribute::OnAbort(v)
            | Attribute::OnCanPlay(v)
            | Attribute::OnCanPlayThrough(v)
            | Attribute::OnCueChange(v)
            | Attribute::OnDurationChange(v)
            | Attribute::OnEmptied(v)
            | Attribute::OnEnded(v)
            | Attribute::OnLoadedData(v)
            | Attribute::OnLoadedMetadata(v)
            | Attribute::OnLoadStart(v)
            | Attribute::OnPause(v)
            | Attribute::OnPlay(v)
            | Attribute::OnPlaying(v)
            | Attribute::OnProgress(v)
            | Attribute::OnRateChange(v)
            | Attribute::OnSeeked(v)
            | Attribute::OnSeeking(v)
            | Attribute::OnStalled(v)
            | Attribute::OnSuspend(v)
            | Attribute::OnTimeUpdate(v)
            | Attribute::OnVolumeChange(v)
            | Attribute::OnWaiting(v)
            | Attribute::OnToggle(v)
            | Attribute::OnBegin(v)
            | Attribute::OnEnd(v)
            | Attribute::OnRepeat(v) => write!(f, "=\"{}\"", v),
            Attribute::KeyPoints(v) => write_semicolon_separated(f, v.iter()),
            Attribute::Path(paths) => write_space_separated(f, paths.0.iter()),
            Attribute::Rotate(v) => write!(f, "=\"{}\"", v),
            Attribute::CalcMode(v) => write!(f, "=\"{}\"", v),
            Attribute::Values(v) => write_semicolon_separated(f, v.iter()),
            Attribute::KeyTimes(v) => write_semicolon_separated(f, v.iter()),
            Attribute::KeySplines(v) => write_semicolon_separated(f, v.iter()),
            Attribute::From(v) => write!(f, "=\"{}\"", v),
            Attribute::To(v) => write!(f, "=\"{}\"", v),
            Attribute::By(v) => write!(f, "=\"{}\"", v),
            Attribute::PathLength(v) => write!(f, "=\"{}\"", v),
            Attribute::X1(v) => write!(f, "=\"{}\"", v),
            Attribute::Y1(v) => write!(f, "=\"{}\"", v),
            Attribute::X2(v) => write!(f, "=\"{}\"", v),
            Attribute::Y2(v) => write!(f, "=\"{}\"", v),
            Attribute::Points(points) => write_space_separated(f, points.iter()),
            Attribute::Download(v) => write!(f, "=\"{}\"", v),
            Attribute::HrefLang(v) => write!(f, "=\"{}\"", v),
            Attribute::InterestFor(v) => write!(f, "=\"{}\"", v),
            Attribute::Ping(urls) => write_space_separated(f, urls.iter()),
            Attribute::ReferrerPolicy(v) => write!(f, "=\"{}\"", v),
            Attribute::Rel(items) => write_space_separated(f, items.iter()),
            Attribute::Target(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerHeight(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::MarkerWidth(v) => write!(f, "=\"{}\"", v),
            Attribute::Orient(v) => write!(f, "=\"{}\"", v),
            Attribute::PreserveAspectRatio(v) => write!(f, "=\"{}\"", v),
            Attribute::RefX(v) => write!(f, "=\"{}\"", v),
            Attribute::RefY(v) => write!(f, "=\"{}\"", v),
            Attribute::ViewBox(view_box) => write_space_separated(f, view_box.0.iter()),
            Attribute::MaskContentUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::MaskUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternContentUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PatternTransform(v) => write!(f, "=\"{}\"", v),
            Attribute::Result(v) => write!(f, "=\"{}\"", v),
            Attribute::In(v) => write!(f, "=\"{}\"", v),
            Attribute::In2(v) => write!(f, "=\"{}\"", v),
            Attribute::Mode(v) => write!(f, "=\"{}\"", v),
            Attribute::Operator(v) => write!(f, "=\"{}\"", v),
            Attribute::K1(v) => write!(f, "=\"{}\"", v),
            Attribute::K2(v) => write!(f, "=\"{}\"", v),
            Attribute::K3(v) => write!(f, "=\"{}\"", v),
            Attribute::K4(v) => write!(f, "=\"{}\"", v),
            Attribute::Order(v) => write!(f, "=\"{}\"", v),
            Attribute::KernelMatrix(items) => write_space_separated(f, items.iter()),
            Attribute::Divisor(v) => write!(f, "=\"{}\"", v),
            Attribute::Bias(v) => write!(f, "=\"{}\"", v),
            Attribute::TargetX(v) => write!(f, "=\"{}\"", v),
            Attribute::TargetY(v) => write!(f, "=\"{}\"", v),
            Attribute::EdgeMode(v) => write!(f, "=\"{}\"", v),
            Attribute::KernelUnitLength(a, None) => write!(f, "=\"{}\"", a),
            Attribute::KernelUnitLength(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::PreserveAlpha(v) => write!(f, "=\"{}\"", if *v { "1" } else { "0" }),
            Attribute::SurfaceScale(v) => write!(f, "=\"{}\"", v),
            Attribute::DiffuseConstant(v) => write!(f, "=\"{}\"", v),
            Attribute::Scale(v) => write!(f, "=\"{}\"", v),
            Attribute::XChannelSelector(v) => write!(f, "=\"{}\"", v),
            Attribute::YChannelSelector(v) => write!(f, "=\"{}\"", v),
            Attribute::Dx(v) => write!(f, "=\"{}\"", v),
            Attribute::Dy(v) => write!(f, "=\"{}\"", v),
            Attribute::StdDeviation(a, None) => write!(f, "=\"{}\"", a),
            Attribute::StdDeviation(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::CrossOrigin(v) => write!(f, "=\"{}\"", v),
            Attribute::Radius(a, None) => write!(f, "=\"{}\"", a),
            Attribute::Radius(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::SpecularConstant(v) => write!(f, "=\"{}\"", v),
            Attribute::SpecularExponent(v) => write!(f, "=\"{}\"", v),
            Attribute::Azimuth(v) => write!(f, "=\"{}\"", v),
            Attribute::Elevation(v) => write!(f, "=\"{}\"", v),
            Attribute::Z(v) => write!(f, "=\"{}\"", v),
            Attribute::PointsAtX(v) => write!(f, "=\"{}\"", v),
            Attribute::PointsAtY(v) => write!(f, "=\"{}\"", v),
            Attribute::PointsAtZ(v) => write!(f, "=\"{}\"", v),
            Attribute::LimitingConeAngle(v) => write!(f, "=\"{}\"", v),
            Attribute::BaseFrequency(a, None) => write!(f, "=\"{}\"", a),
            Attribute::BaseFrequency(a, Some(b)) => write!(f, "=\"{} {}\"", a, b),
            Attribute::NumOctaves(v) => write!(f, "=\"{}\"", v),
            Attribute::Seed(v) => write!(f, "=\"{}\"", v),
            Attribute::StitchTiles(v) => write!(f, "=\"{}\"", v),
            Attribute::GradientUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::GradientTransform(v) => write!(f, "=\"{}\"", v),
            Attribute::SpreadMethod(v) => write!(f, "=\"{}\"", v),
            Attribute::Fx(v) => write!(f, "=\"{}\"", v),
            Attribute::Fy(v) => write!(f, "=\"{}\"", v),
            Attribute::Fr(v) => write!(f, "=\"{}\"", v),
            Attribute::Decoding(v) => write!(f, "=\"{}\"", v),
            Attribute::FetchPriority(v) => write!(f, "=\"{}\"", v),
            Attribute::LengthAdjust(v) => write!(f, "=\"{}\"", v),
            Attribute::TextLength(v) => write!(f, "=\"{}\"", v),
            Attribute::ClipPathUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::Method(v) => write!(f, "=\"{}\"", v),
            Attribute::Side(v) => write!(f, "=\"{}\"", v),
            Attribute::Spacing(v) => write!(f, "=\"{}\"", v),
            Attribute::StartOffset(v) => write!(f, "=\"{}\"", v),
            Attribute::FilterUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::PrimitiveUnits(v) => write!(f, "=\"{}\"", v),
            Attribute::Version(v) => write!(f, "=\"{}\"", v),
        }
    }

    pub fn write_svg(&self, f: &mut impl fmt::Write) -> fmt::Result {
        write!(f, "{}", self.name())?;
        self.write_value(f)
    }
}
