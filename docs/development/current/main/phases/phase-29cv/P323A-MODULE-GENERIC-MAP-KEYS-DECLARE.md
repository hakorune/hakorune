---
Status: Active
Decision: accepted
Date: 2026-05-03
Scope: phase-29cv P323a, module generic map keys declaration
Related:
  - docs/development/current/main/phases/phase-29cv/P322A-NUMERIC-LEAF-GENERIC-EMIT-DEDUP.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P323a: Module Generic Map Keys Declaration

## Problem

P322a removes duplicate numeric leaf definitions and advances `opt` to a
declaration blocker:

```text
opt: /tmp/hako_p322_probe.ll:38630:19: error: use of undefined value '@nyash.map.keys_h'
```

The emitted body is `MirJsonEmitBox._emit_flags/1`, whose `flags.keys()` call is
already authorized through the exact LoweringPlan route:

```text
generic_method.keys / MirJsonFlagsKeys -> nyash.map.keys_h
```

The module generic prelude declares map birth/get/set when planned module
generic definitions exist, but it only declares `nyash.map.keys_h` from the
entry-function need scan.  Planned same-module definitions can use `keys()` even
when the entry body itself does not.

## Decision

Declare `nyash.map.keys_h` whenever module generic definitions are planned,
matching the existing broad declaration policy for map birth/get/set and array
helpers.

This is a declaration ownership fix only.  The exact `keys()` acceptance remains
owned by LoweringPlan / `generic_method.keys`.

## Non-Goals

- no generic MapBox semantics widening
- no new body shape
- no source rewrite in `MirJsonEmitBox`
- no unconditional runtime fallback

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p323.exe lang/src/runner/stage1_cli_env.hako
opt -S -passes=mem2reg /tmp/hako_p323_probe.ll -o /tmp/hako_p323_mem2reg.ll
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

```text
The `nyash.map.keys_h` undefined declaration error is gone.
```

## Result

Accepted.  The module generic declaration prelude now declares
`nyash.map.keys_h` whenever same-module generic definitions are planned.

Validation:

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/hakorune --emit-exe /tmp/hakorune_p323.exe lang/src/runner/stage1_cli_env.hako
opt -S -passes=mem2reg /tmp/hako_p323_probe.ll -o /tmp/hako_p323_mem2reg.ll
```

The IR now includes the declaration before `MirJsonEmitBox._emit_flags/1` uses
it:

```text
declare i64 @"nyash.map.keys_h"(i64)
%r28 = call i64 @"nyash.map.keys_h"(i64 %r24)
```

`opt -passes=mem2reg` succeeds.  The probe advances beyond IR verification to
object link:

```text
Error: /usr/bin/ld: /tmp/hakorune_p323.o: in function `BuilderProgramJsonInputContractBox._program_json_header_present/1':
```
