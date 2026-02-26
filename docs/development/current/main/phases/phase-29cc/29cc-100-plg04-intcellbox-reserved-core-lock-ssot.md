---
Status: Done
Decision: accepted
Date: 2026-02-25
Scope: plugin lane `PLG-04-min2` として IntegerBox(core) / IntCellBox(plugin) 分離契約を docs-first + fixture/smoke で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-95-plugin-lane-bootstrap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-99-plg04-arraybox-wave1-min1-ssot.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - src/runtime/box_registry.rs
  - plugins/nyash-integer-plugin/nyash_box.toml
  - plugins/nyash-integer-plugin/src/lib.rs
  - nyash.toml
  - hako.toml
---

# 29cc-100 PLG-04 IntCellBox Reserved-Core Lock SSOT

## 0. Goal

`PLG-04-min2` として、`IntegerBox` 名の衝突を構造で解消する。

- core 側: `IntegerBox` は数値箱として固定（reserved）
- plugin 側: mutable integer は `IntCellBox` へ分離
- registry 側: reserved core 名の plugin override は fail-fast 拒否

## 1. Boundary (fixed)

In scope:
1. `plugins/nyash-integer-plugin` の公開 Box 名を `IntCellBox` へ変更
2. `nyash.toml` / `hako.toml` の integer plugin 設定を `IntCellBox` 契約へ更新
3. runtime registry で `IntegerBox` の plugin provider 登録を拒否（reserved-core lock）
4. `PLG-04-min2` の fixture/smoke を追加し、`set/get` 契約を `IntCellBox` で固定

Out of scope:
1. core `IntegerBox` の意味変更（算術/比較/literal ルート）
2. wave-1 他 plugin の同時 rollout
3. plugin ABI v1/v2 の仕様変更

## 2. Contract Lock

1. `IntegerBox` は core 予約名として plugin override しない
2. mutable integer は `IntCellBox` のみで提供する
3. `IntCellBox` の最小契約は `new IntCellBox(v) -> set(n) -> get()==n`

## 3. Acceptance (PLG-04-min2)

1. `phase29cc_plg04_intcellbox_pilot_vm` が PASS
2. `tools/vm_plugin_smoke.sh` が PASS（CounterBox + ArrayBox + IntCellBox）
3. `phase29bq_fast_gate_vm.sh --only bq` が PASS
4. `phase134_plugin_best_effort_init.sh` が PASS
5. `cargo check --bin hakorune` が PASS

## 4. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_plg04_intcellbox_pilot_vm.sh` -> PASS
2. `bash tools/vm_plugin_smoke.sh` -> PASS
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` -> PASS
4. `bash tools/smokes/v2/profiles/integration/apps/archive/phase134_plugin_best_effort_init.sh` -> PASS
5. `cargo check --bin hakorune` -> PASS

## 5. Decision

Decision: accepted

- `PLG-04-min2`（IntCellBox reserved-core lock）は完了。
- active next は `PLG-04-min3`（wave-1 rollout を 1 plugin ずつ継続）。
