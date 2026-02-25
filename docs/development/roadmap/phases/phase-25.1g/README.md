# Phase 25.1g — Conservative PHI ↔ ControlForm 統合（Rust統合完了）

Status: completed（ControlForm 導線統合＋レガシー縮退／挙動は不変）

## ゴール

- 25.1f で整えた **ControlForm 観測レイヤー**（LoopShape / IfShape + ControlForm）を、  
  Conservative PHI Box（If/Loop 用 SSA/PHI ロジック）の入口として使えるようにする。
- いきなりすべてを書き換えるのではなく、
  1. If 用 Conservative PHI → ControlForm 対応
  2. LoopForm v2 Exit PHI → ControlForm 対応
  の順に、小さく段階的に寄せる。
- 各ステップごとに Rust テストと `tools/test_stageb_min.sh` を流しつつ、  
  SSA/PHI の赤ログが増えていないことを確認して進む。

## 前提（25.1f までで揃ったもの）

- Rust:
  - `src/mir/control_form.rs`:
    - `LoopShape` / `IfShape` / `ControlKind` / `ControlForm` / `CfgLike` / `is_control_form_trace_on()` が定義済み。
  - `src/mir/loop_builder.rs`:
    - LoopForm v2 経路（`build_loop_with_loopform`）の出口で `LoopShape` → `ControlForm::Loop` を生成・トレース。
    - `lower_if_in_loop` の merge 部分で `IfShape` → `ControlForm::If` を生成・トレース。
    - `NYASH_CONTROL_FORM_TRACE`（未設定=ON, 0/false=OFF）でトレース ON/OFF 切り替え可能。
  - Conservative PHI:
    - If 用: `src/mir/builder/phi.rs` / `src/mir/phi_core/if_phi.rs`（Conservative PHI Box 実装）。
    - Loop 用: `src/mir/phi_core/loopform_builder.rs`（LoopForm v2 の Exit PHI / Carrier/Pinned 対応）。
- .hako:
  - `lang/src/shared/mir/control_form_box.hako`:
    - `static box ControlFormBox`（kind_name + loop_* / if_* + entry/exits）だけ実装済み（まだ未使用）。

## 方針（Option B の範囲）

- **このフェーズでは**:
  - Conservative PHI の「インターフェースと呼び出し位置」に ControlForm の導線を用意する。
  - ただし既存のロジック（引数で BlockId を受け取る形）は残し、ControlForm 導線は **観測＋補助的な入口** として扱う。
  - 影響範囲は If / Loop の PHI 部分に限定し、他の MIR 降下には触れない。
- 実装方針:
  - 新しい API を「足す」→ 既存コードから段階的に使い始める、という形で進める。
  - 例:  
    - If 用: `merge_modified_at_merge_with_control(form: &ControlForm, ...)` を追加。  
    - Loop 用: `build_exit_phis_for_control(form: &ControlForm, ...)` を追加。
  - 最初のステップでは「ControlForm から必要な BlockId を取り出して、既存の関数に委譲するだけ」の薄いラッパにする。

## 実施内容

### G‑1: If 用 Conservative PHI への ControlForm 導線（完了）

- 目的:
  - `loop_builder.rs::lower_if_in_loop` から `ControlForm::If` を PHI ロジックに渡し、Conservative PHI Box が ControlForm ベースで動けるようにする。
- 実装:
  - `src/mir/phi_core/if_phi.rs` に ControlForm ベースの薄いラッパを追加:
    ```rust
    pub fn merge_modified_with_control<O: PhiMergeOps>(
        ops: &mut O,
        form: &crate::mir::control_form::ControlForm,
        pre_if_snapshot: &HashMap<String, ValueId>,
        then_map_end: &HashMap<String, ValueId>,
        else_map_end_opt: &Option<HashMap<String, ValueId>>,
        skip_var: Option<&str>,
        then_pred_opt: Option<crate::mir::BasicBlockId>,
        else_pred_opt: Option<crate::mir::BasicBlockId>,
    ) -> Result<(), String> {
        use crate::mir::control_form::ControlKind;

        let shape = match &form.kind {
            ControlKind::If(shape) => shape,
            _ => return Ok(()),
        };

        let merge_bb = shape.merge_block;

        let trace = std::env::var("NYASH_IF_TRACE").ok().as_deref() == Some("1");
        if trace {
            eprintln!(
                "[if-phi/control-form] Using ControlForm wrapper: merge={:?} then={:?} else={:?}",
                merge_bb, shape.then_block, shape.else_block
            );
        }

        merge_modified_at_merge_with(
            ops,
            merge_bb,
            shape.then_block,
            shape.else_block.unwrap_or(shape.then_block),
            then_pred_opt,
            else_pred_opt,
            pre_if_snapshot,
            then_map_end,
            else_map_end_opt,
            skip_var,
        )
    }
    ```
  - `src/mir/loop_builder.rs::lower_if_in_loop` では、IfShape→ControlForm を構築してこのラッパを直接呼び出すように統合:
    ```rust
    let if_shape = IfShape {
        cond_block: pre_branch_bb,
        then_block: then_bb,
        else_block: Some(else_bb),
        merge_block: merge_bb,
    };
    let form = ControlForm::from_if(if_shape.clone());

    crate::mir::phi_core::if_phi::merge_modified_with_control(
        &mut ops,
        &form,
        &pre_if_var_map,
        &then_var_map_end,
        &else_var_map_end_opt,
        None,
        Some(then_pred_to_merge),
        Some(else_pred_to_merge),
    )?;
    ```
  - その後ろで `is_control_form_trace_on()` が true のときに `form.debug_dump()` と `if_shape.debug_validate()` を呼び、  
    If の形を ControlForm 経由で常時観測できるようにした。
