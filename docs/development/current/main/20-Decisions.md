# Self Current Task — Decisions (main)

Status: Public Stub
Private Canonical Path: `docs/private/development/current/main/20-Decisions.md`

## Purpose

- Public repo には最小の方針サマリだけを置く。
- 実運用の詳細 decision log は private canonical で管理する。

## Public Summary

- Selfhost / de-rust mainline priority を維持する。
- perf lane は monitor-only（failure-driven restart）を維持する。
- portability CI は cost-aware（macOS は常時必須にしない）。
- Rune metadata lane は `@rune` を canonical surface とし、legacy `@hint/@contract/@intrinsic_candidate` は compat alias に格下げした。first backend-active consumer は引き続き narrow で、optimization metadata は parse/noop keep のままに固定する。
- `stage2+` への entry task pack は `docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md` を正本にし、`stage0 keep / stage1 bridge+proof / stage2+ final mainline` を canonical reading に固定する。first optimization wave は `.hako -> ny-llvmc(boundary) -> C ABI` の route/perf only に固定し、Rune optimization metadata は parse/noop keep のままにする。

## Migration Rule

- private 側で decision を更新した場合、public 側には必要最小限の summary のみ反映する。
- machine guard が依存する文書（`CURRENT_TASK.md` など）へは、必要な同期のみ行う。
