---
Status: SSOT
Scope: MIR命令契約（MIR enum → MIR JSON emit → VM → LLVM/fallback）の実装同期
Decision: accepted
Updated: 2026-02-10
Related:
- src/mir/instruction.rs
- src/runner/mir_json_emit/emitters/mod.rs
- src/runner/mir_json_emit/emitters/control_flow.rs
- src/runner/mir_json_emit/helpers.rs
- src/backend/mir_interpreter/handlers/mod.rs
- src/llvm_py/builders/instruction_lower.py
- src/llvm_py/mir_call_compat.py
- src/llvm_py/instructions/mir_call/__init__.py
- src/runner/modes/llvm/fallback_executor.rs
- docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md
- docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md
- docs/development/current/main/design/compiler-pipeline-ssot.md
---

# MIR VM/LLVM Instruction Contract Fix (SSOT)

## Goal

MIR命令の契約を、以下4層で同じ語彙と失敗条件に揃える。

1. MIR enum（命令語彙の真実）
2. MIR JSON emitter（語彙の直列化）
3. VM interpreter（実行受理語彙）
4. LLVM lowering/fallback（実行受理語彙）

このSSOTの目的は「理想像」ではなく「現状の不一致」を固定し、修正順序を固定すること。

## Non-goals

1. 新言語機能の追加
2. selfhost workaroundでの回避
3. 大規模リファクタを伴う一括置換
4. 本書だけでの仕様確定（実装なしで「完了」扱いにしない）

## Observed Contract Gaps (current)

2026-02-10 時点の事実。`Closed` は実装反映済み、`Partial/Open` は未達を示す。

| ID | 現状事実 | 証拠 | 影響 | 現状 |
|---|---|---|---|---|
| G1 | MIR enumヘッダと実体語彙の不一致は解消済み。`TypeCheck`/`Cast`/`PluginInvoke`/`WeakNew`/`WeakLoad`/`BarrierRead`/`BarrierWrite` は enum から除去済み。 | `src/mir/instruction.rs:2`, `src/mir/instruction.rs:18`, `src/mir/instruction.rs:166` | ヘッダ/実装の認識ズレを抑止。 | Closed（C7a/C7z, 2026-02-10） |
| G2 | MIR JSON emitter の unsupported `None` drop は除去済み。unsupported は `Err` で停止。 | `src/runner/mir_json_emit/emitters/mod.rs:33`, `src/runner/mir_json_emit/emitters/mod.rs:53`, `src/runner/mir_json_emit/emitters/control_flow.rs:21`, `src/runner/mir_json_emit/emitters/control_flow.rs:23` | 命令欠落の silent 化を抑止。 | Closed（C2, 2026-02-10） |
| G3 | `mir_call` emitter は Constructor=`name`、Value=`value` に統一済み。LLVM compat/dispatcher 側も旧キー（`box_type`/`function_value`）読取りを撤去し、legacyキーは fail-fast で拒否する。 | `src/runner/mir_json_emit/helpers.rs`, `src/llvm_py/mir_call_compat.py`, `src/llvm_py/instructions/mir_call/__init__.py`, `src/llvm_py/tests/test_mir_call_compat.py` | canonical schema のみ受理し、経路依存の解釈ズレを抑止。 | Closed（C10, 2026-02-10） |
| G4 | VM preflight（unsupported命令/terminatorの前倒し拒否）は strict/dev + planner_required gate 下で有効。release既定は実行時 `unimplemented instruction` が残る。 | `src/backend/mir_interpreter/exec/diagnostics.rs:6`, `src/backend/mir_interpreter/exec/diagnostics.rs:81`, `src/backend/mir_interpreter/exec/diagnostics.rs:92`, `src/backend/mir_interpreter/handlers/mod.rs:183` | strict/dev の fail-fast は達成。release既定は互換維持のため現状固定。 | Partial（C5完了、D4で現状維持を確定） |
| G5 | LLVM lowerer の unknown op は `RuntimeError` で fail-fast 化済み。 | `src/llvm_py/builders/instruction_lower.py:62`, `src/llvm_py/builders/instruction_lower.py:64`, `src/llvm_py/builders/instruction_lower.py:269`, `src/llvm_py/builders/instruction_lower.py:271` | unknown op の silent 継続を抑止。 | Closed（C4, 2026-02-10） |
| G6 | LLVM fallback の return 判定は terminator 参照へ移行済み。 | `src/runner/modes/llvm/fallback_executor.rs:46`, `src/runner/modes/llvm/fallback_executor.rs:48` | MIR構造（terminator責務）と整合。 | Closed（C6, 2026-02-10） |
| G7 | 命令 x backend（VM/JSON/LLVM）の coverage matrix テストを導入し、`backend_core_ops` でドリフト検出を固定した。 | `src/mir/contracts/backend_core_ops.rs`（`backend_coverage_matrix_*` tests） | 将来ドリフトをCI/ローカルで早期検知できる。 | Closed（C8, 2026-02-10） |

