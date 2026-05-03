# SVG 2 Filter Effects

Comprehensive documentation of all SVG filter effects and filter primitive elements.

## Table of Contents

- [Overview](#overview)
- [Filter Element](#filter-element)
- [Filter Primitive Elements](#filter-primitive-elements)
  - [feBlend](#feblend)
  - [feColorMatrix](#fecolormatrix)
  - [feComponentTransfer](#fecompon transfer)
  - [feComposite](#fecomposite)
  - [feConvolveMatrix](#feconvolvematrix)
  - [feDiffuseLighting](#fedisfulfeligthting)
  - [feDisplacementMap](#fedisplacementmap)
  - [feDistantLight](#fedistantlight)
  - [feDropShadow](#fedropshadow)
  - [feFlood](#feflood)
  - [feFuncA/B/G/R](#fefunca-b-g-r)
  - [feGaussianBlur](#fegaussianblur)
  - [feImage](#feimage)
  - [feMerge](#femerge)
  - [feMergeNode](#femergenode)
  - [feMorphology](#femorphology)
  - [feOffset](#feoffset)
  - [fePointLight](#fepointlight)
  - [feSpecularLighting](#fespecularlighting)
  - [feSpotLight](#fespotlight)
  - [feTile](#fetile)
  - [feTurbulence](#feturbulence)
- [Input Sources](#input-sources)
- [References](#references)

---

## Overview

SVG Filter Effects are defined in the [Filter Effects Module Level 1](https://www.w3.org/TR/filter-effects-1/) specification.

**Key Concepts:**

1. **Filter Effects**: Visual effects applied to graphics elements (blur, drop shadow, color manipulation, etc.)
2. **Filter Primitives**: Basic building blocks that can be combined to create complex effects
3. **Filter Chain**: Sequence of filter primitives where output of one feeds into the next
4. **Input Sources**: Images that filter primitives can operate on

---

## Filter Element

The `<filter>` element defines a filter effect that can be referenced by other elements via the `filter` property or attribute.

**Content Model:** Filter primitive elements (one or more)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `id` | `<string>` | Any string | Required | Element identifier |
| `x` | `<coordinate>` | Any coordinate | `-10%` | Filter region start X |
| `y` | `<coordinate>` | Any coordinate | `-10%` | Filter region start Y |
| `width` | `<length>` | Length/percentage | `120%` | Filter region width |
| `height` | `<length>` | Length/percentage | `120%` | Filter region height |
| `filterUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `objectBoundingBox` | Coordinate system for filter region |
| `primitiveUnits` | `<coordinate-units>` | `userSpaceOnUse` \| `objectBoundingBox` | `userSpaceOnUse` | Coordinate system for filter primitives |
| `href` | `<url>` | URL to filter | - | Reference to another filter |

**Filter Region:**

The filter region is the area where the filter is applied. The default values (`-10%`, `-10%`, `120%`, `120%`) provide a 10% padding around the affected object.

**Examples:**

```xml
<filter id="myBlur">
  <feGaussianBlur in="SourceGraphic" stdDeviation="5" />
</filter>

<rect width="100" height="100" filter="url(#myBlur)" />
```

**References:**
- [W3C SVG 2 - Filter Element](https://www.w3.org/TR/SVG2/filters.html#FilterElement)
- [Filter Effects Module - Filter Element](https://www.w3.org/TR/filter-effects-1/#filterElement)

---

## Filter Primitive Elements

The following 18 filter primitive elements can be nested inside a `<filter>` element.

### feBlend

Blends two input images using a specified blend mode.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | First input |
| `in2` | `<string>` | Input identifier | `SourceAlpha` | Second input |
| `mode` | `<blend-mode>` | `normal` \| `multiply` \| `screen` \| `overlay` \| `darken` \| `lighten` \| `color-dodge` \| `color-burn` \| `hard-light` \| `soft-light` \| `difference` \| `exclusion` \| `hue` \| `saturation` \| `color` \| `luminosity` | `normal` | Blend mode |
| `id` | `<string>` | Any string | Required | Element identifier |

**Blend Modes:**

| Mode | Description |
|------|-------------|
| `normal` | Simple overlay |
| `multiply` | Multiply colors |
| `screen` | Screen colors |
| `overlay` | Multiply or screen |
| `darken` | Darker of two |
| `lighten` | Lighter of two |
| `color-dodge` | Brighten |
| `color-burn` | Darken |
| `hard-light` | Multiply or screen |
| `soft-light` | Soft multiply |
| `difference` | Difference |
| `exclusion` | Exclusion |
| `hue` | Hue |
| `saturation` | Saturation |
| `color` | Color |
| `luminosity` | Luminosity |

**Examples:**

```xml
<feBlend in="SourceGraphic" in2="blur-result" mode="multiply" />
```

**References:**
- [Filter Effects - feBlend](https://www.w3.org/TR/filter-effects-1/#feBlend)

---

### feColorMatrix

Performs a matrix multiplication on the input image's color and alpha values.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `type` | `<color-matrix-type>` | `matrix` \| `saturate` \| `hue-rotate` \| `luminance-to-alpha` | `matrix` | Matrix type |
| `values` | `<list>` | Space-separated numbers | - | Matrix values |
| `id` | `<string>` | Any string | Required | Element identifier |

**Matrix Types:**

#### `matrix`
Full 5×4 matrix (20 values):
```
Rr Rg Rb Ra 0
Gr Gg Gb Ga 0
Br Bg Bb Ba 0
Ar Ag Ab Aa 1
```

#### `saturate`
Single value (0-10):
- `0` = grayscale
- `1` = no change
- `>1` = increased saturation

#### `hue-rotate`
Angle (0-360):
- Rotates hue around the color circle

#### `luminance-to-alpha`
No parameters:
- Uses luminance of input as alpha channel

**Examples:**

```xml
<!-- Full matrix -->
<feColorMatrix type="matrix" values="1 0 0 0 0
                                     0 1 0 0 0
                                     0 0 1 0 0
                                     0 0 0 1 0" />

<!-- Saturate -->
<feColorMatrix type="saturate" values="2" />

<!-- Grayscale -->
<feColorMatrix type="saturate" values="0" />

<!-- Hue rotate 180 degrees -->
<feColorMatrix type="hue-rotate" values="180" />

<!-- Luminance to alpha -->
<feColorMatrix type="luminance-to-alpha" />
```

**References:**
- [Filter Effects - feColorMatrix](https://www.w3.org/TR/filter-effects-1/#feColorMatrix)

---

### feComponentTransfer

Transfers color components independently using transfer functions.

**Content Model:** feFuncR, feFuncG, feFuncB, feFuncA (optional)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `id` | `<string>` | Any string | Required | Element identifier |

**Child Elements:**

| Element | Description |
|---------|-------------|
| `feFuncR` | Red component function |
| `feFuncG` | Green component function |
| `feFuncB` | Blue component function |
| `feFuncA` | Alpha component function |

**Transfer Functions:**

| Function | Attribute | Description |
|----------|-----------|-------------|
| `identity` | `type="identity"` | No change (default) |
| `table` | `type="table"` | Table lookup |
| `discrete` | `type="discrete"` | Discretize values |
| `linear` | `type="linear"` | Linear function |
| `gamma` | `type="gamma"` | Gamma correction |

**Examples:**

```xml
<feComponentTransfer>
  <feFuncR type="linear" slope="0.5" intercept="0.5" />
  <feFuncG type="table" tableValues="0 0.5 1" />
  <feFuncB type="gamma" amplitude="1" exponent="2" offset="0" />
  <feFuncA type="discrete" tableValues="0 1" />
</feComponentTransfer>
```

**References:**
- [Filter Effects - feComponentTransfer](https://www.w3.org/TR/filter-effects-1/#feComponentTransfer)

---

### feComposite

Performs compositing operations on two input images.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | First input |
| `in2` | `<string>` | Input identifier | `SourceAlpha` | Second input |
| `operator` | `<composite-operator>` | `over` \| `in` \| `out` \| `atop` \| `xor` \| `arithmetic` \| `lighten` \| `darken` | `over` | Composite operator |
| `k1-k4` | `<number>` | Coefficients | `0` | Arithmetic coefficients |
| `id` | `<string>` | Any string | Required | Element identifier |

**Composite Operators:**

| Operator | Description |
|----------|-------------|
| `over` | Source over destination |
| `in` | Source inside destination |
| `out` | Source outside destination |
| `atop` | Source atop destination |
| `xor` | Exclusive OR |
| `arithmetic` | (k1×input1×input2) + (k2×input1) + (k3×input2) + k4 |
| `lighten` | Maximum of two |
| `darken` | Minimum of two |

**Examples:**

```xml
<feComposite in="SourceGraphic" in2="mask" operator="in" />
<feComposite in="SourceGraphic" in2="mask" operator="xor" />
```

**References:**
- [Filter Effects - feComposite](https://www.w3.org/TR/filter-effects-1/#feComposite)

---

### feConvolveMatrix

Applies a convolution matrix to the input image.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `kernelMatrix` | `<list>` | Space-separated numbers | - | Kernel matrix values |
| `order` | `<integer>` | Odd number | `3` | Kernel size (3, 5, 7, etc.) |
| `divisor` | `<number>` | Number | (sum of matrix values) | Divisor for result |
| `bias` | `<number>` | Number | `0` | Bias added to each result |
| `targetX` | `<integer>` | Integer | `0` | X coordinate for kernel application |
| `targetY` | `<integer>` | Integer | `0` | Y coordinate for kernel application |
| `edgeMode` | `<edge-mode>` | `wrap` \| `duplicate` \| `none` | `duplicate` | How to handle edges |
| `preserveAlpha` | `<boolean>` | `true` \| `false` | `false` | Preserve alpha channel |
| `kernelUnitLength` | `<length>` | X Y | `1 1` | Physical units of kernel |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<!-- Blur kernel -->
<feConvolveMatrix order="3" kernelMatrix="1 2 1 2 4 2 1 2 1" divisor="16" />

<!-- Sharpen kernel -->
<feConvolveMatrix order="3" kernelMatrix="0 -1 0 -1 5 -1 0 -1 0" />

<!-- Emboss kernel -->
<feConvolveMatrix order="3" kernelMatrix="-2 -1 0 -1 1 1 0 1 2" bias="128" />
```

**References:**
- [Filter Effects - feConvolveMatrix](https://www.w3.org/TR/filter-effects-1/#feConvolveMatrix)

---

### feDiffuseLighting

Creates a three-dimensional lighting effect using a diffuse lighting model.

**Content Model:** feDiffuseLighting, fePointLight, feSpotLight, feDistantLight

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image (displacement map) |
| `surfaceScale` | `<number>` | Number | `5` | Surface scaling factor |
| `diffuseConstant` | `<number>` | Number | `1` | Diffuse constant (k_d) |
| `kernelUnitLength` | `<length>` | X Y | `1 1` | Physical units of kernel |
| `id` | `<string>` | Any string | Required | Element identifier |

**Lighting Color:**
Can be set via `lighting-color` CSS property or attribute.

**Child Light Elements:**

| Element | Description |
|---------|-------------|
| `fePointLight` | Point light source |
| `feSpotLight` | Spot light source |
| `feDistantLight` | Distant light source |

**Examples:**

```xml
<feDiffuseLighting surfaceScale="5" diffuseConstant="1" lighting-color="white">
  <feDistantLight azimuth="45" elevation="60" />
</feDiffuseLighting>
```

**References:**
- [Filter Effects - feDiffuseLighting](https://www.w3.org/TR/filter-effects-1/#feDiffuseLighting)

---

### feDisplacementMap

Displaces pixels in an image based on another image.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Primary input |
| `in2` | `<string>` | Input identifier | `SourceAlpha` | Displacement map |
| `scale` | `<number>` | Number | `0` | Displacement scale |
| `xChannelSelector` | `<channel-selector>` | `R` \| `G` \| `B` \| `A` \| `auto` | `auto` | X channel |
| `yChannelSelector` | `<channel-selector>` | `R` \| `G` \| `B` \| `A` \| `auto` | `auto` | Y channel |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feDisplacementMap scale="10" xChannelSelector="R" yChannelSelector="G" />
```

**References:**
- [Filter Effects - feDisplacementMap](https://www.w3.org/TR/filter-effects-1/#feDisplacementMap)

---

### feDistantLight

Defines a distant light source for lighting effects.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `azimuth` | `<angle>` | Angle | `0` | Horizontal angle |
| `elevation` | `<angle>` | Angle | `0` | Vertical angle |
| `id` | `<string>` | Any string | Required | Element identifier |

**Notes:**
- Must be child of `feDiffuseLighting` or `feSpecularLighting`
- Azimuth: 0 = North, increases clockwise
- Elevation: 0 = horizon, 90 = zenith

**Examples:**

```xml
<feDistantLight azimuth="45" elevation="60" />
```

**References:**
- [Filter Effects - feDistantLight](https://www.w3.org/TR/filter-effects-1/#feDistantLight)

---

### feDropShadow

Creates a drop shadow effect.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `dx` | `<length>` | Length | `0` | Horizontal offset |
| `dy` | `<length>` | Length | `0` | Vertical offset |
| `stdDeviation` | `<length>` | Length | `0` | Blur radius |
| `color` | `<color>` | Color | `black` | Shadow color |
| `opacity` | `<number>` | Number | `1` | Shadow opacity |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feDropShadow dx="2" dy="2" stdDeviation="3" />

<!-- With color -->
<feDropShadow dx="5" dy="5" stdDeviation="5" flood-color="blue" />
```

**CSS Equivalent:**
```css
filter: drop-shadow(2px 2px 3px rgba(0,0,0,0.5));
```

**References:**
- [Filter Effects - feDropShadow](https://www.w3.org/TR/filter-effects-1/#feDropShadow)

---

### feFlood

Creates a flooded rectangle filled with color and opacity.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `flood-color` | `<color>` | Color | `black` | Flood color |
| `flood-opacity` | `<number>` | Number | `1` | Flood opacity |
| `id` | `<string>` | Any string | Required | Element identifier |

**Notes:**
- Creates a solid color rectangle
- Typically combined with `feComposite` or `feMorphology` for effects

**Examples:**

```xml
<feFlood flood-color="red" flood-opacity="0.5" />
```

**References:**
- [Filter Effects - feFlood](https://www.w3.org/TR/filter-effects-1/#feFlood)

---

### feFuncA / feFuncB / feFuncG / feFuncR

Define transfer functions for individual color channels.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `type` | `<transfer-function-type>` | `identity` \| `table` \| `discrete` \| `linear` \| `gamma` | `identity` | Function type |
| `tableValues` | `<list>` | Space-separated numbers | - | Table values |
| `slope` | `<number>` | Number | `1` | Linear slope |
| `intercept` | `<number>` | Number | `0` | Linear intercept |
| `amplitude` | `<number>` | Number | `1` | Gamma amplitude |
| `exponent` | `<number>` | Number | `1` | Gamma exponent |
| `offset` | `<number>` | Number | `0` | Gamma offset |
| `id` | `<string>` | Any string | Required | Element identifier |

**Function Types:**

| Type | Description | Parameters |
|------|-------------|------------|
| `identity` | No change | - |
| `table` | Table lookup | `tableValues` |
| `discrete` | Discretize | `tableValues` |
| `linear` | Linear function | `slope`, `intercept` |
| `gamma` | Gamma correction | `amplitude`, `exponent`, `offset` |

**Examples:**

```xml
<feFuncR type="linear" slope="0.5" intercept="0.5" />
<feFuncG type="gamma" exponent="2" />
<feFuncB type="table" tableValues="0 0.5 1" />
<feFuncA type="discrete" tableValues="0 1" />
```

**References:**
- [Filter Effects - feFuncA/B/G/R](https://www.w3.org/TR/filter-effects-1/#feFuncAElement)

---

### feGaussianBlur

Applies a Gaussian blur to the input image.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `stdDeviation` | `<length>` | Length | `0` | Blur radius |
| `id` | `<string>` | Any string | Required | Element identifier |

**Notes:**
- Can specify X and Y separately: `stdDeviation="5 10"`
- Large values create more blur
- `0` creates no blur

**Examples:**

```xml
<feGaussianBlur stdDeviation="5" />
<feGaussianBlur stdDeviation="2 10" />
```

**References:**
- [Filter Effects - feGaussianBlur](https://www.w3.org/TR/filter-effects-1/#feGaussianBlur)

---

### feImage

References an external image as a filter input.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `href` | `<url>` | URL | Required | Image URL |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | Aspect ratio |
| `x` | `<coordinate>` | Any coordinate | `0` | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Y position |
| `width` | `<length>` | Length/percentage | `100%` | Width |
| `height` | `<length>` | Length/percentage | `100%` | Height |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feImage href="texture.jpg" />
```

**References:**
- [Filter Effects - feImage](https://www.w3.org/TR/filter-effects-1/#feImage)

---

### feMerge

Merges multiple inputs into a single output.

**Content Model:** feMergeNode (one or more)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | - | - |
| `id` | `<string>` | Any string | Required | Element identifier |

**Child Elements:**

| Element | Description |
|---------|-------------|
| `feMergeNode` | Single input for merge |

**Examples:**

```xml
<feMerge>
  <feMergeNode in="blur-result" />
  <feMergeNode in="SourceGraphic" />
</feMerge>
```

**References:**
- [Filter Effects - feMerge](https://www.w3.org/TR/filter-effects-1/#feMerge)

---

### feMergeNode

Single input for feMerge.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feMergeNode in="blur-result" />
<feMergeNode in="SourceAlpha" />
```

**References:**
- [Filter Effects - feMergeNode](https://www.w3.org/TR/filter-effects-1/#feMergeNode)

---

### feMorphology

Performs morphological operations (erode or dilate).

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `operator` | `<morphology-operator>` | `erode` \| `dilate` | `erode` | Morphology operator |
| `radius` | `<length>` | Length | `1` | Radius |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feMorphology operator="erode" radius="2" />
<feMorphology operator="dilate" radius="3" />
```

**References:**
- [Filter Effects - feMorphology](https://www.w3.org/TR/filter-effects-1/#feMorphology)

---

### feOffset

Offsets the input image.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `dx` | `<length>` | Length | `0` | Horizontal offset |
| `dy` | `<length>` | Length | `0` | Vertical offset |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feOffset dx="5" dy="5" />
```

**References:**
- [Filter Effects - feOffset](https://www.w3.org/TR/filter-effects-1/#feOffset)

---

### fePointLight

Defines a point light source for lighting effects.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Y position |
| `z` | `<coordinate>` | Any coordinate | `0` | Z position |
| `id` | `<string>` | Any string | Required | Element identifier |

**Notes:**
- Must be child of `feDiffuseLighting` or `feSpecularLighting`
- 3D position in coordinate space

**Examples:**

```xml
<fePointLight x="100" y="100" z="500" />
```

**References:**
- [Filter Effects - fePointLight](https://www.w3.org/TR/filter-effects-1/#fePointLight)

---

### feSpecularLighting

Creates a three-dimensional lighting effect using a specular lighting model.

**Content Model:** feSpecularLighting, fePointLight, feSpotLight, feDistantLight

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image (displacement map) |
| `surfaceScale` | `<number>` | Number | `5` | Surface scaling factor |
| `specularConstant` | `<number>` | Number | `1` | Specular constant (k_s) |
| `specularExponent` | `<number>` | Number | `1` | Specular exponent (shininess) |
| `kernelUnitLength` | `<length>` | X Y | `1 1` | Physical units of kernel |
| `id` | `<string>` | Any string | Required | Element identifier |

**Lighting Color:**
Can be set via `lighting-color` CSS property or attribute.

**Examples:**

```xml
<feSpecularLighting surfaceScale="5" specularConstant="1" specularExponent="20" lighting-color="white">
  <feSpotLight x="100" y="100" z="500" pointsAt="0 0 0" specularExponent="10" />
</feSpecularLighting>
```

**References:**
- [Filter Effects - feSpecularLighting](https://www.w3.org/TR/filter-effects-1/#feSpecularLighting)

---

### feSpotLight

Defines a spot light source for lighting effects.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | X position |
| `y` | `<coordinate>` | Any coordinate | `0` | Y position |
| `z` | `<coordinate>` | Any coordinate | `0` | Z position |
| `pointsAt` | `<coordinate>` | X Y Z | `0 0 0` | X Y Z point |
| `specularExponent` | `<number>` | Number | `1` | Specular exponent |
| `limitingConeAngle` | `<angle>` | Angle | `0` | Limiting cone angle |
| `id` | `<string>` | Any string | Required | Element identifier |

**Notes:**
- Must be child of `feDiffuseLighting` or `feSpecularLighting`
- Creates a cone-shaped light source

**Examples:**

```xml
<feSpotLight x="100" y="100" z="500" pointsAt="0 0 0" specularExponent="10" limitingConeAngle="0.5" />
```

**References:**
- [Filter Effects - feSpotLight](https://www.w3.org/TR/filter-effects-1/#feSpotLight)

---

### feTile

Tiles the input image to fill the filter region.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `in` | `<string>` | Input identifier | `SourceGraphic` | Input image |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<feTile in="pattern" />
```

**References:**
- [Filter Effects - feTile](https://www.w3.org/TR/filter-effects-1/#feTile)

---

### feTurbulence

Generates turbulent textures using fractal noise.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `baseFrequency` | `<frequency>` | X Y | `0` | Base frequency |
| `numOctaves` | `<integer>` | Integer | `1` | Number of octaves |
| `seed` | `<integer>` | Integer | `0` | Random seed |
| `stitchTiles` | `<stitch>` | `stitch` \| `noStitch` | `noStitch` | Whether to stitch tiles |
| `type` | `<turbulence-type>` | `turbulence` \| `fractalNoise` | `turbulence` | Noise type |
| `id` | `<string>` | Any string | Required | Element identifier |

**Examples:**

```xml
<!-- Fractal noise -->
<feTurbulence type="fractalNoise" baseFrequency="0.01" numOctaves="3" />

<!-- Turbulence -->
<feTurbulence type="turbulence" baseFrequency="0.1" numOctaves="2" seed="5" />

<!-- Displacement map texture -->
<feTurbulence type="fractalNoise" baseFrequency="0.05" numOctaves="4" />
```

**References:**
- [Filter Effects - feTurbulence](https://www.w3.org/TR/filter-effects-1/#feTurbulence)

---

## Input Sources

Filter primitives can receive input from various sources:

| Input Source | Description |
|--------------|-------------|
| `SourceGraphic` | The SVG element being filtered |
| `SourceAlpha` | Alpha channel of SourceGraphic |
| `BackgroundImage` | Background image behind the element |
| `BackgroundAlpha` | Alpha channel of BackgroundImage |
| `SourceGraphicFill` | Fill of SourceGraphic |
| `SourceGraphicStroke` | Stroke of SourceGraphic |
| `Result Name` | Name given to output of another primitive via `result` attribute |

**Setting Result Names:**

```xml
<feGaussianBlur in="SourceGraphic" stdDeviation="5" result="blur" />
<feOffset in="blur" dx="5" dy="5" result="shadow" />
<feComposite in="SourceGraphic" in2="shadow" operator="over" />
```

---

## Examples

### Drop Shadow Effect

```xml
<defs>
  <filter id="dropShadow">
    <feDropShadow dx="5" dy="5" stdDeviation="5" flood-color="black" />
  </filter>
</defs>

<rect width="100" height="100" filter="url(#dropShadow)" />
```

### Blur Effect

```xml
<defs>
  <filter id="blur">
    <feGaussianBlur stdDeviation="5" />
  </filter>
</defs>

<rect width="100" height="100" filter="url(#blur)" />
```

### Glow Effect

```xml
<defs>
  <filter id="glow">
    <feGaussianBlur stdDeviation="5" result="coloredBlur" />
    <feMerge>
      <feMergeNode in="coloredBlur" />
      <feMergeNode in="SourceGraphic" />
    </feMerge>
  </filter>
</defs>

<text filter="url(#glow)">Glowing Text</text>
```

### Sepia Effect

```xml
<defs>
  <filter id="sepia">
    <feColorMatrix type="matrix" values="
      0.393 0.769 0.189 0 0
      0.349 0.686 0.168 0 0
      0.272 0.534 0.131 0 0
      0 0 0 1 0" />
  </filter>
</defs>

<image filter="url(#sepia)" href="photo.jpg" />
```

### Invert Effect

```xml
<defs>
  <filter id="invert">
    <feComponentTransfer>
      <feFuncR type="table" tableValues="1" />
      <feFuncG type="table" tableValues="1" />
      <feFuncB type="table" tableValues="1" />
    </feComponentTransfer>
  </filter>
</defs>

<rect width="100" height="100" filter="url(#invert)" />
```

---

## See Also

- [Filter Effects Module Level 1](https://www.w3.org/TR/filter-effects-1/)
- [SVG 2 - Filters](https://www.w3.org/TR/SVG2/filters.html)
- [CSS Filter Effects](https://developer.mozilla.org/en-US/docs/Web/CSS/filter)

---

*Generated from W3C Filter Effects Module Level 1 specification. For the most up-to-date information, always refer to the official specification.*
