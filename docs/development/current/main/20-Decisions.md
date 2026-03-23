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
- Rune v0 は provisional docs/task lock のみ先行し、`@rune` grammar activation は Rust parser / `.hako` parser parity 後、first backend consumer は `ny-llvmc` のみに固定する。

## Migration Rule

- private 側で decision を更新した場合、public 側には必要最小限の summary のみ反映する。
- machine guard が依存する文書（`CURRENT_TASK.md` など）へは、必要な同期のみ行う。
