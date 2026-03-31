# Self Current Task — Decisions (main)

Status: Public Stub
Private Canonical Path: `docs/private/development/current/main/20-Decisions.md`

## Purpose

- Public repo には最小の方針サマリだけを置く。
- 実運用の詳細 decision log は private canonical で管理する。

## Public Summary

- Selfhost / de-rust mainline priority を維持する。
- `stage0 / stage1 / stage2-mainline / stage2+` は build/distribution vocabulary のまま固定し、replacement progress は `K0 / K1 / K2(core|wide)` の別軸で読む。
- visible execution reading is `Rune lane (parallel)` plus `K0 -> K-migration`; `K1` / `K2` are kept as separate gates.
- `K2` は substrate era として扱い、`K2-core = RawArray first`、`K2-wide = RawMap second + capability widening + metal review` に固定する。
- `K1` の canonical noun は `Semantic Owner Swap` のまま維持し、public reading では `.hako` 側で semantic kernel が完了する gate と読む。
- `zero-rust` は default daily/distribution policy として扱う。ただし bootstrap/recovery/reference/buildability keep と native metal keep は常設 keep にする。
- same-boundary の daily swap code は `.hako kernel module` / `.hako substrate module` と呼び、`plugin` は cold loader lane の語に限定する。
- perf lane は monitor-only（failure-driven restart）を維持する。
- Array/Map perf evidence は appendix/monitor-only で、task order を決める材料にはしない。
- semantic `MapBox` work is already `K1 done-enough`; `RawMap` substrate work is deferred in `K2-wide`.
- portability CI は cost-aware（macOS は常時必須にしない）。
- Rune metadata lane は `@rune` を canonical surface とし、legacy `@hint/@contract/@intrinsic_candidate` は compat alias に格下げした。first backend-active consumer は引き続き narrow で、optimization metadata は parse/noop keep のままに固定する。
- `stage2-mainline` への entry task pack は `docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md` を正本にし、`stage0 keep / stage1 bridge+proof / stage2-mainline daily mainline / stage2+ umbrella` を canonical reading に固定する。first optimization wave は `.hako -> ny-llvmc(boundary) -> C ABI` の route/perf only に固定し、collection/map perf は regression/evidence pack として扱う。Rune optimization metadata は parse/noop keep のままにする。

## Migration Rule

- private 側で decision を更新した場合、public 側には必要最小限の summary のみ反映する。
- machine guard が依存する文書（`CURRENT_TASK.md` など）へは、必要な同期のみ行う。
