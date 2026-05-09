---
Status: done
Date: 2026-05-09
Scope: M13 allocator fast-path EXE proof
---

# 293x-065 M13 Allocator Fast-Path EXE Proof

## Decision

`M13 allocator fast-path EXE proof` is live-narrow.

The accepted shape is intentionally scalar-only:

```text
@rune Profile(allocator.fast)
-> InlinePlan request=required + EffectPlan + CapabilityPlan
-> MIR optimizer consumes verified required InlinePlan for a same-module leaf
-> pure-first EXE receives already-expanded scalar MIR
```

The backend and `.inc` do not read profile names, do not inline functions, and
do not infer allocator semantics from symbols.

## Owned

- `apps/allocator-fast-path-exe-proof/` as the M13 scalar EXE fixture.
- `tools/checks/k2_wide_allocator_fast_path_exe_guard.sh` as the acceptance
  guard.
- Required InlinePlan consumption in the MIR optimizer for verified narrow leaf
  functions.
- Post-cleanup rune-plan refresh before the inline pass, because cleanup can
  reduce a required-inline body from over-budget to an accepted leaf.

## Not Owned

- RawBuf/RawArray EXE lowering.
- Native pointer, TLS, atomic, or `hako.mem` backend lowering.
- Runtime allocator ownership changes.
- Backend or `.inc` profile-name dispatch.
- Strong LLVM pointer attrs.

## Acceptance

```bash
bash tools/checks/k2_wide_allocator_fast_path_exe_guard.sh
cargo test -q mir_optimizer_consumes_verified_profile_allocator_fast_required_inline -- --nocapture
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next Reading

Full `apps/mimalloc-raw-page-proof` EXE remains future work. The next split
must choose one concrete blocker, such as RawBuf/RawArray capability-route EXE
lowering or user-box method-body expansion, and must not hide that work behind
the scalar M13 proof.
