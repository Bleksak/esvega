# SVG 2 Animation

Comprehensive documentation of SVG animation elements and attributes.

## Table of Contents

- [Overview](#overview)
- [Animation Timing](#animation-timing)
- [Animation Elements](#animation-elements)
  - [animate](#animate)
  - [animateMotion](#animatemotion)
  - [animateTransform](#animatetransform)
  - [set](#set)
  - [mpath](#mpath)
  - [discard](#discard)
- [Animation Attributes](#animation-attributes)
- [Animation Interpolation](#animation-interpolation)
- [Animation Examples](#animation-examples)

---

## Overview

SVG animation allows dynamic changes to element properties over time without JavaScript.

**Four Animation Elements:**

| Element | Purpose |
|---------|---------|
| `<animate>` | Animate any attribute value |
| `<animateMotion>` | Animate along a path |
| `<animateTransform>` | Animate transform values |
| `<set>` | Set attribute to a value at a specific time |

**Animation Types:**

1. **Simple Animation**: Linear interpolation between `from` and `to` values
2. **Keyframe Animation**: Use `values` and `keyTimes` for custom interpolation

---

## Animation Timing

### Time Values

```
<time-value> = <duration> | <relative-time>
<duration> = <number>s | <number>ms | <number>m
<relative-time> = begin-value | end-value
```

**Examples:**
- `2s` - 2 seconds
- `500ms` - 500 milliseconds
- `1m` - 1 minute
- `begin` - When animation element starts

### Count Values

```
<count-value> = <integer> | indefinite
```

**Examples:**
- `3` - 3 times
- `indefinite` - Forever (default)

---

## Animation Elements

### animate

Animates an attribute value over time.

**Content Model:** None (empty element)

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `attributeName` | `<string>` | SVG/CSS attribute | Attribute to animate |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `from` | Any | Start value | Required | Start value |
| `to` | Any | End value | Required | End value |
| `values` | `<list>` | List of values | Required | Keyframe values |
| `keyTimes` | `<list>` | List of percentages | None | Keyframe times |
| `dur` | `<time>` | Duration | `1s` | Animation duration |
| `begin` | `<time-value>` | Time | Required | Start time |
| `end` | `<time-value>` | Time | None | End time |
| `repeatCount` | `<count>` | Count | `indefinite` | Number of repeats |
| `repeatDur` | `<time>` | Time | `indefinite` | Duration of repeats |
| `fill` | `<time-fill>` | `freeze` \| `remove` \| `auto` | `remove` | After-end behavior |
| `additive` | `<additive>` | `sum` \| `replace` | `replace` | Addition mode |
| `accumulate` | `<accumulate>` | `sum` \| `none` | `none` | Accumulation mode |
| `calcMode` | `<calc-mode>` | `discrete` \| `linear` \| `paced` \| `spline` | `linear` | Calculation mode |
| `keySplines` | `<list>` | Bezier splines | None | Cubic bezier splines |
| `href` | `<url>` | URL | None | Reference to animate |
| `id` | `<string>` | Any string | None | Element identifier |

**Attribute Type:**

| Attribute Type | Value |
|----------------|-------|
| SVG attribute | `XML` |
| CSS property | `CSS` |

**Fill Behavior:**

| Value | Description |
|-------|-------------|
| `freeze` | Keep last value after animation ends |
| `remove` | Remove animation, revert to original |
| `auto` | Default behavior |

**Additive Mode:**

| Value | Description |
|-------|-------------|
| `replace` | Replace current value |
| `sum` | Add to current value |

**Accumulate Mode:**

| Value | Description |
|-------|-------------|
| `none` | Reset each iteration |
| `sum` | Accumulate across iterations |

**Examples:**

```xml
<!-- Simple fade in/out -->
<animate attributeName="opacity"
         from="0" to="1" dur="2s"
         begin="0s" fill="freeze" />

<!-- Color animation -->
<animate attributeName="fill"
         from="red" to="blue" dur="3s"
         calcMode="linear" />

<!-- Keyframe animation -->
<animate attributeName="r"
         values="10; 20; 10"
         keyTimes="0; 0.5; 1"
         dur="2s"
         repeatCount="indefinite" />
```

**References:**
- [W3C SVG 2 - Animate Element](https://www.w3.org/TR/SVG2/animate.html#AnimateElement)

---

### animateMotion

Animates an element along a path.

**Content Model:** Optional `<mpath>` element

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `path` | `<path-data>` | Path data | Path to follow |
| `keyPoints` | `<list>` | Numbers | Key points on path |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `from` | Any | Start value | None | Start value |
| `to` | Any | End value | None | End value |
| `values` | `<list>` | List of values | None | Keyframe values |
| `dur` | `<time>` | Duration | `1s` | Animation duration |
| `begin` | `<time-value>` | Time | Required | Start time |
| `end` | `<time-value>` | Time | None | End time |
| `repeatCount` | `<count>` | Count | `indefinite` | Number of repeats |
| `repeatDur` | `<time>` | Time | `indefinite` | Duration of repeats |
| `fill` | `<time-fill>` | `freeze` \| `remove` | `remove` | After-end behavior |
| `rotate` | `<angle>` \| `<list>` | Rotation | `0` | Rotation along path |
| `origin` | `<coordinate>` | X Y | `auto` | Motion origin |
| `calcMode` | `<calc-mode>` | `linear` \| `paced` \| `spline` | `linear` | Calculation mode |
| `keySplines` | `<list>` | Bezier splines | None | Cubic bezier splines |
| `href` | `<url>` | URL | Required | Reference to element |
| `id` | `<string>` | Any string | None | Element identifier |

**Rotation:**

| Value | Description |
|-------|-------------|
| `0` | No rotation |
| `<angle>` | Fixed rotation angle |
| `auto` | Rotate tangent to path |
| `auto reverse` | Rotate opposite to path tangent |

**Origin:**

| Value | Description |
|-------|-------------|
| `auto` | Center of element |
| `x y` | Specific origin point |

**Examples:**

```xml
<!-- Move along a path -->
<animateMotion dur="5s" repeatCount="indefinite">
  <mpath href="#path1" />
</animateMotion>

<!-- With path data inline -->
<animateMotion path="M 10 10 L 100 100 L 50 150 Z"
               dur="3s" fill="freeze" />

<!-- Rotate along path -->
<animateMotion path="M 10 10 L 100 100"
               rotate="auto"
               begin="0s" dur="2s" />

<!-- Multiple key points -->
<animateMotion path="M 10 10 L 50 50 L 100 100"
               keyPoints="0; 0.5; 1"
               keyTimes="0; 0.5; 1"
               dur="3s" />
```

**References:**
- [W3C SVG 2 - AnimateMotion Element](https://www.w3.org/TR/SVG2/animate.html#AnimateMotionElement)

---

### animateTransform

Animates transform attributes.

**Content Model:** None (empty element)

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `attributeName` | `<string>` | `transform` | Must be `transform` |
| `type` | `<transform-type>` | `translate` \| `scale` \| `rotate` \| `skewX` \| `skewY` \| `matrix` | Transform type |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `from` | Any | Start value | Required | Start value |
| `to` | Any | End value | Required | End value |
| `values` | `<list>` | List of values | Required | Keyframe values |
| `dur` | `<time>` | Duration | `1s` | Animation duration |
| `begin` | `<time-value>` | Time | Required | Start time |
| `end` | `<time-value>` | Time | None | End time |
| `repeatCount` | `<count>` | Count | `indefinite` | Number of repeats |
| `repeatDur` | `<time>` | Time | `indefinite` | Duration of repeats |
| `fill` | `<time-fill>` | `freeze` \| `remove` | `remove` | After-end behavior |
| `additive` | `<additive>` | `sum` \| `replace` | `replace` | Addition mode |
| `accumulate` | `<accumulate>` | `sum` \| `none` | `none` | Accumulation mode |
| `calcMode` | `<calc-mode>` | `discrete` \| `linear` \| `paced` \| `spline` | `linear` | Calculation mode |
| `keyTimes` | `<list>` | Percentages | None | Keyframe times |
| `keySplines` | `<list>` | Bezier splines | None | Cubic bezier splines |
| `href` | `<url>` | URL | None | Reference to animateTransform |
| `id` | `<string>` | Any string | None | Element identifier |

**Transform Types:**

| Type | From Value | To Value | Example |
|------|------------|----------|---------|
| `translate` | `x [, y]` | `x [, y]` | `from="0 0"` `to="100 100"` |
| `scale` | `x [, y]` | `x [, y]` | `from="1"` `to="2"` |
| `rotate` | `angle [, cx [, cy]]` | `angle [, cx [, cy]]` | `from="0 50 50"` `to="360 50 50"` |
| `skewX` | `<angle>` | `<angle>` | `from="0deg"` `to="45deg"` |
| `skewY` | `<angle>` | `<angle>` | `from="0deg"` `to="30deg"` |
| `matrix` | `a b c d e f` | `a b c d e f` | `from="1 0 0 1 0 0"` `to="1 0 0 1 50 50"` |

**Examples:**

```xml
<!-- Translation -->
<animateTransform attributeName="transform"
                  type="translate"
                  from="0 0" to="100 100"
                  dur="2s" />

<!-- Rotation -->
<animateTransform attributeName="transform"
                  type="rotate"
                  from="0 50 50" to="360 50 50"
                  dur="3s"
                  repeatCount="indefinite" />

<!-- Scaling -->
<animateTransform attributeName="transform"
                  type="scale"
                  from="1" to="2"
                  begin="0s" dur="1s"
                  calcMode="spline"
                  keySplines="0.4 0 0.2 1"
                  repeatCount="indefinite" />

<!-- Combined transforms -->
<animateTransform attributeName="transform"
                  type="matrix"
                  values="1 0 0 1 0 0; 
                          1 0 0 1 50 0; 
                          1 0 0 1 100 0"
                  keyTimes="0; 0.5; 1"
                  dur="3s" />
```

**References:**
- [W3C SVG 2 - AnimateTransform Element](https://www.w3.org/TR/SVG2/animate.html#AnimateTransformElement)

---

### set

Sets an attribute to a specific value at a specific time.

**Content Model:** None (empty element)

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `attributeName` | `<string>` | SVG/CSS attribute | Attribute to set |
| `to` | Any | Target value | Target value |
| `begin` | `<time-value>` | Time | Start time |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `dur` | `<time>` | Duration | `0` | Duration |
| `end` | `<time-value>` | Time | None | End time |
| `fill` | `<time-fill>` | `freeze` \| `remove` | `freeze` | After-end behavior |
| `href` | `<url>` | URL | Required | Reference to element |
| `id` | `<string>` | Any string | None | Element identifier |

**Examples:**

```xml
<!-- Set opacity at specific time -->
<set attributeName="opacity"
     to="0"
     begin="2s"
     end="4s" />

<!-- Set fill color -->
<set attributeName="fill"
     to="red"
     begin="click" />

<!-- Chain multiple sets -->
<set attributeName="opacity" to="1" begin="0s" />
<set attributeName="opacity" to="0.5" begin="2s" />
<set attributeName="opacity" to="0" begin="4s" />
```

**References:**
- [W3C SVG 2 - Set Element](https://www.w3.org/TR/SVG2/animate.html#SetElement)

---

### mpath

Provides path data for animateMotion.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `href` | `<url>` | URL to path | Required | Reference to path element |
| `id` | `<string>` | Any string | None | Element identifier |

**Examples:**

```xml
<defs>
  <path id="motionPath" d="M 10 10 L 100 100 L 50 150 Z" />
</defs>

<circle cx="0" cy="0" r="10">
  <animateMotion begin="0s" dur="5s" repeatCount="indefinite">
    <mpath href="#motionPath" />
  </animateMotion>
</circle>
```

**References:**
- [W3C SVG 2 - Mpath Element](https://www.w3.org/TR/SVG2/animate.html#MpathElement)

---

### discard

Discards animation events.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `href` | `<url>` | URL to element | Required | Reference to animated element |
| `begin` | `<time-value>` | Time | Required | Time to discard |
| `id` | `<string>` | Any string | None | Element identifier |

**Examples:**

```xml
<!-- Discard animation after 5 seconds -->
<discard href="#myAnimation" begin="5s" />
```

**References:**
- [W3C SVG 2 - Discard Element](https://www.w3.org/TR/SVG2/animate.html#DiscardElement)

---

## Animation Attributes

### begin

Specifies when the animation starts.

**Syntax:**
```
<begin-value> = <time-value> | <event-value> | <mediaEvent-value> | <systemEvent-value>
```

**Time Values:**
- `2s` - 2 seconds after start
- `100ms` - 100 milliseconds
- `1m` - 1 minute
- `begin` - When animation element is ready
- `end` - When animation element ends
- `begin+1s` - 1 second after begin event

**Event Values:**
- `click` - Mouse click event
- `click.begin` - Click event start
- `click.end` - Click event end
- `mouseover.begin` - Mouse over event start

**Media Events:**
- `idle` - When no other media is playing
- `active` - When media is active
- `resume` - When media resumes
- `pause` - When media pauses
- `restart` - When media restarts
- `end` - When media ends

**Examples:**

```xml
<animate begin="0s" dur="2s" ... />
<animate begin="click" dur="1s" ... />
<animate begin="mouseover.begin" end="mouseout.end" ... />
<animate begin="mediaEvent.active" ... />
<animate begin="1s; 3s; 5s" ... /> <!-- Multiple begin times -->
```

---

### dur

Specifies the duration of the animation.

**Syntax:**
```
<duration> = <number>s | <number>ms
```

**Examples:**
- `dur="2s"` - 2 seconds
- `dur="500ms"` - 500 milliseconds
- `dur="1m"` - 1 minute

---

### begin / end

Combines multiple events with semicolons:

```xml
<animate begin="0s; click" dur="2s" ... />
<!-- Starts at 0s OR on click, whichever comes first -->
```

---

### repeatCount

Specifies how many times the animation repeats.

**Syntax:**
```
<repeat-count> = <integer> | indefinite
```

**Examples:**
- `repeatCount="3"` - Repeats 3 times (total of 4 executions)
- `repeatCount="indefinite"` - Repeats forever

---

### fill

Specifies what happens after the animation ends.

**Values:**
- `freeze` - Keep the last value
- `remove` - Remove the animation (default)
- `auto` - Default behavior

---

### calcMode

Specifies how values are interpolated between keyframes.

**Values:**

| Value | Description |
|-------|-------------|
| `linear` | Linear interpolation (default) |
| `discrete` | Jump between values |
| `paced` | Pace to maintain constant speed |
| `spline` | Use cubic Bézier splines |

**Examples:**

```xml
<animate attributeName="opacity"
         values="0; 0.5; 1"
         keyTimes="0; 0.5; 1"
         calcMode="spline"
         keySplines="0.4 0 0.2 1"
         dur="2s" />
```

---

### additive

Specifies whether to add to or replace current values.

**Values:**
- `replace` - Replace current value (default)
- `sum` - Add to current value

**Example:**

```xml
<animate attributeName="opacity"
         from="0.5" to="1"
         additive="sum"
         dur="1s" />
<!-- Adds to existing opacity -->
```

---

### accumulate

Specifies whether to accumulate across iterations.

**Values:**
- `none` - Reset each iteration (default)
- `sum` - Accumulate values across iterations

**Example:**

```xml
<animate attributeName="translate"
         from="0 0" to="10 10"
         dur="1s"
         repeatCount="5"
         accumulate="sum" />
<!-- Accumulates position: 0,0 → 10,10 → 20,20 → ... -->
```

---

## Animation Interpolation

### Value Interpolation

SVG can interpolate between compatible values:

**Numbers:**
```xml
<animate attributeName="opacity" from="0" to="1" dur="2s" />
```

**Colors:**
```xml
<animate attributeName="fill" from="red" to="blue" dur="2s" />
```

**Lengths:**
```xml
<animate attributeName="width" from="100px" to="200px" dur="2s" />
```

**Angles:**
```xml
<animate attributeName="rotate" from="0deg" to="360deg" dur="2s" />
```

**Transforms:**
```xml
<animateTransform attributeName="transform"
                  from="scale(1)"
                  to="scale(2)"
                  type="scale"
                  dur="2s" />
```

### Keyframe Interpolation

Use `values` and `keyTimes` for custom interpolation:

```xml
<animate attributeName="opacity"
         values="0; 1; 0.5; 1"
         keyTimes="0; 0.25; 0.5; 1"
         dur="2s"
         calcMode="linear" />
```

---

## Animation Examples

### Pulsing Effect

```xml
<circle cx="100" cy="100" r="50">
  <animate attributeName="r"
           values="50; 60; 50"
           keyTimes="0; 0.5; 1"
           dur="2s"
           repeatCount="indefinite" />
</circle>
```

### Rotation Animation

```xml
<rect x="50" y="50" width="100" height="100">
  <animateTransform attributeName="transform"
                    type="rotate"
                    from="0 100 100"
                    to="360 100 100"
                    dur="5s"
                    repeatCount="indefinite" />
</rect>
```

### Bouncing Ball

```xml
<circle cx="50" cy="50" r="20">
  <animate attributeName="cy"
           values="50; 200; 50"
           keyTimes="0; 0.5; 1"
           keySplines="0.5 0 0.5 1; 0.5 0 0.5 1"
           dur="1s"
           repeatCount="indefinite" />
  <animate attributeName="opacity"
           values="1; 0.5; 1"
           keyTimes="0; 0.5; 1"
           dur="1s"
           repeatCount="indefinite" />
</circle>
```

### Color Cycle

```xml
<rect width="100" height="100">
  <animate attributeName="fill"
           values="red; yellow; green; blue; red"
           keyTimes="0; 0.25; 0.5; 0.75; 1"
           dur="4s"
           repeatCount="indefinite" />
</rect>
```

### Path Following

```xml
<defs>
  <path id="orbit" d="M 100 50 A 50 50 0 1 1 99 50" />
</defs>

<circle cx="0" cy="0" r="10">
  <animateMotion dur="3s" repeatCount="indefinite">
    <mpath href="#orbit" />
  </animateMotion>
</circle>
```

### Multiple Animations on Same Element

```xml
<circle cx="100" cy="100" r="30">
  <animate attributeName="cx"
           values="100; 150; 100"
           keyTimes="0; 0.5; 1"
           dur="2s"
           repeatCount="indefinite" />
  <animate attributeName="cy"
           values="100; 80; 100"
           keyTimes="0; 0.5; 1"
           dur="2s"
           repeatCount="indefinite" />
  <animate attributeName="fill"
           values="red; blue; red"
           keyTimes="0; 0.5; 1"
           dur="2s"
           repeatCount="indefinite" />
</circle>
```

### Animation with Events

```xml
<rect x="0" y="50" width="50" height="50" fill="gray">
  <animate attributeName="x"
           from="0" to="500"
           begin="click"
           dur="2s"
           fill="freeze" />
</rect>
```

---

## See Also

- [SVG 2 - Animation](https://www.w3.org/TR/SVG2/animate.html)
- [W3C SMIL Animation](https://www.w3.org/TR/SMI L/)
- [CSS Animations](https://www.w3.org/TR/css-animations-1/)

---

*Generated from W3C SVG 2 Specification. For the most up-to-date information, always refer to the official specification.*
