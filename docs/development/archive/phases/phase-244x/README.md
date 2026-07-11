# Phase 244x: VM await contract pin

Status: Landed

Purpose
- pin the current VM-side `await` contract without widening async runtime behavior

Scope
- fix the docs around the current `await` path
- pin the subset shape gate for `await`
- pin the VM runtime failure rule for non-`Future` operands
- pin that current VM `await` has no timeout/cancel result shape
- keep detached/root-scope/cancel integration out of scope

Acceptance
- pre-selfhost async SSOT states the exact current `await` failure and blocking rules
- reference concurrency docs no longer imply a current cancel-aware `await`
- focused VM/subset tests lock the current shape/type-error contract

Follow-on
- detached/root-scope policy
- final `await` failure/cancel contract after a real cancel-capable runtime path exists
