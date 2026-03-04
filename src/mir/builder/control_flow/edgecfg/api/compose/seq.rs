#[cfg(test)]
use std::collections::BTreeMap;

#[cfg(test)]
use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
#[cfg(test)]
use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
#[cfg(test)]
use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;

/// 順次合成: `a; b`
///
/// # Phase 280: Composition SSOT
///
/// ## Constraint (Caller Allocates)
///
/// - **Caller allocates**: `b.entry` (`BasicBlockId`)
/// - **Composition wires**: `a.Normal` → `b.entry`
///
/// ## Composition Law (Input → Output)
///
/// - `a.Normal` exits → `wires` (target = `Some(b.entry)`)
/// - Non-Normal exits (Return/Break/Continue/Unwind) → propagate upward (`exits`)
/// - Result: `seq.entry = a.entry`, `seq.exits = a.non-Normal + b.all`
///
/// ## Invariants Preserved
///
/// - Wires/Exits separation: wires have `target = Some`, exits have `target = None`
/// - Terminator uniqueness: 1 block = 1 terminator (from-grouping in emit_frag)
/// - Entry consistency: `seq.entry` is valid `BasicBlockId`
///
/// # Phase 265 P2: wires/exits 分離実装完了
/// - a.Normal → b.entry を wires に追加（内部配線）
/// - seq の exits[Normal] は b の Normal のみ（外へ出る exit）
///
/// # 配線ルール
/// - a.Normal の EdgeStub.target = Some(b.entry) → wires
/// - seq の exits = a の非 Normal + b の全 exits
/// - seq の wires = a.Normal → b.entry + a.wires + b.wires
///
/// # 引数
/// - `a`: 前段の断片
/// - `b`: 後段の断片
#[cfg(test)]
pub(crate) fn seq(a: Frag, b: Frag) -> Frag {
    let mut exits = BTreeMap::new();
    let mut wires = Vec::new();
    let mut block_params = a.block_params;
    let b_entry = b.entry;

    if let Err(message) = super::merge_block_params(
        &mut block_params,
        b.block_params,
        "compose::seq",
    ) {
        panic!("{}", message);
    }

    // a の全 exit を処理
    for (kind, stubs) in a.exits {
        match kind {
            ExitKind::Normal => {
                // a.Normal → b.entry への配線を wires に追加
                let wired_stubs: Vec<EdgeStub> = stubs
                    .into_iter()
                    .map(|mut stub| {
                        stub.target = Some(b_entry);
                        stub
                    })
                    .collect();
                wires.extend(wired_stubs);
                // exits[Normal] には入れない（内部配線）
            }
            // Return, Unwind, Break, Continue は上位へ伝搬
            _ => {
                exits.insert(kind, stubs);
            }
        }
    }

    // a の wires をマージ
    wires.extend(a.wires);

    // b の全 exit をマージ（b.Normal が seq の Normal exit になる）
    for (kind, stubs) in b.exits {
        exits.entry(kind).or_insert_with(Vec::new).extend(stubs);
    }

    // b の wires もマージ
    wires.extend(b.wires);

    // Phase 267 P0: branches もマージ
    let mut branches = Vec::new();
    branches.extend(a.branches);
    branches.extend(b.branches);

    Frag {
        entry: a.entry,  // seq の入口は a の入口
        block_params,
        exits,           // a の非 Normal + b の全 exit
        wires,           // a.Normal → b.entry + a.wires + b.wires
        branches,        // Phase 267 P0: a.branches + b.branches
    }
}

#[cfg(test)]
mod tests {
    use super::seq;
    use crate::mir::basic_block::{BasicBlockId, EdgeArgs};
    use crate::mir::builder::control_flow::edgecfg::api::block_params::BlockParams;
    use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
    use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
    use crate::mir::builder::control_flow::edgecfg::api::frag::Frag;
    use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
    use crate::mir::ValueId;
    use std::collections::BTreeMap;

    #[test]
    fn seq_preserves_edgeargs_for_normal_exit() {
        let a_entry = BasicBlockId::new(1);
        let b_entry = BasicBlockId::new(2);
        let args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(10)],
        };
        let stub = EdgeStub::new(
            a_entry,
            ExitKind::Normal,
            None,
            args.clone(),
        );
        let mut exits = BTreeMap::new();
        exits.insert(ExitKind::Normal, vec![stub]);
        let a = Frag {
            entry: a_entry,
            block_params: BTreeMap::new(),
            exits,
            wires: vec![],
            branches: vec![],
        };
        let b = Frag::new(b_entry);
        let composed = seq(a, b);
        assert_eq!(composed.wires.len(), 1);
        assert_eq!(composed.wires[0].target, Some(b_entry));
        assert_eq!(composed.wires[0].args, args);
    }

    #[test]
    fn seq_preserves_block_params() {
        let a_entry = BasicBlockId::new(1);
        let b_entry = BasicBlockId::new(2);
        let join_bb = BasicBlockId::new(3);
        let a = Frag::new(a_entry);
        let mut b = Frag::new(b_entry);
        b.block_params.insert(
            join_bb,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![ValueId(42)],
            },
        );

        let composed = seq(a, b);
        assert!(composed.block_params.contains_key(&join_bb));
    }
}
