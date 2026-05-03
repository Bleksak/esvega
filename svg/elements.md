# SVG 2 Elements

Comprehensive documentation of all SVG 2 elements and their attributes.

## Table of Contents

- [Document Structure Elements](#document-structure-elements)
- [Basic Shapes](#basic-shapes)
- [Paths](#paths)
- [Text Elements](#text-elements)
- [Graphics Elements](#graphics-elements)
- [Paint Servers](#paint-servers)
- [Gradient Elements](#gradient-elements)
- [Pattern Elements](#pattern-elements)
- [Filter Effects](#filter-effects)
- [Animation Elements](#animation-elements)
- [Interactive Elements](#interactive-elements)
- [Linking Elements](#linking-elements)
- [Embedded Content](#embedded-content)
- [Conditional Processing](#conditional-processing)

---

## Document Structure Elements

### `<svg>`

**Category:** Document container element

**Description:** Root element or nested container for SVG graphics.

**Content Model:** Graphics containers, shapes, text, images, or special content.

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Length/percentage | `100%` | Yes | Width |
| `height` | `<length>` | Length/percentage | `100%` | Yes | Height |
| `viewBox` | `<viewbox>` | Any valid viewBox | None | Yes | Viewport definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | No | Aspect ratio behavior |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**References:**
- [W3C SVG 2 - SVG Element](https://www.w3.org/TR/SVG2/struct.html#SVGElement)
- [W3C SVG 2 - Coordinate System](https://www.w3.org/TR/SVG2/coords.html#SettingUpSVGViewport)

---

### `<g>`

**Category:** Grouping element

**Description:** Groups graphics elements together.

**Content Model:** Graphics containers, shapes, text, images, etc.

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `clip-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule |
| `display` | `<display>` | See `<display>` | Varies | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - G Element](https://www.w3.org/TR/SVG2/struct.html#GElement)

---

### `<defs>`

**Category:** Definition container

**Description:** Container for graphical objects that will be reused.

**Content Model:** Any graphics elements (typically gradients, patterns, symbols, etc.)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |

**Notes:**
- Elements in `<defs>` are not rendered directly
- Referenced by other elements via `url()` references
- Common contents: gradients, patterns, symbols, markers, filters

**References:**
- [W3C SVG 2 - Defs Element](https://www.w3.org/TR/SVG2/struct.html#DefsElement)

---

### `<symbol>`

**Category:** Template definition

**Description:** Defines a reusable graphic template.

**Content Model:** Any graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `viewBox` | `<viewbox>` | Any valid viewBox | Required if not in document | Yes | Viewport definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | No | Aspect ratio |
| `overflow` | `<overflow>` | See `<overflow>` | `visible` | Yes | Overflow behavior |

**References:**
- [W3C SVG 2 - Symbol Element](https://www.w3.org/TR/SVG2/struct.html#SymbolElement)

---

### `<use>`

**Category:** Reference/instance

**Description:** Creates an instance of another element.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `href` / `xlink:href` | `<url>` | URL to element | Required | Yes | Reference to element |
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Length/percentage | Inherited | Yes | Width |
| `height` | `<length>` | Length/percentage | Inherited | Yes | Height |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**Notes:**
- `href` is the SVG 2 attribute (use `xlink:href` for XHTML compatibility)
- Can reference any SVG element or HTML element
- Instance inherits all attributes unless overridden

**References:**
- [W3C SVG 2 - Use Element](https://www.w3.org/TR/SVG2/struct.html#UseElement)

---

### `<switch>`

**Category:** Conditional rendering

**Description:** Renders only the first child that meets its requirements.

**Content Model:** Graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |

**Child Element Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `requiredFeatures` | `<string>` | Feature list | Required SVG features |
| `requiredExtensions` | `<string>` | Extension list | Required extensions |
| `systemLanguage` | `<string>` | Language list | Required languages |

**References:**
- [W3C SVG 2 - Switch Element](https://www.w3.org/TR/SVG2/struct.html#SwitchElement)

---

### `<title>`

**Category:** Metadata

**Description:** Provides a title for SVG document elements.

**Content Model:** Text only

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**Notes:**
- Should be first child of parent element
- Used for accessibility (screen readers, tooltips)
- Not visually rendered by default

**References:**
- [W3C SVG 2 - Title Element](https://www.w3.org/TR/SVG2/struct.html#TitleElement)

---

### `<desc>`

**Category:** Metadata

**Description:** Provides a description for SVG document elements.

**Content Model: Text only**

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**Notes:**
- More detailed than `<title>`
- Used for accessibility
- Not visually rendered

**References:**
- [W3C SVG 2 - Desc Element](https://www.w3.org/TR/SVG2/struct.html#DescElement)

---

### `<metadata>`

**Category:** Metadata

**Description:** Contains non-visual metadata about the SVG.

**Content Model:** Any XML data

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**Notes:**
- Contains RDF, Dublin Core, or other metadata
- Not visually rendered
- Used for document processing

**References:**
- [W3C SVG 2 - Metadata Element](https://www.w3.org/TR/SVG2/struct.html#MetadataElement)

---

### `<unknown>`

**Category:** Unknown/extended element

**Description:** Placeholder for unknown elements (parser-specific).

**Content Model:** Varies

**References:**
- [W3C SVG 2 - Unknown Element](https://www.w3.org/TR/SVG2/struct.html#UnknownElement)

---

## Basic Shapes

### `<rect>`

**Category:** Basic shape

**Description:** Rectangular shape.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Positive length | `0` | Yes | Width |
| `height` | `<length>` | Positive length | `0` | Yes | Height |
| `rx` | `<length>` | Radius | `0` | Yes | Horizontal corner radius |
| `ry` | `<length>` | Radius | `0` | Yes | Vertical corner radius |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `clip-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**Notes:**
- `rx` and `ry` create rounded corners
- If only `rx` is specified, `ry` defaults to `rx`
- If `rx > width/2`, `rx` defaults to `width/2`
- Negative width/height creates shapes in opposite direction

**References:**
- [W3C SVG 2 - Rect Element](https://www.w3.org/TR/SVG2/shapes.html#RectElement)

---

### `<circle>`

**Category:** Basic shape

**Description:** Circular shape.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `cx` | `<coordinate>` | Any coordinate | `0` | Yes | Center X position |
| `cy` | `<coordinate>` | Any coordinate | `0` | Yes | Center Y position |
| `r` | `<length>` | Positive length | `0` | Yes | Radius |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Circle Element](https://www.w3.org/TR/SVG2/shapes.html#CircleElement)

---

### `<ellipse>`

**Category:** Basic shape

**Description:** Elliptical shape.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `cx` | `<coordinate>` | Any coordinate | `0` | Yes | Center X position |
| `cy` | `<coordinate>` | Any coordinate | `0` | Yes | Center Y position |
| `rx` | `<length>` | Positive length | `0` | Yes | Horizontal radius |
| `ry` | `<length>` | Positive length | `0` | Yes | Vertical radius |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Ellipse Element](https://www.w3.org/TR/SVG2/shapes.html#EllipseElement)

---

### `<line>`

**Category:** Basic shape

**Description:** Straight line between two points.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x1` | `<coordinate>` | Any coordinate | `0` | Yes | Start X position |
| `y1` | `<coordinate>` | Any coordinate | `0` | Yes | Start Y position |
| `x2` | `<coordinate>` | Any coordinate | `0` | Yes | End X position |
| `y2` | `<coordinate>` | Any coordinate | `0` | Yes | End Y position |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Line Element](https://www.w3.org/TR/SVG2/shapes.html#LineElement)

---

### `<polyline>`

**Category:** Basic shape

**Description:** Series of connected straight line segments.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `points` | `<coordinate-list>` | List of points | Required | Yes | List of vertex coordinates |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `fill-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**Notes:**
- `points` attribute format: `x1 y1 x2 y2 ...` or `x1,y1 x2,y2 ...`
- Not automatically closed (unlike polygon)
- Can be filled if `fill` is specified

**References:**
- [W3C SVG 2 - Polyline Element](https://www.w3.org/TR/SVG2/shapes.html#PolylineElement)

---

### `<polygon>`

**Category:** Basic shape

**Description:** Closed series of connected straight line segments.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `points` | `<coordinate-list>` | List of points | Required | Yes | List of vertex coordinates |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `fill-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**Notes:**
- Automatically closed (last point connects to first)
- `points` attribute format: `x1 y1 x2 y2 ...`
- Can be filled

**References:**
- [W3C SVG 2 - Polygon Element](https://www.w3.org/TR/SVG2/shapes.html#PolygonElement)

---

## Paths

### `<path>`

**Category:** Graphics element

**Description:** Complex vector shape defined by path data commands.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `d` | `<path-data>` | Path data | Required | Yes | Path data commands |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `clip-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule |
| `fill-rule` | `<fill-rule>` | `nonzero` \| `evenodd` | `nonzero` | Yes | Fill rule (deprecated) |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |
| `stroke-dasharray` | `<list>` | Dash array | `none` | Yes | Dash pattern |
| `stroke-dashoffset` | `<length>` | Length | `0` | Yes | Dash offset |
| `pathLength` | `<number>` | Number | `1` | Yes | Path length normalization |

**Path Data Commands:**

| Command | Parameter | Description |
|---------|-----------|-------------|
| `M` / `m` | x y | Move to |
| `L` / `l` | x y | Line to |
| `H` / `h` | x | Horizontal line to |
| `V` / `v` | y | Vertical line to |
| `C` / `c` | x1 y1 x2 y2 x y | Cubic Bézier |
| `S` / `s` | x2 y2 x y | Smooth cubic Bézier |
| `Q` / `q` | x1 y1 x y | Quadratic Bézier |
| `T` / `t` | x y | Smooth quadratic Bézier |
| `A` / `a` | rx ry x-axis-rotation large-arc sweep x y | Arc |
| `Z` / `z` | (none) | Close path |

**References:**
- [W3C SVG 2 - Path Element](https://www.w3.org/TR/SVG2/paths.html#PathElement)
- [W3C SVG 2 - Path Data](https://www.w3.org/TR/SVG2/paths.html#PathData)

---

## Text Elements

### `<text>`

**Category:** Text element

**Description:** Contains textual content.

**Content Model:** Text content and text child elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Single or list | `0` | Yes | X position |
| `y` | `<coordinate>` | Single or list | `0` | Yes | Y position |
| `dx` | `<list>` | List of lengths | `0` | Yes | Horizontal offset |
| `dy` | `<list>` | List of lengths | `0` | Yes | Vertical offset |
| `rotate` | `<list>` | List of angles | `0` | Yes | Rotation |
| `text-anchor` | `<text-anchor>` | `start` \| `middle` \| `end` | `start` | Yes | Text anchor |
| `textLength` | `<length>` | Length | `auto` | Yes | Target text length |
| `lengthAdjust` | `<length-adjust>` | `spacing` \| `spacingAndGlyphs` | `spacing` | Yes | Length adjustment |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Text Element](https://www.w3.org/TR/SVG2/text.html#TextElement)

---

### `<tspan>`

**Category:** Text element

**Description:** Defines a subregion of text for styling or positioning.

**Content Model:** Text content and text child elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Single or list | Inherits | Yes | X position |
| `y` | `<coordinate>` | Single or list | Inherits | Yes | Y position |
| `dx` | `<list>` | List of lengths | Inherits | Yes | Horizontal offset |
| `dy` | `<list>` | List of lengths | Inherits | Yes | Vertical offset |
| `rotate` | `<list>` | List of angles | Inherits | Yes | Rotation |
| `text-anchor` | `<text-anchor>` | `start` \| `middle` \| `end` | Inherits | Yes | Text anchor |
| `textLength` | `<length>` | Length | Inherits | Yes | Target text length |
| `lengthAdjust` | `<length-adjust>` | `spacing` \| `spacingAndGlyphs` | Inherits | Yes | Length adjustment |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Tspan Element](https://www.w3.org/TR/SVG2/text.html#TspanElement)

---

### `<textPath>`

**Category:** Text element

**Description:** Renders text along a path.

**Content Model:** Text content and text child elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `href` / `xlink:href` | `<url>` | URL to path | Required | Yes | Reference to path |
| `method` | `<text-path-method>` | `align` \| `stretch` | `align` | Yes | Spacing method |
| `side` | `<text-path-side>` | `left` \| `right` | `right` | Yes | Side of path |
| `startOffset` | `<length>` | Length | `0` | Yes | Starting offset |
| `text-anchor` | `<text-anchor>` | `start` \| `middle` \| `end` | `start` | Yes | Text anchor |
| `spacing` | `<text-path-spacing>` | `auto` \| `exact` | `auto` | Yes | Spacing handling |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |

**References:**
- [W3C SVG 2 - TextPath Element](https://www.w3.org/TR/SVG2/text.html#TextPathElement)

---

## Graphics Elements

### `<image>`

**Category:** Embedded content

**Description:** Embeds raster or vector images.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Length/percentage | Required | Yes | Width |
| `height` | `<length>` | Length/percentage | Required | Yes | Height |
| `href` / `xlink:href` | `<url>` | Image URL | Required | Yes | Image source |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | No | Aspect ratio |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `clip-path` | `<url>` | URL to clip path | None | Yes | Clipping path |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |
| `mask` | `<url>` | URL to mask | None | Yes | Masking |

**References:**
- [W3C SVG 2 - Image Element](https://www.w3.org/TR/SVG2/embedded.html#ImageElement)

---

### `<foreignObject>`

**Category:** Embedded content

**Description:** Allows embedding non-SVG XML content (e.g., HTML).

**Content Model:** Non-SVG XML content

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Length/percentage | Required | Yes | Width |
| `height` | `<length>` | Length/percentage | Required | Yes | Height |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |
| `display` | `<display>` | See `<display>` | `inline` | No | Display control |
| `visibility` | `<visibility>` | `visible` \| `hidden` \| `opaque` | `visible` | Yes | Visibility |
| `opacity` | `<number>` | 0 to 1 | `1` | Yes | Opacity |

**References:**
- [W3C SVG 2 - ForeignObject Element](https://www.w3.org/TR/SVG2/embedded.html#ForeignObjectElement)

---

### `<canvas>`

**Category:** Embedded content

**Description:** Canvas element for raster graphics.

**Common Attributes:**
- Inherits all common attributes
- Canvas-specific attributes from HTML5 Canvas

**References:**
- [W3C SVG 2 - Canvas Element](https://www.w3.org/TR/SVG2/embedded.html#CanvasElement)

---

## Paint Servers

### `<linearGradient>`

**Category:** Paint server

**Description:** Defines a linear gradient pattern.

**Content Model:** `<stop>` elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `x1` | `<coordinate>` | Any coordinate | `0%` | Yes | Start X |
| `y1` | `<coordinate>` | Any coordinate | `0%` | Yes | Start Y |
| `x2` | `<coordinate>` | Any coordinate | `100%` | Yes | End X |
| `y2` | `<coordinate>` | Any coordinate | `0%` | Yes | End Y |
| `gradientUnits` | `<gradient-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `objectBoundingBox` | Yes | Coordinate system |
| `gradientTransform` | `<transform-list>` | List of transforms | `none` | Yes | Additional transform |
| `spreadMethod` | `<spread-method>` | `pad` \| `repeat` \| `reflect` | `pad` | Yes | Spread method |
| `href` / `xlink:href` | `<url>` | URL to gradient | None | Yes | Reference to gradient |

**References:**
- [W3C SVG 2 - LinearGradient Element](https://www.w3.org/TR/SVG2/painting.html#LinearGradientElement)

---

### `<radialGradient>`

**Category:** Paint server

**Description:** Defines a radial gradient pattern.

**Content Model:** `<stop>` elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `cx` | `<coordinate>` | Any coordinate | `50%` | Yes | Center X |
| `cy` | `<coordinate>` | Any coordinate | `50%` | Yes | Center Y |
| `r` | `<length>` | Positive length | `50%` | Yes | Radius |
| `fx` | `<coordinate>` | Any coordinate | `cx` | Yes | Focal point X |
| `fy` | `<coordinate>` | Any coordinate | `cy` | Yes | Focal point Y |
| `fr` | `<length>` | Positive length | `0` | Yes | Focal radius |
| `gradientUnits` | `<gradient-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `objectBoundingBox` | Yes | Coordinate system |
| `gradientTransform` | `<transform-list>` | List of transforms | `none` | Yes | Additional transform |
| `spreadMethod` | `<spread-method>` | `pad` \| `repeat` \| `reflect` | `pad` | Yes | Spread method |
| `href` / `xlink:href` | `<url>` | URL to gradient | None | Yes | Reference to gradient |

**References:**
- [W3C SVG 2 - RadialGradient Element](https://www.w3.org/TR/SVG2/painting.html#RadialGradientElement)

---

### `<stop>`

**Category:** Gradient stop

**Description:** Defines a color stop within a gradient.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `offset` | `<percentage>` | 0% to 100% | Required | Yes | Stop position |
| `stop-color` | `<color>` | Any color | `black` | Yes | Stop color |
| `stop-opacity` | `<number>` | 0 to 1 | `1` | Yes | Stop opacity |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**Notes:**
- Must be child of `<linearGradient>` or `<radialGradient>`
- `offset` can be `<number>` (0 to 1) or `<percentage>`

**References:**
- [W3C SVG 2 - Stop Element](https://www.w3.org/TR/SVG2/painting.html#StopElement)

---

### `<pattern>`

**Category:** Paint server

**Description:** Defines a reusable pattern.

**Content Model:** Any graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `x` | `<coordinate>` | Any coordinate | `0` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Yes | Y position |
| `width` | `<length>` | Length/percentage | `100%` | Yes | Width |
| `height` | `<length>` | Length/percentage | `100%` | Yes | Height |
| `patternUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `userSpaceOnUse` | Yes | Pattern coordinate system |
| `patternContentUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `userSpaceOnUse` | Yes | Pattern content coordinate system |
| `patternTransform` | `<transform-list>` | List of transforms | `none` | Yes | Pattern transform |
| `href` / `xlink:href` | `<url>` | URL to pattern | None | Yes | Reference to pattern |

**References:**
- [W3C SVG 2 - Pattern Element](https://www.w3.org/TR/SVG2/painting.html#PatternElement)

---

## Filter Effects

### `<filter>`

**Category:** Filter effect

**Description:** Defines a filter effect.

**Content Model:** Filter primitive elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `x` | `<coordinate>` | Any coordinate | `-10%` | Yes | X position |
| `y` | `<coordinate>` | Any coordinate | `-10%` | Yes | Y position |
| `width` | `<length>` | Length/percentage | `120%` | Yes | Width |
| `height` | `<length>` | Length/percentage | `120%` | Yes | Height |
| `filterUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `objectBoundingBox` | Yes | Filter coordinate system |
| `primitiveUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `userSpaceOnUse` | Yes | Primitive coordinate system |
| `href` / `xlink:href` | `<url>` | URL to filter | None | Yes | Reference to filter |

**References:**
- [W3C SVG 2 - Filter Element](https://www.w3.org/TR/SVG2/filters.html#FilterElement)

---

### Filter Primitive Elements

The following 26 filter primitive elements are defined in the Filter Effects Module Level 1:

| Element | Description |
|---------|-------------|
| `<feBlend>` | Blends two inputs |
| `<feColorMatrix>` | Color matrix transformation |
| `<feComponentTransfer>` | Component transfer function |
| `<feComposite>` | Composite operations |
| `<feConvolveMatrix>` | Convolution matrix |
| `<feDiffuseLighting>` | Diffuse lighting |
| `<feDisplacementMap>` | Displacement map |
| `<feDistantLight>` | Distant light source |
| `<feDropShadow>` | Drop shadow |
| `<feFlood>` | Flood fill |
| `<feFuncA>` | Alpha component function |
| `<feFuncB>` | Blue component function |
| `<feFuncG>` | Green component function |
| `<feFuncR>` | Red component function |
| `<feGaussianBlur>` | Gaussian blur |
| `<feImage>` | External image as filter |
| `<feMerge>` | Merge multiple inputs |
| `<feMergeNode>` | Single input for merge |
| `<feMorphology>` | Morphology operation |
| `<feOffset>` | Offset input |
| `<fePointLight>` | Point light source |
| `<feSpecularLighting>` | Specular lighting |
| `<feSpotLight>` | Spot light source |
| `<feTile>` | Tile input |
| `<feTurbulence>` | Turbulence/fractional Brownian noise |
| `<feBlend>` | Blends two inputs |

**Notes:**
- Each filter primitive has specific input requirements
- See Filter Effects Module Level 1 for complete documentation
- Input can be `SourceGraphic`, `SourceAlpha`, `BackgroundImage`, `BackgroundAlpha`, or result of other primitives

---

## Animation Elements

### `<animate>`

**Category:** Animation

**Description:** Animates attribute values over time.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `attributeName` | `<string>` | Attribute name | Required | No | Attribute to animate |
| `attributeType` | `<string>` | `CSS` \| `XML` | `CSS` | No | Attribute type |
| `from` | Any | Start value | Required | No | Start value |
| `to` | Any | End value | Required | No | End value |
| `values` | `<list>` | List of values | Required | No | Keyframe values |
| `dur` | `<time>` | Duration | `1s` | No | Animation duration |
| `repeatCount` | `<count>` | Count | `indefinite` | No | Repeat count |
| `repeatDur` | `<time>` | Time | `indefinite` | No | Repeat duration |
| `fill` | `<time-fill>` | `freeze` \| `remove` \| `reverse` \| `auto` | `remove` | No | After-end behavior |
| `additive` | `<additive>` | `sum` \| `replace` | `replace` | No | Addition mode |
| `accumulate` | `<accumulate>` | `sum` \| `none` | `none` | No | Accumulation mode |
| `keyTimes` | `<list>` | List of percentages | None | No | Keyframe times |
| `keySplines` | `<list>` | List of bezier splines | None | No | Cubic bezier splines |
| `calcMode` | `<calc-mode>` | `discrete` \| `linear` \| `paced` \| `spline` | `linear` | No | Calculation mode |
| `href` / `xlink:href` | `<url>` | URL to animate | None | Yes | Reference to animate |

**References:**
- [W3C SVG 2 - Animate Element](https://www.w3.org/TR/SVG2/animate.html#AnimateElement)

---

### `<animateMotion>`

**Category:** Animation

**Description:** Animates element along a path.

**Content Model:** Optional `<mpath>` element

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `path` | `<path-data>` | Path data | Required | No | Path data |
| `keyPoints` | `<list>` | List of numbers | None | No | Key points |
| `rotate` | `<angle>` \| `<list>` | Rotation | `0` | No | Rotation |
| `origin` | `<coordinate>` | Any coordinate | `auto` | No | Motion origin |
| `href` / `xlink:href` | `<url>` | URL to animateMotion | Required | Yes | Reference to element |

**References:**
- [W3C SVG 2 - AnimateMotion Element](https://www.w3.org/TR/SVG2/animate.html#AnimateMotionElement)

---

### `<animateTransform>`

**Category:** Animation

**Description:** Animates transform attributes.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `attributeName` | `<string>` | `transform` | Required | No | Must be `transform` |
| `type` | `<transform-type>` | `translate` \| `scale` \| `rotate` \| `skewX` \| `skewY` \| `matrix` | Required | No | Transform type |
| `from` | Any | Start value | Required | No | Start value |
| `to` | Any | End value | Required | No | End value |
| `values` | `<list>` | List of values | Required | No | Keyframe values |
| `dur` | `<time>` | Duration | `1s` | No | Animation duration |
| `repeatCount` | `<count>` | Count | `indefinite` | No | Repeat count |
| `fill` | `<time-fill>` | See `<time-fill>` | `remove` | No | After-end behavior |
| `additive` | `<additive>` | `sum` \| `replace` | `replace` | No | Addition mode |
| `accumulate` | `<accumulate>` | `sum` \| `none` | `none` | No | Accumulation mode |
| `keyTimes` | `<list>` | List of percentages | None | No | Keyframe times |
| `keySplines` | `<list>` | List of bezier splines | None | No | Cubic bezier splines |
| `calcMode` | `<calc-mode>` | See `<calc-mode>` | `linear` | No | Calculation mode |
| `href` / `xlink:href` | `<url>` | URL to animateTransform | None | Yes | Reference to animateTransform |

**References:**
- [W3C SVG 2 - AnimateTransform Element](https://www.w3.org/TR/SVG2/animate.html#AnimateTransformElement)

---

### `<set>`

**Category:** Animation

**Description:** Sets attribute values over time.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `attributeName` | `<string>` | Attribute name | Required | No | Attribute to set |
| `to` | Any | Target value | Required | No | Target value |
| `begin` | `<time-value>` | Time | Required | No | Start time |
| `dur` | `<time>` | Duration | `0` | No | Duration |
| `fill` | `<time-fill>` | See `<time-fill>` | `freeze` | No | After-end behavior |
| `href` / `xlink:href` | `<url>` | URL to set | Required | Yes | Reference to element |

**References:**
- [W3C SVG 2 - Set Element](https://www.w3.org/TR/SVG2/animate.html#SetElement)

---

### `<mpath>`

**Category:** Animation

**Description:** Provides path data for animateMotion.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `href` / `xlink:href` | `<url>` | URL to path | Required | Yes | Reference to path element |

**References:**
- [W3C SVG 2 - Mpath Element](https://www.w3.org/TR/SVG2/animate.html#MpathElement)

---

### `<discard>`

**Category:** Animation

**Description:** Discards animation events.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `href` / `xlink:href` | `<url>` | URL to element | Required | Yes | Reference to animated element |
| `begin` | `<time-value>` | Time | Required | No | Time to discard |

**References:**
- [W3C SVG 2 - Discard Element](https://www.w3.org/TR/SVG2/animate.html#DiscardElement)

---

## Linking Elements

### `<a>`

**Category:** Linking

**Description:** Creates hyperlinks.

**Content Model:** Any graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `href` | `<url>` | URL | Required | Yes | Target URL |
| `target` | `<string>` | Target name | `_blank` | No | Target window/frame |
| `hreflang` | `<string>` | Language code | None | No | Link language |
| `type` | `<string>` | MIME type | None | No | Link type |
| `rel` | `<string>` | Relationship | None | No | Relationship |
| `rev` | `<string>` | Reverse relationship | None | No | Reverse relationship |
| `download` | `<string>` | Filename | None | No | Download filename |
| `ping` | `<string>` | URL list | None | No | Ping URLs |
| `referrerpolicy` | `<string>` | Policy | None | No | Referrer policy |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Yes | Transformations |

**References:**
- [W3C SVG 2 - A Element](https://www.w3.org/TR/SVG2/links.html#AElement)

---

### `<view>`

**Category:** Linking

**Description:** Defines a viewport view.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `viewBox` | `<viewbox>` | viewBox | Required | Yes | ViewBox definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | No | Aspect ratio |
| `zoomAndPan` | `<zoom-and-pan>` | `disable` \| `magnify` | `magnify` | No | Zoom/pan behavior |

**References:**
- [W3C SVG 2 - View Element](https://www.w3.org/TR/SVG2/links.html#ViewElement)

---

## Interactive Elements

### `<script>`

**Category:** Scripting

**Description:** Contains or references scripts.

**Content Model:** Script code (text)

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `type` | `<string>` | MIME type | `text/ecmascript` | No | Script type |
| `src` | `<url>` | URL | None | No | External script source |
| `href` | `<url>` | URL | None | No | Script URL (alternative) |
| `event` | `<string>` | Event name | Required | No | Event handler |
| `id` | `<string>` | Any string | None | No | Element identifier |
| `class` | `<string>` | Class names | None | No | CSS classes |
| `style` | `<string>` | CSS properties | None | No | Inline styles |

**References:**
- [W3C SVG 2 - Script Element](https://www.w3.org/TR/SVG2/script.html#ScriptElement)

---

### `<style>`

**Category:** Styling

**Description:** Contains CSS styles.

**Content Model:** CSS text

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `type` | `<string>` | MIME type | `text/css` | No | Style type |
| `media` | `<string>` | Media queries | `all` | No | Media queries |
| `title` | `<string>` | Style sheet title | None | No | Style title |
| `id` | `<string>` | Any string | None | No | Element identifier |

**References:**
- [W3C SVG 2 - Style Element](https://www.w3.org/TR/SVG2/styling.html#StyleElement)

---

### `<marker>`

**Category:** Marker

**Description:** Defines a marker symbol.

**Content Model:** Any graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Animatable | Description |
|-----------|------|--------|---------|------------|-------------|
| `id` | `<string>` | Any string | None | No | Element identifier |
| `viewBox` | `<viewbox>` | viewBox | Required | Yes | ViewBox definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | No | Aspect ratio |
| `refX` | `<coordinate>` | Coordinate | `0` | Yes | Reference X |
| `refY` | `<coordinate>` | Coordinate | `0` | Yes | Reference Y |
| `markerUnits` | `<marker-units>` | `strokeWidth` \| `userSpaceOnUse` | `strokeWidth` | Yes | Marker units |
| `markerWidth` | `<length>` | Length | `3` | Yes | Marker width |
| `markerHeight` | `<length>` | Length | `3` | Yes | Marker height |
| `orient` | `<angle>` \| `<string>` | Orientation | `auto` | Yes | Orientation |

**References:**
- [W3C SVG 2 - Marker Element](https://www.w3.org/TR/SVG2/painting.html#MarkerElement)

---

## Conditional Processing

### Common Conditional Processing Attributes

The following attributes are available on many elements for conditional processing:

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `requiredFeatures` | `<string>` | Feature list | Required SVG features |
| `requiredExtensions` | `<string>` | Extension list | Required extensions |
| `systemLanguage` | `<string>` | Language list | Required languages |

**Feature List Format:**
```
feature1 [feature2 ...]
```

**Example:** `requiredFeatures="http://www.w3.org/TR/SVG2/feature#BasicGraphics"`

**References:**
- [W3C SVG 2 - Conditional Processing](https://www.w3.org/TR/SVG2/struct.html#ConditionalProcessing)

---

## HTML Integration Elements

### `<html>`

**Category:** HTML integration

**Description:** HTML element in SVG context.

**References:**
- [W3C SVG 2 - HTML Integration](https://www.w3.org/TR/SVG2/struct.html#HTMLIntegration)

---

## Appendix: Common Attributes Reference

### Presentation Attributes

All SVG elements support the following presentation attributes:

| Attribute | Type | Description |
|-----------|------|-------------|
| `fill` | `<color>` \| `<url>` \| `none` | Fill color |
| `fill-opacity` | `<number>` | Fill opacity (0-1) |
| `fill-rule` | `<fill-rule>` | Fill rule |
| `stroke` | `<color>` \| `<url>` \| `none` | Stroke color |
| `stroke-width` | `<length>` \| `<number>` | Stroke width |
| `stroke-linecap` | `<stroke-linecap>` | Stroke cap |
| `stroke-linejoin` | `<stroke-linejoin>` | Stroke join |
| `stroke-miterlimit` | `<number>` | Miter limit |
| `stroke-dasharray` | `<list>` | Dash array |
| `stroke-dashoffset` | `<length>` \| `<number>` | Dash offset |
| `stroke-opacity` | `<number>` | Stroke opacity (0-1) |
| `opacity` | `<number>` | Overall opacity |
| `display` | `<display>` | Display control |
| `visibility` | `<visibility>` | Visibility |
| `clip-path` | `<url>` | Clipping path |
| `clip-rule` | `<fill-rule>` | Clip fill rule |
| `mask` | `<url>` | Masking |
| `filter` | `<url>` \| `<string>` | Filter effect |
| `flood-color` | `<color>` | Flood fill color |
| `flood-opacity` | `<number>` | Flood opacity |
| `lighting-color` | `<color>` | Lighting color |

**References:**
- [W3C SVG 2 - Presentation Attributes](https://www.w3.org/TR/SVG2/styling.html#PresentationAttributes)

---

### ARIA Attributes

All SVG elements support the following ARIA attributes:

| Attribute | Type | Description |
|-----------|------|-------------|
| `aria-label` | `<string>` | Accessible label |
| `aria-labelledby` | `<string>` | Reference to label elements |
| `aria-describedby` | `<string>` | Reference to description elements |
| `aria-hidden` | `<boolean>` | Hide from accessibility |
| `role` | `<string>` | ARIA role |

**References:**
- [W3C WAI-ARIA](https://www.w3.org/TR/wai-aria/)

---

### Event Handler Attributes

All SVG elements support the following event handlers:

| Attribute | Type | Description |
|-----------|------|-------------|
| `onabort` | `<string>` | Abort handler |
| `onerror` | `<string>` | Error handler |
| `onresize` | `<string>` | Resize handler |
| `onscroll` | `<string>` | Scroll handler |
| `onload` | `<string>` | Load handler |
| `onfocus` | `<string>` | Focus handler |
| `onblur` | `<string>` | Blur handler |
| `onmousedown` | `<string>` | Mouse down handler |
| `onmouseup` | `<string>` | Mouse up handler |
| `onmouseover` | `<string>` | Mouse over handler |
| `onmousemove` | `<string>` | Mouse move handler |
| `onmouseout` | `<string>` | Mouse out handler |
| `onkeydown` | `<string>` | Key down handler |
| `onkeyup` | `<string>` | Key up handler |
| `onkeypress` | `<string>` | Key press handler |

**References:**
- [W3C SVG 2 - Events](https://www.w3.org/TR/SVG2/interact.html#Events)

---

*Generated from W3C SVG 2 Specification and Filter Effects Module Level 1. For the most up-to-date information, always refer to the official specifications.*
