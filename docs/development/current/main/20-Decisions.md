# Self Current Task — Decisions (main)

Status: Public Stub
Private Canonical Path: `docs/private/development/current/main/20-Decisions.md`

## Purpose

- Public repo には最小の方針サマリだけを置く。
- 実運用の詳細 decision log は private canonical で管理する。

## Public Summary

- Selfhost / de-rust mainline priority を維持する。
- `stage0 / stage1 / stage2-mainline / stage2+` は execution-lanes-and-axis-separation-ssot.md の build/distribution vocabulary として読む。
- `K0 / K1 / K2` は kernel-replacement-axis-ssot.md の replacement reading として読む。
- except for OS/kernel/substrate boundaries and explicit compat/bootstrap keeps, implementation should move to `.hako` rather than grow new Rust meaning owners.
- backend lane vocabulary (`llvmlite`, `ny-llvmc`, `native`) は stage2-aot-fast-lane-crossing-inventory.md と llvm-harness.md を正本にする。
- current active order と current slice は CURRENT_TASK.md と 15-Workstream-Map.md を正本にする。
- `stage2-mainline` への entry task pack は `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md` を正本にする。

## Migration Rule

- private 側で decision を更新した場合、public 側には必要最小限の summary のみ反映する。
- machine guard が依存する文書（`CURRENT_TASK.md` など）へは、必要な同期のみ行う。
