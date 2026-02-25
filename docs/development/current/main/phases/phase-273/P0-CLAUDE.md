# Phase 273 P0: Plan Extractor（pure）導入の最小収束（Claude Code 指示書）

Status: instructions / design-first

目的:
- pattern を “分岐ロジック” ではなく “Plan抽出プラグイン” に降格する。
- block/value/phi の生成責務を **PlanLowerer に集約**し、裾広がりを止める。

制約:
- Extractor は **builder を触らない**（`next_block_id/next_value_id/insert_phi/...` 禁止）
- 新しい env var 禁止
- by-name hardcode 禁止（Box名/Pattern名の文字列分岐を増やさない）
- terminator SSOT は維持（`Frag → emit_frag()`）

参照:
- SSOT設計: `docs/development/current/main/design/edgecfg-fragments.md`
- 参照実装（既存）: Phase 272/269 の Frag 経路
  - `docs/development/current/main/phases/phase-272/README.md`
  - `docs/development/current/main/phases/phase-269/README.md`

---

## Step 1: Plan の固定語彙を作る（増殖しない型）

新規モジュール（例）:
- `src/mir/builder/control_flow/plan/`（または `src/mir/control_plan/`）

最低限の語彙（案）:
- `Plan::Seq(Vec<Plan>)`
- `Plan::Loop { params, body }`
- `Plan::If { cond, then_, else_ }`
- `Plan::Effect { ... }`（副作用の順序だけ表す）
- `Plan::Exit { kind, values }`

重要:
- Plan は AST/expr を “参照” するだけ（必要なら node id / span を持つ）
- Plan は “CFGを作らない”

---

## Step 2: Extractor を pure にする（pattern の責務縮退）

対象:
- `src/mir/builder/control_flow/joinir/patterns/*`

方針:
- 既存の `extract_*_parts(...)` のような抽出関数を “Plan を返す” 形に寄せる
- `Ok(None)` / `Ok(Some(plan))` / `Err(...)` を明確に使い分ける

Fail-fast:
- “一致した” のに必要条件が満たせない場合は `Err`（close-but-unsupported を黙って通さない）

---

## Step 3: PlanLowerer を作る（唯一の builder 触り役）

新規モジュール（例）:
- `src/mir/builder/control_flow/plan/lowerer.rs`

責務:
- block/value/phi を作るのはここだけ
- Frag を組み立てて `emit_frag()` に渡す
- 既存の emission（Phase 272 の `loop_scan_with_init` / `loop_split_scan` など）を呼ぶのは Lowerer 側に寄せる

---

## Step 4: まず1つのパターンで PoC（最小差分）

推奨:
- Pattern6 または Pattern8 を 1つ選び、以下の順で移行:
  1) extractor は plan を返すだけにする
  2) lowerer が既存 emission を呼ぶ
  3) router は “plan を得たら lowerer へ” にする

Acceptance:
- 既存の fixture/smoke が変わらず PASS（既定挙動不変）
- Extractor が builder を触っていないこと（grep で確認）

---

## Step 5: docs（SSOT）更新

更新対象:
- `docs/development/current/main/phases/phase-273/README.md`

最低限:
- “Extractorはpure / Lowererだけが作る” を SSOT として明文化
- Plan語彙が固定であること（増殖しない理由）を短く書く

---

## 完了条件（P0）

- Plan の語彙が導入され、Extractor が pure になっている（最低1パターンでPoC）
- Lowerer が block/value/phi 生成の唯一の場所になっている
- terminator SSOT（emit_frag）が維持されている
- 既存スモークに退行がない
