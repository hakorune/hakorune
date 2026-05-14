# 293x-305 ASTCLEAN-012 host provider compare bridge dead_code rationale guard

Status: complete

## Decision

Decision: accepted.

The hako-ll host-provider compare/recipe bridge modules are staged archive-lane surfaces, not first-pass deletion targets. They may keep `#[allow(dead_code)]` only with explicit Phase 291x-126 rationale comments.

## Scope

- Guard `src/host_providers/llvm_codegen.rs` staged module allowances.
- Require rationale comments for hako-ll compare bridge, recipe route, transport IO, and transport path modules.
- Keep host-provider behavior unchanged.

## Non-goals

- No hako-ll bridge retirement.
- No host-provider routing change.
- No LLVM backend behavior change.

## Guard

- `tools/checks/k2_wide_astclean_host_provider_rationale_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_host_provider_rationale_guard.sh` passed locally.
