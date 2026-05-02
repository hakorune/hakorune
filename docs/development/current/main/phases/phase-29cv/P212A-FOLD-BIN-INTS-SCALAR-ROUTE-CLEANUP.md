---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P212a, LowerIfCompareFoldBinInts scalar route cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P204-LOWER-IF-COMPARE-FOLD-VARINT-EXPLICIT-I64-COERCE.md
  - lang/src/mir/builder/internal/lower_if_compare_fold_binints_box.hako
---

# P212a: Fold Bin Ints Scalar Route Cleanup

## Problem

P211a advances the source-exe probe to:

```text
target_shape_blocker_symbol=LowerIfCompareFoldBinIntsBox._fold_bin_ints/2
target_shape_blocker_reason=generic_string_unsupported_instruction
backend_reason=missing_multi_function_emitter
```

`_fold_bin_ints/2` computes a folded integer result from two integer JSON
fragments, then returns the folded value as text for the caller's existing
string-or-null path. The helper currently uses mutable scalar temporaries:

```hako
local li = 0; local ri = 0
li = me._coerce_i64_compat(l); ri = me._coerce_i64_compat(r)
local res = 0
if op == "+" { res = li + ri }
...
return me._coerce_text_compat(res)
```

That shape creates a scalar result PHI and lifecycle instructions around
temporaries that the generic string body does not need to understand. Adding
support for these generated instructions to `generic_string_body.rs` would
grow Stage0 for a source-owner local cleanup.

## Decision

Do not add another generic string instruction acceptance rule.

Keep the computation source-owned and expose it as simple scalar control flow:

```text
read int fragments -> explicit i64 parse -> direct branch return text
```

Also mirror the P204 explicit coercion route and parse through
`JsonFragBox._str_to_int/1` instead of the implicit `0 + text` compatibility
idiom.

## Non-Goals

- no `generic_string_body.rs` instruction expansion
- no `generic_i64_body.rs` expansion
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no change to `try_lower` or `PatternUtilBox` semantics

## Acceptance

Probe result should move past `_fold_bin_ints/2`; a later blocker may remain:

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p212a_fold_bin_ints.exe lang/src/runner/stage1_cli_env.hako
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
