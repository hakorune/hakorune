/*!
 * RegionObserver – 関数の Region を観測する薄いレイヤだよ。
 *
 * - NYASH_REGION_TRACE=1 のときだけ動作し、関数の Region 情報を ring0 log に出力するよ。
 * - 既存の SSA/PHI 挙動には一切影響しない（読み取り専用）。
 */

use crate::mir::builder::MirBuilder;
use crate::mir::region::{Region, RegionId, RegionKind};
use crate::runtime::get_global_ring0;
use std::sync::atomic::{AtomicU32, Ordering};

static NEXT_REGION_ID: AtomicU32 = AtomicU32::new(0);

fn is_region_trace_on() -> bool {
    std::env::var("NYASH_REGION_TRACE").ok().as_deref() == Some("1")
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
        .current_function_name()
        .unwrap_or("<unknown>")
        .to_string();

    // まずは Stage‑B 系だけを対象にしてログ量を抑えるよ。
    if !func_name.contains("StageB") {
        return;
    }

    let id = RegionId(NEXT_REGION_ID.fetch_add(1, Ordering::Relaxed));

    let entry_block = builder
        .current_function_entry_block()
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
