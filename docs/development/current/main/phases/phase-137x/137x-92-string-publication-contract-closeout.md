---
Status: Closed
Date: 2026-04-19
Scope: string-only `publish.text` lane の contract closeout を phase-137x の明示 subphase として切り出し、optimization return より前に終えるべき gate を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-137x/137x-91-task-board.md
  - docs/development/current/main/phases/phase-289x/README.md
---

# 137x-92 String Publication Contract Closeout

## Goal

最適化へ戻る前に、string-only lane の publication contract を docs / task / verifier vocabulary の 3 面で閉じる。

この subphase は **perf widening phase ではない**。
keeper 前の「契約を言語化して stop-line を固定する」ための closeout phase だよ。

## Phase Cut

phase-137x は今、この 3 段に分けて読む。

1. **137x-A: string publication contract closeout**（closed）
   - `repr` request / downgrade contract
   - `StableView` legality contract
   - provenance / borrow-scope / `freeze.str -> publish.text` verifier vocabulary
   - publish idempotence / stable cache policy
2. **137x-B: container / primitive design cleanout**（after 137x-A）
   - array typed-slot / map demand / primitive residual docs を同期する
   - runtime-wide implementation は開かない
3. **137x-C: owner-first optimization return**（after 137x-B）
   - active read-side owner proof に戻る
   - narrow perf seam だけを reopen する

## Why This Must Land Before Optimization

review feedback で、設計方向そのものは accept された。
ただし keeper-grade にするには、次の 4 つを先に固定する必要がある。

- `repr` は request であり guarantee ではない
- `StableView` が合法になる provenance 条件を先に固定する
- verifier は boundary tuple だけでなく provenance / borrow-scope / `freeze.str -> publish.text` separation も見る
- repeated `publish.text` の stable cache / fresh allocation policy を決める

この 4 つを曖昧なまま最適化へ戻ると、
helper/site ごとの special case や runtime re-inference が再発しやすい。

## Active Cards

順序はこのまま固定する。

1. [x] `repr-downgrade-contract`
   - `repr` request vs legality owner を lock する
   - landed: direct-kernel verifier rejects unproven `StableView` requests before runtime; lowering must downgrade to `stable_owned` until legality is verifier-visible
2. [x] `stableview-legality-contract`
   - immutable / pinned / already-stable provenance の条件を lock する
   - landed: `stable_view_provenance={already_stable|immutable_host_owned|pinned_no_mutation}` is now MIR/JSON/verifier-visible; `stable_view` without a witness fails before runtime
3. [x] `provenance-freeze-verifier-contract`
   - boundary tuple 以外の verifier-visible contract を lock する
   - landed: `publish.text` now requires `borrow_contract=borrow_text_from_obj`, `source_root`, and `publication_contract=publish_now_not_required_before_first_external_boundary` before codegen
4. [x] `publish-idempotence-policy`
   - repeated `publish.text` が cold stable cache reuse か fresh object かを lock する
   - landed: repeated slot publish is no-op after `Published`; stable cache may reissue handles to cached objects/views, but must not rebirth fresh text objects for the same stable source/cell

## Exit Gate for 137x-A

137x-A は、最低でも次を満たしたら close できる。

- string semantic SSOT に `repr` downgrade rule が明記されている
- `StableView` legality 条件が string-only で固定されている
- verifier が将来見るべき visibility contract が docs 上で分離されている
- `publish.text` の idempotence / cache policy が phase-local decision として読める
- `publish.any` は blocked のままで、next implementation と誤認されない

Status: satisfied. 137x-B may begin the container / primitive design cleanout.
Owner-first optimization return moves to 137x-C, while `publish.any` and
runtime-wide `Value Lane Architecture` implementation remain blocked here.

## Stop-Line

この subphase から開いてはいけないもの:

- runtime-wide `Value Lane Architecture` 実装
- bytes / scalar / array / map lane 実装
- `publish.any`
- public ABI widening
- allocator / arena work
- helper-name driven legality or runtime re-inference の復活

## Relationship to Phase 289x

phase-289x は **parked successor planning** のままにする。

- long-range reading:
  - `Value world -> publish/promote boundary -> Object world`
- current rule:
  - text lane が first proving ground
  - 137x-A が終わるまで runtime-wide implementation は始めない
- reading order:
  1. phase-137x string-only contract closeout
  2. container / primitive design cleanout
  3. owner-first perf return
  4. only then reconsider parked phase-289x successor cards
