# Phase 256: JoinIR Contract Questions (for ChatGPT Pro)

目的: Phase 256 の詰まり（Jump/continuation/params/jump_args の暗黙契約）を、設計として固めるための相談メモ。

---

## Q1. SSOT をどこに置くべき？

JoinIR の「意味論 SSOT」をどこに置くべきか。

- A) Structured JoinIR を SSOT として維持し、bridge/merge が意味解釈する
- B) Normalized JoinIR を SSOT とし、Structured→Normalized の正規化箱を必須化する

判断材料として、現在の層の境界と責務:
- Pattern lowerer（Structured JoinIR 生成）
- `join_ir_vm_bridge`（JoinIR→MIR 変換）
- merge（MIR inline + PHI/ExitLine wiring）

---

## Q2. `JoinInst::Jump` の正規形（不変条件）

現状の詰まりは `Jump` が層を跨ぐときに「tail call 等価」になったり「Return 化」になったりして、continuation が失われる点にある。

相談したい:
- `JoinInst::Jump { cont, args, cond }` を SSOT 的にどう定義するべきか？
- cond 付き Jump は JoinIR 語彙として残すべきか？それとも IfMerge に寄せるべきか？

最小コード（JoinIR 命令）:

```rust
// src/mir/join_ir/mod.rs
pub enum JoinInst {
    // ...
    Jump {
        cont: JoinContId,
        args: Vec<VarId>,
        cond: Option<VarId>,
    },
    Call {
        func: JoinFuncId,
        args: Vec<VarId>,
        dst: Option<VarId>,
        k_next: Option<JoinContId>,
    },
    Ret { value: Option<VarId> },
    // ...
}
```

---

## Q3. boundary/params/jump_args の順序契約をどこで固定する？

Phase 256 では、次の対応関係が暗黙で、崩れると SSA undef / PHI wiring fail-fast になりやすい。

- `JoinInlineBoundary.join_inputs` ↔ `JoinModule.entry.params`
- `exit_bindings` ↔ ExitLine の carrier PHI reconnect
- `jump_args`（tail call args metadata）↔ ExitLine の latch incoming 復元

最小コード（boundary）:

```rust
// src/mir/join_ir/lowering/inline_boundary.rs
pub struct JoinInlineBoundary {
    pub join_inputs: Vec<ValueId>,
    pub host_inputs: Vec<ValueId>,
    pub loop_invariants: Vec<(String, ValueId)>,
    pub exit_bindings: Vec<LoopExitBinding>,
    pub expr_result: Option<ValueId>,
    pub loop_var_name: Option<String>,
    pub continuation_func_ids: std::collections::BTreeSet<String>,
    // ...
}
```

契約の fail-fast は現在ここで行っている:

```rust
// src/mir/builder/control_flow/joinir/merge/contract_checks.rs
pub(in crate::mir::builder::control_flow::joinir) fn run_all_pipeline_checks(
    join_module: &crate::mir::join_ir::JoinModule,
    boundary: &JoinInlineBoundary,
) -> Result<(), String> {
    verify_boundary_entry_params(join_module, boundary)?;
    // ...
    Ok(())
}
```

相談したい:
- この順序契約は「boundary」「normalizer」「bridge」「merge」のどの層が SSOT になるべきか？
- fail-fast の責務をどこに置くべきか（今は conversion_pipeline 直前）？

---

## Q4. continuation の識別: `JoinFuncId` vs `String`（関数名）

Phase 256 P1.7 で「continuation 関数が merge で見つからず SSA undef」になった。
原因は bridge 側と merge 側の “関数名 SSOT” 不一致。

現状の暫定 SSOT は `canonical_names`:

```rust
// src/mir/join_ir/lowering/canonical_names.rs
pub const K_EXIT: &str = "k_exit";
pub const K_EXIT_LEGACY: &str = "join_func_2";
pub const LOOP_STEP: &str = "loop_step";
pub const MAIN: &str = "main";
```

相談したい:
- continuation を `JoinFuncId` で保持し、bridge で 1 回だけ名前解決するべきか？
- それとも `String` を SSOT にして “MirModule key” と一致させ続けるべきか？
- 併存するなら、変換境界（片方→片方）をどこに置くべきか？

---

## Q5. 正規化 shadow（`join_func_N`）との共存戦略

normalized_shadow 側は `join_func_2` のような命名を使う箇所がある。
この legacy をいつ・どう統一するべきか（または統一しないなら境界をどう明文化するか）。

---

## Q6. `jump_args` は MIR のどの層の SSOT か？

観測:
- ExitLine/merge 側は `BasicBlock.jump_args` を「exit/carry 値の SSOT」として参照する。
- bridge 側で tail call / Jump を生成するときに `jump_args` を落とし忘れると、ExitLine が fallback 経路へ入りやすく、
  SSA/dominance の破綻につながる。

最小コード（MIR basic block）:

```rust
// src/mir/mod.rs
pub struct BasicBlock {
    pub instructions: Vec<MirInstruction>,
    pub instruction_spans: Vec<Span>,
    pub terminator: Option<MirInstruction>,
    pub jump_args: Option<Vec<ValueId>>,
    // ...
}
```

