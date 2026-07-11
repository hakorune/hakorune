---
Status: Landed
Date: 2026-04-24
Scope: Clarify String semantic ownership without changing String behavior or widening import policy.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/reference/language/using.md
  - src/boxes/basic/string_surface_catalog.rs
---

# String Semantic Owner Cleanup Card

## Decision

Keep one String semantic owner split and remove the dead scaffold:

```text
Rust semantic owner
  -> src/boxes/basic/string_surface_catalog.rs

Public std sugar
  -> apps/std/string.hako

Internal selfhost helper
  -> apps/lib/boxes/string_std.hako

Dead scaffold
  -> apps/std/string_std.hako (delete)
```

This card is ownership cleanup only. It does not change String runtime behavior,
catalog rows, or the current import-policy story.

The public sugar smoke is pinned through the exact manifest alias
`apps.std.string` in `hako.toml`; this does not claim a broader `std.string`
packaging decision.

## Preconditions

- `StringBox` stable surface rows are already cataloged and pinned.
- `apps/std/string.hako` is documented as sugar, not semantic owner.
- `apps/lib/boxes/string_std.hako` is still used by `apps/selfhost-runtime/ops_calls.hako`
  for the selfhost-runtime `pref == "ny"` route.
- `apps/std/string_std.hako` has no live import route and is a stale placeholder.

## Implementation Slice

- delete dead `apps/std/string_std.hako`
- update `apps/std/string.hako` header comments to describe it as public sugar
  and replace the old `include` guidance with `using`
- mark `apps/lib/boxes/string_std.hako` as an internal selfhost helper, not a
  public std owner
- sync phase docs / current pointers so the owner split is one-screen obvious

## Non-Goals

- do not change `StringBox` behavior or slot/catalog rows
- do not rewrite `apps/std/string.hako` to match `StringStd`
- do not introduce `std.string` module-root packaging in this card
- do not fold `apps/lib/boxes/string_std.hako` into `apps/std/string.hako`
- do not touch `toUpper` / `toLower` route ownership yet

## Acceptance

```bash
rg -n 'apps/std/string_std\.hako|static box StdString\b' apps src tools hako.toml
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
./target/release/hakorune --backend vm apps/smokes/std/string_smoke.hako
bash tools/checks/current_state_pointer_guard.sh
```

## Exit Condition

The repo has one clear String semantic-owner split:
Rust catalog owner, public std sugar, internal selfhost helper, and no dead
public String scaffold left behind.
