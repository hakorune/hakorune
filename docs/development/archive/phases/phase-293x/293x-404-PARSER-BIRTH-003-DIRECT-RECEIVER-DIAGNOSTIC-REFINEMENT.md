# 293x-404 PARSER-BIRTH-003 Direct Receiver Diagnostic Refinement

Status: landed
Date: 2026-06-07

## Decision

Refine the direct `birth(...)` parser diagnostic so the error names the
receiver-call form explicitly, while keeping the canonical `new Box(...)`
construction hint.

The parser must continue to reject direct receiver calls such as:

```hako
page.birth(PageId(0), Bytes(32), 2, 2)
```

## Scope

- Clarify the user-facing direct receiver `birth(...)` diagnostic wording.
- Keep the canonical construction hint pointing at `new Box(...)`.
- Keep the shared lifecycle helper as the single owner of the direct-call
  diagnostic text.

## Stop Lines

- Do not widen source syntax to accept `obj.birth(...)`.
- Do not change constructor declaration semantics.
- Do not add named constructor arguments or reuse semantics in this row.
- Do not duplicate the diagnostic string across parser call sites.

## Required Evidence

```text
bash tools/checks/k2_wide_parser_birth_direct_call_guard.sh
bash tools/checks/k2_wide_parser_birth_diagnostic_hint_guard.sh
bash tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Implementation

- Update the parser lifecycle direct-birth diagnostic text to mention the
  receiver-call form explicitly.
- Keep both parser paths wired through the shared lifecycle helper.
- Update the focused parser tests and guard scripts to match the refined
  wording.

## Evidence

```text
bash tools/checks/k2_wide_parser_birth_direct_call_guard.sh
bash tools/checks/k2_wide_parser_birth_diagnostic_hint_guard.sh
bash tools/checks/k2_wide_lifecycle_birth_new_only_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
