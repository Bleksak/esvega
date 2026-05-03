# SVG 2 CSS Properties

Comprehensive documentation of all CSS properties that can be applied to SVG elements.

## Table of Contents

- [Painting Properties](#painting-properties)
- [Text Properties](#text-properties)
- [Effects Properties](#effects-properties)
- [Graphics Properties](#graphics-properties)
- [Styling Properties](#styling-properties)

---

## Painting Properties

### `fill`

**Syntax:**
```
fill = <color> | <url> | none | inherit
```

**Initial Value:** `rgb(0, 0, 0)` (black)

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the color or paint method used to fill the interior of shapes and text.

**Values:**
- `<color>` - Any CSS color value
- `<url>` - Reference to a `<paint>` element (gradient, pattern)
- `none` - No fill

**Examples:**
```css
rect { fill: red; }
circle { fill: url(#myGradient); }
text { fill: rgba(255, 0, 0, 0.5); }
```

**References:**
- [W3C SVG 2 - Fill Property](https://www.w3.org/TR/SVG2/painting.html#FillProperty)

---

### `fill-opacity`

**Syntax:**
```
fill-opacity = <number> | inherit
```

**Initial Value:** `1`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the opacity of the fill paint.

**Values:**
- `<number>` - Value from 0 (fully transparent) to 1 (fully opaque)

**Examples:**
```css
rect { fill-opacity: 0.5; }
circle { fill-opacity: 100%; } /* Percentage equivalent */
```

**References:**
- [W3C SVG 2 - Fill Opacity Property](https://www.w3.org/TR/SVG2/painting.html#FillOpacityProperty)

---

### `fill-rule`

**Syntax:**
```
fill-rule = nonzero | evenodd | inherit
```

**Initial Value:** `nonzero`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the algorithm used to determine which parts of the canvas are inside a shape.

**Values:**
- `nonzero` - Non-zero winding number rule (default)
- `evenodd` - Even-odd rule

**Examples:**
```css
polygon { fill-rule: nonzero; }
path { fill-rule: evenodd; }
```

**References:**
- [W3C SVG 2 - Fill Rule Property](https://www.w3.org/TR/SVG2/painting.html#FillRuleProperty)

---

### `stroke`

**Syntax:**
```
stroke = <color> | <url> | none | inherit
```

**Initial Value:** `none`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the color or paint method used to stroke the outline of shapes and text.

**Values:**
- `<color>` - Any CSS color value
- `<url>` - Reference to a `<paint>` element (gradient, pattern)
- `none` - No stroke

**Examples:**
```css
rect { stroke: blue; }
circle { stroke: url(#myGradient); }
line { stroke: rgba(0, 0, 255, 0.5); }
```

**References:**
- [W3C SVG 2 - Stroke Property](https://www.w3.org/TR/SVG2/painting.html#StrokeProperty)

---

### `stroke-width`

**Syntax:**
```
stroke-width = <length> | <percentage> | <number> | inherit
```

**Initial Value:** `1`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the width of the stroke.

**Values:**
- `<length>` - Absolute length value
- `<percentage>` - Percentage of current font-size
- `<number>` - Multiplier of current stroke width

**Examples:**
```css
line { stroke-width: 2px; }
circle { stroke-width: 0.5; } /* Half of default */
rect { stroke-width: 5%; }
```

**References:**
- [W3C SVG 2 - Stroke Width Property](https://www.w3.org/TR/SVG2/painting.html#StrokeWidthProperty)

---

### `stroke-linecap`

**Syntax:**
```
stroke-linecap = butt | round | square | inherit
```

**Initial Value:** `butt`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the shape of the end caps of stroke paths.

**Values:**
- `butt` - No cap, ends at endpoint (default)
- `round` - Rounded cap
- `square` - Square cap extending beyond endpoint

**Examples:**
```css
line { stroke-linecap: round; }
path { stroke-linecap: square; }
polyline { stroke-linecap: butt; }
```

**References:**
- [W3C SVG 2 - Stroke Line Cap Property](https://www.w3.org/TR/SVG2/painting.html#StrokeLinecapProperty)

---

### `stroke-linejoin`

**Syntax:**
```
stroke-linejoin = miter | round | bevel | inherit
```

**Initial Value:** `miter`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the shape of the joins between path segments.

**Values:**
- `miter` - Sharp corner (default)
- `round` - Rounded corner
- `bevel` - Beveled corner

**Examples:**
```css
polygon { stroke-linejoin: round; }
path { stroke-linejoin: bevel; }
```

**References:**
- [W3C SVG 2 - Stroke Line Join Property](https://www.w3.org/TR/SVG2/painting.html#StrokeLinejoinProperty)

---

### `stroke-miterlimit`

**Syntax:**
```
stroke-miterlimit = <number> | inherit
```

**Initial Value:** `4`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Limits the ratio of miter length to stroke width for `stroke-linejoin: miter`.

**Values:**
- `<number>` - Ratio threshold (default 4)

**Examples:**
```css
polygon { stroke-miterlimit: 2; }
path { stroke-miterlimit: 10; }
```

**References:**
- [W3C SVG 2 - Stroke Miterlimit Property](https://www.w3.org/TR/SVG2/painting.html#StrokeMiterlimitProperty)

---

### `stroke-dasharray`

**Syntax:**
```
stroke-dasharray = <dash-array> | none | inherit
<dash-array> = <length> [<length>]*
```

**Initial Value:** `none`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies a dashed stroke pattern.

**Values:**
- `<dash-array>` - Space-separated list of lengths (dash, gap, dash, gap...)
- `none` - Solid stroke

**Examples:**
```css
path { stroke-dasharray: 5px 3px; }
line { stroke-dasharray: 10, 5, 2, 5; } /* Comma-separated also works */
rect { stroke-dasharray: 10px; } /* Single dash value */
```

**References:**
- [W3C SVG 2 - Stroke Dasharray Property](https://www.w3.org/TR/SVG2/painting.html#StrokeDasharrayProperty)

---

### `stroke-dashoffset`

**Syntax:**
```
stroke-dashoffset = <length> | <number> | inherit
```

**Initial Value:** `0`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the offset for the dashed stroke pattern.

**Values:**
- `<length>` - Absolute length offset
- `<number>` - Number of dash pattern units to offset

**Examples:**
```css
path { stroke-dashoffset: 5px; }
line { stroke-dashoffset: 10; }
```

**References:**
- [W3C SVG 2 - Stroke Dashoffset Property](https://www.w3.org/TR/SVG2/painting.html#StrokeDashoffsetProperty)

---

### `stroke-opacity`

**Syntax:**
```
stroke-opacity = <number> | inherit
```

**Initial Value:** `1`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the opacity of the stroke paint.

**Values:**
- `<number>` - Value from 0 (fully transparent) to 1 (fully opaque)

**Examples:**
```css
rect { stroke-opacity: 0.5; }
circle { stroke-opacity: 0.8; }
```

**References:**
- [W3C SVG 2 - Stroke Opacity Property](https://www.w3.org/TR/SVG2/painting.html#StrokeOpacityProperty)

---

### `flood-color`

**Syntax:**
```
flood-color = <color> | inherit
```

**Initial Value:** `black`

**Applies to:** Filter primitive elements (feFlood, feGaussianBlur, feMerge)

**Animatable:** Yes

**Description:**
Specifies the color used by feFlood filter primitive.

**References:**
- [W3C SVG 2 - Flood Color Property](https://www.w3.org/TR/SVG2/filters.html#FloodColorProperty)

---

### `flood-opacity`

**Syntax:**
```
flood-opacity = <number> | inherit
```

**Initial Value:** `1`

**Applies to:** Filter primitive elements

**Animatable:** Yes

**Description:**
Specifies the opacity used by feFlood filter primitive.

**References:**
- [W3C SVG 2 - Flood Opacity Property](https://www.w3.org/TR/SVG2/filters.html#FloodOpacityProperty)

---

### `lighting-color`

**Syntax:**
```
lighting-color = <color> | inherit
```

**Initial Value:** `white`

**Applies to:** Filter primitive elements (feDiffuseLighting, feSpecularLighting)

**Animatable:** Yes

**Description:**
Specifies the color used by lighting filter primitives.

**References:**
- [W3C SVG 2 - Lighting Color Property](https://www.w3.org/TR/SVG2/filters.html#LightingColorProperty)

---

### `paint-order`

**Syntax:**
```
paint-order = normal | [fill | stroke | markers] [fill | stroke | markers] [fill | stroke | markers] | inherit
```

**Initial Value:** `normal`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the order in which the following painting properties should be applied: fill, stroke, and markers.

**Values:**
- `normal` - Default painting order (fill, stroke, markers)
- List - Custom painting order (e.g., `stroke fill`)

**Examples:**
```css
text { paint-order: stroke fill; } /* Stroke before fill */
rect { paint-order: markers fill stroke; }
```

**References:**
- [CSS Painting Order](https://www.w3.org/TR/css-paint-api-1/#paint-order-property)

---

## Text Properties

### `font-family`

**Syntax:**
```
font-family = <family-name> | <generic-family> | inherit
```

**Initial Value:** Depends on user agent

**Applies to:** Text elements (text, tspan, textPath)

**Animatable:** Yes (discrete)

**Description:**
Specifies the font family or families to use.

**Generic Families:**
- `serif`
- `sans-serif`
- `monospace`
- `cursive`
- `fantasy`
- `system-ui`

**Examples:**
```css
text { font-family: Arial, sans-serif; }
tspan { font-family: "Times New Roman", serif; }
```

**References:**
- [CSS Fonts Module - Font Family](https://www.w3.org/TR/css-fonts-4/#font-family-prop)

---

### `font-style`

**Syntax:**
```
font-style = normal | italic | oblique [ <angle> ] | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies whether to use a normal, italic, or oblique face for a font.

**Values:**
- `normal` - Normal face
- `italic` - Italic face
- `oblique` - Oblique face (optionally with angle)

**Examples:**
```css
text { font-style: italic; }
tspan { font-style: oblique 10deg; }
```

**References:**
- [CSS Fonts Module - Font Style](https://www.w3.org/TR/css-fonts-4/#font-style-prop)

---

### `font-variant`

**Syntax:**
```
font-variant = normal | small-caps | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies whether to use a condensed font from a condensed family.

**Values:**
- `normal` - Normal character presentation
- `small-caps` - Small capitals

**Examples:**
```css
text { font-variant: small-caps; }
```

**References:**
- [CSS Fonts Module - Font Variant](https://www.w3.org/TR/css-fonts-4/#font-variant-prop)

---

### `font-weight`

**Syntax:**
```
font-weight = normal | bold | bolder | lighter | <number> | inherit
```

**Initial Value:** `normal` (400)

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the weight of the font.

**Values:**
- `normal` - Normal weight (400)
- `bold` - Bold weight (700)
- `<number>` - Weight from 1 to 1000

**Examples:**
```css
text { font-weight: bold; }
tspan { font-weight: 300; }
```

**References:**
- [CSS Fonts Module - Font Weight](https://www.w3.org/TR/css-fonts-4/#font-weight-prop)

---

### `font-size`

**Syntax:**
```
font-size = <absolute-size> | <relative-size> | <length> | inherit
```

**Initial Value:** `medium`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the size of the font.

**Absolute Sizes:**
- `xx-small`
- `x-small`
- `small`
- `medium`
- `large`
- `x-large`
- `xx-large`

**Relative Sizes:**
- `larger`
- `smaller`

**Examples:**
```css
text { font-size: 12px; }
tspan { font-size: 1.2em; }
```

**References:**
- [CSS Fonts Module - Font Size](https://www.w3.org/TR/css-fonts-4/#font-size-prop)

---

### `font-stretch`

**Syntax:**
```
font-stretch = normal | ultra-condensed | extra-condensed | condensed | semi-condensed | semi-expanded | expanded | extra-expanded | ultra-expanded | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the width of the font.

**Examples:**
```css
text { font-stretch: condensed; }
```

**References:**
- [CSS Fonts Module - Font Stretch](https://www.w3.org/TR/css-fonts-4/#font-stretch-prop)

---

### `letter-spacing`

**Syntax:**
```
letter-spacing = normal | <length> | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the spacing between characters.

**Values:**
- `normal` - Normal spacing
- `<length>` - Extra space or negative space

**Examples:**
```css
text { letter-spacing: 2px; }
tspan { letter-spacing: -1px; }
```

**References:**
- [CSS Fonts Module - Letter Spacing](https://www.w3.org/TR/css-fonts-4/#letter-spacing-prop)

---

### `word-spacing`

**Syntax:**
```
word-spacing = normal | <length> | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the spacing between words.

**Examples:**
```css
text { word-spacing: 5px; }
```

**References:**
- [CSS Fonts Module - Word Spacing](https://www.w3.org/TR/css-fonts-4/#word-spacing-prop)

---

### `text-decoration`

**Syntax:**
```
text-decoration = <text-decoration-line> || <text-decoration-style> || <text-decoration-color>
```

**Initial Value:** Varies (usually `none`)

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies decoration on text.

**Values:**
- `<text-decoration-line>`: `none` | `underline` | `overline` | `line-through`
- `<text-decoration-style>`: `solid` | `double` | `dotted` | `dashed` | `wavy`
- `<text-decoration-color>`: Any color

**Examples:**
```css
text { text-decoration: underline; }
tspan { text-decoration: 2px solid red; }
```

**References:**
- [CSS Text Module - Text Decoration](https://www.w3.org/TR/css-text-3/#text-decoration-property)

---

### `text-anchor`

**Syntax:**
```
text-anchor = start | middle | end | inherit
```

**Initial Value:** `start`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies how text is anchored horizontally.

**Values:**
- `start` - Text starts at the position (default)
- `middle` - Text centered at position
- `end` - Text ends at position

**Examples:**
```css
text { text-anchor: middle; }
tspan { text-anchor: end; }
```

**References:**
- [W3C SVG 2 - Text Anchor Property](https://www.w3.org/TR/SVG2/text.html#TextAnchorProperty)

---

### `dominant-baseline`

**Syntax:**
```
dominant-baseline = auto | text-bottom | alphabetic | ideographic | middle | central | mathematical | text-top | inherit
```

**Initial Value:** `auto`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the baseline alignment of text.

**Values:**
- `auto` - Browser determines baseline (default)
- `text-bottom` - Text bottom baseline
- `alphabetic` - Alphabetic baseline (default for SVG)
- `ideographic` - Ideographic baseline
- `middle` - Middle of the em square
- `central` - Center of the em square
- `mathematical` - Mathematical baseline
- `text-top` - Text top

**Examples:**
```css
text { dominant-baseline: middle; }
tspan { dominant-baseline: central; }
```

**References:**
- [W3C SVG 2 - Dominant Baseline Property](https://www.w3.org/TR/SVG2/text.html#DominantBaselineProperty)

---

### `alignment-baseline`

**Syntax:**
```
alignment-baseline = auto | baseline | before-edge | text-before-edge | middle | central | after-edge | text-after-edge | ideographic | alphabetic | hanging | mathematical | inherit
```

**Initial Value:** `auto`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies baseline alignment.

**Examples:**
```css
text { alignment-baseline: alphabetic; }
```

**References:**
- [CSS Line Box Alignment](https://www.w3.org/TR/css-align-3/#valdef-align-baseline-alphabetic)

---

### `writing-mode`

**Syntax:**
```
writing-mode = horizontal-tb | vertical-rl | vertical-lr | inherit
```

**Initial Value:** `horizontal-tb`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies whether lines of text are laid out horizontally or vertically.

**Values:**
- `horizontal-tb` - Horizontal text, top-to-bottom (default)
- `vertical-rl` - Vertical text, right-to-left
- `vertical-lr` - Vertical text, left-to-right

**Examples:**
```css
text { writing-mode: vertical-rl; }
```

**References:**
- [W3C SVG 2 - Writing Mode Property](https://www.w3.org/TR/SVG2/text.html#WritingModeProperty)

---

### `direction`

**Syntax:**
```
direction = ltr | rtl | inherit
```

**Initial Value:** `ltr`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the direction of text flow.

**Values:**
- `ltr` - Left-to-right (default)
- `rtl` - Right-to-left

**Examples:**
```css
text { direction: rtl; }
```

**References:**
- [CSS Basic UI Module - Direction](https://www.w3.org/TR/css-ui-4/#propdef-direction)

---

### `unicode-bidi`

**Syntax:**
```
unicode-bidi = normal | embed | bidi-override | inherit
```

**Initial Value:** `normal`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies how to handle bidirectional text.

**Values:**
- `normal` - No special handling (default)
- `embed` - Generate an embedding
- `bidi-override` - Generate an override

**Examples:**
```css
text { unicode-bidi: embed; }
```

**References:**
- [CSS Basic UI Module - Unicode Bidi](https://www.w3.org/TR/css-ui-4/#propdef-unicode-bidi)

---

### `text-transform`

**Syntax:**
```
text-transform = none | uppercase | lowercase | capitalize | full-width | inherit
```

**Initial Value:** `none`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies text capitalization.

**Values:**
- `none` - No transformation (default)
- `uppercase` - Convert to uppercase
- `lowercase` - Convert to lowercase
- `capitalize` - Convert first character to uppercase
- `full-width` - Use full-width forms (CJK)

**Examples:**
```css
text { text-transform: uppercase; }
tspan { text-transform: lowercase; }
```

**References:**
- [CSS Fonts Module - Text Transform](https://www.w3.org/TR/css-fonts-4/#text-transform-prop)

---

## Effects Properties

### `opacity`

**Syntax:**
```
opacity = <number> | inherit
```

**Initial Value:** `1`

**Applies to:** All elements

**Animatable:** Yes

**Description:**
Specifies the transparency of an element and its children.

**Values:**
- `<number>` - Value from 0 (fully transparent) to 1 (fully opaque)

**Examples:**
```css
rect { opacity: 0.5; }
circle { opacity: 0.8; }
```

**References:**
- [W3C SVG 2 - Opacity Property](https://www.w3.org/TR/SVG2/painting.html#OpacityProperty)

---

### `display`

**Syntax:**
```
display = inline | block | list-item | inline-block | inline-list-item | run-in | compact | marker | table | inline-table | table-row-group | table-header-group | table-footer-group | table-row | table-column-group | table-column | table-cell | table-caption | grid | inline-flex | flex | none | inherit
```

**Initial Value:** Varies by element

**Applies to:** All elements

**Animatable:** No

**Description:**
Controls whether an element is rendered and how it affects layout.

**Common Values:**
- `inline` - Inline element (default)
- `block` - Block element
- `none` - Not rendered

**Examples:**
```css
text { display: inline; }
g { display: block; }
circle { display: none; }
```

**References:**
- [W3C SVG 2 - Display Property](https://www.w3.org/TR/SVG2/painting.html#DisplayProperty)

---

### `visibility`

**Syntax:**
```
visibility = visible | hidden | collapse | inherit
```

**Initial Value:** `visible`

**Applies to:** All elements

**Animatable:** Yes

**Description:**
Controls whether an element is rendered.

**Values:**
- `visible` - Visible (default)
- `hidden` - Hidden but occupies space
- `collapse` - Collapse (table-specific)

**Examples:**
```css
circle { visibility: hidden; }
```

**References:**
- [W3C SVG 2 - Visibility Property](https://www.w3.org/TR/SVG2/painting.html#VisibilityProperty)

---

### `clip`

**Syntax:**
```
clip = <shape> | auto | inherit
<shape> = rect(<top> <right> <bottom> <left>) | ellipse(<radius-x> <radius-y> at <position>)
```

**Initial Value:** `auto`

**Applies to:** All elements

**Animatable:** Yes

**Description:**
Clips an element to a specific shape.

**Examples:**
```css
rect { clip: rect(10px, 50px, 100px, 20px); }
circle { clip: ellipse(50% 50%); }
```

**References:**
- [CSS Overflow Module - Clip](https://www.w3.org/TR/css-overflow-3/#the-clip-property)

---

### `clip-path`

**Syntax:**
```
clip-path = <url> | <clip-source> | none | inherit
```

**Initial Value:** `none`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Defines a clipping path for an element.

**Values:**
- `<url>` - Reference to a `<clipPath>` element
- `<clip-source>` - Reference to another element's bounding box
- `none` - No clipping

**Examples:**
```css
rect { clip-path: url(#myClip); }
circle { clip-path: polygon(50% 0%, 0% 100%, 100% 100%); }
```

**References:**
- [W3C SVG 2 - Clip Path Property](https://www.w3.org/TR/SVG2/masking.html#ClipPathProperty)

---

### `clip-rule`

**Syntax:**
```
clip-rule = nonzero | evenodd | inherit
```

**Initial Value:** `nonzero`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies the algorithm used to determine the interior of a clipping path.

**Values:**
- `nonzero` - Non-zero winding number rule (default)
- `evenodd` - Even-odd rule

**Examples:**
```css
circle { clip-rule: evenodd; }
```

**References:**
- [W3C SVG 2 - Clip Rule Property](https://www.w3.org/TR/SVG2/masking.html#ClipRuleProperty)

---

### `mask`

**Syntax:**
```
mask = <mask-reference> | none | inherit
<mask-reference> = <url> | <css-mask>
```

**Initial Value:** `none`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Applies a mask to an element.

**Examples:**
```css
rect { mask: url(#myMask); }
```

**References:**
- [CSS Masking Module - Mask](https://www.w3.org/TR/masking-1/#the-mask-property)

---

### `filter`

**Syntax:**
```
filter = <url> | <filter-function-list> | none | inherit
```

**Initial Value:** `none`

**Applies to:** All elements

**Animatable:** Yes

**Description:**
Applies filter effects (blur, drop shadow, etc.) to an element.

**Values:**
- `<url>` - Reference to a `<filter>` element
- `<filter-function-list>` - CSS filter functions (e.g., `blur(5px)`, `drop-shadow(10px 10px 5px rgba(0,0,0,0.5))`)
- `none` - No filter

**Examples:**
```css
rect { filter: url(#myBlur); }
circle { filter: blur(5px); }
text { filter: drop-shadow(2px 2px 3px rgba(0,0,0,0.5)); }
```

**References:**
- [W3C SVG 2 - Filter Property](https://www.w3.org/TR/SVG2/filters.html#FilterProperty)
- [CSS Filter Effects Module - Filter](https://www.w3.org/TR/filter-effects-1/#filter-property)

---

### `enable-background`

**Syntax:**
```
enable-background = new | accumulate | <length> | <coordinate>
```

**Initial Value:** `accumulate`

**Applies to:** All elements

**Animatable:** Yes

**Description:**
Specifies how to handle the background for filter effects.

**Examples:**
```css
g { enable-background: new 0 0 100 100; }
```

**References:**
- [W3C SVG 2 - Enable Background Property](https://www.w3.org/TR/SVG2/filters.html#EnableBackgroundProperty)

---

## Graphics Properties

### `vector-effect`

**Syntax:**
```
vector-effect = none | non-scaling-stroke | non-scaling-size | non-rotation | fixed-position | inherit
```

**Initial Value:** `none`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies vector effects for non-raster rendering.

**Values:**
- `none` - Normal vector effects (default)
- `non-scaling-stroke` - Stroke width doesn't scale with zoom
- `non-scaling-size` - Element size doesn't scale
- `non-rotation` - Element doesn't rotate
- `fixed-position` - Element stays fixed during panning

**Examples:**
```css
path { vector-effect: non-scaling-stroke; }
text { vector-effect: non-scaling-size; }
```

**References:**
- [W3C SVG 2 - Vector Effect Property](https://www.w3.org/TR/SVG2/painting.html#VectorEffectProperty)

---

### `shape-rendering`

**Syntax:**
```
shape-rendering = auto | optimizeSpeed | crispEdges | geometricPrecision | inherit
```

**Initial Value:** `auto`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Provides a hint to the renderer about how to balance rendering speed and quality.

**Values:**
- `auto` - Renderer chooses (default)
- `optimizeSpeed` - Prioritize rendering speed
- `crispEdges` - Minimize anti-aliasing for sharp edges
- `geometricPrecision` - Prioritize geometric accuracy

**Examples:**
```css
rect { shape-rendering: crispEdges; }
path { shape-rendering: geometricPrecision; }
```

**References:**
- [W3C SVG 2 - Shape Rendering Property](https://www.w3.org/TR/SVG2/painting.html#ShapeRenderingProperty)

---

### `text-rendering`

**Syntax:**
```
text-rendering = auto | optimizeSpeed | optimizeLegibility | geometricPrecision | inherit
```

**Initial Value:** `auto`

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Provides a hint to the renderer about how to balance rendering speed and text legibility.

**Values:**
- `auto` - Browser chooses (default)
- `optimizeSpeed` - Prioritize speed
- `optimizeLegibility` - Prioritize readability (kerning, ligatures)
- `geometricPrecision` - Prioritize geometric accuracy

**Examples:**
```css
text { text-rendering: optimizeLegibility; }
```

**References:**
- [W3C SVG 2 - Text Rendering Property](https://www.w3.org/TR/SVG2/text.html#TextRenderingProperty)

---

### `image-rendering`

**Syntax:**
```
image-rendering = auto | optimizeSpeed | optimizeQuality | inherit
```

**Initial Value:** `auto`

**Applies to:** Image elements

**Animatable:** Yes

**Description:**
Provides a hint to the renderer about how to balance rendering speed and image quality.

**Values:**
- `auto` - Browser chooses (default)
- `optimizeSpeed` - Fast scaling, lower quality
- `optimizeQuality` - High-quality scaling

**Examples:**
```css
image { image-rendering: optimizeQuality; }
```

**References:**
- [W3C SVG 2 - Image Rendering Property](https://www.w3.org/TR/SVG2/painting.html#ImageRenderingProperty)

---

### `pointer-events`

**Syntax:**
```
pointer-events = visiblePainted | visibleFill | visibleStroke | visible | painted | fill | stroke | all | none | inherit
```

**Initial Value:** `visiblePainted`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Determines whether an element responds to pointer events.

**Values:**
- `visiblePainted` - Events on painted areas (default)
- `visibleFill` - Events on fill areas
- `visibleStroke` - Events on stroke areas
- `visible` - Events on visible areas
- `painted` - Events on painted areas (including invisible)
- `fill` - Events on fill areas
- `stroke` - Events on stroke areas
- `all` - Events on all areas
- `none` - No events

**Examples:**
```css
circle { pointer-events: none; }
rect { pointer-events: visibleStroke; }
```

**References:**
- [W3C SVG 2 - Pointer Events Property](https://www.w3.org/TR/SVG2/interact.html#PointerEventsProperty)

---

## Styling Properties

### `overflow`

**Syntax:**
```
overflow = visible | hidden | scroll | auto | clip | inherit
```

**Initial Value:** `visible`

**Applies to:** Container elements

**Animatable:** Yes

**Description:**
Specifies what to do when content overflows its container.

**Values:**
- `visible` - Content visible (default)
- `hidden` - Content clipped
- `scroll` - Scrollbars shown
- `auto` - Scrollbars only if needed
- `clip` - Content clipped (no scrollbars)

**Examples:**
```css
svg { overflow: hidden; }
```

**References:**
- [W3C SVG 2 - Overflow Property](https://www.w3.org/TR/SVG2/interact.html#OverflowProperty)

---

### `color`

**Syntax:**
```
color = <color> | inherit
```

**Initial Value:** Depends on user agent

**Applies to:** Text elements

**Animatable:** Yes

**Description:**
Specifies the current color.

**Examples:**
```css
text { color: blue; }
```

**References:**
- [CSS Color Module - Color](https://www.w3.org/TR/css-color-4/#color-val)

---

### `color-interpolation`

**Syntax:**
```
color-interpolation = sRGB | linearRGB | inherit
```

**Initial Value:** `sRGB`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Specifies which color space to use for interpolating colors.

**Values:**
- `sRGB` - Standard RGB color space (default)
- `linearRGB` - Linear RGB color space

**Examples:**
```css
svg { color-interpolation: linearRGB; }
```

**References:**
- [W3C SVG 2 - Color Interpolation Property](https://www.w3.org/TR/SVG2/painting.html#ColorInterpolationProperty)

---

### `color-rendering`

**Syntax:**
```
color-rendering = auto | optimizeSpeed | optimizeQuality | inherit
```

**Initial Value:** `auto`

**Applies to:** All graphics elements

**Animatable:** Yes

**Description:**
Provides a hint to the renderer about how to balance color rendering speed and quality.

**Examples:**
```css
svg { color-rendering: optimizeQuality; }
```

**References:**
- [W3C SVG 2 - Color Rendering Property](https://www.w3.org/TR/SVG2/painting.html#ColorRenderingProperty)

---

### `shape-image-threshold`

**Syntax:**
```
shape-image-threshold = <number> | inherit
```

**Initial Value:** `0.0`

**Applies to:** Image elements

**Animatable:** Yes

**Description:**
Specifies the transparency threshold for creating shapes from images.

**Values:**
- `<number>` - Value from 0.0 (fully transparent) to 1.0 (fully opaque)

**Examples:**
```css
image { shape-image-threshold: 0.5; }
```

**References:**
- [CSS Sizing Module - Shape Image Threshold](https://www.w3.org/TR/css-shapes-1/#shape-image-threshold-property)

---

### `shape-margin`

**Syntax:**
```
shape-margin = <length> | <percentage> | inherit
```

**Initial Value:** `0`

**Applies to:** Image elements

**Animatable:** Yes

**Description:**
Specifies a margin around the shape created from an image.

**Examples:**
```css
image { shape-margin: 10px; }
```

**References:**
- [CSS Sizing Module - Shape Margin](https://www.w3.org/TR/css-shapes-1/#shape-margin-property)

---

## Appendix: Property Tables

### All SVG Presentation Attributes as CSS Properties

| Property | SVG Attribute | Initial Value | Animatable |
|----------|---------------|---------------|------------|
| `fill` | fill | `rgb(0, 0, 0)` | Yes |
| `fill-opacity` | fill-opacity | `1` | Yes |
| `fill-rule` | fill-rule | `nonzero` | Yes |
| `stroke` | stroke | `none` | Yes |
| `stroke-width` | stroke-width | `1` | Yes |
| `stroke-linecap` | stroke-linecap | `butt` | Yes |
| `stroke-linejoin` | stroke-linejoin | `miter` | Yes |
| `stroke-miterlimit` | stroke-miterlimit | `4` | Yes |
| `stroke-dasharray` | stroke-dasharray | `none` | Yes |
| `stroke-dashoffset` | stroke-dashoffset | `0` | Yes |
| `stroke-opacity` | stroke-opacity | `1` | Yes |
| `opacity` | opacity | `1` | Yes |
| `display` | display | Varies | No |
| `visibility` | visibility | `visible` | Yes |
| `clip-path` | clip-path | `none` | Yes |
| `clip-rule` | clip-rule | `nonzero` | Yes |
| `mask` | mask | `none` | Yes |
| `filter` | filter | `none` | Yes |
| `color` | color | Varies | Yes |
| `color-interpolation` | color-interpolation | `sRGB` | Yes |
| `shape-rendering` | shape-rendering | `auto` | Yes |
| `text-rendering` | text-rendering | `auto` | Yes |
| `image-rendering` | image-rendering | `auto` | Yes |
| `pointer-events` | pointer-events | `visiblePainted` | Yes |
| `vector-effect` | vector-effect | `none` | Yes |

---

*Generated from W3C SVG 2 Specification and related CSS modules. For the most up-to-date information, always refer to the official specifications.*