相談したい:
- `jump_args` は terminator（Jump/Branch/Return/Call）に埋め込むべきか、それとも BasicBlock の外部メタのままでよいか？
- `jump_args` の契約（順序・長さ・expr_result の有無・invariants の混在）をどこで固定するべきか？

---

## Q7. Optimizer/DCE の不変条件（spans 同期と jump_args）

観測:
- DCE が `jump_args` だけで使われる値を “unused” とみなすと、Copy/Const が消えて merge が壊れる。
- DCE が `instructions` だけを削って `instruction_spans` を同期しないと、SPAN MISMATCH が発生しデバッグが困難になる。

相談したい:
- `jump_args` は “use” として扱うのが SSOT として正しいか？
- spans 同期は「各パスの責務」か、それとも `BasicBlock` の API（例: spanned filter）に閉じ込めるべきか？

---

## 観測された失敗例（短く）

- SSA undef（関数名不一致で continuation が merge 対象にならず到達不能/未定義が露出）
- ExitLine: `jump_args` の長さ契約ミスマッチ（carriers と invariants の混在）
- `JoinInst::Jump` が bridge で “Return 化” され、continuation の意味が落ちる疑い
 - DCE が `jump_args` 由来の use を落とし、Copy が消えて SSA/dominance が崩れる

---

## ChatGPT Pro からの設計回答（要約）

診断（短く）:
- JoinIR/MIR 間に「暗黙 ABI（呼び出し規約）」が生えており、SSOT が分裂している
- `Jump/cont/params/jump_args` が層を跨ぐたびに意味が揺れて、SSA/dominance/DCE が壊れやすい

命名:
- この収束先（north star）を **Join-Explicit CFG Construction** と呼ぶ

提案の大枠（3案）:

### 案1: JoinIR ABI / Contract モジュール（推奨）

狙い:
- いま暗黙のまま散っている契約（順序/長さ/名前/役割）を “ABI オブジェクト” として 1 箇所に封印する
- `Vec` の順序契約に魂を預けない（pack/unpack を ABI 経由にする）

最小イメージ（雰囲気）:

```rust
pub struct JoinAbi {
    pub cont_sigs: Vec<ContSig>,      // continuation signature SSOT
    pub special: SpecialConts,        // main/loop_step/k_exit
    pub legacy_alias: AliasTable,     // join_func_2 -> k_exit 等
}

pub struct ContSig {
    pub params: Vec<Param>,
}

pub enum ParamRole {
    Carrier,
    Invariant,
    Result,
    // ...
}
```

Jump の正規形（Normalized JoinIR）:
- `Jump` は cond を持たず、必ず終端
- cond 付きは `Branch { then_cont+args, else_cont+args }` に寄せる

### 案2: MIR を “ブロック引数 SSA” に昇格（強力）

狙い:
- JoinIR の cont/args と MIR の block/jump_args を同型化し、bridge/merge の解釈余地を消す

即効の 2 点（最優先）:
- `jump_args` を BasicBlock 外メタではなく terminator に埋め込む（use-def / DCE が自然に追える）
- spans を並行 Vec から `Vec<Spanned<_>>` へ（SPAN MISMATCH を構造で防ぐ）

### 案3: JoinIR をやめて CPS CFG 一本化（最終収束案）

狙い:
- SSOT を 1 個にする（JoinIR/MIR の “橋” を消す）
- ただし移行は重いので、案2 の延長として収束させるのが現実的

---

## 推奨（この repo の制約込み）

Phase 256 の “今日の詰まり” に効く順で:

1) **案1（JoinIR ABI/Contract）を設計 SSOT として採用**
   - まず “順序契約を殺す” のが最大のデバッグ短縮になる
2) **案2 のうち即効ポイントを段階導入**
   - `jump_args` の SSOT は “terminator operand” に移す（大工事なので Phase 256 を緑に戻した後に着手が安全）
   - spans の `Spanned` 化も同様に段階導入（いまは pass 側で不変条件をテストで固定）

ここまでで、JoinIR を増やさずに「暗黙 ABI を明文化」できる。

---

## 次に設計として決めたいこと（Decision 候補）

- `JoinInst::Jump` の SSOT は “tail call 等価” ではなく、Normalized JoinIR の terminator 語彙として固定する
- continuation の識別は **ID SSOT**（String は debug/serialize 用に限る）
- `jump_args` を SSOT にするなら、最終的に MIR terminator に埋め込む（DCE/CFG の整合性が自然になる）

---

## Phase 256 実装から得た追加の教訓（SSOT）

- `expr_result` と LoopState carrier が同一 ValueId になるケースが現実に起きる（例: ループ式の返り値が `result`）。
  このとき “legacy expr_result slot（jump_args[0]）” を機械的に仮定すると、offset がずれて ExitLine の配線が崩れる。
- 対策として `ExitArgsCollector` には “slot があるかどうか” を推測させず、呼び出し側が `expect_expr_result_slot`
  を明示して渡すのが安全（Fail-Fast + 構造）。
