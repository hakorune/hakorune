---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: wasm lane `WSM-02d-min4` として WSM-02d 境界パックを milestone gate へ昇格固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-129-wsm02d-min1-boundary-fastfail-tests-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-130-wsm02d-min2-demo-min-fixture-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-131-wsm02d-min3-demo-unsupported-boundary-lock-ssot.md
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_milestone_gate_vm.sh
  - tools/checks/dev_gate.sh
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cc/README.md
---

# 29cc-132 WSM-02d-min4 Milestone Gate Promotion Lock SSOT

## 0. Goal

`WSM-02d` の min4 として、daily の `wasm-boundary-lite` と独立に、節目実行用 milestone gate を固定する。

1. WSM-02d min1/min2/min3 の boundary pack を 1本の milestone smoke に統合
2. `dev_gate milestone-runtime` から実行される契約を固定
3. pointer を同期し、以後の節目検証を deterministic にする

## 1. Boundary (fixed)

In scope:
1. `tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_milestone_gate_vm.sh` を追加
2. `tools/checks/dev_gate.sh` の `milestone-runtime` に wasm gate pack を追加
3. phase pointer 同期

Out of scope:
1. 新しい WASM 命令対応（`Load`/`Store` など）
2. browser実行（G2）
3. runtime parity 追加改善

## 2. Contract Lock

1. milestone gate は `min2`/`min3` smoke を連続実行する
2. min1 の boundary unit（extern/boxcall fail-fast）を同 pack で検証する
3. 失敗時は fail-fast で最初の boundary で止まる

## 3. Evidence (2026-02-26)

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm02d_milestone_gate_vm.sh` -> PASS
2. `tools/checks/dev_gate.sh wasm-boundary-lite` -> PASS
3. `tools/checks/dev_gate.sh --list` で milestone-runtime に wasm gate が表示される -> PASS

## 4. Decision

Decision: accepted

- `WSM-02d-min4` は完了。
- wasm lane active next は `WSM-G2-min1`（browser demo-run minimal plan, docs-first）とする。
