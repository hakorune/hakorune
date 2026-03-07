# Phase 188.3: Nested loop lowering (1-level) — make Pattern 6 real

**Date**: 2025-12-27
**Status**: Complete (Pattern6 lowering + merge/latch fix)
**Prereq**: Phase 188.2 Option A is complete (StepTree depth SSOT + strict Fail-Fast)

---

## Goal

`max_loop_depth == 2`（1-level nested loop）を **JoinIR lowering で実際に通す**。

- 既知の事実: `LoopForm (=LoopShape)` にはネスト情報が無いので、**LoopFormベースの Pattern6 検出/ルーティングでは実装できない**
- 実装は **StepTree（AST側）**を SSOT として扱う（Phase 188.2 Option A を継続）

---

## Definition: `max_loop_depth` (SSOT)

`StepTreeFeatures.max_loop_depth` は「その関数（or 評価対象ブロック）の中で、最も深い `loop{}` ネストの深さ」を表す。

- loop が無い: `0`（※StepTree実装に依存。現状の運用では “loopがある前提”で扱う）
- loop が 1 段: `1`
- loop の中に loop が 1 つ: `2`
- `loop{ loop{ loop{ ... } } }`: `3`

Phase 188.2 の strict ガードは「depth > 2 を明示エラー」にしている。
これは言語仕様の制限ではなく、「JoinIR lowering の実装範囲（compiler capability）」の制限。

---

## Scope (minimal)

対応するのは “NestedLoop Minimal” の 1形だけに限定する。

- depth: `max_loop_depth == 2` のみ
- inner loop: Pattern1相当（break/continue 無し）
- outer loop: Pattern1相当（break/continue 無し）を優先
- それ以外:
  - strict mode: 明示エラー（Phase 188.2 の depth check とは別タグで良い）
  - non-strict mode: 既存の fallback 経路に任せる（ただし silent fallback を増やさない）

---

## Current Code Status (reality)

Phase 188.3 は “選択ロジック（NestedLoopMinimal route 選定）→ lowering → merge/rewriter 安定化” まで完了している。

- Selection SSOT: `src/mir/builder/control_flow/joinir/routing.rs` の `choose_route_kind()`
  - cheap check → StepTree → AST validation
  - `max_loop_depth == 2` かつ “NestedLoopMinimal lowerable” のときだけ `LoopRouteKind::NestedLoopMinimal` を返す
- Lowering: `src/mir/join_ir/lowering/loop_routes/nested_minimal.rs`
  - historical physical path at the time: `src/mir/builder/control_flow/joinir/patterns/pattern6_nested_minimal.rs`
  - JoinIR pipeline で `inner_step/k_inner_exit/k_exit` を含む関数群を生成して merge する
  - Fixture: `apps/tests/phase1883_nested_minimal.hako`（RC=9）
- Merge/Rewrite contract（SSOT）:
  - `latch_incoming` を記録してよいのは `TailCallKind::BackEdge` のみ（LoopEntry は上書き禁止）
  - entry-like は “JoinIR main の entry block のみ”
  - 二重 latch は `debug_assert!` で fail-fast
  - 実装: `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`, `src/mir/builder/control_flow/joinir/merge/loop_header_phi_info.rs`

---

## SSOT (what to rely on)

- nesting depth SSOT: `StepTreeFeatures.max_loop_depth`
- depth > 2: strict mode で `control_tree/nested_loop/depth_exceeded`（Phase 188.2）

---

## Scope & Variable Model (SSOT)

Nyash の変数スコープ方針に合わせて、nested loop lowering でも以下を SSOT として固定する。

### Visibility (read)

- inner loop から outer の binding は参照できる（lexical scope: “1つ上は見える”）
- ただし JoinIR lowering では「見える」を **明示的な引数/継続で表現**する（暗黙キャプチャを増やさない）

### Mutation (write-back)

Phase 188.3 では段階的に進める：

- **P188.3 (minimal)**: “最小の write-back” を **明示的な carrier** として許す
  - inner loop が outer 変数を更新する場合、その変数は **carrier として引数で受けて、`k_inner_exit(...)` で返す**
  - 対応範囲は最小（fixtureが要求する 1 carrier 程度）に限定する
- **P188.4+ (generalize)**: write-back を一般化（複数 carrier / 代入形 / break/continue を含む形）

この分離により、write-back の複雑さ（複数 carrier / break / continue / PHI の再接続）を Phase 188.3 に持ち込まずに済む。

---

## Lowering sketch (how it should look)

Nested loop は JoinIR の「tail recursion + continuation」を再帰的に合成して表現する。

- outer: `outer_step(state..., k_outer_exit)`
- inner: `inner_step(state..., k_inner_exit)`
- inner が終わったら `k_inner_exit(...)` で outer の“残り”へ戻る

この `k_inner_exit` がスコープ境界として働くので、将来の write-back もここに集約できる。

### Minimal function graph (recommended)

