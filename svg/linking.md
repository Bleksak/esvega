# SVG 2 Linking and Interactive Elements

Comprehensive documentation of SVG linking, scripting, and interactive elements.

## Table of Contents

- [Overview](#overview)
- [Linking Elements](#linking-elements)
  - [a (anchor)](#a-anchor)
  - [view](#view)
- [Scripting Elements](#scripting-elements)
  - [script](#script)
  - [style](#style)
- [Interactive Elements](#interactive-elements)
  - [cursor](#cursor)
  - [color-profile](#color-profile)
  - [symbol](#symbol)
- [Markers](#markers)
  - [marker](#marker)
- [Events](#events)
- [Event Handlers](#event-handlers)
- [References](#references)

---

## Overview

SVG provides several mechanisms for linking, scripting, and interactivity.

**Key Components:**

| Component | Purpose |
|-----------|---------|
| Linking | Create hyperlinks and views |
| Scripting | Add behavior via JavaScript |
| Styling | Apply CSS styles |
| Interactive | Handle user events |
| Markers | Define reusable symbols |

---

## Linking Elements

### a (anchor)

Creates hyperlinks in SVG graphics.

**Content Model:** Any graphics elements

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `href` | `<url>` | Any URL | Target URL |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `target` | `<string>` | Target name | `_blank` | Target window/frame |
| `hreflang` | `<string>` | Language code | None | Link language |
| `type` | `<string>` | MIME type | None | Link type |
| `rel` | `<string>` | Relationship | None | Relationship |
| `rev` | `<string>` | Reverse relationship | None | Reverse relationship |
| `download` | `<string>` | Filename | None | Download filename |
| `ping` | `<string>` | URL list | None | Ping URLs |
| `referrerpolicy` | `<string>` | Policy | None | Referrer policy |
| `id` | `<string>` | Any string | None | Element identifier |
| `class` | `<string>` | Class names | None | CSS classes |
| `style` | `<string>` | CSS properties | None | Inline styles |
| `transform` | `<transform-list>` | List of transforms | `none` | Transformations |

**Target Values:**

| Value | Description |
|-------|-------------|
| `_self` | Current frame |
| `_blank` | New window/tab (default) |
| `_parent` | Parent frame |
| `_top` | Topmost frame |
| `framename` | Specific frame name |

**MIME Types:**

| Type | Value |
|------|-------|
| SVG | `image/svg+xml` |
| HTML | `text/html` |
| PDF | `application/pdf` |
| Plain text | `text/plain` |

**Examples:**

```xml
<!-- Simple link -->
<a href="https://www.w3.org">
  <text x="0" y="20">W3C</text>
</a>

<!-- Link with target -->
<a href="page.html" target="_blank">
  <rect width="100" height="50" />
</a>

<!-- External image link -->
<a href="large-image.jpg">
  <image href="thumb.jpg" width="100" height="100" />
</a>

<!-- Email link -->
<a href="mailto:example@example.com">
  <text x="0" y="20">Contact Us</text>
</a>

<!-- Download link -->
<a href="document.pdf" download="report.pdf">
  <text x="0" y="20">Download Report</text>
</a>

<!-- Group of linked elements -->
<a href="https://example.com">
  <rect x="0" y="0" width="100" height="100" fill="blue" />
  <text x="10" y="50">Click me</text>
</a>
```

**References:**
- [W3C SVG 2 - A Element](https://www.w3.org/TR/SVG2/links.html#AElement)

---

### view

Defines a viewport view that can be linked to.

**Content Model:** None (empty element)

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `id` | `<string>` | Any string | View identifier |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `viewBox` | `<viewbox>` | viewBox | Required | ViewBox definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | Aspect ratio |
| `zoomAndPan` | `<zoom-and-pan>` | `disable` \| `magnify` | `magnify` | Zoom/pan behavior |
| `id` | `<string>` | Any string | None | Element identifier |

**Zoom and Pan Values:**

| Value | Description |
|-------|-------------|
| `magnify` | Allow zooming and panning (default) |
| `disable` | Disable zooming and panning |

**Examples:**

```xml
<defs>
  <view id="zoom1" viewBox="0 0 100 100" />
  <view id="zoom2" viewBox="0 0 200 200" />
</defs>

<svg viewBox="0 0 500 500" xmlns="http://www.w3.org/2000/svg">
  <!-- Link to specific view -->
  <a href="#zoom1">
    <text x="10" y="30">View 1</text>
  </a>
  <a href="#zoom2">
    <text x="10" y="60">View 2</text>
  </a>
</svg>
```

**References:**
- [W3C SVG 2 - View Element](https://www.w3.org/TR/SVG2/links.html#ViewElement)

---

## Scripting Elements

### script

Contains or references scripts for SVG behavior.

**Content Model:** Script code (text) or None (external)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `type` | `<string>` | MIME type | `text/ecmascript` | Script type |
| `src` | `<url>` | URL | None | External script source |
| `href` | `<url>` | URL | None | Script URL (alternative) |
| `event` | `<string>` | Event name | Required | Event handler |
| `id` | `<string>` | Any string | None | Element identifier |
| `class` | `<string>` | Class names | None | CSS classes |
| `style` | `<string>` | CSS properties | None | Inline styles |

**Script Types:**

| Type | Value | Description |
|------|-------|-------------|
| ECMAScript | `text/ecmascript` | JavaScript (default) |
| JavaScript | `text/javascript` | JavaScript |
| JScript | `text/jscript` | Microsoft JScript |
| VBScript | `text/vbscript` | Microsoft VBScript |

**Examples:**

```xml
<!-- Inline script -->
<script type="text/ecmascript">
  <![CDATA[
    function rotateElement() {
      var element = document.getElementById('myCircle');
      element.setAttribute('transform', 'rotate(90)');
    }
  ]]>
</script>

<!-- Event-based script -->
<script type="text/ecmascript" event="click rotateElement()">
</script>

<!-- External script -->
<script type="text/ecmascript" src="script.js">
</script>

<!-- Link to function -->
<a href="javascript:rotateElement()">
  <text x="0" y="20">Click me</text>
</a>
```

**Notes:**
- Use `<![CDATA[` for inline scripts to avoid XML parsing issues
- `event` attribute triggers script on specific events
- External scripts are preferred for larger codebases

**References:**
- [W3C SVG 2 - Script Element](https://www.w3.org/TR/SVG2/script.html#ScriptElement)

---

### style

Contains CSS styles for the SVG document.

**Content Model:** CSS text

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `type` | `<string>` | MIME type | `text/css` | Style type |
| `media` | `<string>` | Media queries | `all` | Media queries |
| `title` | `<string>` | Style sheet title | None | Style title |
| `id` | `<string>` | Any string | None | Element identifier |

**Examples:**

```xml
<style type="text/css">
  <![CDATA[
    circle {
      fill: red;
      stroke: blue;
      stroke-width: 2px;
    }
    
    rect:hover {
      opacity: 0.5;
    }
    
    .class1 {
      fill: green;
    }
    
    #id1 {
      fill: yellow;
    }
  ]]>
</style>

<!-- External stylesheets -->
<link rel="stylesheet" href="styles.css" type="text/css" />
```

**CSS Features:**
- All standard CSS properties
- SVG presentation attributes as CSS properties
- CSS selectors (class, ID, element)
- Media queries for responsive design
- Animations via CSS

**References:**
- [W3C SVG 2 - Style Element](https://www.w3.org/TR/SVG2/styling.html#StyleElement)

---

## Interactive Elements

### cursor

Specifies the cursor to display when the mouse is over an element.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `x` | `<coordinate>` | Any coordinate | `0` | X position of hotspot |
| `y` | `<coordinate>` | Any coordinate | `0` | Y position of hotspot |
| `href` | `<url>` | URL to cursor | Required | Cursor image reference |
| `id` | `<string>` | Any string | None | Element identifier |

**Examples:**

```xml
<defs>
  <cursor id="customCursor" x="10" y="10" href="cursor.png" />
</defs>

<circle cx="100" cy="100" r="50" cursor="url(#customCursor)" />

<!-- Multiple cursors -->
<cursor id="handCursor" x="0" y="0" href="hand.cur" />
<cursor id="crossCursor" x="8" y="8" href="cross.png" />
```

**Notes:**
- Cursor is an SVG image or external cursor file
- Hotspot defines which point of cursor image aligns with mouse position
- Browser may ignore custom cursors for accessibility

**References:**
- [W3C SVG 2 - Cursor Element](https://www.w3.org/TR/SVG2/interact.html#CursorElement)

---

### color-profile

Specifies color profile information.

**Content Model:** None (empty element)

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `name` | `<string>` | Profile name | Required | Profile identifier |
| `href` | `<url>` | URL to profile | Required | ICC profile URL |
| `local` | `<string>` | Local profile | None | Local profile name |
| `intent` | `<rendering-intent>` | `auto` \| `perceptual` \| `relative` \| `saturation` \| `absolute` | `auto` | Rendering intent |
| `id` | `<string>` | Any string | None | Element identifier |

**Rendering Intents:**

| Value | Description |
|-------|-------------|
| `auto` | Browser chooses (default) |
| `perceptual` | Preserves visual relationships |
| `relative` | Relative colorimetric |
| `saturation` | Preserves saturation |
| `absolute` | Absolute colorimetric |

**Examples:**

```xml
<defs>
  <color-profile name="sRGB" href="sRGB.icc" />
  <color-profile name="AdobeRGB" href="adobeRGB.icc" intent="perceptual" />
</defs>

<svg color-profile="url(#sRGB)">
  <!-- Content -->
</svg>
```

**References:**
- [W3C SVG 2 - Color Profile Element](https://www.w3.org/TR/SVG2/struct.html#ColorProfileElement)

---

### symbol

Defines a reusable graphic template.

**Content Model:** Any graphics elements

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `id` | `<string>` | Any string | Required | Element identifier |
| `viewBox` | `<viewbox>` | viewBox | Required | ViewBox definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | Aspect ratio |
| `overflow` | `<overflow>` | See `<overflow>` | `visible` | Overflow behavior |
| `class` | `<string>` | Class names | None | CSS classes |
| `style` | `<string>` | CSS properties | None | Inline styles |

**Examples:**

```xml
<defs>
  <symbol id="icon-star" viewBox="0 0 24 24">
    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
  </symbol>
</defs>

<!-- Use symbol -->
<use href="#icon-star" x="10" y="10" width="24" height="24" />
<use href="#icon-star" x="50" y="10" width="48" height="48" style="fill: gold;" />
```

**References:**
- [W3C SVG 2 - Symbol Element](https://www.w3.org/TR/SVG2/struct.html#SymbolElement)

---

## Markers

### marker

Defines a marker symbol for stroke ends and joints.

**Content Model:** Any graphics elements

**Required Attributes:**

| Attribute | Type | Values | Description |
|-----------|------|--------|-------------|
| `id` | `<string>` | Any string | Marker identifier |

**Common Attributes:**

| Attribute | Type | Values | Default | Description |
|-----------|------|--------|---------|-------------|
| `viewBox` | `<viewbox>` | viewBox | Required | ViewBox definition |
| `preserveAspectRatio` | `<preserve-aspect-ratio>` | See `<preserve-aspect-ratio>` | `xMidYMid meet` | Aspect ratio |
| `refX` | `<coordinate>` | Coordinate | `0` | Reference X position |
| `refY` | `<coordinate>` | Coordinate | `0` | Reference Y position |
| `markerUnits` | `<marker-units>` | `strokeWidth` \| `userSpaceOnUse` | `strokeWidth` | Marker coordinate system |
| `markerWidth` | `<length>` | Length | `3` | Marker width |
| `markerHeight` | `<length>` | Length | `3` | Marker height |
| `orient` | `<angle>` \| `<string>` | Orientation | `auto` | Marker orientation |
| `class` | `<string>` | Class names | None | CSS classes |
| `style` | `<string>` | CSS properties | None | Inline styles |

**Marker Units:**

| Value | Description |
|-------|-------------|
| `strokeWidth` | Size relative to stroke width (default) |
| `userSpaceOnUse` | Size in user coordinate system |

**Orientation:**

| Value | Description |
|-------|-------------|
| `auto` | Rotate to match stroke angle |
| `auto-reverse` | Rotate opposite to stroke angle |
| `<angle>` | Fixed rotation angle |

**Marker Types:**

| Attribute | Description |
|-----------|-------------|
| `marker-start` | Marker at start of stroke |
| `marker-mid` | Marker at each vertex |
| `marker-end` | Marker at end of stroke |

**Examples:**

```xml
<defs>
  <!-- Arrowhead marker -->
  <marker id="arrowhead" markerWidth="10" markerHeight="7" 
          refX="0" refY="3.5" orient="auto">
    <polygon points="0 0, 10 3.5, 0 7" fill="black" />
  </marker>
  
  <!-- Circle marker -->
  <marker id="circle" markerWidth="5" markerHeight="5" 
          refX="2.5" refY="2.5" orient="auto">
    <circle cx="2.5" cy="2.5" r="2.5" fill="red" />
  </marker>
</defs>

<!-- Use markers -->
<polyline points="0,0 50,50 100,25"
          fill="none"
          stroke="black"
          stroke-width="2"
          marker-start="url(#circle)"
          marker-mid="url(#circle)"
          marker-end="url(#arrowhead)" />
```

**References:**
- [W3C SVG 2 - Marker Element](https://www.w3.org/TR/SVG2/painting.html#MarkerElement)

---

## Events

SVG supports various events for interactivity.

### Event Categories

| Category | Events |
|----------|--------|
| Mouse | click, mousedown, mouseup, mouseover, mouseout, mousemove |
| Keyboard | keydown, keypress, keyup |
| Focus | focus, blur |
| Document | load, unload, abort, error, resize, scroll |
| Media | begin, end, repeat, inactive, active, idle, suspend, resume |

### Event Syntax

```xml
on<event-name>="<script-code>"
```

### Common Event Handlers

| Event | Handler | Description |
|-------|---------|-------------|
| `onclick` | onclick | Mouse click |
| `ondblclick` | ondblclick | Double click |
| `onmousedown` | onmousedown | Mouse button pressed |
| `onmouseup` | onmouseup | Mouse button released |
| `onmouseover` | onmouseover | Mouse enters element |
| `onmouseout` | onmouseout | Mouse leaves element |
| `onmousemove` | onmousemove | Mouse moves over element |
| `onmouseenter` | onmouseenter | Mouse enters element (no bubbling) |
| `onmouseleave` | onmouseleave | Mouse leaves element (no bubbling) |
| `onkeydown` | onkeydown | Key pressed |
| `onkeyup` | onkeyup | Key released |
| `onkeypress` | onkeypress | Character key pressed |
| `onfocus` | onfocus | Element receives focus |
| `onblur` | onblur | Element loses focus |
| `onload` | onload | Element loaded |
| `onunload` | onunload | Element unloaded |
| `onresize` | onresize | Element resized |
| `onscroll` | onscroll | Element scrolled |
| `onabort` | onabort | Loading aborted |
| `onerror` | onerror | Loading error |

### Event Examples

```xml
<!-- Mouse click handler -->
<circle cx="100" cy="100" r="50"
        onclick="alert('Clicked!')" />

<!-- Mouse over effect -->
<rect x="0" y="0" width="100" height="100"
      onmouseover="this.style.opacity='0.5'"
      onmouseout="this.style.opacity='1'" />

<!-- Key handler -->
<text x="0" y="20"
      onkeydown="handleKey(event)"
      onkeypress="handleKeyPress(event)" />

<!-- Focus handler -->
<circle cx="100" cy="100" r="50"
        onclick="this.setAttribute('fill', 'red')"
        onfocus="this.setAttribute('stroke', 'yellow')" />
```

### Animation Events

| Event | Description |
|-------|-------------|
| `begin` | Animation starts |
| `end` | Animation ends |
| `repeat` | Animation repeats |
| `inactive` | Animation becomes inactive |
| `active` | Animation becomes active |
| `idle` | Animation is idle |

```xml
<animate id="myAnim" attributeName="opacity"
         from="0" to="1" dur="2s"
         onbegin="log('Animation started')"
         onend="log('Animation ended')"
         onrepeat="log('Animation repeated')" />
```

---

## Event Handlers

All SVG elements support event handler attributes.

### Event Handler Syntax

Event handlers are defined as attributes on elements:

```xml
<element on<event>="<script-code>" />
```

### Complete List of Event Handlers

| Event Handler | Event | Description |
|---------------|-------|-------------|
| `onabort` | abort | Loading aborted |
| `onerror` | error | Loading error |
| `onresize` | resize | Element resized |
| `onscroll` | scroll | Element scrolled |
| `onload` | load | Element loaded |
| `onfocus` | focus | Element receives focus |
| `onblur` | blur | Element loses focus |
| `onmousedown` | mousedown | Mouse button pressed |
| `onmouseup` | mouseup | Mouse button released |
| `onmouseover` | mouseover | Mouse enters element |
| `onmousemove` | mousemove | Mouse moves over element |
| `onmouseout` | mouseout | Mouse leaves element |
| `onmouseenter` | mouseenter | Mouse enters (no bubbling) |
| `onmouseleave` | mouseleave | Mouse leaves (no bubbling) |
| `onkeydown` | keydown | Key pressed |
| `onkeyup` | keyup | Key released |
| `onkeypress` | keypress | Character key pressed |

### Event Handler with External Function

```xml
<!-- Define function in script -->
<script type="text/ecmascript">
  function handleClick(event) {
    var element = event.target;
    element.setAttribute('fill', 'blue');
  }
  
  function handleKey(event) {
    if (event.keyCode === 13) {
      // Enter key pressed
      doSomething();
    }
  }
</script>

<!-- Use function in event handler -->
<circle cx="100" cy="100" r="50"
        onclick="handleClick(event)" />

<text x="0" y="20"
      onkeydown="handleKey(event)"
      tabindex="0">Press Enter</text>
```

---

## References

- [W3C SVG 2 - Linking](https://www.w3.org/TR/SVG2/links.html)
- [W3C SVG 2 - Scripting](https://www.w3.org/TR/SVG2/script.html)
- [W3C SVG 2 - Events](https://www.w3.org/TR/SVG2/interact.html#Events)
- [W3C SVG 2 - Interaction](https://www.w3.org/TR/SVG2/interact.html)
- [W3C DOM Events](https://www.w3.org/TR/dom/#events)

---

*Generated from W3C SVG 2 Specification. For the most up-to-date information, always refer to the official specification.*
