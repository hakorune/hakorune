use super::{canonicalize_callsites, canonicalize_for_site, CallsiteCanonicalizeScheduleSite};
use crate::ast::Span;
use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
use crate::mir::{
    BasicBlockId, Callee, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirType, UserBoxFieldDecl, ValueId,
};

mod mcl;
mod ncl;
mod ucm;

#[test]
fn schedule_facade_rejects_late_legacy_target_repair() {
    let mut module = MirModule::new("schedule_facade".to_string());
    let callee_sig = FunctionSignature {
        name: "Known.run/1".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    module.add_function(MirFunction::new(callee_sig, BasicBlockId(0)));

    let signature = FunctionSignature {
        name: "main/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::String("Known.run".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(2),
        value: crate::mir::ConstValue::Integer(7),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(3)),
        func: ValueId(1),
        callee: None,
        args: vec![ValueId(2)],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_for_site(
        &mut module,
        CallsiteCanonicalizeScheduleSite::MirCompilerPostRc,
    );
    assert_eq!(rewritten, 0);

    let inst = &module
        .get_function("main/0")
        .expect("function exists")
        .blocks
        .get(&BasicBlockId(0))
        .expect("entry block exists")
        .instructions[2];
    assert!(matches!(
        inst,
        MirInstruction::Call {
            func,
            callee: None,
            ..
        } if *func == ValueId(1)
    ));
}

#[test]
fn program_json_v0_site_does_not_issue_a_late_legacy_target() {
    let mut module = MirModule::new("program_site_policy".to_string());
    let signature = FunctionSignature {
        name: "main/0".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut func = MirFunction::new(signature, BasicBlockId(0));
    let block = func
        .blocks
        .get_mut(&BasicBlockId(0))
        .expect("entry block exists");
    block.instructions.push(MirInstruction::Const {
        dst: ValueId(1),
        value: crate::mir::ConstValue::String("Known.run/0".to_string()),
    });
    block.instruction_spans.push(Span::unknown());
    block.instructions.push(MirInstruction::Call {
        dst: Some(ValueId(2)),
        func: ValueId(1),
        callee: None,
        args: vec![],
        effects: EffectMask::PURE,
    });
    block.instruction_spans.push(Span::unknown());
    module.add_function(func);

    let rewritten = canonicalize_for_site(
        &mut module,
        CallsiteCanonicalizeScheduleSite::ProgramJsonV0Bridge,
    );
    assert_eq!(rewritten, 0);
    assert!(matches!(
        &module
            .get_function("main/0")
            .expect("function exists")
            .blocks
            .get(&BasicBlockId(0))
            .expect("entry block exists")
            .instructions[1],
        MirInstruction::Call { callee: None, .. }
    ));
}