## Decision

項目ごとに decision と follow-up（必要時）を分離する。

### D1: Backend-core命令集合をSSOT化する
Decision: accepted

- 1つの命令集合（backend-core allowlist）を基準に、emitter/VM/LLVMが同じ受理語彙を参照する。
- 受理外は「黙殺」ではなく契約エラーで停止する。

### D2: MIR JSON emitで `None` dropを禁止する
Decision: accepted

- `emit_instruction` / `emit_terminator` の未対応分岐を `Err` 化する。
- `emit_mir_json` 呼び出し元へエラー伝搬し、strict/devで停止できる形にする。

### D3: `mir_call` callee schemaの正規キーをSSOT化する
Decision: accepted

- 正規キーを `name` / `value`（Method/Constructor/Value）へ固定する。
- 書き出しは正規キーのみ（Rust emitter）。
- LLVM compat/dispatcher は legacyキー（`method` / `box_type` / `function_value` / `func`）を受理しない。
- legacyキー入力は `ValueError` で fail-fast する。

### D4: VM未対応命令は実行前に落とす
Decision: accepted

- `handlers/mod.rs` の最終ガードは残す。
- ただし主停止点は verifier/contract-check 側へ前倒しする。
- strict/dev + planner_required gate 下の前倒しを現行SSOTとして固定する。
- release既定への拡張は本SSOTの対象外（互換維持を優先）。必要時は別Proposalで扱う。

### D5: LLVM unknown instructionは既定でfail-fast
Decision: accepted

- `trace_debug` のみで継続する挙動をやめる。
- dev診断ログは残してよいが、既定挙動はエラーで停止する。

### D6: MIR命令ダイエット（legacy整理）
Decision: accepted

- P0/P1 の契約固定後に C7 系を実施し、C7z で `TypeCheck/Cast/PluginInvoke/WeakNew/WeakLoad/BarrierRead/BarrierWrite/Print` を enum から除去した。
- 現在は `kept/lowered-away/removed` 台帳が実装と同期している（`lowered-away = 0`、`removed = ArrayGet, ArraySet, BarrierRead, BarrierWrite, Cast, PluginInvoke, Print, RefGet, RefSet, TypeCheck, WeakLoad, WeakNew`）。

## Ordered Fix Plan

### P0: Contract Wiring (first priority)

Status: 完了（C1-C4, 2026-02-10）

1. `Backend-core allowlist` を定義し、契約参照点を1箇所に固定する。  
Done: `src/mir/contracts/backend_core_ops.rs`（C1）
2. MIR JSON emitterの `Option` 経路を `Result` へ変更し、unsupported命令をfail-fastにする。  
Done: `src/runner/mir_json_emit/emitters/mod.rs`, `src/runner/mir_json_emit/emitters/control_flow.rs`, `src/runner/mir_json_emit/mod.rs`（C2）
3. `mir_call` schemaを `name` / `value` に統一し、互換層は読取り専用に寄せる。  
Done: `src/runner/mir_json_emit/helpers.rs`, `src/llvm_py/mir_call_compat.py`, `src/llvm_py/instructions/mir_call/__init__.py`（C3）
4. LLVM `instruction_lower.py` の unknown op を既定エラーに変更する。  
Done: `src/llvm_py/builders/instruction_lower.py`（C4）

### P1: Backend Coverage Alignment

Status: 完了（C5-C6 + C8, 2026-02-10）

1. VMで未対応命令を実行前検証で弾く（実行時 `unimplemented` 依存をやめる）。  
Done: `src/backend/mir_interpreter/exec/diagnostics.rs`（C5, strict/dev gate）
2. LLVM fallbackのreturn判定を terminator 参照へ変更する。  
Done: `src/runner/modes/llvm/fallback_executor.rs`（C6）
3. 命令 x backend（VM/JSON/LLVM）のcoverage matrixテストを導入する。
Done: `src/mir/contracts/backend_core_ops.rs`（C8）

### P2: MIR Instruction Diet (after contract freeze)

Status: 完了（C7a-C7z, 2026-02-10）

