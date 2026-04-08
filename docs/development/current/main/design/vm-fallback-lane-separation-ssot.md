---
Status: Provisional SSOT
Decision: provisional
Date: 2026-04-09
Scope: optimization の前に `vm fallback` を 3 軸へ分離し、runner compat fallback / kernel Rust fallback / `vm-hako` reference lane の owner split を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/phases/phase-162x/README.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - src/config/env/vm_backend_flags.rs
  - src/runner/route_orchestrator.rs
  - src/runner/keep/vm_fallback.rs
  - crates/nyash_kernel/src/hako_forward_bridge.rs
---

# VM Fallback Lane Separation SSOT

## Goal

- optimization の前に `vm fallback` を 1 つの曖昧な語として扱うのをやめる
- current repo に残る 3 つの keep/reference/fallback surface を別 owner に固定する
- current implementation lane と string guardrail lane の両方が runner / compat / reference lane に引きずられないようにする

## Fixed Split

| Surface | Meaning | Owner files | Current role |
| --- | --- | --- | --- |
| runner compat fallback | explicit `vm-compat-fallback` interpreter lane | [vm_backend_flags.rs](/home/tomoaki/git/hakorune-selfhost/src/config/env/vm_backend_flags.rs), [route_orchestrator.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/route_orchestrator.rs), [vm_fallback.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/keep/vm_fallback.rs) | explicit keep/debug lane only |
| kernel Rust fallback | `.hako` hook miss 時に Rust route を許す policy | [hako_forward_bridge.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/hako_forward_bridge.rs) and hookable kernel callers | runtime-side fallback policy |
| `vm-hako` reference lane | explicit reference/conformance backend | [route_orchestrator.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/route_orchestrator.rs), [artifact-policy-ssot.md](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/artifact-policy-ssot.md) | internal reference lane |

## Reading Lock

- `NYASH_VM_USE_FALLBACK=1` has two readings today:
  1. runner-side compat interpreter selection
  2. kernel-side Rust fallback permission
- this wave does **not** remove that shared env yet
- this wave only fixes owner reading and naming so future removal can proceed without ambiguity

## Stop Lines

- do not remove `vm-compat-fallback` in this wave
- do not retune `substring_hii` or `len_h` in this wave
- do not promote `vm-hako` out of reference/conformance
- do not widen `.hako`, MIR, or `@rune` for fallback mechanics here

## Cleanup Order

1. docs-first: make current pointers say that cleanup is inserted before the next optimization proof
2. naming/comment cleanup: make runner compat fallback vs kernel Rust fallback readable in code
3. no behavior change verification
4. return to `phase-163x` as the current implementation lane while keeping `phase-137x` as sibling string guardrail

## Success Condition

- restart docs no longer blur the 3 surfaces together
- code comments/names no longer suggest that runner compat fallback and kernel Rust fallback are the same seam
- current pointers can resume optimization without dragging `vm` cleanup ambiguity into either lane
