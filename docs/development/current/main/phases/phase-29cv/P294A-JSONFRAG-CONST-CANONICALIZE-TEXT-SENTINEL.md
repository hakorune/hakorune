---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P294a, JsonFrag const canonicalize text sentinel
Related:
  - docs/development/current/main/phases/phase-29cv/P293A-BUILDER-REGISTRY-DISPATCH-PLAN-CONSUME.md
  - docs/development/current/main/phases/phase-29cv/P290A-JSONFRAG-CONST-VALUE-SIG-TEXT-INT.md
  - lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
---

# P294a: JsonFrag Const Canonicalize Text Sentinel

## Problem

After P293a, the source-execution probe advances back to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```

The normalizer body has complete method/global call plan rows, but its first
unsupported child helper is still:

```text
target_symbol=JsonFragNormalizerBox._const_canonicalize/1
tier=Unsupported
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

`_const_canonicalize/1` is a text helper used only to normalize const-object
text. Its current null input branch returns `null`, and its numeric-token guard
compares string-or-null helper results with `null`, pulling a void sentinel into
a string-return helper.

## Decision

Normalize the helper boundary to the existing owner-local text sentinel:

```hako
local s = me._text_or_empty(obj_text)
if s == "" { return "" }
```

Numeric token reads also cross the owner boundary through `_text_or_empty`.
This keeps malformed/null input explicit as empty text while avoiding null
return/compare flow in the const canonicalization helper.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C shim acceptance widening
- no collection method acceptance widening
- no const canonicalization policy change
- no fallback or externalization

## Acceptance

- `_const_canonicalize/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- `_normalize_instructions_array/1` advances to the next blocker or emits
  successfully.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

Targeted source-execution metadata after the change:

```text
JsonFragNormalizerBox._normalize_instructions_array/1
  JsonFragNormalizerBox._const_canonicalize/1 DirectAbi typed_global_call_generic_pure_string
  JsonFragNormalizerBox._const_value_sig/1     DirectAbi typed_global_call_generic_pure_string
```

The full source-execution probe still stops before exe emission, but no longer
on the JsonFrag const canonicalization helper. The next exposed issue is a
separate C shim contract-reader gap for the already-planned
`typed_global_call_builder_registry_dispatch` route; keep that as P295a rather
than mixing it into this source cleanup.