1. `src/mir/instruction.rs` ヘッダと実体の不一致を解消する。  
Done: `src/mir/instruction.rs`（C7a）
2. 命令を `kept / lowered-away / removed` で台帳化し、削減順序を固定する。  
Done: `docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md`（C7b）
3. 台帳ドリフト検知を追加し、legacy判定が同じSSOTを参照するようにする。  
Done: `src/mir/contracts/backend_core_ops.rs`, `src/mir/verification/legacy.rs`（C7c）
4. `TypeCheck` / `Cast` / `PluginInvoke` / `WeakNew` / `WeakLoad` / `BarrierRead` / `BarrierWrite` / `Print` / `ArrayGet` / `ArraySet` / `RefGet` / `RefSet` を段階置換し、legacy語彙の実残存を減らす。  
Done: `src/mir/optimizer_passes/normalize.rs`, `src/mir/optimizer.rs`（C7d1-C7d12）
5. enum からの段階 remove を実施する（C7z）。  
Done: `src/mir/instruction.rs`, `src/mir/contracts/backend_core_ops.rs`, `src/mir/instruction_kinds/mod.rs` ほか参照除去

## Acceptance Criteria

計測可能な条件のみを採用する。

1. MIR JSON emitter
- `src/runner/mir_json_emit/emitters/mod.rs` と `src/runner/mir_json_emit/emitters/control_flow.rs` から `_ => None` が消えている。

2. `mir_call` schema
- emitter書き出しで Constructorに `name`、Valueに `value` が出力される。
- LLVM compat/dispatcherが同じキーを読んでいる（`function_value` 依存が消える）。

3. VM/LLVM fail-fast
- VM: unsupported命令は verifier段階で検出され、`unimplemented instruction` 到達がcontract違反ケースに限定される。
- LLVM: unknown instruction でエラー終了し、終了コードが非0。

4. fallback整合
- `fallback_executor` が terminator を見てreturn判定している（`block.instructions` 直接走査に依存しない）。

5. 回帰
- `cargo check --bin hakorune`
- `cargo test -q backend_core_ops::tests::`
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- LLVM代表ケース1件以上（harness有効構成）
- WSL/環境依存で `Invalid cross-device link (os error 18)` が発生する場合は
  `tools/checks/cargo_check_safe.sh -q --bin hakorune` を代替入口として使用する（`tools/checks/exdev_rename_copy_fallback.c` が `rename*` の `EXDEV` を `copy+unlink` へフォールバック）。
- `rg -n "MirInstruction::TypeCheck|MirInstruction::Cast|MirInstruction::PluginInvoke|MirInstruction::WeakNew|MirInstruction::WeakLoad|MirInstruction::BarrierRead|MirInstruction::BarrierWrite|MirInstruction::Print|\\bTypeCheck\\s*\\{|\\bCast\\s*\\{|\\bPluginInvoke\\s*\\{|\\bWeakNew\\s*\\{|\\bWeakLoad\\s*\\{|\\bBarrierRead\\s*\\{|\\bBarrierWrite\\s*\\{" src/mir` が 0 件

## Commit Slicing (1目的1コミット)

1. C1: backend-core allowlist導入（仕様定義のみ）
2. C2: emitter fail-fast化（non-phi/terminator）
3. C3: `mir_call` schema統一（emitter + compat + dispatcher）
4. C4: LLVM unknown op fail-fast化
5. C5: VM pre-verification gating（未対応命令の前倒し検出）
6. C6: fallback terminator判定化
7. C7a: enumヘッダ同期
8. C7b: diet ledger固定
9. C7c: diet drift check + legacy参照のSSOT化
10. C7d1: TypeCheck -> TypeOp(Check) 置換
11. C7d2: Cast -> TypeOp(Cast) 置換
12. C7d3: PluginInvoke -> BoxCall 置換
13. C7d4: WeakNew -> WeakRef(New) 置換
14. C7d5: WeakLoad -> WeakRef(Load) 置換
15. C7d6: BarrierRead -> Barrier(Read) 置換
16. C7d7: BarrierWrite -> Barrier(Write) 置換
17. C7d8: Print -> ExternCall(env.console.log) 置換
18. C7d9: ArrayGet -> BoxCall(get) 置換
19. C7d10: ArraySet -> BoxCall(set) 置換
20. C7d11: RefGet -> BoxCall(getField) 置換
21. C7d12: RefSet -> BoxCall(setField) 置換
22. C7z: enum prune/remove（段階） [Done]
23. C8: 命令 x backend coverage matrix テスト導入
24. C9: D3 sunset / D4 release policy をSSOT確定（docs lock）
25. C10: D3 sunset 実装（legacy callee keys reject）

各コミットで `1目的 = 1受け入れ基準` を固定し、混ぜない。

## Progress Snapshot (fact-based)