Phase 188.3 の最小形は「outer の loop_step の中で inner の loop_step を呼び、inner が終わったら k_inner_exit で outer に戻る」。

推奨の JoinIR 関数群（概念）:

- `main(i0, sum0)`:
  - `Call(loop_step, [i0, sum0])`
  - `Ret 0`（statement-position loop の場合）
- `loop_step(i, sum)`（outer の step。canonical name は `loop_step` に寄せる）:
  - `exit_cond = !(i < N_outer)`
  - `Jump(k_exit, [sum], cond=exit_cond)`
  - `Call(inner_step, [j0, i, sum])`
- `inner_step(j, i_outer, sum)`:
  - `exit_cond = !(j < N_inner)`
  - `Jump(k_inner_exit, [i_outer, sum], cond=exit_cond)`
  - `sum_next = sum + 1`（fixture の最小形）
  - `j_next = j + 1`
  - `Call(inner_step, [j_next, i_outer, sum_next])`
- `k_inner_exit(i, sum)`:
  - `i_next = i + 1`
  - `Call(loop_step, [i_next, sum])`
- `k_exit(sum)`（canonical name `k_exit`）:
  - `Ret sum`

重要:
- **carrier はグローバル扱いしない**。`sum` は引数で運んで戻す。
- merge 側の “loop_step 関数の選定” を壊さないため、`inner_step` と `k_inner_exit` は boundary の `continuation_func_ids` に含めて除外する（詳細は指示書）。

---

## Merge/Rewrite contract (SSOT) — “undef ValueId” を防ぐ

JoinIR merge は「JoinIR の param ValueId は SSA 命令で定義されない」前提なので、**適切な Copy（param binding）** が入らないと即 `use of undefined value ValueId(...)` になる。

特に Pattern6 では `loop_step`（outer）から `inner_step` へ tail-call するため、以下を SSOT として固定する：

- **Skip するのは “target が loop header” のときだけ**
  - header では PHI dst が carrier の SSOT なので、param Copy を入れて上書きしてはいけない
  - 一方で `loop_step → inner_step` は “target が header ではない” ため、**inner_step params を定義する Copy が必要**
- **param の index ベース remap は危険**
  - `inner_step(j, i, sum)` のように、先頭に loop-local（`j`）が混ざると carrier の index と一致しない
  - index remap は `j` を `i` の PHI dst に誤接続しやすい

この契約に反する場合の典型症状：
- `Function 'inner_step' params: [ValueId(104), ...]` の直後に `use of undefined value ValueId(104)`

修正の第一候補は `merge/instruction_rewriter.rs` 側（tail-call param binding の skip 条件）であり、merge/mod.rs の “パラメータ再マップを拡張する” で逃げない（責務を混ぜない）。

---

## Deliverables

1. **Fixture + integration smoke（exit code SSOT）**
   - 1-level nested loop を最小で再現する `.hako` を追加
   - integration で実行し、exit code で判定（stdout比較はしない）

2. **StepTree-based lowering implementation**
   - StepTree を辿って、inner loop を outer loop の中で正しく lowering できるようにする
   - 入口は “StepTree→JoinIR” のどこか（LoopFormベースの router は使わない）

3. **Docs update**
   - Phase 188.1 の “Pattern6 specification” が design であることは維持
   - Phase 188.3 で “実装済み/未実装の境界” を明確に書く

---

## Acceptance Criteria

- `./tools/smokes/v2/run.sh --profile quick` が常にグリーン維持
- integration selfhost が FAIL=0 を維持
- 追加した nested loop fixture が PASS（JoinIR lowering が使われたことをログ/タグで確認可能）
  - 実測: `./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako` → RC=9

---

## Instructions (handoff)

実装者（Claude Code）向けの手順書は `docs/development/current/main/phases/phase-188.3/P1-INSTRUCTIONS.md` を参照。

---

## Next (schedule)

- **Phase 188.3**: depth=2 の最小形を “確実に通す” + PoC fixture を smoke 固定
- **Phase 188.4+**: write-back（outer carrier reconnection）と “再帰 lowering の一般化（depthを増やしても壊れない）” を docs-first で設計してから実装

### Post-completion (refactoring window)

実装完了後のリファクタ（意味論不変）を挟む場合は、`P2-REFACTORING-INSTRUCTIONS.md` を入口にする。

### Planned cleanup (after Phase 188.3)

Pattern6 を通す過程で露出しやすい “暗黙ルール” を SSOT 化して、今後の nested/generalization を楽にする：

- `JoinInlineBoundary` に **loop header func name を明示する SSOT**（merge の “entry func 推定” を段階的に減らす）
- `ParamBinding` の規則を “source-based ではなく target-based” に統一（header だけ特別扱い）
- （将来）param role（carrier vs local）を明文化し、index remap の誘惑を消す

---

## Out of Scope

- nested loop + break/continue の一般対応
- LoopRegion を使った MIR-level nesting SSOT（Option B）