- テスト:
  - `cargo test -q mir_stage1_using_resolver_min_fragment_verifies -- --nocapture`
  - `cargo test -q mir_stage1_using_resolver_full_collect_entries_verifies -- --nocapture`
  - いずれも PASS（Conservative PHI の挙動変化なし）。

### G‑2: LoopForm v2 Exit PHI への ControlForm 導線（完了）

- 目的:
  - LoopForm v2 Exit PHI が ControlForm（LoopShape）を入口に使えるようにする。
- 実装:
  - `src/mir/phi_core/loopform_builder.rs` に ControlForm 用ラッパを追加:
    ```rust
    pub fn build_exit_phis_for_control<O: LoopFormOps>(
        loopform: &LoopFormBuilder,
        ops: &mut O,
        form: &crate::mir::control_form::ControlForm,
        exit_snapshots: &[(BasicBlockId, HashMap<String, ValueId>)],
        branch_source_block: BasicBlockId,
    ) -> Result<(), String> {
        use crate::mir::control_form::ControlKind;

        let shape = match &form.kind {
            ControlKind::Loop(shape) => shape,
            _ => return Ok(()),
        };

        let exit_id = shape.exit;
        let trace = std::env::var("NYASH_LOOPFORM_DEBUG").ok().is_some();
        if trace {
            eprintln!(
                "[loopform/exit-phi/control-form] Using ControlForm wrapper: exit={:?} branch_source={:?}",
                exit_id, branch_source_block
            );
        }

        loopform.build_exit_phis(ops, exit_id, branch_source_block, exit_snapshots)
    }
    ```
  - `src/mir/loop_builder.rs::build_loop_with_loopform` では、Exit PHI 部分をこのラッパ経由に統一し、  
    その直後で `ControlForm::Loop` の `debug_dump` / `debug_validate` を行うように整理:
    ```rust
    let loop_shape = LoopShape {
        preheader: preheader_id,
        header: header_id,
        body: body_id,
        latch: latch_id,
        exit: exit_id,
        continue_targets,
        break_targets,
    };
    let form = ControlForm::from_loop(loop_shape.clone());

    let exit_snaps = self.exit_snapshots.clone();
    crate::mir::phi_core::loopform_builder::build_exit_phis_for_control(
        &loopform,
        self,
        &form,
        &exit_snaps,
        branch_source_block,
    )?;

    if is_control_form_trace_on() {
        form.debug_dump();
        #[cfg(debug_assertions)]
        if let Some(ref func) = self.parent_builder.current_function {
            loop_shape.debug_validate(func);
        }
    }
    ```
- テスト:
  - `cargo test -q mir_loopform_exit_phi -- --nocapture`
  - `cargo test -q mir_stageb_loop_break_continue -- --nocapture`
  - いずれも PASS。Exit PHI の SSA/PHI 挙動は従来と同じで、ControlForm ベースでも構造が崩れていないことを確認。

### G‑3: ControlForm ↔ Conservative PHI の設計メモ更新（完了）

- 目的:
  - 25.1d/e/f/g の成果をまとめ、「If/Loop の PHI が最終的にどのレイヤで SSOT を持つか」を文書で固定する。
- 実施:
  - 本 README と `CURRENT_TASK.md` に、Conservative PHI Box と ControlForm レイヤの役割分担を整理して追記。
  - 25.1g は「Rust 側での Conservative PHI ↔ ControlForm 統合（If/Loop）完了」フェーズとして締める。

## このフェーズで残っていること / 先送りしたこと

- `.hako` 側 LoopSSA / Stage‑B パイプラインはまだ ControlFormBox 未統合のまま。
  - Test 3（Stage‑B MIR verify）の `%0` 問題や、BreakFinderBox 周辺の undefined ValueId は次フェーズ（LoopSSA 実装側）で扱う。
- LoopBuilder/If 降下そのものを ControlForm ベースに再構成する「Option C」は、Phase 25.2 以降の大きめリファクタとして残しておく。
