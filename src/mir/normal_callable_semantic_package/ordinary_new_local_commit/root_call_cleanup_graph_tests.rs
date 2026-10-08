//! Ingress model with genuine source-derived kind; not an emission acceptance.
use super::*;
use crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog;
use crate::mir::{BasicBlock, EffectMask, FunctionSignature, MirType};

#[test]
fn direct_source_kind_ingress_rejects_mir_kind_without_matching_source_permission() {
    for nullable in [false, true] {
        let body = if nullable {
            "if size > 0 { return new Token() } return null"
        } else {
            "return new Token()"
        };
        let package = issue_with_brand_catalog(&format!("box Token {{}} box Maker {{ make(size: i64) {{ {body} }} relay() {{ return me.make(7) }} }} static box Main {{ main() {{ return 0 }} }}")).unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let (owner, exit) = ledger
            .normal_return_dispositions
            .as_ref()
            .unwrap()
            .keys()
            .next()
            .unwrap();
        let source = ledger
            .verified_direct_object_return_source_v1(*owner, exit)
            .unwrap()
            .unwrap();
        let expected = source.result();
        let projection = (
            BasicBlockId(1),
            MirInstruction::InvokeNormalResult {
                dst: ValueId(4),
                invoke_block: BasicBlockId(0),
            },
        );
        for kind in [
            InvokeCallResultKind::I64,
            InvokeCallResultKind::Handle,
            InvokeCallResultKind::NullableHandle,
        ] {
            let mut function = MirFunction::new(
                FunctionSignature {
                    name: "kind-ingress-model".into(),
                    params: vec![],
                    return_type: MirType::Box("Token".into()),
                    effects: EffectMask::PURE,
                },
                BasicBlockId(0),
            );
            let invoke = (
                BasicBlockId(0),
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call {
                        call: crate::mir::definitions::MirCall::new(
                            None,
                            crate::mir::Callee::SameModuleInstance {
                                key: source.target().target().clone(),
                                receiver: ValueId(2),
                            },
                            vec![ValueId(3)],
                        ),
                        result: kind,
                    },
                    fault_frame: ValueId(9),
                    normal_landing: BasicBlockId(1),
                    fault_landing: BasicBlockId(2),
                },
            );
            function
                .blocks
                .get_mut(&BasicBlockId(0))
                .unwrap()
                .set_terminator(invoke.1.clone());
            let bindings = vec![
                (
                    BasicBlockId(1),
                    MirInstruction::Return {
                        value: Some(ValueId(4)),
                    },
                ),
                (
                    BasicBlockId(2),
                    MirInstruction::ReturnFault {
                        fault_frame: ValueId(9),
                    },
                ),
            ];
            for (id, terminal) in &bindings {
                let mut block = BasicBlock::new(*id);
                if *id == projection.0 {
                    block.instructions.push(projection.1.clone());
                }
                block.set_terminator(terminal.clone());
                function.add_block(block);
            }
            assert_eq!(
                ingress(&function, &bindings, &invoke, &projection, expected).is_ok(),
                kind == expected
            );
        }
    }
}
