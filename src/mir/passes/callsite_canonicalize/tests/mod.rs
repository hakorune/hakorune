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
fn schedule_facade_delegates_to_canonical_transform() {
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
    assert_eq!(rewritten, 1);

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
            callee: Some(Callee::Global(name)),
            ..
        } if *func == ValueId::INVALID && name == "Known.run/1"
    ));
}
