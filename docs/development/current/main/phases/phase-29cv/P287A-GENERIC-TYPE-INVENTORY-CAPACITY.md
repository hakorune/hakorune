---
Status: Done
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P287a, generic pure type inventory capacity
Related:
  - docs/development/current/main/phases/phase-29cv/P285A-MODULE-GENERIC-STRING-CONST-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P286A-FALLBACK-COUNT-PARAM-TEXT-SENTINEL-CALLSITE.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc
---

# P287a: Generic Type Inventory Capacity

## Problem

After P286a, the source-execution probe advances to:

```text
reason=generic_type_inventory_full
target_shape_blocker_symbol=BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1
target_shape_blocker_reason=generic_type_inventory_full
```

`BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1` is a large
owner-local dispatch helper. The generated MIR has:

```text
blocks=364
instructions=3974
metadata.value_types=4146
phis=2188
```

The C-side generic pure lowering state currently has:

```text
GENERIC_PURE_TYPE_BINDING_CAP = 2048
```

That is smaller than the shape it is now asked to lower. P285a already made
inventory overflow explicit; this card keeps the same capacity/diagnostic lane
for type facts.

## Decision

Increase only the named type-binding capacity:

```text
GENERIC_PURE_TYPE_BINDING_CAP = 8192
```

This keeps the fix in the generic pure lowering inventory SSOT. The change does
not add a body shape and does not widen generic method or collection semantics.

## Non-Goals

- no new `GlobalCallTargetShape`
- no generic MapBox/ArrayBox/string method widening
- no `.hako` source workaround in `BuilderFallbackAuthorityBox`
- no body-specific C emitter
- no fallback or compat route behavior change
- no unrelated inventory capacity changes in this card

## Acceptance

- `BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1` no longer fails
  because the generic type inventory is limited to 2048 entries.
- If another generic pure inventory fills, the probe reports that next explicit
  capacity reason.
- `bash tools/build_hako_llvmc_ffi.sh`
- source-execution probe advances to the next blocker or produces the exe.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done. The source-execution probe advanced past
`BuilderFallbackAuthorityBox._try_inline_jsonfrag_cases/1`; the next blocker is:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragBox._decode_escapes/1
```

This card intentionally leaves the next module-generic prepass blocker for a
separate acceptance card.
