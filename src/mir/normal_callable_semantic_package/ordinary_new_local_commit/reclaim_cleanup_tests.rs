//! A coordinated old-caller cleanup cannot evade the retained New validator.
use super::*;
use crate::mir::instruction::{FaultFrameMode, InvokeCallResultKind};
use crate::mir::normal_callable_semantic_package::{
    ConstructionStoreRhsV1, OrdinaryNewConstructorDispositionV1,
};
use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType};

#[test]
fn reclaim_rejects_coordinated_duplicate_field_cleanup_after_healthy_control() {
    let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
        "box Leaf { value: i64 = 0 birth() { } }
         box Parent { child: Leaf = new Leaf() birth() { } }
         static box Main { main() { local parent = new Parent() return 0 } }",
    ).unwrap();
    let rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims: Vec<_> = rows.values().collect();
    let [claim] = claims.as_slice() else {
        panic!("one root claim")
    };
    let site = claim.site().clone();
    let destination = claim.home_prefix().unwrap().destination();
    let child = claim.construction().as_ref().unwrap().stores()[0].field();
    let ConstructionStoreRhsV1::ProviderConstruction {
        object: Some(child_object),
        ..
    } = claim.construction().as_ref().unwrap().stores()[0].rhs()
    else {
        panic!("owned child")
    };
    let child_object = *child_object;
    let owner_row = package
        .batch()
        .declarations()
        .find(|row| row.owner() == site.owner())
        .unwrap();
    let declaration = package
        .batch()
        .with_lowering_input(owner_row.batch_slot(), |input| {
            input
                .function()
                .expression_source()
                .initializers()
                .find(|row| row.initializer_site() == Some(site.site()))
                .unwrap()
                .declaration_site()
                .clone()
        })
        .unwrap();
    drop(claims);
    drop(rows);
    let ledger = package.ordinary_new_claim_ledger;
    ledger.register_new_root(site.owner()).unwrap();
    let claim = ledger.try_take(&site, "Parent", 0).unwrap().unwrap();
    assert!(ledger.prepare_new_emission(&claim).unwrap());
    let (_, origin) = ledger.begin_new_emission(&site).unwrap();
    let origin = origin.unwrap();
    let frame = ValueId(1);
    let result = ValueId(2);
    let local = ValueId(3);
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "root".into(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId(0),
    );
    let enter = MirInstruction::FaultFrameEnter {
        dst: frame,
        mode: FaultFrameMode::RootOwned,
    };
    let allocation = MirInstruction::Invoke {
        operation: InvokeOperation::NewBox {
            object: origin.object(),
        },
        fault_frame: frame,
        normal_landing: BasicBlockId(1),
        fault_landing: BasicBlockId(4),
    };
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: BasicBlockId(0),
        dst: result,
    };
    let OrdinaryNewConstructorDispositionV1::Birth(recipe) = claim.constructor() else {
        panic!("Birth")
    };
    let MirInstruction::Call(call) = MirInstruction::call(
        None,
        crate::mir::Callee::BirthConstructor {
            key: recipe.target_ref().clone(),
            receiver: result,
        },
        vec![],
        recipe.physical_effect_mask(),
    ) else {
        unreachable!()
    };
    let birth = MirInstruction::Invoke {
        operation: InvokeOperation::Call {
            call,
            result: InvokeCallResultKind::Unit,
        },
        fault_frame: frame,
        normal_landing: BasicBlockId(2),
        fault_landing: BasicBlockId(3),
    };
    let reclaim = MirInstruction::Invoke {
        operation: InvokeOperation::ReclaimUnpublished {
            object: origin.object(),
            value: result,
        },
        fault_frame: frame,
        normal_landing: BasicBlockId(4),
        fault_landing: BasicBlockId(5),
    };
    let mut entry = BasicBlock::new(BasicBlockId(0));
    entry.add_instruction(enter.clone());
    entry.set_terminator(allocation.clone());
    function.add_block(entry);
    let mut normal = BasicBlock::new(BasicBlockId(1));
    normal.add_instruction(projection.clone());
    normal.add_instruction(MirInstruction::Copy {
        dst: local,
        src: result,
    });
    normal.set_terminator(birth.clone());
    function.add_block(normal);
    let mut done = BasicBlock::new(BasicBlockId(2));
    done.add_instruction(MirInstruction::Const {
        dst: ValueId(4),
        value: ConstValue::Integer(0),
    });
    done.set_terminator(MirInstruction::Return {
        value: Some(ValueId(4)),
    });
    function.add_block(done);
    let mut storage = BasicBlock::new(BasicBlockId(3));
    storage.set_terminator(reclaim.clone());
    function.add_block(storage);
    for id in [4, 5] {
        let mut fault = BasicBlock::new(BasicBlockId(id));
        fault.set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
        function.add_block(fault);
    }
    ledger
        .record_new_emission(
            &site,
            result,
            Vec::new(),
            Some((origin, BasicBlockId(3), reclaim.clone())),
            vec![
                (BasicBlockId(0), enter),
                (BasicBlockId(0), allocation),
                (BasicBlockId(1), projection),
                (BasicBlockId(1), birth),
                (BasicBlockId(3), reclaim),
            ],
        )
        .unwrap();
    ledger
        .complete_new_expression(&site, "Parent", result)
        .unwrap();
    let SourceBindingSiteV1::Local { statement, ordinal } = declaration else {
        panic!("local")
    };
    ledger
        .complete_local_installation(
            site.owner(),
            statement.node(),
            &[(destination, ordinal, result, local)],
        )
        .unwrap();
    ledger
        .validate_new_emissions(site.owner(), &function)
        .expect("healthy source-issued storage-only control");

    // Restore the old caller field cleanup in both retained bindings and MIR.
    // Exact physical correspondence still holds; the ownership fence must reject.
    let release = MirInstruction::Invoke {
        operation: InvokeOperation::OwnedObjectFieldRelease {
            field: child,
            base: result,
            child: child_object,
        },
        fault_frame: frame,
        normal_landing: BasicBlockId(7),
        fault_landing: BasicBlockId(8),
    };
    let mut cleanup = BasicBlock::new(BasicBlockId(6));
    cleanup.set_terminator(release.clone());
    function.add_block(cleanup);
    let mut additions = vec![(BasicBlockId(6), release)];
    for id in [7, 8] {
        let jump = MirInstruction::Jump {
            target: BasicBlockId(3),
            edge_args: None,
        };
        let mut block = BasicBlock::new(BasicBlockId(id));
        block.set_terminator(jump.clone());
        function.add_block(block);
        additions.push((BasicBlockId(id), jump));
    }
    let birth = function
        .blocks
        .get_mut(&BasicBlockId(1))
        .unwrap()
        .terminator
        .as_mut()
        .unwrap();
    let MirInstruction::Invoke { fault_landing, .. } = birth else {
        unreachable!()
    };
    *fault_landing = BasicBlockId(6);
    let birth = birth.clone();
    {
        let mut rows = ledger.local_commits.borrow_mut();
        let NewEmissionProgress::Emitted { bindings, .. } =
            rows.get_mut(&site).unwrap().new_emission_mut().unwrap()
        else {
            unreachable!()
        };
        *bindings
            .iter_mut()
            .find(|(block, ins)| {
                *block == BasicBlockId(1) && matches!(ins, MirInstruction::Invoke { .. })
            })
            .unwrap() = (BasicBlockId(1), birth);
        bindings.extend(additions);
    }
    {
        let rows = ledger.local_commits.borrow();
        let NewEmissionProgress::Emitted { bindings, .. } =
            rows.get(&site).unwrap().new_emission().unwrap()
        else {
            unreachable!()
        };
        for (block, instruction) in bindings {
            assert!(
                super::super::physical_boundary::check_binding(
                    &function,
                    None,
                    *block,
                    instruction
                )
                .unwrap(),
                "mutant retains exact physical correspondence"
            );
        }
    }
    assert_eq!(
        ledger
            .validate_new_emissions(site.owner(), &function)
            .unwrap_err(),
        "[freeze:contract][ordinary-new/local-commit/reclaim-duplicate-field-cleanup]"
    );
}
