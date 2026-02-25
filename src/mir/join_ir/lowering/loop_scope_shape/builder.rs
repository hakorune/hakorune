//! LoopScopeShape の組み立てロジック
//!
//! LoopForm / 既存箱から LoopScopeShape を構築し、Case-A minimal ターゲットは
//! analyze_case_a パスにルーティングする。
//!
//! Trio legacy boxes は完全に除去済み。
//! LoopForm / LoopFormIntake から LoopScopeShape を構築し、変数分類と定義位置を
//! LoopScopeShape の内部に閉じ込める。
//!
//! # Phase 183-3: LoopForm-Based Construction Context
//!
//! This builder constructs LoopScopeShape from **LoopForm** during JoinIR lowering.
//! For AST-based construction (MIR building), see:
//! - `src/mir/builder/control_flow/joinir/patterns/loop_scope_shape_builder.rs`
//!
//! Both builders maintain consistent field initialization for LoopScopeShape.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::control_form::LoopId;
use crate::mir::join_ir::lowering::loop_form_intake::LoopFormIntake;
use crate::mir::loop_form::LoopForm;
use crate::mir::{BasicBlockId, MirQuery};

use super::case_a::{is_case_a_minimal_target, validate_case_a_structural};
use crate::runtime::get_global_ring0;
use super::shape::{LoopScopeShape, LoopVarClass};

impl LoopScopeShape {
    /// Case-A ルーティング込みで LoopScopeShape を構築
    pub(crate) fn from_loop_form(
        loop_form: &LoopForm,
        intake: &LoopFormIntake,
        _query: &impl MirQuery,
        func_name: Option<&str>,
    ) -> Option<Self> {
        if let Some(name) = func_name {
            if is_case_a_minimal_target(name) {
                return Self::analyze_case_a(loop_form, intake, name);
            }
        }

        Self::build_from_intake(loop_form, intake)
    }

    /// Case-A minimal 用の解析パス（Phase 48-5: 構造判定検証追加）
    fn analyze_case_a(
        loop_form: &LoopForm,
        intake: &LoopFormIntake,
        func_name: &str,
    ) -> Option<Self> {
        let result = Self::build_from_intake(loop_form, intake)?;

        // Phase 48-5: 構造判定検証（警告のみ、将来的に厳格化）
        validate_case_a_structural(loop_form, &result, func_name);

        if std::env::var("NYASH_LOOPSCOPE_DEBUG").is_ok() {
            get_global_ring0().log.debug(&format!(
                "[loopscope/case_a] {} via analyze_case_a path (pinned={}, carriers={}, exit_live={})",
                func_name,
                result.pinned.len(),
                result.carriers.len(),
                result.exit_live.len(),
            ));
        }

        Some(result)
    }

    fn build_from_intake(loop_form: &LoopForm, intake: &LoopFormIntake) -> Option<Self> {
        let layout = block_layout(loop_form);

        if std::env::var("NYASH_LOOPSCOPE_DEBUG").is_ok() {
            let loop_id = LoopId(0);
            let control = loop_form.to_control_view(loop_id);
            get_global_ring0().log.debug(&format!(
                "[loopscope/view] region.header={:?}, latches={}, exit_edges={}, control.exits={}",
                layout.header,
                loop_form.to_region_view(loop_id).latches.len(),
                loop_form.to_exit_edges(loop_id).len(),
                control.exits.len()
            ));
        }

        let pinned: BTreeSet<String> = intake.pinned_ordered.iter().cloned().collect();
        let carriers: BTreeSet<String> = intake.carrier_ordered.iter().cloned().collect();

        let variable_definitions = collect_variable_definitions(intake, &layout);
        let (body_locals, exit_live) =
            classify_body_and_exit(intake, &pinned, &carriers, &variable_definitions, &layout);

        let progress_carrier = carriers.iter().next().cloned();

        Some(Self {
            header: layout.header,
            body: layout.body,
            latch: layout.latch,
            exit: layout.exit,
            pinned,
            carriers,
            body_locals,
            exit_live,
            progress_carrier,
            variable_definitions,
        })
    }
}

#[derive(Debug, Copy, Clone)]
struct LoopBlockLayout {
    header: BasicBlockId,
    body: BasicBlockId,
    latch: BasicBlockId,
    exit: BasicBlockId,
}

