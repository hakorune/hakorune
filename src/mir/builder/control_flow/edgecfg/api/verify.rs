/*!
 * Frag 検証関数（Phase 264: Fail-Fast の置き場所）
 *
 * Phase 264 では空実装。
 * 実装適用時（Phase 265+）に検証項目を段階的に追加。
 */

use super::frag::Frag;
use std::collections::BTreeMap;

/// Frag の不変条件を厳格検証（Phase 266: strict 版、警告→Err 化）
///
/// # 検証項目
/// - wires/exits 分離契約（厳格）
///   - exits に target=Some があったら Err
///   - wires に target=None があったら Err（Return を除く）
///
/// # 戻り値
/// - Ok(()): 検証成功
/// - Err(String): 検証失敗（エラーメッセージ）
///
/// # 使用箇所
/// - Phase 266 の emit_wires() と PoC テストのみ
/// - Phase 267+ で段階的に既存コードへ適用
pub fn verify_frag_invariants_strict(frag: &Frag) -> Result<(), String> {
    use super::exit_kind::ExitKind;
    use crate::mir::control_form::LoopId;
    use crate::mir::edge_args::JumpArgsLayout;
    use crate::mir::{BasicBlockId, EdgeArgs};
    use std::collections::BTreeSet;

    debug_assert!(ExitKind::Break(LoopId(0)).is_loop_exit());
    debug_assert!(ExitKind::Continue(LoopId(0)).is_loop_exit());
    debug_assert!(ExitKind::Return.is_function_exit());
    debug_assert!(ExitKind::Unwind.is_function_exit());
    debug_assert!(!ExitKind::Cancel.is_function_exit());

    // 1. exits と wires の両方が空の場合は警告（非致命的）
    if frag.exits.is_empty() && frag.wires.is_empty() {
        #[cfg(debug_assertions)]
        {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug(&format!(
                "[verify_frag_strict] Warning: Frag entry={:?} has no exits and no wires (dead end?)",
                frag.entry
            ));
        }
    }

    // 2. exits 内に target=Some がいたら Err（厳格）
    for (kind, stubs) in &frag.exits {
        for stub in stubs {
            if stub.target.is_some() {
                return Err(format!(
                    "[verify_frag_strict] Exits[{:?}] contains wired EdgeStub (target={:?}). \
                     Wired stubs must be in wires instead.",
                    kind, stub.target
                ));
            }
        }
    }

    // 3. wires 内に target=None がいたら Err（Return を除く、厳格）
    for stub in &frag.wires {
        // Return は target 不要なので許可
        if stub.target.is_none() && !matches!(stub.kind, ExitKind::Return) {
            return Err(format!(
                "[verify_frag_strict] Wires contains unwired EdgeStub (from={:?}, kind={:?}). \
                 Wires (except Return) must have target=Some(...). This should be in exits instead.",
                stub.from, stub.kind
            ));
        }
    }

    let strict = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();

    if strict {
        if let Some(stubs) = frag.exits.get(&ExitKind::Unwind) {
            if !stubs.is_empty() {
                return Err(format!(
                    "[edgecfg/unwind] Unwind exits require catch wiring (stubs={})",
                    stubs.len()
                ));
            }
        }
    }

    if strict {
        let check_edge_args = |target: Option<BasicBlockId>,
                               args: &EdgeArgs,
                               context: &str|
         -> Result<(), String> {
            if args.layout != JumpArgsLayout::ExprResultPlusCarriers {
                return Ok(());
            }
            let target = target.ok_or_else(|| {
                format!(
                    "[verify_frag_strict] ExprResultPlusCarriers requires target block (context={})",
                    context
                )
            })?;
            let params = frag.block_params.get(&target).ok_or_else(|| {
                format!(
                    "[verify_frag_strict] Missing block_params for target {:?} (context={})",
                    target, context
                )
            })?;
            if params.layout != args.layout {
                return Err(format!(
                    "[verify_frag_strict] BlockParams layout mismatch at {:?} (context={}, block={:?}, edge={:?})",
                    target, context, params.layout, args.layout
                ));
            }
            if params.params.len() != args.values.len() {
                return Err(format!(
                    "[verify_frag_strict] BlockParams length mismatch at {:?} (context={}, params={}, args={})",
                    target,
                    context,
                    params.params.len(),
                    args.values.len()
                ));
            }
            Ok(())
        };

        for stub in &frag.wires {
            check_edge_args(stub.target, &stub.args, "wire")?;
        }
        for branch in &frag.branches {
            check_edge_args(Some(branch.then_target), &branch.then_args, "branch/then")?;
            check_edge_args(Some(branch.else_target), &branch.else_args, "branch/else")?;
        }
    }

    if strict && !frag.block_params.is_empty() {
        let mut incoming: BTreeMap<BasicBlockId, Vec<(BasicBlockId, &EdgeArgs)>> = BTreeMap::new();
        for stub in &frag.wires {
            if let Some(target) = stub.target {
                incoming
                    .entry(target)
                    .or_default()
                    .push((stub.from, &stub.args));
            }
        }
        for branch in &frag.branches {
            incoming
                .entry(branch.then_target)
                .or_default()
                .push((branch.from, &branch.then_args));
            incoming
                .entry(branch.else_target)
                .or_default()
                .push((branch.from, &branch.else_args));
        }

        for (target, params) in &frag.block_params {
            let Some(edges) = incoming.get(target) else {
                return Err(format!(
                    "[verify_frag_strict] BlockParams target {:?} has no incoming edges",
                    target
                ));
            };
            if edges.is_empty() {
                return Err(format!(
                    "[verify_frag_strict] BlockParams target {:?} has no incoming edges",
                    target
                ));
            }
            let mut seen = BTreeSet::new();
            for (pred, args) in edges {
                if !seen.insert(*pred) {
                    return Err(format!(
                        "[verify_frag_strict] Duplicate incoming edge {:?}->{:?}",
                        pred, target
                    ));
                }
                if args.layout != params.layout {
                    return Err(format!(
                        "[verify_frag_strict] BlockParams layout mismatch at {:?} (block={:?}, edge={:?})",
                        target, params.layout, args.layout
                    ));
                }
                if args.values.len() != params.params.len() {
                    return Err(format!(
                        "[verify_frag_strict] BlockParams length mismatch at {:?} (params={}, args={})",
                        target,
                        params.params.len(),
                        args.values.len()
                    ));
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::edgecfg::api::block_params::BlockParams;
    use crate::mir::builder::control_flow::edgecfg::api::edge_stub::EdgeStub;
    use crate::mir::builder::control_flow::edgecfg::api::exit_kind::ExitKind;
    use crate::mir::edge_args::JumpArgsLayout;
    use crate::mir::ValueId;
    use crate::mir::{BasicBlockId, EdgeArgs};
    use std::collections::BTreeMap;

    fn strict_env_guard() -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[("NYASH_JOINIR_STRICT", Some("1"))])
    }

    #[test]
    fn unwind_exit_is_failfast_only_in_strict() {
        let entry = BasicBlockId(1);

        let mut exits = BTreeMap::new();
        exits.insert(
            ExitKind::Unwind,
            vec![EdgeStub::new(
                entry,
                ExitKind::Unwind,
                None,
                EdgeArgs {
                    layout: JumpArgsLayout::CarriersOnly,
                    values: vec![],
                },
            )],
        );

        let frag = Frag {
            entry,
            block_params: BTreeMap::new(),
            exits,
            wires: vec![],
            branches: vec![],
        };

        // non-strict: no-op (Unwind is reserved, no wiring required yet)
        assert!(verify_frag_invariants_strict(&frag).is_ok());

        // strict: Fail-Fast with stable tag
        let _guard = strict_env_guard();
        let err = verify_frag_invariants_strict(&frag).unwrap_err();
        assert!(err.contains("[edgecfg/unwind]"), "unexpected err: {}", err);
    }

    #[test]
    fn strict_requires_block_params_for_expr_result_layout() {
        let _guard = strict_env_guard();
        let entry = BasicBlockId(1);
        let target = BasicBlockId(2);
        let args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(10), ValueId(11)],
        };

        let frag = Frag {
            entry,
            block_params: BTreeMap::new(),
            exits: BTreeMap::new(),
            wires: vec![EdgeStub::new(entry, ExitKind::Normal, Some(target), args)],
            branches: vec![],
        };

        let err = verify_frag_invariants_strict(&frag).unwrap_err();
        assert!(
            err.contains("Missing block_params"),
            "unexpected err: {}",
            err
        );
    }

    #[test]
    fn strict_accepts_matching_block_params_layout_and_len() {
        let _guard = strict_env_guard();
        let entry = BasicBlockId(1);
        let target = BasicBlockId(2);
        let args = EdgeArgs {
            layout: JumpArgsLayout::ExprResultPlusCarriers,
            values: vec![ValueId(10), ValueId(11)],
        };

        let mut block_params = BTreeMap::new();
        block_params.insert(
            target,
            BlockParams {
                layout: JumpArgsLayout::ExprResultPlusCarriers,
                params: vec![ValueId(100), ValueId(101)],
            },
        );

        let frag = Frag {
            entry,
            block_params,
            exits: BTreeMap::new(),
            wires: vec![EdgeStub::new(entry, ExitKind::Normal, Some(target), args)],
            branches: vec![],
        };

        assert!(verify_frag_invariants_strict(&frag).is_ok());
    }
}
