---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P239a, LowerLoopSimple direct MIR JSON owner path
Related:
  - docs/development/current/main/phases/phase-29cv/P207A-STAGE0-SIZE-GUARD.md
  - docs/development/current/main/phases/phase-29cv/P238A-LOWER-LOOP-COUNT-PARAM-DIRECT-JSON.md
  - lang/src/mir/builder/internal/lower_loop_simple_box.hako
  - lang/src/mir/builder/internal/loop_opts_adapter_box.hako
---

# P239a: Lower Loop Simple Direct JSON

## Problem

P238a moved the source-exe probe past
`LowerLoopCountParamBox.try_lower/1`. The next propagated blocker still includes:

```text
target_shape_blocker_symbol=LoopOptsBox.new_map/0
target_shape_blocker_reason=generic_string_return_object_abi_not_handle_compatible
backend_reason=missing_multi_function_emitter
```

One remaining owner path is `LowerLoopSimpleBox.try_lower/1`, which recognizes
the stable `loop(i < Int)` count shape and still routes through the
`LoopOptsBox` MapBox adapter.

## Decision

Keep the fix source-owned. `LowerLoopSimpleBox.try_lower/1` owns the complete
simple count-loop facts:

```text
init = 0
limit
step = 1
cmp = Lt
```

Emit the canonical count-loop MIR JSON directly from this owner path instead
of constructing a `LoopOptsBox` option map.

Also prune the stale `HAKO_MIR_BUILDER_LOOP_JSONFRAG` opt-in branch that
remained inside `LowerLoopCountParamBox.try_lower/1` after P238a. That branch
duplicated the new direct owner path and introduced `env.get`/`print` void
surface into the string-or-null classifier.

## Non-Goals

- no `LoopOptsBox` body shape
- no generic MapBox option-builder support
- no new `GlobalCallTargetShape`
- no C body-specific emitter
- no multi-carrier lowering change
- no change to the recognized simple-loop pattern
- no `LowerLoopCountParamBox` recognized-pattern change

## Acceptance

The source-exe probe should no longer report
`LowerLoopSimpleBox.try_lower/1` as a function carrying the
`LoopOptsBox.new_map/0` target blocker. A later `LoopOptsBox.new_map/0` blocker
may remain for `LowerLoopMultiCarrierBox.try_lower/2`.

```bash
cargo build -q --release --bin hakorune
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p239a_lower_loop_simple_direct_json.exe lang/src/runner/stage1_cli_env.hako
jq -r '.functions[] | select((.. | objects | select(.target_shape_blocker_symbol? == "LoopOptsBox.new_map/0"))?) | .name' tmp/nyash_cli_emit.json | sort -u
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Observed probe:

```text
target_shape_blocker_symbol=LowerLoadStoreLocalBox.try_lower/1
target_shape_blocker_reason=generic_string_unsupported_void_sentinel_const
backend_reason=missing_multi_function_emitter
```

`LowerLoopSimpleBox.try_lower/1` no longer carries the `LoopOptsBox.new_map/0`
blocker. The remaining `LoopOptsBox.new_map/0` carrier inventory is narrowed to:

```text
LowerLoopMultiCarrierBox.try_lower/2
```

`LowerLoopCountParamBox.try_lower/1` is now classified as:

```text
target_shape=generic_string_or_void_sentinel_body
tier=DirectAbi
```
