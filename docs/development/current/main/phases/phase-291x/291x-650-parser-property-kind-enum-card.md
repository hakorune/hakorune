---
Status: Landed
Date: 2026-04-28
Scope: replace parser-local property kind strings with an enum
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/members/properties.rs
---

# 291x-650: Parser Property Kind Enum

## Goal

Keep parser-side property kind dispatch typed inside `properties.rs`.

This is BoxShape cleanup. It does not change stored/get/once/birth_once syntax
or emitted synthetic method names.

## Evidence

Block-first property parsing carried local string state:

```text
"computed" | "once" | "birth_once"
```

and later matched on that string to choose emission. Header-first
`once`/`birth_once` used another string check. The parser-side kind decision is
local to `properties.rs`, so it should not be stringly inside the function body.

## Decision

Introduce a private `PropertyMemberKind` enum in `properties.rs` and use it for
both header-first `once`/`birth_once` and block-first property emission.

The MIR-side `PropertyKind` remains separate because it owns synthetic getter
method-name classification, not parser syntax.

## Boundaries

- Do not expose the parser enum outside `properties.rs`.
- Do not change parser routes or gate behavior.
- Do not change `property_emit.rs`.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Replaced parser-local property kind strings with `PropertyMemberKind`.
- Shared the enum emission helper between header-first and block-first property
  parsing.
