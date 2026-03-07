# Phase 29ai P12: Facts SSOT（loop_break body-local promotion）

Date: 2025-12-29
Status: Historical reference (implemented)
Scope: loop_break route の body-local promotion（A-3 Trim / A-4 DigitPos）を Facts として仕様化した slice（仕様不変）
Goal: “promotion が必要な形” を Facts で 1 箇所に固定し、JoinIR 側に解析が散らないようにする

Historical note:
- `Pattern2LoopBodyLocalFacts`, `pattern2_loopbodylocal_facts.rs`,
  `phase29ab_pattern2_loopbodylocal_*` は P12 実行時の type/path/pin token だよ。
- current runtime vocabulary は `LoopBreakBodyLocalFacts`,
  `try_extract_loop_break_body_local_facts`, `loop_break_body_local` に揃っているよ。

## Background

loop_break route には break 条件が loop-body で生成した一時値（body-local）に依存する形があり、
ここが promotion（host binding / derived slot / trim 形状など）の主要露出点になっている。
既存実装は JoinIR 側の箱群で成立していたが、長期的には Facts→Plan→Emit の 1 本導線へ寄せたい。

P12 では “解析（何が body-local で、どの形か）” を Facts として SSOT 化し、Planner/Emitter が再解析・分岐増殖しない形にする。

## Non-goals（この P12 ではやらない）

- 実際の promotion ロジックの差し替え（既定挙動の変更）
- derived slot emitter / merge 側の契約変更
- loop_break の全形対応（real-world まで一気に拡張しない）
- 新しい env var / debug トグル追加
- by-name 分岐の追加（禁止）

## Target Fixtures（SSOT）

この 2 本を “promotion が必要な代表” として SSOT にする（既存のまま）:

- `apps/tests/phase29ab_pattern2_loopbodylocal_min.hako`（legacy fixture pin token / A-4 DigitPos）
- `apps/tests/phase29ab_pattern2_loopbodylocal_seg_min.hako`（legacy fixture pin token / A-3 Trim / seg）

## Deliverables

1. Facts 型と抽出入口（SSOT）:
   - `LoopBreakBodyLocalFacts`（例: `shape=TrimSeg|DigitPos` + 関連 var 名 + 依存グラフの最小）
   - 抽出: `try_extract_loop_break_body_local_facts(condition, body) -> Result<Option<_>, Freeze>`
2. Facts→Planner/Plan への橋渡しのための “最低限の情報” を明文化（どの情報が必須か）。
3. unit tests（AST を組んで `Ok(Some)` / `Ok(None)` の境界を固定）。
4. 追跡 docs（phase-29ai README / Now / Backlog）更新。

## Proposed Data Model（最小）

`LoopBreakBodyLocalFacts` は “promotion の判断に必要な最小セット” だけ持つ（推測は禁止。取れなければ `Ok(None)`）。

例:

- 共通:
  - `loop_var: String`
  - `loopbodylocal_var: String`（例: `seg` / `digit_pos`）
  - `break_uses_loopbodylocal: bool`（必ず true で Some を返す）
  - `shape: LoopBodyLocalShape`（下の enum）
- `LoopBodyLocalShape`（P12は2つだけ）
  - `TrimSeg { s_var, i_var }`（`seg = s.substring(i, i+1)` + `if seg == " " || seg == "\\t" { break }`）
  - `DigitPos { digits_var, ch_var }`（`digit_pos = digits.indexOf(ch)` + `if digit_pos < 0 { break }`）

## Implementation Steps（Critical Order）

### Step 1: Facts module を追加

- 新規: `src/mir/builder/control_flow/plan/facts/loop_break_body_local_facts.rs`
- 更新: `src/mir/builder/control_flow/plan/facts/mod.rs`（module 登録）

### Step 2: LoopFacts に “補助 facts” を接続（ただし既定挙動不変）

- 更新:
  - `src/mir/builder/control_flow/plan/facts/loop_types.rs`
  - `src/mir/builder/control_flow/plan/facts/loop_builder.rs`
  - `LoopFacts.loop_break_body_local: Option<LoopBreakBodyLocalFacts>`
  - `try_build_loop_facts()` で `try_extract_loop_break_body_local_facts(condition, body)` を呼ぶ

方針:
- 抽出が曖昧なら `Ok(None)`（Freeze は出さない）。
- `loop_break` が `None` のときは `loop_break_body_local` も `None` にする（矛盾を作らない）。

### Step 3: unit tests で境界固定

- `src/mir/builder/control_flow/plan/facts/loop_break_body_local_facts.rs` に `#[cfg(test)]` を追加
- 2 fixture 相当の AST を直接組んで `Ok(Some(shape=...))` を固定
- “似てるけど違う” 形（例: `seg = substring(i,i+2)` / break で seg を使ってない）を `Ok(None)` に固定

### Step 4: docs / tracking 更新

- `docs/development/current/main/phases/phase-29ai/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`

書くこと（最小）:
- P12 の scope（Facts のみ、仕様不変）
- P13 候補（例: Facts→Planner で “promotion-required” を Plan に載せる）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Notes（Fail-Fast / Freeze）

P12 は実行経路で Freeze を増やさない（曖昧なら `Ok(None)`）。Fail-Fast/Freeze を導入するのは、P13 以降で
“planner-first で確実に採用される範囲” が固まってからにする。
