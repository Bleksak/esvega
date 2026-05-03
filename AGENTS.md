# esvega - OpenCode Configuration

## Overview
`esvega` is a zero-dependency Rust SVG parser and renderer. It tokenizes SVG bytes using `memchr`, builds an arena-allocated AST via `slotmap`, and serializes it back to valid SVG.

## Build & Run
```bash
cargo build
cargo run
cargo test -- --nocapture
```

## Architecture
- **Entry**: `src/main.rs` → reads `410_2.svg` → `lexer::Lexer` → `parser::Parser` → `AST` → `to_svg()`
- **Lexer** (`src/lexer/`): Byte-level scanner using `memchr::memmem::find`. Tracks mode in `src/lexer/mode.rs` (`Text`, `Markup`, `Quote`). Outputs `Token { kind, value, span }` from `src/token.rs`.
- **Parser** (`src/parser/`): `StateMachine` consumes tokens via `consume()`. Maintains `element_stack: Vec<NodeId>`. Handles tag open/close, attributes, and text nodes.
- **AST** (`src/parser/ast.rs`): Arena-backed using `slotmap`. `Node` enum: `Text(TextNode)`, `Element(Element)`, `Comment(String)`, `CData(String)`. `TextNode` carries `parent: Option<NodeId>`. `Element` carries `parent: Option<NodeId>` for tree navigation. `NodeId` defined via `new_key_type!`.
- **Elements** (`src/element/`): `Element` struct holds `element_type`, `attributes: Vec<Attribute>`, `children: Vec<NodeId>`. Validation via `is_allowed_as_child()`.
- **Attributes** (`src/element/attributes/`): Enums like `GradientUnits`, `ReferrerPolicy`, `ClipPathUnits`. Parsed via `TryFrom<&str>` / `FromStr`.
- **SVG Types** (`src/svg/types/`): `Angle`, `Length`, `Number`, `Boolean`, `Url`.

## Conventions
- **Parsing**: Use `memchr`/`memmem::find` for fast byte scanning in `src/lexer/`. Never use `std::str` methods on raw bytes.
- **AST Nodes**: Always allocate via `self.ast.nodes.insert(Node::Element(ElementNode { element, parent }))`. Never use raw indices or `Vec<usize>`. Parent pointers should be set when attaching nodes as children.
- **Attributes**: Parse strings into typed enums using `str::parse::<T>()` or `TryFrom::try_from()`. Implement `Display`, `Default`, and `FromStr`/`TryFrom`.
- **Rendering**: Use helper formatters from `src/element/attributes/shared.rs`: `write_semicolon_separated`, `write_space_separated`, `write_comma_separated`.
- **Error Handling**: Parser panics on invalid input (`panic!("Parse error...")`). Do not use `Result` for parser state transitions.
- **Formatting**: Run `cargo fmt` after modifying `src/lexer/`, `src/parser/`, or `src/element/`.

## File Mapping
- `src/token.rs` → `TokenKind` enum
- `src/lexer/mod.rs` → `Lexer`, `Input`
- `src/parser/mod.rs` → `Parser`, `StateMachine`
- `src/parser/ast.rs` → `AST`, `Node`, `NodeId`
- `src/element/mod.rs` → `Element`
- `src/element/attributes/mod.rs` → Attribute re-exports
- `svg/*.md` → Reference specs for SVG element/attribute definitions

<!-- caliber:managed:pre-commit -->
## Before Committing

**IMPORTANT:** Before every git commit, you MUST ensure Caliber syncs agent configs with the latest code changes.

First, check if the pre-commit hook is already installed:
```bash
grep -q "caliber" .git/hooks/pre-commit 2>/dev/null && echo "hook-active" || echo "no-hook"
```

- If **hook-active**: the hook handles sync automatically — just commit normally. Tell the user: "Caliber will sync your agent configs automatically via the pre-commit hook."
- If **no-hook**: run Caliber manually before committing:
  1. Tell the user: "Caliber: Syncing agent configs with your latest changes..."
  2. Run: `caliber refresh && git add CLAUDE.md .claude/ .cursor/ .cursorrules .github/copilot-instructions.md .github/instructions/ AGENTS.md CALIBER_LEARNINGS.md .agents/ .opencode/ 2>/dev/null`
  3. After it completes, briefly tell the user what Caliber updated. Then proceed with the commit.

**Valid `caliber refresh` options:** `--quiet` (suppress output) and `--dry-run` (preview without writing). Do not pass any other flags — options like `--auto-approve`, `--debug`, or `--force` do not exist and will cause errors.

**`caliber config`** takes no flags — it runs an interactive provider setup. Do not pass `--provider`, `--api-key`, or `--endpoint`.

If `caliber` is not found, read `.agents/skills/setup-caliber/SKILL.md` and follow its instructions to install Caliber.
<!-- /caliber:managed:pre-commit -->

<!-- caliber:managed:learnings -->
## Session Learnings

Read `CALIBER_LEARNINGS.md` for patterns and anti-patterns learned from previous sessions.
These are auto-extracted from real tool usage — treat them as project-specific rules.
<!-- /caliber:managed:learnings -->
