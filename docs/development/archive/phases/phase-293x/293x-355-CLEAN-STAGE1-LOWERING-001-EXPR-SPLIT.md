# 293x-355 CLEAN-STAGE1-LOWERING-001 Expression Lowering Split

Status: landed
Date: 2026-05-15

## Decision

Split the large `expression_to_json_v0` dispatcher into case helpers while
keeping Program(JSON v0) output behavior unchanged.

## Scope

- Keep `expression_to_json_v0` as the top-level expression dispatch owner.
- Move function call, dynamic/static call, method call, field access, block
  expression, record literal/update, and match expression lowering into focused
  helpers.
- Do not change AST shapes, Program(JSON v0) schema, Stage-3 token policy,
  Local/Outbox variants, or MIMAP acceptance behavior.

## Acceptance

- Build:
  `cargo build -q --bin hakorune`
- Current pointer:
  `bash tools/checks/current_state_pointer_guard.sh`

Both passed for this slice.

## Follow-up

`CLEAN-TOKEN-STAGE3-001` remains a small ready cleanup. `CLEAN-AST-DECL-001`
remains parked because Local/Outbox unification is a broader AST/API change and
should not be mixed into MIMAP-013.
