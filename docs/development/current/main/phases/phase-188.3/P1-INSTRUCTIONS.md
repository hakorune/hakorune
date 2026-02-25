# Phase 188.3 P1: Pattern6（NestedLoopMinimal）lowering を “実装済み” にする

**Date**: 2025-12-27  
**Scope**: Pattern6 の lowering 実装（fixture を PASS）  
**Non-goals**: 汎用 nested loop / break/continue / 多段ネスト / 多重 inner loop

---

## ✅ Success Criteria

- `apps/tests/phase1883_nested_minimal.hako` が `--backend vm` で **exit=9**
- `./tools/smokes/v2/run.sh --profile quick` が **154/154 PASS**
- integration selfhost が **FAIL=0 維持**
- Pattern6 を選んだ後に **silent fallback しない**（Fail-Fast）

---

## SSOT / Constraints

- ネスト深さ SSOT: `StepTreeFeatures.max_loop_depth`
- Pattern6 選択 SSOT: `src/mir/builder/control_flow/joinir/routing.rs::choose_pattern_kind()`
- Phase 188.2: strict では depth > 2 を明示エラー
- 本タスクで触るのは “lowering stub を実装” のみ

---

## Fixture（目標）

`apps/tests/phase1883_nested_minimal.hako` を SSOT とする（Add/Compare のみ）。

---

## 実装方針（重要）

### 1) `sum` は carrier として渡す（グローバル禁止）

inner loop が outer の `sum` を更新しているので、**JoinIR では `sum` を引数で運び、`k_inner_exit` で outer に戻す**。

- `sum` を “グローバル変数” 扱いにしてはいけない（箱理論と Fail-Fast 的にも事故る）

### 2) merge の「loop_step 選定」を壊さない

JoinIR merge は「main でも continuation でもない関数」を 1つ選んで “loop header” とみなす。
nested loop では `inner_step` が混ざるので、**`inner_step` / `k_inner_exit` を boundary の `continuation_func_ids` に入れて除外**する。

これをしないと、merge が誤って `inner_step` を loop header として選び、PHI/exit binding が壊れる。

---

## 実装タスク（順番）

### Task A: lowering を実装する

対象: `src/mir/builder/control_flow/joinir/patterns/pattern6_nested_minimal.rs`

やること:
- `lower()` で `Err` している stub を置き換えて、JoinIR pipeline を呼ぶ
- 最小形だけ対応し、外れた形は **Pattern6 選択前に落とす**（`is_pattern6_lowerable()` 側を強化）か、ここで明示エラー

推奨構成（Pattern1 と同じ流儀）:
- JoinIR 生成は `src/mir/join_ir/lowering/nested_loop_minimal.rs`（新規）に切り出す
- builder 側は「context 作成 → JoinModule → boundary → conversion pipeline」のみ

（ただし PoC なので builder 側に直書きでも可。差分は最小に。）

### Task B: JoinIR（nested minimal）を生成する

参考: `src/mir/join_ir/lowering/simple_while_minimal.rs`

最小形の JoinIR 関数構成（推奨）:

- `main(i0, sum0)`:
  - `Call(loop_step, [i0, sum0])`
  - `Ret 0`（statement-position）
- `loop_step(i, sum)`（outer）:
  - `exit_cond = !(i < N_outer)`
  - `Jump(k_exit, [sum], cond=exit_cond)`
  - `Call(inner_step, [j0, i, sum])`
- `inner_step(j, i_outer, sum)`:
  - `exit_cond = !(j < N_inner)`
  - `Jump(k_inner_exit, [i_outer, sum], cond=exit_cond)`
  - `sum_next = sum + 1`
  - `j_next = j + 1`
  - `Call(inner_step, [j_next, i_outer, sum_next])`
- `k_inner_exit(i, sum)`:
  - `i_next = i + 1`
  - `Call(loop_step, [i_next, sum])`
- `k_exit(sum)`:
  - `Ret sum`

命名:
- `k_exit` は canonical name: `src/mir/join_ir/lowering/canonical_names.rs::K_EXIT`
- `loop_step` は canonical name: `...::LOOP_STEP`
- `inner_step` / `k_inner_exit` は Phase 188.3 で追加（文字列でよいが、可能なら canonical_names に追加）

### Task C: boundary（exit binding + continuation funcs）を正しく構築する

参考: `src/mir/builder/control_flow/joinir/patterns/pattern1_minimal.rs`

必須:
- join_inputs/host_inputs は **JoinModule.entry.params と同順・同数**
- `exit_bindings` は `sum` を host slot に reconnect
  - join_exit_value は `k_exit` の param（Pattern1 と同じく `join_module.require_function("k_exit").params[0]` から取得）
- `continuation_func_ids` に以下を含める:
  - `k_exit`
  - `k_inner_exit`
  - `inner_step`

### Task D: integration smoke を追加する（exit code SSOT）

新規:
- `tools/smokes/v2/profiles/integration/joinir/phase1883_nested_minimal_vm.sh`
  - `apps/tests/phase1883_nested_minimal.hako` を実行し、exit code == 9 で PASS
  - stdout 比較はしない

注意:
- `tools/smokes/v2` は `manifest.txt` 方式ではない（`find` ベース）
- 既存の helper を使う: `source tools/smokes/v2/lib/test_runner.sh`

---

## 検証手順

1. `cargo build --release`
2. `./target/release/hakorune --backend vm apps/tests/phase1883_nested_minimal.hako`（exit=9）
3. `./tools/smokes/v2/run.sh --profile integration --filter "phase1883_nested_minimal"`
4. `./tools/smokes/v2/run.sh --profile quick`（154/154 PASS）
5. `./tools/smokes/v2/run.sh --profile integration --filter "selfhost_"`（FAIL=0）

---

## Troubleshooting: `use of undefined value ValueId(...)`（Pattern6）

典型ログ：

- `[cf_loop/joinir] Function 'inner_step' params: [ValueId(104), ...]`
- `use of undefined value ValueId(104)`

意味：
- JoinIR の “param ValueId” は SSA 命令で定義されないため、`Copy(dst=param, src=arg)` が入らないと undefined になる。

優先して疑う場所（責務の順）：

1. `src/mir/builder/control_flow/joinir/merge/instruction_rewriter.rs`
   - `plan_rewrites()` の “tail-call param binding” の **skip 条件**
   - SSOT: **skip は “target が loop header” のときだけ**（header PHI dst を上書きしないため）
   - `loop_step（entry func）→ inner_step` まで “entry func だから” という理由で skip すると、inner_step param が未定義になる
2. `JoinInlineBoundary.continuation_func_ids`
   - `inner_step` / `k_inner_exit` が continuation 扱いになっていないと、merge 側の entry func 選定がズレて skip 判定も破綻しやすい

注意（避けたい対処）：
- `merge/mod.rs` で “param ValueId を index で PHI dst にリマップする” は危険
  - Pattern6 は `inner_step(j, i, sum)` のように先頭に loop-local があり、carrier index と一致しない
  - index remap は `j` を `i` の PHI dst に誤接続しやすい

---

## 追加の注意（Fail-Fast）

- Pattern6 が選ばれたあとに `Ok(None)` で他パターンに流すのは禁止（silent fallback）
- “選ぶ前に落とす” が最も安全:
  - `is_pattern6_lowerable()` を「lowering が確実に通る形だけ true」に強化する
