# SVG 2 Data Types

Comprehensive documentation of all SVG data types based on CSS Value Definition Syntax and SVG 2 specification.

## Table of Contents

- [Basic Types](#basic-types)
- [Number Types](#number-types)
- [Length Types](#length-types)
- [Angle Types](#angle-types)
- [Color Types](#color-types)
- [Coordinate Types](#coordinate-types)
- [List Types](#list-types)
- [Special Types](#special-types)
- [Enumeration Types](#enumeration-types)
- [Interface Definitions](#interface-definitions)

---

## Basic Types

### `<boolean>`

**Definition:** Represents a boolean value.

**Syntax:**
```
<boolean> = "true" | "false"
```

**Initial Value:** See individual attributes

**Animatable:** Yes (for discrete animation, value transitions between 0 and 1)

**Examples:**
- `"true"` - enabled state
- `"false"` - disabled state

**References:**
- [W3C SVG 2 - Boolean](https://www.w3.org/TR/SVG2/types.html#DataTypeBoolean)

---

### `<integer>`

**Definition:** An integer value (can be negative).

**Syntax:**
```
<integer> = [-]?[0-9]+
```

**Initial Value:** See individual attributes

**Animatable:** Yes

**Examples:**
- `0`
- `42`
- `-17`

**References:**
- [W3C SVG 2 - Integer](https://www.w3.org/TR/SVG2/types.html#DataTypeInteger)

---

### `<number>`

**Definition:** A floating-point number.

**Syntax:**
```
<number> = [-]?[0-9]*\.[0-9]+([eE][+-]?[0-9]+)? | [-]?[0-9]+([eE][+-]?[0-9]+)?
```

**Initial Value:** See individual attributes

**Animatable:** Yes

**Examples:**
- `0.5`
- `3.14159`
- `-2.5e-3`
- `100`

**References:**
- [W3C SVG 2 - Number](https://www.w3.org/TR/SVG2/types.html#DataTypeNumber)

---

### `<string>`

**Definition:** A string value.

**Syntax:**
```
<string> = any unicode string (quoted in CSS syntax)
```

**Initial Value:** See individual attributes

**Animatable:** No (except for discrete string interpolation in some cases)

**Examples:**
- `"hello"`
- `"test value"`

**References:**
- [W3C SVG 2 - String](https://www.w3.org/TR/SVG2/types.html#DataTypeString)

---

### `<url>`

**Definition:** A Uniform Resource Locator reference.

**Syntax:**
```
<url> = url("#" <target>) | url(<uri>)
```

**Target Types:**
- Fragment identifiers pointing to elements with `id` attributes
- External resources (images, stylesheets, etc.)

**Initial Value:** See individual attributes

**Animatable:** Yes (for discrete URL changes)

**Examples:**
- `url(#myGradient)` - internal reference
- `url(#rect1)` - reference to element with id="rect1"
- `url(images/logo.svg)` - external reference

**References:**
- [W3C SVG 2 - URL](https://www.w3.org/TR/SVG2/types.html#DataTypeURL)

---

## Length Types

### `<length>`

**Definition:** A distance value. Can be specified with various units.

**Syntax:**
```
<length> = <number><unit>
<unit> = px | pt | pc | mm | cm | in | cm | in | cm | in | rem | em | ex | ch | vw | vh | vmin | vmax
```

**Supported Units:**

| Unit | Type | Description |
|------|------|-------------|
| `px` | pixel | CSS pixels (1/96th of an inch) |
| `pt` | point | 1/72nd of an inch |
| `pc` | pica | 12 points |
| `mm` | millimeter | 1/25.4th of an inch |
| `cm` | centimeter | 10 millimeters |
| `in` | inch | 2.54 centimeters |
| `rem` | root em | Relative to root element's font-size |
| `em` | em | Relative to current element's font-size |
| `ex` | ex | Relative to x-height of current font |
| `ch` | ch | Relative to width of "0" glyph |
| `vw` | view width | 1% of viewport width |
| `vh` | view height | 1% of viewport height |
| `vmin` | view min | 1% of smaller viewport dimension |
| `vmax` | view max | 1% of larger viewport dimension |

**Initial Value:** See individual attributes

**Animatable:** Yes

**Notes:**
- When `<length>` appears as a standalone type, it means any length value
- When part of a union (e.g., `<length> | <percentage>`), additional constraints may apply
- Negative lengths are generally not allowed unless explicitly permitted

**References:**
- [W3C SVG 2 - Length](https://www.w3.org/TR/SVG2/types.html#DataTypeLength)

---

### `<percentage>`

**Definition:** A percentage value.

**Syntax:**
```
<percentage> = <number>%
```

**Initial Value:** See individual attributes

**Animatable:** Yes

**Base for Percentage Calculation:**
- Depends on the attribute context:
  - For dimensions: base is the relevant dimension of the bounding box
  - For offsets: base is the relevant container dimension
  - For gradients: base is the gradient bounding box
  - For coordinates: base is the coordinate system

**Examples:**
- `50%` - half of the base value
- `100%` - full base value
- `25.5%` - quarter plus half

**References:**
- [W3C SVG 2 - Percentage](https://www.w3.org/TR/SVG2/types.html#DataTypePercentage)

---

## Angle Types

### `<angle>`

**Definition:** An angle value.

**Syntax:**
```
<angle> = <number><unit>
<unit> = deg | rad | grad | turn
```

**Supported Units:**

| Unit | Symbol | Description |
|------|--------|-------------|
| `deg` | degrees | 1/360 of a circle |
| `rad` | radians | Radians (π rad = 180°) |
| `grad` | gradians | 1/400 of a circle |
| `turn` | turns | Full rotations |

**Initial Value:** See individual attributes

**Animatable:** Yes

**Conversion:**
- `1 turn = 360 deg = 400 grad = 2π rad`

**Examples:**
- `0deg` - zero degrees
- `90deg` - quarter turn
- `180deg` - half turn
- `1.5turn` - one and a half turns
- `3.14159rad` - approximately π radians

**References:**
- [W3C SVG 2 - Angle](https://www.w3.org/TR/SVG2/types.html#DataTypeAngle)

---

## Color Types

### `<color>`

**Definition:** An RGB or RGBA color value.

**Syntax:**
```
<color> = <rgb()> | <rgba()> | <hsl()> | <hsla()> | <name>
```

**Color Formats:**

#### RGB (Red, Green, Blue)
```
rgb(<number>[, <number>]{2,3})
```
- Values: 0-255 for integers, 0%-100% for percentages
- Example: `rgb(255, 0, 0)` or `rgb(100%, 0%, 0%)`

#### RGBA (Red, Green, Blue, Alpha)
```
rgba(<number>[, <number>]{2,3}, <alpha>)
```
- Alpha: 0.0 (fully transparent) to 1.0 (fully opaque)
- Example: `rgba(255, 0, 0, 0.5)`

#### HSL (Hue, Saturation, Lightness)
```
hsl(<angle>, <percentage>, <percentage>)
```
- Hue: 0-360deg
- Saturation: 0%-100%
- Lightness: 0%-100%
- Example: `hsl(0deg, 100%, 50%)`

#### HSLA (Hue, Saturation, Lightness, Alpha)
```
hsla(<angle>, <percentage>, <percentage>, <alpha>)
```
- Alpha: 0.0 to 1.0
- Example: `hsla(0deg, 100%, 50%, 0.5)`

#### Named Colors
```
<name> = <valid CSS color keyword>
```

**Common Named Colors:**
- `black`, `white`, `red`, `green`, `blue`
- `gray`, `grey`, `yellow`, `magenta`, `cyan`
- `transparent` - fully transparent color
- See [W3C CSS Color Module](https://www.w3.org/TR/css-color-4/) for full list

**Initial Value:** See individual attributes

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Color](https://www.w3.org/TR/SVG2/types.html#DataTypeColor)
- [W3C CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/)

---

## Coordinate Types

### `<coordinate>`

**Definition:** A single coordinate value (X or Y).

**Syntax:**
```
<coordinate> = <length> | <percentage> | <number>
```

**Initial Value:** See individual attributes

**Animatable:** Yes

**Notes:**
- Represents either X or Y coordinate
- Used independently or in pairs (point, coordinate pair, etc.)

**References:**
- [W3C SVG 2 - Coordinate](https://www.w3.org/TR/SVG2/types.html#DataTypeCoordinate)

---

### `<point>`

**Definition:** An X, Y coordinate pair.

**Syntax:**
```
<point> = <coordinate> <coordinate>
```

**Initial Value:** See individual attributes

**Animatable:** Yes

**Examples:**
- `0 0` - origin
- `100 200` - X=100, Y=200
- `50% 50%` - center point (relative)

**References:**
- [W3C SVG 2 - Point](https://www.w3.org/TR/SVG2/types.html#DataTypePoint)

---

### `<matrix>`

**Definition:** A 3x3 transformation matrix (6 values in SVG).

**Syntax:**
```
<matrix> = matrix(<number> <number> <number> <number> <number> <number>)
```

**Matrix Representation:**
```
[a c e]
[b d f]
[0 0 1]
```

**Parameters:**
- `a` - Scale X
- `b` - Rotate/Shear Y
- `c` - Rotate/Shear X
- `d` - Scale Y
- `e` - Translate X
- `f` - Translate Y

**Initial Value:** `matrix(1, 0, 0, 1, 0, 0)` (identity matrix)

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Matrix](https://www.w3.org/TR/SVG2/types.html#DataTypeMatrix)

---

### `<transform-list>`

**Definition:** A list of one or more transform functions.

**Syntax:**
```
<transform-list> = [<transform>]+
<transform> = <transform-function> [space <transform-function>]*
<transform-function> = matrix(...) | translate(...) | scale(...) | rotate(...) | skewX(...) | skewY(...)
```

**Transform Functions:**

| Function | Syntax | Description |
|----------|--------|-------------|
| `translate` | `translate(<tx> [, <ty>])` | Translation |
| `scale` | `scale(<sx> [, <sy>])` | Scaling |
| `rotate` | `rotate(<angle> [, <cx> [, <cy>]])` | Rotation |
| `skewX` | `skewX(<angle>)` | Skewing along X axis |
| `skewY` | `skewY(<angle>)` | Skewing along Y axis |
| `matrix` | `matrix(<a> <b> <c> <d> <e> <f>)` | Full transformation matrix |

**Initial Value:** None (empty list)

**Animatable:** Yes

**Examples:**
```
translate(100, 200)
scale(2)
rotate(45)
skewX(10deg)
matrix(1, 0, 0, 1, 50, 50)
translate(10, 10) rotate(45) scale(1.5)
```

**References:**
- [W3C SVG 2 - Transform List](https://www.w3.org/TR/SVG2/types.html#DataTypeTransformList)

---

### `<viewbox>`

**Definition:** Specifies the coordinates and size of the viewBox.

**Syntax:**
```
<viewbox> = [<min-x> <min-y> <width> <height>]
```

**Parameters:**
- `<min-x>` - Minimum X coordinate
- `<min-y>` - Minimum Y coordinate
- `<width>` - Width of the viewBox
- `<height>` - Height of the viewBox

**Initial Value:** None

**Animatable:** Yes

**Examples:**
- `viewBox="0 0 100 100"` - 100x100 starting at origin
- `viewBox="0 0 500 500"` - 500x500 starting at origin
- `viewBox="-10 -10 120 120"` - centered viewBox

**References:**
- [W3C SVG 2 - ViewBox](https://www.w3.org/TR/SVG2/types.html#DataTypeViewBox)

---

### `<preserve-aspect-ratio>`

**Definition:** Specifies how to preserve aspect ratio when scaling.

**Syntax:**
```
<preserve-aspect-ratio> = <x-align> <y-align> [<scale>]
<x-align> = xMin | xMid | xMax
<y-align> = yMin | yMid | yMax
<scale> = none | xMinYMin | xMidYMid | xMaxYMax | meet | slice
```

**Align Values:**
- `xMin` - Align to left
- `xMid` - Center horizontally
- `xMax` - Align to right
- `yMin` - Align to top
- `yMid` - Center vertically
- `yMax` - Align to bottom

**Scale Values:**
- `none` - Don't preserve aspect ratio
- `meet` - Preserve aspect ratio, fit within bounds
- `slice` - Preserve aspect ratio, fill bounds

**Initial Value:** `xMidYMid meet`

**Animatable:** No

**Examples:**
- `preserveAspectRatio="none"` - no aspect ratio preservation
- `preserveAspectRatio="xMinYMin meet"` - top-left, scaled to fit
- `preserveAspectRatio="xMidYMid slice"` - center, cropped to fill
- `preserveAspectRatio="meet"` - default behavior

**References:**
- [W3C SVG 2 - Preserve Aspect Ratio](https://www.w3.org/TR/SVG2/types.html#DataTypePreserveAspectRatio)

---

## List Types

### `<list of <type>>`

**Definition:** A comma or whitespace-separated list of values.

**Syntax:**
```
<list> = [<value>]+
```

**Where `<value>` can be:**
- `<integer>` - list of integers
- `<number>` - list of numbers
- `<length>` - list of lengths
- `<percentage>` - list of percentages
- `<coordinate>` - list of coordinates
- `<angle>` - list of angles
- `<color>` - list of colors
- `<string>` - list of strings

**Separator:** Comma or whitespace (or both)

**Initial Value:** See individual attributes

**Animatable:** Yes (when animating the entire list)

**Examples:**
```
0 1 2 3 4           (list of integers)
10.5, 20.3, 30.7    (list of numbers)
100px 200px, 300px  (list of lengths)
red, green, blue    (list of colors)
```

**References:**
- [W3C SVG 2 - List Types](https://www.w3.org/TR/SVG2/types.html#DataTypeLists)

---

## Special Types

### `<path-data>`

**Definition:** SVG path data commands.

**Syntax:** (Based on SVG Path Data Module)
```
<path-data> = <path-command> [<path-command>]+
<path-command> = <path-command-name> <path-parameters>?
```

**Path Command Names:**

| Command | Description | Parameters |
|---------|-------------|------------|
| `M` / `m` | Move to | x y |
| `L` / `l` | Line to | x y |
| `H` / `h` | Horizontal line to | x |
| `V` / `v` | Vertical line to | y |
| `C` / `c` | Cubic Bézier to | x1 y1, x2 y2, x y |
| `S` / `s` | Smooth cubic Bézier to | x2 y2, x y |
| `Q` / `q` | Quadratic Bézier to | x1 y1, x y |
| `T` / `t` | Smooth quadratic Bézier to | x y |
| `A` / `a` | Arc to | rx ry x-axis-rotation large-arc-flag sweep-flag x y |
| `Z` / `z` | Close path | (none) |

**Flags:**
- `large-arc-flag`: 0 or 1
- `sweep-flag`: 0 or 1

**Initial Value:** See individual attributes

**Animatable:** Yes (for path interpolation)

**Examples:**
```
M 10 10 L 100 100 L 10 100 Z
M 50 50 A 25 25 0 0 1 100 100
C 10 10, 50 50, 100 100
```

**References:**
- [W3C SVG 2 - Path Data](https://www.w3.org/TR/SVG2/paths.html#PathData)

---

### `<gradient-units>`

**Definition:** Specifies the coordinate system for gradient elements.

**Syntax:**
```
<gradient-units> = userSpaceOnUse | objectBoundingBox
```

**Values:**
- `userSpaceOnUse` - Gradient coordinates are in the user coordinate system
- `objectBoundingBox` - Gradient coordinates are in the object's bounding box (0-1 range)

**Initial Value:** `objectBoundingBox` (for most gradients)

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Gradient Units](https://www.w3.org/TR/SVG2/painting.html#GradientUnitsAttribute)

---

### `<coordinate-units>`

**Definition:** Specifies the coordinate system for pattern elements.

**Syntax:**
```
<coordinate-units> = userSpaceOnUse | objectBoundingBox
```

**Initial Value:** `userSpaceOnUse`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Pattern Units](https://www.w3.org/TR/SVG2/painting.html#PatternUnitsAttribute)

---

### `<spread-method>`

**Definition:** Specifies how gradient colors are applied outside the gradient bounds.

**Syntax:**
```
<spread-method> = pad | repeat | reflect
```

**Values:**
- `pad` - Extend the last color to the edges (default)
- `repeat` - Repeat the gradient pattern
- `reflect` - Reflect the gradient pattern

**Initial Value:** `pad`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Spread Method](https://www.w3.org/TR/SVG2/painting.html#SpreadMethod)

---

### `<clip-path-type>`

**Definition:** Specifies how clip paths are applied.

**Syntax:**
```
<clip-path-type> = clipPathUnits
<clipPathUnits> = userSpaceOnUse | objectBoundingBox
```

**Values:**
- `userSpaceOnUse` - Clip path coordinates in user space
- `objectBoundingBox` - Clip path coordinates relative to object's bounding box

**References:**
- [W3C SVG 2 - Clip Path Units](https://www.w3.org/TR/SVG2/masking.html#ClipPathUnitsAttribute)

---

## Enumeration Types

Enumeration types are limited sets of string values that represent predefined options.

### `<fill-rule>`

**Syntax:**
```
<fill-rule> = nonzero | evenodd
```

**Values:**
- `nonzero` - Non-zero winding number rule
- `evenodd` - Even-odd rule

**Initial Value:** `nonzero`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Fill Rule](https://www.w3.org/TR/SVG2/painting.html#FillRuleProperty)

---

### `<stroke-linecap>`

**Syntax:**
```
<stroke-linecap> = butt | round | square
```

**Values:**
- `butt` - No cap, ends at the endpoint
- `round` - Rounded cap
- `square` - Square cap extending beyond endpoint

**Initial Value:** `butt`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Stroke Line Cap](https://www.w3.org/TR/SVG2/painting.html#StrokeLinecapProperty)

---

### `<stroke-linejoin>`

**Syntax:**
```
<stroke-linejoin> = miter | round | bevel
```

**Values:**
- `miter` - Sharp corner (default)
- `round` - Rounded corner
- `bevel` - Beveled corner

**Initial Value:** `miter`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Stroke Line Join](https://www.w3.org/TR/SVG2/painting.html#StrokeLinejoinProperty)

---

### `<text-anchor>`

**Syntax:**
```
<text-anchor> = start | middle | end
```

**Values:**
- `start` - Text anchored at the start (left for LTR)
- `middle` - Text centered
- `end` - Text anchored at the end (right for LTR)

**Initial Value:** `start`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Text Anchor](https://www.w3.org/TR/SVG2/text.html#TextAnchorProperty)

---

### `<dominant-baseline>`

**Syntax:**
```
<dominant-baseline> = auto | text-bottom | alphabetic | ideographic | middle | central | mathematical | text-top
```

**Values:**
- `auto` - Browser determines baseline
- `text-bottom` - Text bottom baseline
- `alphabetic` - Alphabetic baseline (default)
- `ideographic` - Ideographic baseline
- `middle` - Middle of the em square
- `central` - Center of the em square
- `mathematical` - Mathematical baseline
- `text-top` - Text top

**Initial Value:** `auto`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Dominant Baseline](https://www.w3.org/TR/SVG2/text.html#DominantBaselineProperty)

---

### `<writing-mode>`

**Syntax:**
```
<writing-mode> = horizontal-tb | vertical-rl | vertical-lr
```

**Values:**
- `horizontal-tb` - Horizontal text, top-to-bottom (default)
- `vertical-rl` - Vertical text, right-to-left
- `vertical-lr` - Vertical text, left-to-right

**Initial Value:** `horizontal-tb`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Writing Mode](https://www.w3.org/TR/SVG2/text.html#WritingModeProperty)

---

### `<display>`

**Syntax:**
```
<display> = inline | block | list-item | inline-block | inline-list-item | run-in | compact | marker | table | inline-table | table-row-group | table-header-group | table-footer-group | table-row | table-column-group | table-column | table-cell | table-caption | grid | inline-grid | flex | inline-flex | none | inherit
```

**Common Values:**
- `inline` - Inline element
- `block` - Block element
- `none` - Not rendered (but may still participate in events)
- `inherit` - Inherits from parent

**Initial Value:** `inline` (varies by element)

**Animatable:** No

**References:**
- [W3C SVG 2 - Display Property](https://www.w3.org/TR/SVG2/painting.html#DisplayProperty)

---

### `<overflow>`

**Syntax:**
```
<overflow> = visible | hidden | scroll | auto | clip | inherit
```

**Values:**
- `visible` - Content visible outside bounds
- `hidden` - Content clipped
- `scroll` - Scrollbars shown
- `auto` - Scrollbars only if needed
- `clip` - Content clipped (no scrollbars)

**Initial Value:** `visible` (varies by context)

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Overflow Property](https://www.w3.org/TR/SVG2/interact.html#OverflowProperty)

---

### `<vector-effect>`

**Syntax:**
```
<vector-effect> = none | non-scaling-stroke | non-scaling-size | non-rotation | fixed-position | inherit
```

**Values:**
- `none` - Normal vector effects (default)
- `non-scaling-stroke` - Stroke width doesn't scale with zoom
- `non-scaling-size` - Element size doesn't scale
- `non-rotation` - Element doesn't rotate
- `fixed-position` - Element stays fixed during panning

**Initial Value:** `none`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Vector Effect](https://www.w3.org/TR/SVG2/painting.html#VectorEffectProperty)

---

### `<shape-rendering>`

**Syntax:**
```
<shape-rendering> = auto | optimizeSpeed | crispEdges | geometricPrecision
```

**Values:**
- `auto` - Renderer chooses (default)
- `optimizeSpeed` - Prioritize rendering speed
- `crispEdges` - Minimize anti-aliasing for sharp edges
- `geometricPrecision` - Prioritize geometric accuracy

**Initial Value:** `auto`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Shape Rendering](https://www.w3.org/TR/SVG2/painting.html#ShapeRenderingProperty)

---

### `<text-rendering>`

**Syntax:**
```
<text-rendering> = auto | optimizeSpeed | optimizeLegibility | geometricPrecision
```

**Values:**
- `auto` - Browser chooses (default)
- `optimizeSpeed` - Prioritize speed
- `optimizeLegibility` - Prioritize readability
- `geometricPrecision` - Prioritize geometric accuracy

**Initial Value:** `auto`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Text Rendering](https://www.w3.org/TR/SVG2/text.html#TextRenderingProperty)

---

### `<image-rendering>`

**Syntax:**
```
<image-rendering> = auto | optimizeSpeed | optimizeQuality | inherit
```

**Values:**
- `auto` - Browser chooses (default)
- `optimizeSpeed` - Fast scaling, lower quality
- `optimizeQuality` - High-quality scaling
- `inherit` - Inherits from parent

**Initial Value:** `auto`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Image Rendering](https://www.w3.org/TR/SVG2/painting.html#ImageRenderingProperty)

---

### `<pointer-events>`

**Syntax:**
```
<pointer-events> = none | visiblePainted | visibleFill | visibleStroke | visible | painted | fill | stroke | all | inherit
```

**Values:**
- `none` - No pointer events
- `visiblePainted` - Events on painted areas (fill + stroke)
- `visibleFill` - Events on fill areas
- `visibleStroke` - Events on stroke areas
- `visible` - Events on visible areas
- `painted` - Events on painted areas (including invisible)
- `fill` - Events on fill areas (including invisible)
- `stroke` - Events on stroke areas (including invisible)
- `all` - Events on all areas
- `inherit` - Inherits from parent

**Initial Value:** `visiblePainted`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Pointer Events](https://www.w3.org/TR/SVG2/interact.html#PointerEventsProperty)

---

### `<fill>`

**Syntax:**
```
<fill> = <color> | <url> | none | inherit
```

**Values:**
- `<color>` - Solid color value
- `<url>` - Reference to paint server (gradient, pattern)
- `none` - No fill
- `inherit` - Inherits from parent

**Initial Value:** `rgb(0, 0, 0)` (black)

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Fill](https://www.w3.org/TR/SVG2/painting.html#FillProperty)

---

### `<stroke>`

**Syntax:**
```
<stroke> = <color> | <url> | none | inherit
```

**Values:**
- `<color>` - Solid color value
- `<url>` - Reference to paint server
- `none` - No stroke
- `inherit` - Inherits from parent

**Initial Value:** `none`

**Animatable:** Yes

**References:**
- [W3C SVG 2 - Stroke](https://www.w3.org/TR/SVG2/painting.html#StrokeProperty)

---

## Interface Definitions

### SVG Angle Interface

```typescript
interface SVGAngle {
  unit: SVGAngle.SVG_ANGLETYPE_UNSPECIFIED | 
        SVGAngle.SVG_ANGLETYPE_DEG | 
        SVGAngle.SVG_ANGLETYPE_RAD | 
        SVGAngle.SVG_ANGLETYPE_GRAD;
  value: number;
  valueInSpecifiedUnits: number;
  specimenNode: SVGElement;
  
  setAngleInSpecifiedUnits(angle: number): void;
  convertToSpecifiedUnits(unit: number): void;
  newValueSpecifiedUnits(unit: number, value: number): void;
}
```

**References:**
- [W3C SVG 2 - SVG Angle Interface](https://www.w3.org/TR/SVG2/types.html#SVGAngleInterface)

---

### SVG Number List Interface

```typescript
interface SVGNumberList {
  numberOfItems: number;
  initialize(newItem: number): number;
  getItem(index: number): number;
  appendItem(newItem: number): number;
  insertItemBefore(newItem: number, index: number): number;
  replaceItem(newItem: number, index: number): number;
  removeItem(index: number): number;
  clear(): void;
}
```

**References:**
- [W3C SVG 2 - SVG Number List Interface](https://www.w3.org/TR/SVG2/types.html#SVGNumberListInterface)

---

### SVG Unit Types

```typescript
// SVG Angle Unit Types
SVGAngle.SVG_ANGLETYPE_UNSPECIFIED = 0;
SVGAngle.SVG_ANGLETYPE_DEG = 1;
SVGAngle.SVG_ANGLETYPE_RAD = 2;
SVGAngle.SVG_ANGLETYPE_GRAD = 3;

// SVG Length Unit Types
SVGLength.SVG_LENGTHTYPE_UNSPECIFIED = 0;
SVGLength.SVG_LENGTHTYPE_PX = 1;
SVGLength.SVG_LENGTHTYPE_CM = 2;
SVGLength.SVG_LENGTHTYPE_MM = 3;
SVGLength.SVG_LENGTHTYPE_IN = 4;
SVGLength.SVG_LENGTHTYPE_PT = 5;
SVGLength.SVG_LENGTHTYPE_PC = 6;
SVGLength.SVG_LENGTHTYPE_PERCENTAGE = 7;
SVGLength.SVG_LENGTHTYPE_EMS = 8;
SVGLength.SVG_LENGTHTYPE_EXS = 9;
```

**References:**
- [W3C SVG 2 - SVG Unit Types](https://www.w3.org/TR/SVG2/types.html#SVGUnitTypes)

---

## See Also

- [SVG 2 Specification - Chapter 4: Basic Data Types and Interfaces](https://www.w3.org/TR/SVG2/types.html)
- [CSS Value Definition Syntax](https://www.w3.org/TR/css-values-4/)
- [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/)
- [CSS Fonts Module Level 4](https://www.w3.org/TR/css-fonts-4/)

---

*Generated from W3C SVG 2 Specification. For the most up-to-date information, always refer to the official specification.*
