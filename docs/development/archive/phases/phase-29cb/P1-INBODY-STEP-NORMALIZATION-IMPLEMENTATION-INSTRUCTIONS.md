---
Status: Done
Scope: Phase 29cb P1 implementation
---

# Phase 29cb P1: in-body step normalization (implementation)

## 目的

generic loop v0.2 として、loop body 中の “途中 step” を末尾へ寄せて受理する。
語彙は増やさず（ExitIf/IfEffect の範囲内）、facts/normalizer で正規化する。

## 実装内容

- facts: in-body step の候補を検出し、1回のみ許可
  - continue と併用は禁止
  - step 後に loop_var を使う形は拒否
  - strict/dev では曖昧さを Freeze へ
- normalizer: body 内の step をスキップし、loop increment を末尾へ統一
- verifier: IfEffect の空 then_body を拒否（局所 fail-fast）
- fixture/smoke: strict/dev は FlowBox adopt タグ、release はタグ無しで固定

## 追加物

- Fixture: `apps/tests/phase29cb_generic_loop_in_body_step_min.hako`
- Smokes:
  - `tools/smokes/v2/profiles/integration/joinir/generic_loop_in_body_step_strict_shadow_vm.sh`
  - `tools/smokes/v2/profiles/integration/joinir/generic_loop_in_body_step_release_adopt_vm.sh`

## Gate

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
