# TODO

## Attribute type precision: R

`R(LengthOrPercentageOrNumber)` is widened like `X`/`Y`/`Width`/`Height`. The precise
types per element are:
- `<circle r>` — `LengthOrPercentage`
- `<radialGradient r>` — `LengthOrPercentage` (defaults to `50%`)

A bare number is accepted by the widened type but is not technically valid per spec for
either element. This would be fully resolved by the per-element attribute structs redesign
(Option B below).

## Per-element attribute structs (Option B)

### What it is

Replace the current flat `Attribute` enum (shared across all 61 element types) with
per-element attribute structs, where each element type has its own strongly-typed
attribute struct with precisely typed fields.

**Current design:**
```rust
pub struct Element {
    pub element_type: ElementType,
    pub attributes: Vec<Attribute>, // flat enum, same for all elements
}
```

**Proposed design:**
```rust
pub struct Element {
    pub element_type: ElementType,
    pub attributes: ElementAttributes, // per-element typed struct
}

pub enum ElementAttributes {
    Svg(SvgAttributes),
    Rect(RectAttributes),
    FePointLight(FePointLightAttributes),
    // ... 61 variants
}

pub struct FePointLightAttributes {
    pub global: GlobalAttributes, // id, class, style, lang, tabindex, ...
    pub x: Option<f64>,           // bare number, not LengthOrPercentage
    pub y: Option<f64>,
    pub z: Option<f64>,
}

pub struct RectAttributes {
    pub global: GlobalAttributes,
    pub x: Option<LengthOrPercentage>,
    pub y: Option<LengthOrPercentage>,
    pub width: Option<LengthOrPercentage>,
    pub height: Option<LengthOrPercentage>,
    pub rx: Option<LengthOrPercentage>,
    pub ry: Option<LengthOrPercentage>,
}
```

### Why

The current design uses `LengthOrPercentageOrNumber` as a "widened" type for `x`, `y`,
`width`, `height` because different elements expect different types for the same attribute
name. For example:
- `<rect x="10">` — x is a `LengthOrPercentage`
- `<fePointLight x="10">` — x is a bare `f64`

With per-element structs, each field has exactly the right type with no widening needed,
and invalid attribute combinations become compile-time errors rather than runtime
validation via `is_valid_for()`.

### What needs to be done

1. **Define `GlobalAttributes` struct** — contains all attributes valid on every element
   (`id`, `class`, `style`, `lang`, `tabindex`, presentation attributes, event handlers).

2. **Write 61 per-element attribute structs** — one for each `ElementType` variant, each
   embedding `GlobalAttributes` and adding element-specific fields with precise types.

3. **Replace `Vec<Attribute>` in `Element`** with `ElementAttributes` enum.

4. **Rewrite the parser** — instead of `TryFrom<(&String, &String)>` into a flat enum,
   dispatch to the correct struct's field setter based on element type.

5. **Rewrite serialization** — implement `write_svg` on each per-element struct instead
   of the current match-based `write_value` on the flat enum.

6. **Delete `is_valid_for()`** — entirely replaced by the type system.

7. **Update `is_animation_value()`, `is_global()`, `is_filter_primitive()`** and other
   attribute category helpers — no longer needed in their current form.

### Pros

- **100% type safety** — impossible to attach an invalid attribute to an element
- **Precise types** — `fePointLight.x` is `f64`, `rect.x` is `LengthOrPercentage`
- **Eliminates `is_valid_for()`** — validation is structural, not runtime
- **Better ergonomics** — access attributes as fields instead of searching a `Vec`
- **No duplicate attributes** — struct fields are inherently unique

### Cons

- **Massive effort** — 61 structs, rewritten parser, rewritten serializer (~weeks of work)
- **Attribute ordering lost** — struct fields have a fixed order; current design preserves
  the original order from the SVG source
- **Less flexible** — custom/unknown attributes become harder to represent
- **More boilerplate** — global attributes must be duplicated (or nested) in every struct
