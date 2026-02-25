use std::collections::BTreeMap;

use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
use crate::mir::builder::control_flow::edgecfg::api::branch_stub::BranchStub;
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
use crate::mir::value_id::ValueId;

/// 条件分岐合成: `if (cond) { t } else { e }`
///
/// # Phase 280: Composition SSOT
///
/// ## Constraint (Caller Allocates)
///
/// - **Caller allocates**: `header`, `t.entry`, `e.entry`, `join_frag.entry` (`BasicBlockId`), `cond` (`ValueId`)
/// - **Caller provides**: `then_entry_args`, `else_entry_args` (`EdgeArgs`) - Phase 268 P1 SSOT
/// - **Composition wires**: `header` → `t.entry`/`e.entry` (BranchStub), `t/e.Normal` → `join_frag.entry`
///
/// ## Composition Law (Input → Output)
///
/// - `header` → `t.entry`/`e.entry` (`BranchStub` → `branches`)
/// - `t/e.Normal` → `join_frag.entry` (`EdgeStub` → `wires`)
/// - Non-Normal exits → propagate upward (`exits`)
/// - Result: `if.entry = header`, `if.exits = t/e.non-Normal + join_frag.all`
///
/// ## Invariants Preserved
///
/// - Wires/Exits separation: BranchStub in `branches`, Normal wiring in `wires`, exits `target = None`
/// - Terminator uniqueness: 1 block = 1 terminator (header gets Branch, t/e/join get Jump/Return)
/// - Entry consistency: `if.entry` is valid `BasicBlockId`
///
/// # Phase 267 P0: Branch 生成実装完了
/// - header → then/else の BranchStub を branches に追加
/// - t/e.Normal → join_frag.entry を wires に追加（内部配線）
/// - if の exits は join_frag.exits（join 以降の外へ出る exit）
///
/// # 配線ルール
/// - header → t.entry / e.entry を BranchStub として branches に追加（Phase 267 P0）
/// - t/e.Normal の EdgeStub.target = Some(join_frag.entry) → wires
/// - if の exits = t/e の非 Normal + join_frag.exits
/// - if の wires = t/e.Normal → join + t/e/join の wires
/// - if の branches = header の BranchStub + t/e/join の branches
///
/// # 引数
/// - `header`: 条件判定を行うブロック
/// - `cond`: 条件値（Phase 267 P0 で使用開始）
/// - `t`: then 分岐の断片
/// - `e`: else 分岐の断片
/// - `join_frag`: join 以降の断片（t/e.Normal の配線先 + join 以降の処理）
pub(crate) fn if_(
    header: BasicBlockId,
    cond: ValueId,       // Phase 267 P0 で使用開始
    t: Frag,             // then 分岐
    then_entry_args: EdgeArgs,  // Phase 268 P1: then entry edge-args (SSOT)
    e: Frag,             // else 分岐
    else_entry_args: EdgeArgs,  // Phase 268 P1: else entry edge-args (SSOT)
    join_frag: Frag,     // join 以降の断片
) -> Frag {
    // Phase 267 P0: header → then/else の BranchStub を作成
    let branch = BranchStub::new(
        header,
        cond,
        t.entry,
        then_entry_args,  // Phase 268 P1: caller provides
        e.entry,
        else_entry_args,  // Phase 268 P1: caller provides
    );

    let mut exits = BTreeMap::new();
    let mut wires = Vec::new();
    let mut block_params = t.block_params;
    let join_entry = join_frag.entry;

    if let Err(message) = super::merge_block_params(
        &mut block_params,
        e.block_params,
        "compose::if_/else",
    ) {
        panic!("{}", message);
    }
    if let Err(message) = super::merge_block_params(
        &mut block_params,
        join_frag.block_params,
        "compose::if_/join",
    ) {
        panic!("{}", message);
    }

    // then の全 exit を処理
    for (kind, stubs) in t.exits {
        match kind {
            ExitKind::Normal => {
                // t.Normal → join_frag.entry への配線を wires に追加
                let wired_stubs: Vec<EdgeStub> = stubs
                    .into_iter()
                    .map(|mut stub| {
                        stub.target = Some(join_entry);
                        stub
                    })
                    .collect();
                wires.extend(wired_stubs);
            }
            // Return, Unwind, Break, Continue は上位へ伝搬
            _ => {
                exits.entry(kind).or_insert_with(Vec::new).extend(stubs);
            }
        }
    }

    // then の wires をマージ
    wires.extend(t.wires);

    // else の全 exit を処理（then と同じロジック）
    for (kind, stubs) in e.exits {
        match kind {
            ExitKind::Normal => {
                let wired_stubs: Vec<EdgeStub> = stubs
                    .into_iter()
                    .map(|mut stub| {
                        stub.target = Some(join_entry);
                        stub
                    })
                    .collect();
                wires.extend(wired_stubs);
            }
            _ => {
                exits.entry(kind).or_insert_with(Vec::new).extend(stubs);
            }
        }
    }

    // else の wires をマージ
    wires.extend(e.wires);

    // join_frag の exits が if 全体の Normal exit になる
    for (kind, stubs) in join_frag.exits {
        exits.entry(kind).or_insert_with(Vec::new).extend(stubs);
    }

    // join_frag の wires もマージ
    wires.extend(join_frag.wires);

    // Phase 267 P0: branches を統合
    let mut branches = vec![branch];
    branches.extend(t.branches);
    branches.extend(e.branches);
    branches.extend(join_frag.branches);

    Frag {
        entry: header,  // if の入口は header
        block_params,
        exits,          // t/e の非 Normal + join_frag.exits
        wires,          // t/e.Normal → join_frag.entry + t/e/join の wires
        branches,       // Phase 267 P0: header の BranchStub + t/e/join の branches
    }
}

