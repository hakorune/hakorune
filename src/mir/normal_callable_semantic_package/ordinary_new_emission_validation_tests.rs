use super::*;
use crate::mir::resolved_semantics::SourceBindingSiteV1;
use crate::mir::{BasicBlock, BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature};

const INTEGER: crate::mir::ValueId = crate::mir::ValueId(10);
const BOOLEAN: crate::mir::ValueId = crate::mir::ValueId(11);
const RESULT: crate::mir::ValueId = crate::mir::ValueId(12);
const LOCAL: crate::mir::ValueId = crate::mir::ValueId(13);
const FRAME: crate::mir::ValueId = crate::mir::ValueId(14);

struct EmissionFixture {
    ledger: std::rc::Rc<OrdinaryNewClaimLedgerV1>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    function: crate::mir::MirFunction,
}

fn fixture() -> EmissionFixture {
    let package = super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth(integer, boolean) { } }
         static box Main { main() {
             local page = new Page(11, true)
             return 0
         } }",
    )
    .expect("source package");
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claims = claim_rows.values().collect::<Vec<_>>();
    let [claim] = claims.as_slice() else {
        panic!("one selected New claim");
    };
    let owner = claim.site().owner();
    let site = claim.site().clone();
    let box_source = claim.box_source().clone();
    let declaration = package
        .batch()
        .declarations()
        .find(|row| row.owner() == owner)
        .and_then(|declaration| {
            package
                .batch()
                .with_lowering_input(declaration.batch_slot(), |input| {
                    input
                        .function()
                        .expression_source()
                        .initializers()
                        .find(|row| row.initializer_site() == Some(site.site()))
                        .map(|row| (row.binding(), row.declaration_site().clone()))
                })
                .expect("same source loan")
        })
        .expect("selected local declaration");
    let target = package
        .instance_constructors
        .birth_for(&box_source, 2)
        .expect("birth lookup")
        .and_then(|row| row.published_birth_key())
        .expect("published Birth key")
        .clone();
    drop(claim_rows);
    let ledger = package.ordinary_new_claim_ledger;
    ledger.register_new_root(owner).expect("root registration");
    let claim = ledger
        .try_take(&site, "Page", 2)
        .expect("claim take")
        .expect("selected claim");
    assert!(ledger.prepare_new_emission(&claim).expect("prepare"));
    let (_, reclaim) = ledger.begin_new_emission(&site).expect("begin emission");
    let reclaim = reclaim.expect("Birth reclaim origin");
    let mut function = crate::mir::MirFunction::new(
        FunctionSignature {
            name: "new_emission_validation".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(0),
    );
    let frame = crate::mir::MirInstruction::FaultFrameEnter {
        dst: FRAME,
        mode: crate::mir::instruction::FaultFrameMode::RootOwned,
    };
    let projection = crate::mir::MirInstruction::InvokeNormalResult {
        invoke_block: BasicBlockId::new(0),
        dst: RESULT,
    };
    let birth = crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::Call(
            crate::mir::definitions::MirCall::new(
                None,
                Callee::BirthConstructor {
                    key: target,
                    receiver: RESULT,
                },
                vec![INTEGER, BOOLEAN],
            ),
        ),
        fault_frame: FRAME,
        normal_landing: BasicBlockId::new(0),
        fault_landing: BasicBlockId::new(0),
    };
    let reclaim_instruction = crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::ReclaimUnpublished {
            object: reclaim.object(),
            value: RESULT,
        },
        fault_frame: FRAME,
        normal_landing: BasicBlockId::new(0),
        fault_landing: BasicBlockId::new(0),
    };
    let mut block = BasicBlock::new(BasicBlockId::new(0));
    block.add_instruction(frame.clone());
    block.add_instruction(crate::mir::MirInstruction::Const {
        dst: INTEGER,
        value: ConstValue::Integer(11),
    });
    block.add_instruction(crate::mir::MirInstruction::Const {
        dst: BOOLEAN,
        value: ConstValue::Bool(true),
    });
    block.add_instruction(projection.clone());
    block.add_instruction(crate::mir::MirInstruction::Copy {
        dst: LOCAL,
        src: RESULT,
    });
    block.add_instruction(birth.clone());
    function.add_block(block);
    let mut reclaim_block = BasicBlock::new(BasicBlockId::new(1));
    reclaim_block.add_instruction(reclaim_instruction.clone());
    function.add_block(reclaim_block);
    ledger
        .record_new_emission(
            &site,
            RESULT,
            vec![INTEGER, BOOLEAN],
            Some((reclaim, BasicBlockId::new(1), reclaim_instruction)),
            vec![
                (BasicBlockId::new(0), frame),
                (BasicBlockId::new(0), projection),
                (BasicBlockId::new(0), birth),
            ],
        )
        .expect("record emission");
    ledger
        .complete_new_expression(&site, "Page", RESULT)
        .expect("complete expression");
    let SourceBindingSiteV1::Local { statement, ordinal } = declaration.1 else {
        panic!("local declaration");
    };
    ledger
        .complete_local_installation(
            owner,
            statement.node(),
            &[(declaration.0, ordinal, RESULT, LOCAL)],
        )
        .expect("complete local");
    EmissionFixture {
        ledger,
        owner,
        function,
    }
}

