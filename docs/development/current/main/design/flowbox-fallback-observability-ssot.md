---
Status: SSOT
Scope: FlowBox fallback observability (strict/dev only)
Related:
- docs/development/current/main/design/flowbox-observability-tags-ssot.md
- docs/development/current/main/design/flowbox-adopt-tag-migration-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# FlowBox fallback observability (SSOT)

目的: strict/dev の fallback を `flowbox/freeze` の code 語彙で可視化し、pattern 名や補助タグに依存しない SSOT を保つ。

## Invariants (SSOT)

- FlowBox タグは strict/dev only（release の恒常ログは不変）
- fallback は “silent” にしない:
  - strict/dev では `flowbox/freeze` で理由が観測できる
- emit/merge は診断のために CFG/Facts を再解析しない（手元の Facts/CorePlan のみ）

## Freeze codes (SSOT)

`[flowbox/freeze code=<code> ...]` の `<code>` は以下に固定する（拡張は docs-first で追記）。

- `planner_none`: planner が Ok(None) を返したが、候補っぽい形（gate-target）だった
- `composer_reject`: Facts/Plan は揃ったが composer が拒否した（subset gate 逸脱など）
- `unstructured`: reducible でない／multi-entry など “Skeleton 外” の形（strict/dev のみ）
- `unwind`: `ExitKind::Unwind` の未実装 wiring を strict/dev で早期検知した

## Gate (SSOT)

- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