#[cfg(test)]
mod tests {
    use super::if_;
    use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
    use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
    use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
    use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::value_id::ValueId;
    use std::collections::BTreeMap;

    #[test]
    fn if_preserves_edgeargs_for_then_else_normal_exits() {
        let header = BasicBlockId::new(1);
        let then_entry = BasicBlockId::new(2);
        let else_entry = BasicBlockId::new(3);
        let join_entry = BasicBlockId::new(4);
        let cond = ValueId(10);
        let then_entry_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };
        let else_entry_args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        };
        let then_args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(20)],
        };
        let else_args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(21)],
        };

        let mut then_exits = BTreeMap::new();
        then_exits.insert(
            ExitKind::Normal,
            vec![EdgeStub::new(
                then_entry,
                ExitKind::Normal,
                None,
                then_args.clone(),
            )],
        );
        let then_frag = Frag {
            entry: then_entry,
            block_params: BTreeMap::new(),
            exits: then_exits,
            wires: vec![],
            branches: vec![],
        };

        let mut else_exits = BTreeMap::new();
        else_exits.insert(
            ExitKind::Normal,
            vec![EdgeStub::new(
                else_entry,
                ExitKind::Normal,
                None,
                else_args.clone(),
            )],
        );
        let else_frag = Frag {
            entry: else_entry,
            block_params: BTreeMap::new(),
            exits: else_exits,
            wires: vec![],
            branches: vec![],
        };

        let join_frag = Frag::new(join_entry);
        let composed = if_(
            header,
            cond,
            then_frag,
            then_entry_args,
            else_frag,
            else_entry_args,
            join_frag,
        );

        assert_eq!(composed.wires.len(), 2);
        assert!(composed.wires.iter().any(|stub| {
            stub.target == Some(join_entry) && stub.args == then_args
        }));
        assert!(composed.wires.iter().any(|stub| {
            stub.target == Some(join_entry) && stub.args == else_args
        }));
    }
}
