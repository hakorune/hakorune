---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381Q, BoxTypeInspector trace flag text cleanup
Related:
  - docs/development/current/main/phases/phase-29cv/P381P-COUNT-PARAM-FINISH-INLINE-OWNER.md
  - lang/src/shared/common/box_type_inspector_box.hako
---

# P381Q: Box Introspect Trace Flag Text

## Problem

After P381P, the direct Stage1 env EXE route reaches LLVM IR emission and fails
with:

```text
error: '%r258' defined with type 'i1' but expected 'i64'
%r323 = call i64 @"nyash.string.concat_hh"(i64 %r324, i64 %r258)
```

The raw IR locates the bad concat inside `BoxTypeInspectorBox.is_array/1`.
The `HAKO_BOX_INTROSPECT_TRACE` debug print concatenates `is_map` /
`is_array` directly into a string.

That is debug-only owner code. Widening generic string concat to accept Bool
would teach Stage0 another source-helper shape.

## Decision

Keep the trace, but materialize scalar flags as `"0"` / `"1"` owner-local text
variables inside the guarded trace block before string concatenation.

## Boundary

Allowed:

- add trace-local text variables for `is_map` / `is_array`
- use those variables only in BoxTypeInspector trace output

Not allowed:

- widen generic string concat for Bool
- add a backend body shape
- change `is_map` / `is_array` return values
- remove the trace guard

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381q_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381q_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- the direct Stage1 env EXE route no longer fails on Bool-to-string concat
- phase29cg remains green
- `BoxTypeInspectorBox.is_map/is_array` still return scalar flags

## Result

Done:

- routed all BoxTypeInspector trace flag concatenations through trace-local
  text variables
