# 293x-1003 RUNE-INLINE-001 Inline Rune Canonical Surface

Status: landed
Date: 2026-05-21

## Purpose

Make inline requests readable at the source surface before more allocator
fast-path work depends on rune names.

## Decision

`@rune Inline(prefer|avoid|required)` is the canonical inline request family.

Compatibility spellings remain accepted:

```text
Hint(inline)              -> Inline(prefer)
Hint(noinline)            -> Inline(avoid)
Lowering(inline_required) -> Inline(required)
```

`Hint(hot|cold)` remains the canonical advisory profile/tuning spelling.

## Scope

- Accept `Inline(prefer|avoid|required)` in parser rune validation.
- Preserve `Inline(...)` into MIR-owned `inline_plans`.
- Keep `Lowering(inline_required)` as a compat spelling.
- Update MIR/reference docs to describe the canonical surface.

## Stop Lines

- No backend-active inline policy.
- No source-level `always_inline` spelling.
- No wildcard profile expansion changes.
- No removal of compat `Hint(inline/noinline)` or `Lowering(inline_required)`.

## Evidence

```bash
cargo test parser_opt_annotations --lib
cargo test mir_inline_plan --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed as a sidecar cleanup. MIMAP-381A remains the current allocator blocker.
