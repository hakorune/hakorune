---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381AA, JsonNumberCanonical numeric sentinel cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381Z-JSONFRAG-NORMALIZER-COUNTER-PREDICATE-SPLIT.md
  - lang/src/shared/json/utils/json_number_canonical_box.hako
---

# P381AA: Json Number Canonical I64 Sentinels

## Problem

After P381Z, the direct Stage1 env EXE route reaches:

```text
JsonNumberCanonicalBox.canonicalize_f64/1
%r271 = phi i64 [ %r195, ... ], [ %r260, ... ]
%r195 defined with type i1 but expected i64
```

The exponent sign local (`esign`) and small scanner flags are written in 0/1
numeric style, but pure-first can infer the positive sentinel as Bool while
later arithmetic requires i64.

## Decision

Keep number canonicalization source-owned and initialize numeric sentinels /
counters explicitly through i64-producing text conversion.

## Boundary

Allowed:

- make numeric sentinel initialization explicit in `JsonNumberCanonicalBox`
- keep parsing/canonicalization output unchanged

Not allowed:

- add backend Bool/i64 repair
- change JSON number canonicalization policy
- add a route shape or C shim emitter

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381aa_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `JsonNumberCanonicalBox` no longer emits Bool values into i64 numeric
  sentinel PHIs

## Result

Implemented. `JsonNumberCanonicalBox` now uses explicit i64 sentinel values for
numeric state.
