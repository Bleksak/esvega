---
name: ast-arena-management
description: Builds and manipulates the SVG AST using slotmap arena allocation with stable node identifiers. Use when extending src/parser/ast.rs, adding Node variants, or modifying Element children/attributes management. Creates NodeId keys via new_key_type!, manages SlotMap<NodeId, Node> arenas, and handles parent-child insertion. Do NOT use for lexer/token logic, parser state machines, or external format serialization.
---

# AST Arena Management with Slotmap

## Critical

- Always use `slotmap::new_key_type!()` for `NodeId` — never use `u32` or raw integers as node identifiers
- The arena is a `SlotMap<NodeId, Node>` stored in `ParserState` or `AstBuilder` — never store SlotMaps on individual Nodes
- Every node insertion must return a `NodeId` and that key must be captured for later reference
- Validate arena capacity before bulk operations: `arena.len()` must stay reasonable for large SVGs
- Children are stored as `Vec<NodeId>`, NOT as `Vec<Node>` — this prevents lifetime issues
- Attributes store `Option<NodeId>` for linked elements (e.g., gradients, animations), not inline attribute values

## Instructions

1. **Define the NodeId key type** in `src/parser/ast.rs`:
   ```rust
   slotmap::new_key_type! { pub struct NodeId; }
   ```
   This must appear at the crate level, not inside impl blocks. Verify: `NodeId` implements `Copy + Clone + PartialEq + Eq` (guaranteed by `new_key_type!`).

2. **Declare the arena** in your builder/state struct in `src/parser/ast.rs`:
   ```rust
   pub struct AstBuilder {
       arena: slotmap::SlotMap<NodeId, Node>,
   }
   ```
   Verify: The arena is the ONLY place `Node` data lives. No `Node` values exist outside the arena.

3. **Insert a new Node** into the arena:
   ```rust
   let node_id = builder.arena.insert(Node {
       kind: NodeKind::Element(ElementKind::Svg),
       attributes: vec![],
       children: vec![],
       parent: None,
   });
   ```
   Verify: `node_id` is captured immediately. If insertion fails (arena full), the slotmap `SlotMap` will panic with `SlotMapIndexOutofBounds` — ensure capacity is sufficient for the expected document size.

4. **Add a child node** to an existing parent:
   ```rust
   let child_id = builder.arena.insert(Node { /* ... */ });
   let parent_node = &mut builder.arena[parent_id];
   parent_node.children.push(child_id);
   parent_node.children.sort_by_key(|&id| {
         let child = &builder.arena[id];
         child.attributes.iter().position(|attr| attr.name == "order").unwrap_or(0)
   });
   ```
   Verify: Only push the `NodeId`. Do NOT clone the Node data. The child's `parent` field should be set to `Some(parent_id)`.

5. **Set parent-child relationships** in both directions:
   ```rust
   let parent_node = &mut builder.arena[parent_id];
   parent_node.children.push(child_id);
   let child_node = &mut builder.arena[child_id];
   child_node.parent = Some(parent_id);
   ```
   Verify: Both sides are always updated together. If you skip one, tree traversal will produce infinite loops or miss nodes.

6. **Retrieve a node** from the arena:
   ```rust
   let node_ref = &builder.arena[node_id]; // immutable borrow
   let node_mut = &mut builder.arena[node_id]; // mutable borrow
   ```
   Verify: Borrowing follows standard SlotMap rules — you cannot mutably borrow a node while holding an immutable reference to any other node in the same arena.

7. **Handle linked attributes** (gradients, filters, animations):
   ```rust
   let gradient_node_id = builder.arena.get_node_id_for_id("url(#myGradient)")?;
   let presentation_node = &mut builder.arena[presentation_id];
   presentation_node.attributes.push(Attribute {
       name: "fill".into(),
       value: AttributeValue::Link(gradient_node_id),
   });
   ```
   Verify: The `Link` variant of `AttributeValue` stores `NodeId`, not a string. String URLs are resolved to `NodeId` before storage.

8. **Remove a node** from the arena:
   ```rust
   let removed = builder.arena.remove(node_id);
   let removed_node = removed.expect("Node should exist");
   // Update parent's children vec to remove this node_id
   if let Some(parent_id) = removed_node.parent {
       let parent = &mut builder.arena[parent_id];
       parent.children.retain(|&id| id != node_id);
   }
   ```
   Verify: Always clean up the parent's children list. Otherwise, dangling `NodeId` references will cause `SlotMap` panics on access.

9. **Traverse the tree** from root:
   ```rust
   fn traverse<'a>(arena: &'a SlotMap<NodeId, Node>, root_id: NodeId, visitor: &mut dyn FnMut(&'a Node)) {
       let root = &arena[root_id];
       visitor(root);
       for child_id in &root.children {
           traverse(arena, *child_id, visitor);
       }
   }
   ```
   Verify: Pass `SlotMap` by reference. Never clone the arena. Root is determined by the node whose `parent` is `None`.

