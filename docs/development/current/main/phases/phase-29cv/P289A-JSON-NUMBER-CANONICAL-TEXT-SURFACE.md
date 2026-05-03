---
Status: Done
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P289a, JsonNumber canonical text-surface method routes
Related:
  - docs/development/current/main/phases/phase-29cv/P288A-JSONFRAG-DECODE-ESCAPES-LENGTH-ROUTE.md
  - lang/src/shared/json/utils/json_number_canonical_box.hako
---

# P289a: JsonNumber Canonical Text-Surface Method Routes

## Problem

After P288a, the source-execution probe advances to:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonNumberCanonicalBox.canonicalize_f64/1
target_shape_blocker_reason=-
```

`JsonNumberCanonicalBox.canonicalize_f64/1` is a large owner-local string
normalizer. Its MIR already has `LoweringPlan` rows for most text methods, but
several implicit text calls remain without plan rows:

```text
indexOf(pattern)
substring(start)
```

This is not a reason to make the C shim rediscover method semantics or to add a
new body shape. The owner source should make the text boundaries explicit so the
existing MIR-owned `StringIndexOf` and `StringSubstring` routes are the only
lowering source.

## Decision

Keep the behavior unchanged while spelling substring calls through explicit
text-surface arguments, and route text search through the existing
`StringHelpers.index_of/3` DirectAbi helper where PHI copies hide the receiver
string class from method-route planning:

```hako
text.substring(start, text.length())
StringHelpers.index_of(text, 0, pattern)
```

This preserves the Stage0 size guard:

- C shim still reads `LoweringPlan` and does not infer missing text methods.
- No new `GlobalCallTargetShape` is added.
- No generic collection or runtime fallback semantics are widened.

## Non-Goals

- no C shim method rediscovery
- no new body-specific emitter
- no generic method acceptance widening
- no number canonicalization semantic change
- no fallback or compat route behavior change

## Acceptance

- `JsonNumberCanonicalBox.canonicalize_f64/1` no longer fails module-generic
  prepass on missing implicit `indexOf(pattern)` or `substring(start)` routes.
- The source-execution probe advances to the next blocker or produces the exe.
- `cargo build -q --release --bin hakorune`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`

## Result

Done. `JsonNumberCanonicalBox.canonicalize_f64/1` now has no unplanned
`mir_call` sites in its generated MIR, and the route trace shows its callsite is
consumed through DirectAbi:

```text
consumer=mir_call_global_generic_string_emit
symbol=JsonNumberCanonicalBox.canonicalize_f64/1
```

The source-execution probe advances to the next blocker:

```text
reason=module_generic_prepass_failed
target_shape_blocker_symbol=JsonFragNormalizerBox._normalize_instructions_array/1
target_shape_blocker_reason=-
```
