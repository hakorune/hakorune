---
Status: Landed
Date: 2026-04-28
Scope: remove stale observe-only parser member classifier scaffold
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/parser/declarations/box_def/mod.rs
  - src/parser/declarations/box_def/members/mod.rs
---

# 291x-649: Member Classifier Scaffold Prune

## Goal

Remove the stale parser member classifier scaffold before it becomes a second
source of truth for unified members.

This is BoxShape cleanup. It does not change parsing behavior.

## Evidence

`members/common.rs` classified member heads into `Field`, `Method`,
`Constructor`, and property variants, but the only caller discarded the result:

```text
classify_member(...) -> kind -> let _ = kind
```

The classifier had already drifted from the four property shapes:

- block-first properties were always reported as computed;
- `once` / `birth_once` variants were marked future/dead;
- header-first `once` / `birth_once` fell toward method classification.

Keeping a stale observe-only classifier next to the real parser creates a
future partial-truth hazard.

## Decision

Remove the no-op classifier call and delete `members/common.rs`.

The parsing SSOT remains the actual member parse routes:

- `fields.rs` for stored/get header-first forms;
- `properties.rs` for once/birth_once and block-first forms;
- `constructors.rs` / `methods.rs` for callable members.

## Boundaries

- Do not replace the classifier with a new classifier.
- Do not change member parse order.
- Do not change any parser gate behavior.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo test parser_unified_members_property_emit --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Removed the observe-only `classify_member(...)` call.
- Deleted the stale `members/common.rs` scaffold module.
