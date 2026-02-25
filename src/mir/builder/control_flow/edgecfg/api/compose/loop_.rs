use std::collections::BTreeMap;

use crate::mir::basic_block::BasicBlockId;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::control_form::LoopId;

/// ループ合成: `loop (cond) { body }`
///
/// # Phase 280: Composition SSOT
///
/// ## Constraint (Caller Allocates)
///
/// - **Caller allocates**: `loop_id` (`LoopId`), `header`, `after` (`BasicBlockId`)
/// - **Composition wires**: `Continue(loop_id)` → `header`, `Break(loop_id)` → `after`
///
/// ## Composition Law (Input → Output)
///
/// - `Continue(loop_id)` → `header` (`EdgeStub` → `wires`)
/// - `Break(loop_id)` → `after` (`EdgeStub` → `wires`)
/// - Normal/Return/Unwind → propagate upward (`exits`)
/// - Result: `loop.entry = header`, `loop.exits = Normal/Return/Unwind only` (no Break/Continue)
///
/// ## Invariants Preserved
///
/// - Wires/Exits separation: Continue/Break have `target = Some`, other exits `target = None`
/// - Terminator uniqueness: 1 block = 1 terminator (from-grouping in emit_frag)
/// - Entry consistency: `loop.entry` is valid `BasicBlockId`
///
/// # Phase 265 P2: wires/exits 分離実装完了
/// - Continue(loop_id) → header へ配線（wires へ）
/// - Break(loop_id) → after へ配線（wires へ）
/// - Normal/Return/Unwind は target = None のまま上位へ伝搬（exits へ）
///
/// # 配線ルール
/// - Continue(loop_id) の EdgeStub.target = Some(header) → wires
/// - Break(loop_id) の EdgeStub.target = Some(after) → wires
/// - その他の ExitKind は target = None（exits へ）
///
/// # 引数
/// - `loop_id`: ループ識別子（配線対象の Break/Continue 判定に使用）
/// - `header`: ループヘッダー（Continue の配線先）
/// - `after`: ループ後のブロック（Break の配線先）
/// - `body`: ループ本体の断片
#[allow(dead_code)]
pub(crate) fn loop_(
    loop_id: LoopId,
    header: BasicBlockId,
    after: BasicBlockId,
    body: Frag,
) -> Frag {
    // Phase 265 P2: exit 集合の配線処理（wires/exits 分離）
    let mut exits = BTreeMap::new();
    let mut wires = Vec::new();  // Phase 265 P2: 配線済み内部配線
    let block_params = body.block_params;

    for (kind, stubs) in body.exits {
        match kind {
            ExitKind::Continue(lid) if lid == loop_id => {
                // Continue → header へ配線（wires に追加）
                let wired: Vec<EdgeStub> = stubs
                    .into_iter()
                    .map(|mut stub| {
                        stub.target = Some(header);
                        stub
                    })
                    .collect();
                wires.extend(wired);
                // exits には入れない（内部配線）
            }
            ExitKind::Break(lid) if lid == loop_id => {
                // Break → after へ配線（wires に追加）
                let wired: Vec<EdgeStub> = stubs
                    .into_iter()
                    .map(|mut stub| {
                        stub.target = Some(after);
                        stub
                    })
                    .collect();
                wires.extend(wired);
                // exits には入れない（内部配線）
            }
            // Normal, Return, Unwind は上位へ伝搬（exits に追加）
            _ => {
                exits.insert(kind, stubs);
            }
        }
    }

    // body の wires もマージ
    wires.extend(body.wires);

    // Phase 267 P0: body の branches もマージ
    let branches = body.branches;

    Frag {
        entry: header,  // ループの入口
        block_params,
        exits,          // Normal, Return, Unwind のみ（未配線）
        wires,          // Continue → header, Break → after（配線済み）
        branches,       // Phase 267 P0: body の branches
    }
}
