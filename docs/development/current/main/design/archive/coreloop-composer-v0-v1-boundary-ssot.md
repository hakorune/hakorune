# CoreLoopComposer v0/v1 boundary SSOT

> **Historical(2026-11-17、メンテナ決定):** この文書は、古い設計か作業の記録である。**新しい作業を選ぶ根拠にしない。** MirBuilder(source → MIR)の設計は [MirBuilder の設計図](../mirbuilder-blueprint.md) を読む。食い違う時は、設計図が勝つ。

This document fixes the responsibility boundary between CoreLoopComposer v0 and v1.

## Scope

- v0 handles loops where `value_join_needed == false`.
- v1 handles loops where `value_join_needed == true`.
- The selection is made at the composer entrypoints (not in the router).

## Gates (current minimal contract)

- v0: `value_join_needed == false`, `skeleton_kind == Loop`, return-only exits, no cleanup.
- v1: `value_join_needed == true`, `skeleton_kind == Loop`, return-only exits, no cleanup.

## Behavior

- If the gate fails, v0/v1 returns `Ok(None)` and the caller falls back to legacy paths.
- In strict/dev shadow adopt, a `None` result is treated as fail-fast with a clear error.

## Notes

- ScanWithInit route currently has no v1 composition yet; v1 must return `None`.
- Value-join routes are intended to live in v1 (`loop_break` / `if_phi_join` / `loop_true_early_exit` / `split_scan`; old numbered labels are traceability-only).