#[test]
fn ordinary_new_finalizer_rejects_literal_order_and_birth_call_drift() {
    let fixture = fixture();
    fixture
        .ledger
        .validate_new_emissions(fixture.owner, &fixture.function)
        .expect("fixture validates");

    let mut literal = fixture.function.clone();
    for instruction in &mut literal
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .instructions
    {
        if let crate::mir::MirInstruction::Const { dst, value } = instruction {
            if *dst == INTEGER {
                *value = ConstValue::Integer(12);
            }
        }
    }
    assert!(fixture
        .ledger
        .validate_new_emissions(fixture.owner, &literal)
        .unwrap_err()
        .contains("argument-literal-drift"));

    let mut order = fixture.function.clone();
    let Some(crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::Call(birth_call),
        ..
    }) = order
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .terminator
        .as_mut()
    else {
        panic!("fixture Birth call must be the block terminator");
    };
    birth_call.args.swap(0, 1);
    assert!(fixture
        .ledger
        .validate_new_emissions(fixture.owner, &order)
        .unwrap_err()
        .contains("argument-call-drift"));

    let mut call = fixture.function.clone();
    let Some(crate::mir::MirInstruction::Invoke {
        operation: crate::mir::instruction::InvokeOperation::Call(birth_call),
        ..
    }) = call
        .blocks
        .get_mut(&BasicBlockId::new(0))
        .unwrap()
        .terminator
        .as_mut()
    else {
        panic!("fixture Birth call must be the block terminator");
    };
    birth_call.args.clear();
    assert!(fixture
        .ledger
        .validate_new_emissions(fixture.owner, &call)
        .unwrap_err()
        .contains("argument-call-drift"));
}

#[test]
fn ordinary_new_finalizer_rejects_residual_emission() {
    let package = super::brand_catalog_tests::issue_with_brand_catalog(
        "box Page { birth(integer, boolean) { } }
         static box Main { main() { local page = new Page(11, true) return 0 } }",
    )
    .expect("source package");
    let claim_rows = package.ordinary_new_claim_ledger.pending_claims_for_test();
    let claim = claim_rows.values().next().expect("claim");
    let owner = claim.site().owner();
    let site = claim.site().clone();
    drop(claim_rows);
    let ledger = package.ordinary_new_claim_ledger;
    ledger.register_new_root(owner).expect("root registration");
    let claim = ledger.try_take(&site, "Page", 2).unwrap().unwrap();
    assert!(ledger.prepare_new_emission(&claim).unwrap());
    ledger.begin_new_emission(&site).expect("emission begun");
    let function = crate::mir::MirFunction::new(
        FunctionSignature {
            name: "residual_new_emission".into(),
            params: vec![],
            return_type: crate::mir::MirType::Void,
            effects: EffectMask::CONTROL,
        },
        BasicBlockId::new(1),
    );
    assert!(ledger
        .validate_new_emissions(owner, &function)
        .unwrap_err()
        .contains("emission-residual"));
}