fn block_layout(loop_form: &LoopForm) -> LoopBlockLayout {
    let loop_id = LoopId(0); // 単一ループの場合は 0
    let region = loop_form.to_region_view(loop_id);
    let exit_edges = loop_form.to_exit_edges(loop_id);

    let header = region.header;
    let body = loop_form.body; // body は region.blocks から推測が難しいので直接参照
    let latch = region.latches.first().copied().unwrap_or(loop_form.latch);
    let exit = exit_edges.first().map(|e| e.to).unwrap_or(loop_form.exit);

    LoopBlockLayout {
        header,
        body,
        latch,
        exit,
    }
}

fn collect_variable_definitions(
    intake: &LoopFormIntake,
    layout: &LoopBlockLayout,
) -> BTreeMap<String, BTreeSet<BasicBlockId>> {
    let mut var_defs: BTreeMap<String, BTreeSet<BasicBlockId>> = BTreeMap::new();

    for var_name in intake.header_snapshot.keys() {
        var_defs
            .entry(var_name.clone())
            .or_default()
            .insert(layout.exit);
        var_defs
            .entry(var_name.clone())
            .or_default()
            .insert(layout.header);
    }

    for (bb, snap) in &intake.exit_snapshots {
        for var_name in snap.keys() {
            var_defs.entry(var_name.clone()).or_default().insert(*bb);
        }
    }

    var_defs
}

fn classify_body_and_exit(
    intake: &LoopFormIntake,
    pinned: &BTreeSet<String>,
    carriers: &BTreeSet<String>,
    variable_definitions: &BTreeMap<String, BTreeSet<BasicBlockId>>,
    layout: &LoopBlockLayout,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut body_locals: BTreeSet<String> = BTreeSet::new();
    let mut exit_live: BTreeSet<String> = BTreeSet::new();

    let mut all_names: BTreeSet<String> = intake.header_snapshot.keys().cloned().collect();
    for (_, snap) in &intake.exit_snapshots {
        all_names.extend(snap.keys().cloned());
    }
    all_names.extend(pinned.iter().cloned());
    all_names.extend(carriers.iter().cloned());

    for name in all_names {
        let class = classify_var(
            &name,
            pinned,
            carriers,
            &intake.exit_preds,
            variable_definitions,
        );

        match class {
            LoopVarClass::Pinned | LoopVarClass::Carrier => {
                exit_live.insert(name.clone());
            }
            LoopVarClass::BodyLocalExit => {
                body_locals.insert(name.clone());
                exit_live.insert(name.clone());
            }
            LoopVarClass::BodyLocalInternal => {
                body_locals.insert(name.clone());
            }
        }
    }

    if std::env::var("NYASH_LOOPSCOPE_DEBUG").is_ok() {
        get_global_ring0().log.debug(&format!(
            "[loopscope/classify] header={:?} exit={} locals={} exit_live={}",
            layout.header,
            layout.exit.0,
            body_locals.len(),
            exit_live.len()
        ));
    }

    (body_locals, exit_live)
}

fn classify_var(
    var_name: &str,
    pinned: &BTreeSet<String>,
    carriers: &BTreeSet<String>,
    exit_preds: &[BasicBlockId],
    variable_definitions: &BTreeMap<String, BTreeSet<BasicBlockId>>,
) -> LoopVarClass {
    if var_name.starts_with("__pin$") && var_name.contains("$@") {
        return LoopVarClass::BodyLocalInternal;
    }

    if pinned.contains(var_name) {
        return LoopVarClass::Pinned;
    }

    if carriers.contains(var_name) {
        return LoopVarClass::Carrier;
    }

    if is_available_in_all(var_name, exit_preds, variable_definitions) {
        LoopVarClass::BodyLocalExit
    } else {
        LoopVarClass::BodyLocalInternal
    }
}

fn is_available_in_all(
    var_name: &str,
    required_blocks: &[BasicBlockId],
    variable_definitions: &BTreeMap<String, BTreeSet<BasicBlockId>>,
) -> bool {
    if let Some(defining_blocks) = variable_definitions.get(var_name) {
        required_blocks
            .iter()
            .all(|block| defining_blocks.contains(block))
    } else {
        false
    }
}
