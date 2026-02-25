/*!
 * RegionObserver – ControlForm から Region/Slot を観測する薄いレイヤだよ。
 *
 * - NYASH_REGION_TRACE=1 のときだけ動作し、Loop/If ごとの Region 情報を ring0 log に出力するよ。
 * - 既存の SSA/PHI 挙動には一切影響しない（読み取り専用）。
 */

use crate::mir::builder::MirBuilder;
use crate::mir::control_form::{ControlForm, ControlKind};
use crate::mir::region::{
    function_slot_registry::FunctionSlotRegistry, RefSlotKind, Region, RegionId, RegionKind,
    SlotMetadata,
};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_REGION_ID: AtomicU32 = AtomicU32::new(0);

fn is_region_trace_on() -> bool {
    std::env::var("NYASH_REGION_TRACE").ok().as_deref() == Some("1")
}

/// ControlForm と MirBuilder から Region 情報を観測・ログ出力するよ。
///
/// - 25.1l では Stage‑B 周辺のデバッグが主目的なので、
///   まずは `StageBBodyExtractorBox.*` などに絞って使う想定だよ。
pub fn observe_control_form(builder: &mut MirBuilder, form: &ControlForm) {
    if !is_region_trace_on() {
        return;
    }

    // いまのところ compilation_context が Some のケースは観測対象外にしておく。
    // （BoxCompilationContext 内の variable_map は別経路で管理されているため）
    if builder.comp_ctx.compilation_context.is_some() {
        return;
    }

    let func_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<unknown>");

    // dev 用フィルタ: Stage‑B 周辺を優先的に見る（必要なら将来拡張）。
    // ここでは "StageB" を含む関数だけログする。
    if !func_name.contains("StageB") {
        return;
    }

    let id = RegionId(NEXT_REGION_ID.fetch_add(1, Ordering::Relaxed));

    let kind = match form.kind {
        ControlKind::Loop(_) => RegionKind::Loop,
        ControlKind::If(_) => RegionKind::If,
    };

    let entry_block = form.entry;
    let exit_blocks = form.exits.clone();

    // 変数スロットは SlotRegistry があればそれを優先し、なければ
    // variable_map と value_types から best-effort で推定するよ。
    let slots: Vec<SlotMetadata> =
        if let Some(reg) = builder.comp_ctx.current_slot_registry.as_mut() {
            classify_slots_from_registry(reg)
        } else {
            classify_slots_from_variable_map(builder)
        };

    let parent = builder.metadata_ctx.current_region_stack().last().copied();

    let region = Region {
        id,
        kind,
        parent,
        entry_block,
        exit_blocks,
        slots,
    };

    get_global_ring0().log.debug(&format!(
        "[region/observe] fn={} id={:?} kind={:?} entry={:?} exits={:?} slots={:?}",
        func_name, region.id, region.kind, region.entry_block, region.exit_blocks, region.slots
    ));
}

/// 関数エントリ時の Region 観測だよ（FunctionRegion を 1 つ作ってスタックに積む）。
pub fn observe_function_region(builder: &mut MirBuilder) {
    if !is_region_trace_on() {
        return;
    }

    if builder.comp_ctx.compilation_context.is_some() {
        return;
    }

    let func_name = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.as_str())
        .unwrap_or("<unknown>");

    // まずは Stage‑B 系だけを対象にしてログ量を抑えるよ。
    if !func_name.contains("StageB") {
        return;
    }

    let id = RegionId(NEXT_REGION_ID.fetch_add(1, Ordering::Relaxed));

    let entry_block = builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.entry_block)
        .unwrap_or_else(|| crate::mir::BasicBlockId::new(0));

    let region = Region {
        id,
        kind: RegionKind::Function,
        parent: None,
        entry_block,
        exit_blocks: Vec::new(),
        slots: Vec::new(),
    };

    builder.metadata_ctx.push_region(id);

    get_global_ring0().log.debug(&format!(
        "[region/observe] fn={} id={:?} kind={:?} entry={:?} exits={:?} slots={:?}",
        func_name, region.id, region.kind, region.entry_block, region.exit_blocks, region.slots
    ));
}

/// 関数終了時に Region スタックを 1 段ポップするよ。
pub fn pop_function_region(builder: &mut MirBuilder) {
    if !is_region_trace_on() {
        return;
    }
    let _ = builder.metadata_ctx.pop_region();
}

fn classify_slots_from_registry(reg: &mut FunctionSlotRegistry) -> Vec<SlotMetadata> {
    // まず SlotRegistry 側に RefKind を埋めてもらうよ（型情報＋名前ヒューリスティック）。
    for info in reg.iter_slots().cloned().collect::<Vec<_>>() {
        if info.ref_kind.is_none() {
            let kind = info
                .ty
                .as_ref()
                .map(Region::classify_ref_kind)
                .unwrap_or_else(|| classify_slot_name_only(info.name.as_str()));
            if let Some(id) = reg.get_slot(info.name.as_str()) {
                reg.set_ref_kind(id, kind);
            }
        }
    }

    let mut out = Vec::new();
    for info in reg.iter_slots() {
        let ref_kind = info
            .ref_kind
            .unwrap_or_else(|| classify_slot_name_only(info.name.as_str()));
        out.push(SlotMetadata {
            name: info.name.clone(),
            ref_kind,
        });
    }
    out
}

fn classify_slots_from_variable_map(builder: &MirBuilder) -> Vec<SlotMetadata> {
    let mut slots = Vec::new();
    for (name, &vid) in builder.variable_ctx.variable_map().iter() {
        let ref_kind = classify_slot(builder, vid, name.as_str());
        slots.push(SlotMetadata {
            name: name.clone(),
            ref_kind,
        });
    }
    slots
}

fn classify_slot(builder: &MirBuilder, v: ValueId, name: &str) -> RefSlotKind {
    if let Some(ty) = builder.type_ctx.value_types.get(&v) {
        return Region::classify_ref_kind(ty);
    }

    // 型情報が無い場合は名前ヒューリスティックで軽く分類する（観測専用）。
    classify_slot_name_only(name)
}

fn classify_slot_name_only(name: &str) -> RefSlotKind {
    if matches!(
        name,
        "args" | "src" | "body_src" | "bundles" | "bundle_names" | "bundle_srcs" | "require_mods"
    ) {
        RefSlotKind::StrongRoot
    } else {
        RefSlotKind::NonRef
    }
}
