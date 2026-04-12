# Phase 245x: await failure first cut

Status: Landed

Purpose
- pin the first real `await` failure taxonomy after the Phase-0 contract pin

Scope
- fix the docs around `ContractError` / `TaskFailed` / reserved `Cancelled`
- add a failed terminal state to `FutureBox`
- surface failed futures through VM `Await` and `env.future.await`
- keep cancellation / deadline wiring out of scope

Acceptance
- pre-selfhost async SSOT states the first failure taxonomy explicitly
- failed futures can be represented in Rust runtime scaffolding
- VM `Await` surfaces failed futures as `TaskFailed`
- plugin/runtime `env.future.await` surfaces failed futures as `ResultBox::Err`

Follow-on
- detached/root-scope policy
- cancellation/deadline-aware `await`
