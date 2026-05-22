# Record Ergonomics Expansion Post-293x SSOT

Status: Active
Decision: accepted
Updated: 2026-05-22
Owner: post-293x record ergonomics lane

## Purpose

Define the clean post-293x expansion contract for record ergonomics without
mixing it into phase-293x mimalloc closeout rows.

This SSOT keeps the existing surface (`default`, shorthand, `with`) and fixes
resolution, naming, and implementation order for the next lane.

## Language Surface

Allowed expansion surface:

```hako
record UserFields {
    id: i64
    name: StringBox = ""
    active: i64 = 1
}

local fields = UserFields { id, name }
local next = fields with { active: 0, name }
```

## Name Resolution Contract

Type names and value names may be identical.

- Type-position resolution (`record T`, `: T`, `T { ... }`) uses the type
  namespace only.
- Value-position shorthand resolution (`{ name }`) uses value/local scope only.
- If either side is unresolved in its own namespace, fail-fast.

No cross-namespace implicit fallback is allowed.

## `with` Keyword Decision

Canonical update keyword remains `with`.

Alternatives (`update`, `patch`, `copy`) are documented as rejected for now to
preserve compatibility with existing record-update surface and fixtures.

## Stop Lines

- No ordinary-box `with` copy/update surface.
- No spread/wildcard field copy.
- No dynamic field key updates.
- No runtime record object materialization.
- No cross-function record return ABI.
- No backend record-lowering route changes in this row.

## Parser / Lowering Ownership

Implementation order:

1. Rust parser + Stage1 lowering contract (first owner).
2. `.hako` parser parity and Stage1 route parity.
3. shared AST/Program JSON contract guards.

Both parsers must accept/reject the same shapes before row closeout.

## Required Documentation Sync (after implementation)

Implementation rows must update these references in the same closeout commit:

1. `docs/reference/language/EBNF.md`
2. `docs/reference/language/quick-reference.md`
3. `docs/reference/language/stage-profiles.md`
4. `docs/reference/language/types.md` (if type/name resolution examples change)

Do not close the implementation row until these reference docs are updated.
