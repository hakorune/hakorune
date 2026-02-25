---
Status: Ready
Scope: code + docs
---

# Phase 29bc P1: Composer cleanup (remove dead_code scaffolds)

## Goal

`composer/mod.rs` に残っている “古い足場 API” を削除し、入口を SSOT（single entry）に寄せる。

## Scope

対象ファイル（主）:

- `src/mir/builder/control_flow/plan/composer/mod.rs`

## Steps

1. `composer/mod.rs` から dead_code の足場関数を削除
   - `try_compose_domain_plan_from_canonical_facts`
   - `try_compose_core_plan_via_normalizer`
   - `try_compose_core_plan_direct`
   - `try_compose_core_plan_from_canonical_facts`

2. 上記に紐づく `#[cfg(test)]` の scaffold テストも削除/更新
   - 入口テストは `coreloop_single_entry.rs` / `shadow_adopt.rs` / 個別 composer に寄せる

3. `pub(in crate::mir::builder) use ...` を最小に引き締める
   - 残すのは P0 SSOT に列挙した entrypoints のみ
   - “未使用だが将来使うかも” は入れない（必要になったら docs-first で復活）

4. docs を P1 完了に更新
   - `docs/development/current/main/phases/phase-29bc/README.md`
   - `docs/development/current/main/10-Now.md`（Next を P2 closeout に）

## Acceptance

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A`
- `git commit -m "phase29bc(p1): cleanup composer scaffolds"`

