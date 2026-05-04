---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381W, return.method arg-presence contract
Related:
  - docs/development/current/main/phases/phase-29cv/P381V-COUNT-PARAM-PUBLIC-WRAPPER-CALL-CONTRACT.md
  - lang/src/mir/builder/func_body/basic_lower_box.hako
---

# P381W: Return Method Arg Presence Contract

## Problem

After P381V, the direct Stage1 env EXE route reaches LLVM mem2reg with:

```text
FuncBodyBasicLowerBox._try_lower_return_method/4
%r1538 = icmp ne i64 %r1514, 0
%r1514 defined with type i1 but expected i64
```

The source owner uses `call_arity` only as a 0/1 flag to reject
`String.length(...)` when arguments are present. That is not an arithmetic
counter and should not be lowered as a numeric carrier.

## Decision

Use the existing `arg_ids` text as the presence contract:

```text
arg_ids == "" -> no method args materialized
arg_ids != "" -> one supported arg materialized
```

This keeps `LowerReturnMethodStringLengthBox` / BasicLower ownership intact and
does not require backend Bool-to-i64 repair.

## Boundary

Allowed:

- remove the local `call_arity` numeric flag from
  `_try_lower_return_method/4`
- use `arg_ids != ""` to guard the String.length zero-arg contract
- keep emitted MIR call JSON unchanged

Not allowed:

- add backend implicit Bool/i64 conversion
- widen method acceptance beyond the existing 0/1-arg shapes
- move dispatch into `MirBuilderMinBox`

## Acceptance

```bash
NYASH_DISABLE_PLUGINS=1 NYASH_LLVM_ROUTE_TRACE=1 \
target/release/hakorune --emit-exe /tmp/p381w_stage1_env.exe \
  lang/src/runner/stage1_cli_env.hako

KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381w_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- `_try_lower_return_method/4` no longer emits an i1 value into an i64
  zero-check for method argument presence
- return.method lowering remains owner-local

## Result

Implemented. `_try_lower_return_method/4` now uses the materialized argument
text as the presence contract instead of a separate numeric arity local.
