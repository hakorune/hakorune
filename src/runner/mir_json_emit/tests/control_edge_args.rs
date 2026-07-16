use super::super::build_mir_json_root;
use super::super::control_edge_args::VerifiedExactNoneControlEdgeArgsV1;
use super::make_function;
use crate::mir::join_ir::lowering::inline_boundary::JumpArgsLayout;
use crate::mir::{BasicBlockId, EdgeArgs, MirInstruction, ValueId};

#[test]
fn exact_none_control_edges_emit_one_function_witness() {
    let mut function = make_function("main", true);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: None,
    });
    function.blocks.insert(
        BasicBlockId::new(1),
        crate::mir::BasicBlock::new(BasicBlockId::new(1)),
    );
    let mut module = crate::mir::MirModule::new("control_edge_args".to_string());
    module.add_function(function);

    let root = build_mir_json_root(&module).expect("exact-none MIR JSON");
    assert_eq!(
        root["functions"][0]["metadata"]["control_edge_args_v1"],
        serde_json::json!({"schema_version": 1, "mode": "exact_none"})
    );
}

#[test]
fn some_empty_jump_edge_args_rejects_the_witness_and_is_not_published() {
    let mut function = make_function("main", true);
    function
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .terminator = Some(MirInstruction::Jump {
        target: BasicBlockId::new(1),
        edge_args: Some(EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![],
        }),
    });
    let error = VerifiedExactNoneControlEdgeArgsV1::verify(&function).unwrap_err();
    assert!(error
        .to_string()
        .contains("[hmi/control-edge-args-v1/not-exact-none]"));
    assert!(error.to_string().contains("edge=jump"));

    let mut module = crate::mir::MirModule::new("control_edge_args".to_string());
    module.add_function(function);
    let root = build_mir_json_root(&module).expect("compatibility JSON remains available");
    assert!(root["functions"][0]["metadata"]
        .get("control_edge_args_v1")
        .is_none());
}

#[test]
fn either_branch_edge_args_rejects_with_exact_edge_kind() {
    for (then_args, expected) in [(true, "branch_then"), (false, "branch_else")] {
        let args = EdgeArgs {
            layout: JumpArgsLayout::CarriersOnly,
            values: vec![ValueId::new(0)],
        };
        let mut function = make_function("main", true);
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .unwrap()
            .terminator = Some(MirInstruction::Branch {
            condition: ValueId::new(0),
            then_bb: BasicBlockId::new(1),
            else_bb: BasicBlockId::new(2),
            then_edge_args: then_args.then_some(args.clone()),
            else_edge_args: (!then_args).then_some(args),
        });
        let error = VerifiedExactNoneControlEdgeArgsV1::verify(&function).unwrap_err();
        assert!(error
            .to_string()
            .contains("[hmi/control-edge-args-v1/not-exact-none]"));
        assert!(error.to_string().contains(&format!("edge={expected}")));
    }
}
