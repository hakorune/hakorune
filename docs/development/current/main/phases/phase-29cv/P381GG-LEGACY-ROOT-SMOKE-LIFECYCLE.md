# P381GG Legacy Root Smoke Lifecycle

Date: 2026-05-06
Scope: classify the held legacy `tools/smokes` zero-ref group from P381GE/P381GF.

## Decision

The four held legacy root-smoke scripts are delete candidates:

- `tools/smokes/archive/smoke_async_spawn.sh`
- `tools/smokes/curated_phi_invariants.sh`
- `tools/smokes/parity_quick.sh`
- `tools/smokes/unified_members.sh`

Delete them in the next commit.

## Reason

`docs/development/current/main/design/tool-entrypoint-lifecycle-ssot.md`
requires all of these before deleting a root helper:

- no active refs from current docs, tools, src, lang, Makefile, or root README
- no current PASS gate owns it
- no compat capsule owns it
- no protected category owns it

The four scripts satisfy that rule. The only refs found after P381GF are the
classification/delete-wave history cards P381GE and P381GF. Those are historical
evidence, not active owners.

## Script Reading

| Script | Reading |
| --- | --- |
| `tools/smokes/archive/smoke_async_spawn.sh` | archived async spawn VM/JIT probe; not a current LLVM/backend gate |
| `tools/smokes/curated_phi_invariants.sh` | historical PyVM vs llvmlite PHI parity wrapper; no current gate owner |
| `tools/smokes/parity_quick.sh` | historical PyVM vs llvmlite parity batch; no current gate owner |
| `tools/smokes/unified_members.sh` | old unified-members LLVM smoke with obsolete env flag; not protected in `tools/ROOT_SURFACE.md` |

## Held Out

All other legacy `tools/smokes` scripts stay held because they still have active
or historical owner refs, or they need a separate lifecycle decision:

- `tools/smokes/archive/aot_smoke_cranelift.sh`
- `tools/smokes/archive/jit_smoke.sh`
- `tools/smokes/archive/mir15_smoke.sh`
- `tools/smokes/archive/smoke_phase_10_10.sh`
- `tools/smokes/archive/smoke_vm_jit.sh`
- `tools/smokes/curated_llvm.sh`
- `tools/smokes/curated_llvm_stage3.sh`
- `tools/smokes/fast_local.sh`
- `tools/smokes/phi_trace_local.sh`
- `tools/smokes/selfhost_local.sh`

## Validation

Reference probe:

```bash
rg -nF -- "$path" docs tools src lang Makefile README.md CURRENT_TASK.md
rg -nF -- "$(basename "$path")" docs tools src lang Makefile README.md CURRENT_TASK.md
```

Expected active references:

```text
none outside P381GE/P381GF history cards
```

Acceptance:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