1. 完了済み
- C1: backend-core allowlist導入（`src/mir/contracts/backend_core_ops.rs`）
- C2: MIR JSON emitter fail-fast化（unsupported `None` drop削除）
- C3: `mir_call` schema統一（Constructor=`name`, Value=`value` + 互換層調整）
- C4: LLVM unknown op fail-fast化（`instruction_lower.py`）
- C5: VM pre-verification gating（unsupported命令/terminatorの事前拒否）
- C6: LLVM fallback return判定のterminator参照化
- C7a: MIR enum ヘッダ表記を現状実装へ同期
- C7b: MIR instruction diet ledger（kept/lowered-away/removed）を固定
- C7c: MIR instruction diet drift check（cohort constants + legacy参照の一本化）
- C7d1: normalize passで TypeCheck を TypeOp(Check) へ正規化
- C7d2: normalize passで Cast を TypeOp(Cast) へ正規化
- C7d3: normalize passで PluginInvoke を BoxCall へ正規化
- C7d4: normalize passで WeakNew を WeakRef(New) へ正規化
- C7d5: normalize passで WeakLoad を WeakRef(Load) へ正規化
- C7d6: normalize passで BarrierRead を Barrier(Read) へ正規化
- C7d7: normalize passで BarrierWrite を Barrier(Write) へ正規化
- C7d8: Print -> ExternCall(env.console.log) 正規化を経由し、C7zで Print variant を enum から除去（現行実装に Print 分岐は存在しない）
- C7d9: normalize passで ArrayGet を BoxCall(get) へ正規化
- C7d10: normalize passで ArraySet を BoxCall(set) へ正規化
- C7d11: normalize pass + ref field pass で RefGet を BoxCall(getField) へ正規化
- C7d12: normalize pass + ref field pass で RefSet を BoxCall(setField) へ正規化
- C7z: `TypeCheck` / `Cast` / `PluginInvoke` / `WeakNew` / `WeakLoad` / `BarrierRead` / `BarrierWrite` / `Print` を enum から除去し、diet台帳で `removed` へ移動（enum実体36 + removed8）
- C8: `backend_core_ops` に命令 x backend coverage matrix テストを追加（drift検出を固定）
- C9: D3 sunset（旧キー撤去期限 2026-03-31）と D4 release policy（現状維持）をSSOTで固定
- C10: `mir_call_compat` / `mir_call` dispatcher から legacy callee key fallback を削除し、canonical-only + fail-fast を固定

2. 部分達成
- C5は strict/dev gate 配下で有効（release既定挙動は意図的に不変）。

3. 未着手 / 残課題
- なし（本SSOTスコープ内）

## Validation Snapshot (2026-02-10)

- `tools/checks/cargo_check_safe.sh -q --bin hakorune` : PASS（EXDEV環境向け代替入口）
- `LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so cargo test -q backend_core_ops::tests::` : PASS（13 tests）
- `python3 -m unittest src/llvm_py/tests/test_mir_call_compat.py` : PASS（8 tests）
- `LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq` : PASS
- `LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so bash tools/smokes/v2/profiles/integration/apps/archive/phase87_llvm_exe_min.sh` : PASS（LLVM harness representative, exit 42）
- `rg -n "MirInstruction::TypeCheck|MirInstruction::Cast|MirInstruction::PluginInvoke|MirInstruction::WeakNew|MirInstruction::WeakLoad|MirInstruction::BarrierRead|MirInstruction::BarrierWrite|MirInstruction::Print|\\bTypeCheck\\s*\\{|\\bCast\\s*\\{|\\bPluginInvoke\\s*\\{|\\bWeakNew\\s*\\{|\\bWeakLoad\\s*\\{|\\bBarrierRead\\s*\\{|\\bBarrierWrite\\s*\\{" src/mir` : 0件
- `git diff --check` : clean

## Minimal Verification Commands

```bash
rg -n "src/mir/instruction.rs|src/mir/contracts/backend_core_ops.rs|src/mir/verification/legacy.rs|src/mir/optimizer_passes/normalize.rs|src/mir/optimizer.rs|src/runner/mir_json_emit/emitters/mod.rs|src/runner/mir_json_emit/helpers.rs|src/backend/mir_interpreter/handlers/mod.rs|src/llvm_py/builders/instruction_lower.py|src/runner/modes/llvm/fallback_executor.rs" docs/development/current/main/design/mir-vm-llvm-instruction-contract-fix-ssot.md
LD_PRELOAD=$PWD/tools/tmp/exdev/librename_copy_fallback.so cargo test -q backend_core_ops::tests::
python3 -m unittest src/llvm_py/tests/test_mir_call_compat.py
git diff --check
```

EXDEV mitigation (optional):

```bash
tools/checks/cargo_check_safe.sh -q --bin hakorune
```
