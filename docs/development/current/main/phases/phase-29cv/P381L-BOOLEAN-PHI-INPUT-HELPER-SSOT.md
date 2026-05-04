---
Status: Done
Decision: accepted
Date: 2026-05-04
Scope: phase-29cv P381L, compiler-cleanliness inventory after Stage1 short-circuit source restore
Related:
  - docs/development/current/main/phases/phase-29cv/P381J-BOOLEAN-SHORT-CIRCUIT-PHI-LOWERING.md
  - docs/development/current/main/phases/phase-29cv/P381K-STAGE1-SHORT-CIRCUIT-SOURCE-RESTORE.md
  - docs/development/current/main/design/stage0-llvm-line-shape-inventory-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_compiler_state.inc
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  - lang/c-abi/shims/hako_llvmc_ffi_module_generic_string_function_emit.inc
---

# P381L: Boolean PHI Input Helper SSOT

## Inventory

P381J fixed pure-entry boolean PHI lowering. P381K restored Stage1 owner source
and exposed the same boolean short-circuit PHI shape in same-module
`generic_pure_string_body` definitions.

The active cleanup inventory is:

| Item | State | Next |
| --- | --- | --- |
| Stage1 short-circuit source workaround | removed | keep `||` / `&&` in owner source |
| Stage0 body shape growth | guarded | keep `tools/checks/stage0_shape_inventory_guard.sh` green |
| bool-like PHI input rule | duplicated | move to one helper in compiler lowering state |
| temporary source-helper body shapes | still present | retire later via uniform multi-function emitter or source-owner cleanup |
| Program(JSON v0) keeper buckets | still active | replace/archive by keeper owner, not by broad backend widening |

## Decision

Centralize the bool-like PHI incoming rule:

```text
PHI input is bool-like iff:
  value type is i1
  OR value is canonical integer const 0/1
```

Both pure-entry prescan and same-module generic string PHI refinement should use
that helper. This keeps the P381K fix from becoming another local rule fork.

## Boundary

Allowed:

- add one lowering-state helper for bool-like PHI inputs
- replace duplicate local checks with that helper
- keep behavior unchanged

Not allowed:

- add a new `GlobalCallTargetShape`
- add C shim body-specific emitters
- widen generic string/i64 acceptance beyond the existing short-circuit PHI
  contract
- change source language semantics

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/entry/phase29ck_boundary_pure_bool_phi_branch_min.sh

rm -rf /tmp/p381l_phase29cg
KEEP_OUT_DIR=1 OUT_DIR=/tmp/p381l_phase29cg \
STAGE1_BIN=target/selfhost/hakorune.stage1_cli_env_seed \
NYASH_LLVM_SKIP_BUILD=1 \
bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh

tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Expected:

- phase29cg remains green
- bool PHI boundary smoke remains green
- Stage0 shape inventory count remains unchanged

## Result

Done:

- added one lowering-state helper for bool-like PHI inputs
- replaced the duplicated pure-entry and same-module generic string PHI checks
  with that helper
- kept Stage0 shape inventory unchanged
- phase29cg replay remains green with `verify_rc=0`
