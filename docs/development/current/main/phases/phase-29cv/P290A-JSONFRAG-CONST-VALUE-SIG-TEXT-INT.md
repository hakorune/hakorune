---
Status: Done
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P290a, JsonFrag const value signature text-int route
Related:
  - docs/development/current/main/phases/phase-29cv/P289A-JSON-NUMBER-CANONICAL-TEXT-SURFACE.md
  - lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
---

# P290a: JsonFrag Const Value Signature Text-Int Route

## Problem

After P289a, `JsonNumberCanonicalBox.canonicalize_f64/1` is DirectAbi-ready and
the source-execution probe advances back to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```

The normalizer body itself has planned calls, but its const signature helper is
still unsupported:

```text
target_symbol=JsonFragNormalizerBox._const_value_sig/1
tier=Unsupported
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

The problematic owner-local seam is the i64 signature path. It calls
`JsonFragBox.get_int/2`, receives a scalar-or-null logical result, then checks
that scalar value against `null` before converting it back to text for the
signature:

```hako
local vi = JsonFragBox.get_int(inner, "value")
if vi != null { return "i64:" + StringHelpers.int_to_str(vi) }
```

That pulls scalar/null comparison into a string-return helper and blocks the
existing generic string route.

## Decision

For a signature string, read numeric tokens as text directly and convert
string-or-null helper results at the owner boundary:

```hako
local vi = JsonFragBox.read_int_after(inner, value_pos)
local vi_text = me._text_or_empty(vi)
if vi_text != "" { return "i64:" + vi_text }
```

This keeps the semantic distinction between a missing numeric token and an
actual `0`, while avoiding scalar/null comparison in `_const_value_sig/1`. The
only null-to-empty conversion lives in a small local text helper that is already
compatible with the existing generic string route shape.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C shim body-specific fallback
- no generic scalar/null comparison widening
- no const canonicalization policy change
- no collection method acceptance widening

## Acceptance

- `_const_value_sig/1` becomes a DirectAbi-compatible string/void helper.
- `_normalize_instructions_array/1` no longer contains an Unsupported route for
  `_const_value_sig/1`.
- The source-execution probe advances to the next blocker or produces the exe.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done. `_const_value_sig/1` is now classified as a DirectAbi generic string
helper from `_normalize_instructions_array/1`:

```text
tier=DirectAbi
target_shape=generic_pure_string_body
return_shape=string_handle
```

The source-execution probe advances to the next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=LowerTypeOpCastBox._accept_shape/1
target_shape_blocker_reason=-
```