10. **Build the final AST** and extract for rendering:
    ```rust
    pub fn build(self) -> ArenaSnapshot {
        ArenaSnapshot {
            arena: self.arena,
            root_id: self.find_root(),
        }
    }
    
    fn find_root(&self) -> NodeId {
        self.arena.iter()
            .find(|(_, node)| node.parent.is_none())
            .expect("SVG AST must have a root node")
            .0
    }
    ```
    Verify: `find_root` must always find exactly one node with `parent: None`. If multiple roots exist, the document is malformed — log a warning.

## Examples

**User says:** "Add support for `<defs>` elements that group gradient and filter definitions"

**Actions taken:**
- Verify `src/parser/ast.rs` has `new_key_type! { pub struct NodeId; }` — if not, add it
- Add `ElementKind::Defs` to the `ElementKind` enum
- In `AstBuilder::insert_child`, handle `Defs` as a valid container by pushing child `NodeId`s to its children vec
- Add `find_gradient_by_id` and `find_filter_by_id` methods that search `Defs` children and return `Option<NodeId>`

**Result:**
```rust
// Added to ElementKind in src/parser/ast.rs
pub enum ElementKind {
    Svg,
    G,
    Rect,
    Defs, // NEW
    LinearGradient,
    Filter,
    // ...
}

// Added to AstBuilder in src/parser/ast.rs
pub fn find_gradient_by_id(&self, id: &str) -> Option<NodeId> {
    let defs_id = self.arena.iter()
        .find(|(_, node)| node.kind == NodeKind::Element(ElementKind::Defs))
        .map(|(id, _)| id)?;
    let defs_node = &self.arena[defs_id];
    for child_id in &defs_node.children {
        let child = &self.arena[*child_id];
        if child.kind == NodeKind::Element(ElementKind::LinearGradient) {
            for attr in &child.attributes {
                if attr.name == "id" && attr.value == AttributeValue::String(id.to_string()) {
                    return Some(*child_id);
                }
            }
        }
    }
    None
}
```

**User says:** "Parse a `<use>` element that references a `<g>` by ID"

**Actions taken:**
- Parse the `href` attribute value to extract the ID string
- Call `find_node_by_id` on the arena to get the target `NodeId`
- Create a new `<use>` node with `ElementKind::Use`
- Store the target `NodeId` in the `<use>` node's attributes as `AttributeValue::Link(target_id)`
- Insert into arena and attach as child to the current parent

**Result:**
```rust
// In the parser, when encountering href="#myGroup"
let target_id = builder.find_node_by_id("myGroup")?;
let use_node_id = builder.arena.insert(Node {
    kind: NodeKind::Element(ElementKind::Use),
    attributes: vec![Attribute {
        name: "href".into(),
        value: AttributeValue::Link(target_id),
    }],
    children: vec![],
    parent: Some(current_parent_id),
});
// Update parent's children list
let parent = &mut builder.arena[current_parent_id];
parent.children.push(use_node_id);
```

## Common Issues

- **`SlotMapIndexOutOfBounds` panic:** You're using a `NodeId` that was already removed from the arena. Fix: After `arena.remove(id)`, set all references to `id` to `None` or update them before removal.

- **Double-free / panic on arena drop:** You stored `Node` data in two places (e.g., both in the arena AND in a separate `Vec<Node>`). Fix: Ensure `Node` data exists ONLY in the arena. Remove all `Vec<Node>` storage.

- **`borrow_mut` conflict:** You tried to mutably borrow two nodes from the same arena simultaneously (e.g., `let a = &mut arena[id1]; let b = &mut arena[id2];`). Fix: Use two-phase borrows or restructure to borrow one, mutate, then borrow the other. Pattern:
  ```rust
  // WRONG:
  let a = &mut arena[id_a];
  let b = &mut arena[id_b];
  
  // RIGHT:
  arena[id_a].children.push(id_b);
  arena[id_b].parent = Some(id_a);
  ```

- **Dangling child references:** A node's `children` vec contains `NodeId`s that no longer exist in the arena. Fix: Clean up parent's children list when removing nodes (see Instruction #9).

- **Missing root node:** `find_root()` returns `None` because every node has `parent: Some(...)`. Fix: Ensure the document's top-level element (e.g., `<svg>`) has `parent: None` during construction.

- **Attribute link resolution returns `None`:** The ID string extracted from `href="#id"` doesn't match any node. Fix: Verify the ID is stripped of the `#` prefix before lookup. Common mistake: storing `#myId` in the arena but searching for `myId` or vice versa.