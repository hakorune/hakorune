Status: LANDED
Owner: Codex
Phase: 255x

# Phase 255x

## Summary

- pin a dedicated timeout payload for `TaskGroupBox.joinAll(timeout_ms)`
- keep first-failure precedence over timeout
- leave explicit scope-exit timeout surfacing for a later narrow cut

## Landed Contract

- `joinAll(timeout_ms)` now returns:
  - `ResultBox::Ok(void)` on in-time success
  - `ResultBox::Err(first_failure_payload)` when a first failure is latched
  - `ResultBox::Err(TaskJoinTimeout: timed out after Nms)` when the bounded join deadline is hit without a first failure
- explicit `task_scope` exit still only surfaces the latched first failure in this cut

## Proof

- `cargo fmt --check`
- `cargo test -q --lib boxes::task_group_box::tests -- --nocapture`
- `cargo test -q --lib runtime::global_hooks::tests -- --nocapture`
- `cargo check -q --lib`
- `bash tools/checks/dev_gate.sh quick`

## Next

- explicit scope-exit timeout surfacing
- aggregate failure reporting on scope exit only if still needed after that
