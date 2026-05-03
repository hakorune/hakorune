---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P297a, JsonFrag const value signature text sentinel
Related:
  - docs/development/current/main/phases/phase-29cv/P296A-VOID-LOGGING-GLOBAL-CALL-NO-DST.md
  - docs/development/current/main/phases/phase-29cv/P294A-JSONFRAG-CONST-CANONICALIZE-TEXT-SENTINEL.md
  - lang/src/mir/builder/internal/jsonfrag_normalizer_box.hako
---

# P297a: JsonFrag Const Value Sig Text Sentinel

## Problem

After P296a, the probe returns to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```

The normalizer body is blocked by its child helper:

```text
callee=JsonFragNormalizerBox._const_value_sig/1
proof=typed_global_call_contract_missing
target_shape_reason=generic_string_unsupported_void_sentinel_const
```

`_const_value_sig/1` returns textual signatures, but still compares
`JsonFragBox.get_str(...)` string-or-null results directly against string
literals.

## Decision

Normalize `JsonFragBox.get_str(...)` results at the owner-local boundary with
`_text_or_empty`.

This keeps the helper on the existing text sentinel contract and avoids adding
new C shim collection/string acceptance.

## Non-Goals

- no new `GlobalCallTargetShape`
- no C shim acceptance widening
- no collection method acceptance change
- no const signature policy change
- no fallback or externalization

## Acceptance

- `_const_value_sig/1` no longer reports
  `generic_string_unsupported_void_sentinel_const`.
- `_normalize_instructions_array/1` advances to the next blocker or emits
  successfully.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Accepted.

`_const_value_sig/1` no longer reports
`generic_string_unsupported_void_sentinel_const`; its `JsonFragBox.get_str`
reads now cross the owner-local `_text_or_empty` boundary.

The source-execution probe advances past the JsonFrag normalizer blocker and
stops at the next module generic prepass blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=MirJsonEmitBox._emit_block/1
target_shape_blocker_reason=-
```
