# Phase 210x: thin-entry actual consumer switch

- Status: Landed
- Purpose: make thin-entry selection a shared actual-consumer seam in the LLVM/Python lowering path, so the current call-site decisions stop living as scattered metadata lookups.
- Target:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/optimization-layer-roadmap-ssot.md`
  - `src/llvm_py/instructions/mir_call/method_call.py`
  - `src/llvm_py/instructions/field_access.py`
  - `src/llvm_py/instructions/user_box_local.py`

## Scope

- centralize thin-entry lookup / decision helpers for user-box method and field routes
- keep the current thin-entry metadata shape intact
- keep the lowering behavior-preserving while making the consumer seam explicit

## Follow-on

- `generic placement / effect`

## Non-goals

- no new MIR semantics
- no closure-call widening
- no generic placement/effect changes
- no agg_local / sum / DCE widening

## Acceptance

- lowering consumers use one shared thin-entry decision helper instead of duplicated ad hoc lookup code
- the current known-receiver and inline-scalar routes stay green
- `git diff --check`
