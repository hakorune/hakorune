---
Status: Active
Decision: accepted
Date: 2026-02-25
Scope: plugin lane の ABI/loader acceptance（PLG-01）を fail-fast 契約で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/design/de-rust-scope-decision-ssot.md
  - docs/reference/plugin-system/bid-ffi-v1-actual-specification.md
  - docs/reference/plugin-system/migration-guide.md
  - docs/reference/plugin-system/plugin_lifecycle.md
  - src/runtime/plugin_loader_v2/enabled/loader/library.rs
  - src/runtime/plugin_loader_unified.rs
  - src/config/nyash_toml_v2.rs
  - tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh
---

# 29cc-96 Plugin ABI/Loader Acceptance Lock SSOT

## 0. Goal

`PLG-01` として、plugin lane の ABI/loader 境界を 1 箇所で固定する。
移植実装前に「何を壊してはいけないか」を fail-fast で明示する。

## 1. Acceptance Contract (PLG-01)

1. plugin ABI entry は `nyash_plugin_invoke` を必須として維持する。
2. plugin metadata は nyash.toml（`libraries` / `boxes` / `path` / Box methods）を唯一の設定入口として扱う。
3. loader policy は strict と best-effort の両契約を維持する。
4. plugin disable は `NYASH_DISABLE_PLUGINS=1` で fail-fast（silent enable しない）。
5. path resolution は OS拡張子（`.so/.dylib/.dll`）+ `NYASH_PLUGIN_PATHS` を維持する。

## 2. Fail-Fast Boundary

1. strict (`HAKO_JOINIR_STRICT=1`) は load error で即失敗する。
2. best-effort (`HAKO_JOINIR_STRICT=0`) は失敗件数をログ化しつつ継続する。
3. config 不在や register失敗は plugin init で明示失敗を返す。
4. workaround fallback を新設して gate を通さない。

## 3. Evidence (2026-02-25)

1. `cargo check --bin hakorune` -> PASS
2. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
3. `bash tools/smokes/v2/profiles/integration/apps/phase134_plugin_best_effort_init.sh` -> PASS

## 4. Decision

Decision: accepted

- `PLG-01`（ABI/loader acceptance lock）は完了。
- 次の active task は `PLG-02`（gate pack lock）。
- `PLG-02` の blocker capture は `29cc-97-plugin-gate-pack-lock-ssot.md` を正本とする。
