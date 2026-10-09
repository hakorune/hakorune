//! Disposable canonical entry for the selected Static I64 Loop.
//! Source/result receipts choose the ABI; no executable packet is admitted here.

use crate::ast::ASTNode;
use crate::mir::builder::emission::constant;
use crate::mir::builder::function_fault_frame::FunctionFaultFrameV1;
use crate::mir::builder::normal_callable_loop_source_facts::VerifiedStaticI64LoopSemanticV2;
use crate::mir::builder::raw_loop_child_entry::{
    StaticI64LoopFunctionEntryV2, StaticI64LoopTaggedPhysicalFormalV2,
};
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::function::{MirFunction, MirParamDecl};
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, VerifiedStaticLoopI64ResultSourceV1, VerifiedStaticLoopPacketSourceV1,
};
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_semantics::{
    SourceBindingSiteV1, VerifiedResolvedBlockExpressionExpectationV1,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirInstruction, MirType, ValueId};

pub(in crate::mir::builder) fn stop_after_unpublished_static_loop_entry_v1(
    builder: &mut MirBuilder,
    input: ResolvedFunctionLoweringInputV1<'_>,
    expectation: &VerifiedResolvedBlockExpressionExpectationV1,
    physical_symbol: &str,
    product: &StaticI64LoopFunctionEntryV2,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
    packet_source: &VerifiedStaticLoopPacketSourceV1,
    claims: &OrdinaryNewClaimLedgerV1,
) -> Result<(), String> {
    let completion = claims.completion_for_owner(input.owner()).ok_or_else(|| {
        "[freeze:contract][callable-loop/static-entry-completion-missing]".to_owned()
    })?;
    if !result_source.corroborates(product.semantic())
        || !result_source.matches_completion(completion)
        || !packet_source.corroborates(product.semantic().source_calls().0, formal.formal())
        || formal.formal() != product.tagged_formal()
        || formal.lane_index() != 0
    {
        return Err("[freeze:contract][callable-loop/static-entry-cohort-mismatch]".to_owned());
    }
    let if_control = VerifiedResolvedFunctionIfControlV1::empty_for_owned_loop_profile(
        input,
        product.semantic().roles().loop_site.node(),
    )?;
    let shell = prepare_shell(input, physical_symbol, formal, result_source)?;

    let mut outer = builder.open_resolved_function_draft_seal_session_v1(physical_symbol);
    let admitted = (|| {
        let draft = outer.builder_view_mut_for_lowering();
        draft
            .function_state
            .resolved_binding_state
            .install(input.function())?;
        draft.install_prepared_physical_function_skeleton(shell)?;
        let mut canonical =
            CanonicalSsaFunctionSessionV2::new_generic(input, if_control, expectation, completion)?;
        let value = canonical.adopt_exact_formal_parameter(
            draft,
            &SourceBindingSiteV1::Parameter { index: 0 },
            formal.formal(),
            formal.lane_index(),
        )?;
        verify_entry(draft, product.semantic(), formal, result_source, value)?;
        emit_unpublished_static_invoke(
            draft,
            &mut canonical,
            product.semantic(),
            packet_source,
            formal,
            value,
        )?;
        Err("[freeze:contract][callable-loop/static-i64-v2/actual-coverage-missing]".to_owned())
    })();
    outer.discard_unpublished();
    admitted
}

fn emit_unpublished_static_invoke(
    draft: &mut MirBuilder,
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    packet: &VerifiedStaticLoopPacketSourceV1,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    entry_value: ValueId,
) -> Result<(), String> {
    let entry = draft
        .function_state
        .current_block
        .ok_or_else(|| "[freeze:contract][callable-loop/static-invoke-entry-missing]".to_owned())?;
    canonical
        .identity
        .claim_variable_use_binding(packet.argument_site(), formal.formal())?;
    let actual = canonical.identity.read_entry_receipt(
        draft,
        &mut canonical.phis,
        entry,
        formal.formal(),
    )?;
    if actual.physical_block() != entry || actual.physical_value() != entry_value {
        return Err("[freeze:contract][callable-loop/static-invoke-actual-drift]".into());
    }
    let call = packet.materialize_unpublished_call(formal.formal(), actual.physical_value())?;
    let mut frame_owner = FunctionFaultFrameV1::borrowed();
    let frame = frame_owner.materialize(draft)?;
    let normal = draft.next_block_id();
    let fault = draft.next_block_id();
    let result = draft.next_value_id();
    {
        let function = draft
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| {
                "[freeze:contract][callable-loop/static-invoke-function-missing]".to_owned()
            })?;
        canonical
            .cfg
            .create_block(function, normal)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .create_block(function, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_i64_invoke(function, entry, call, frame, normal, fault)
            .map_err(|error| error.to_string())?;
        canonical
            .cfg
            .emit_invoke_fault(function, fault, frame)
            .map_err(|error| error.to_string())?;
    }
    canonical
        .cfg
        .select_block(draft, normal)
        .map_err(|error| error.to_string())?;
    draft.emit_instruction(MirInstruction::InvokeNormalResult {
        invoke_block: entry,
        dst: result,
    })?;
    draft
        .function_state
        .type_ctx
        .value_types
        .insert(result, MirType::Integer);
    let n = canonical.identity.publish_declaration_exact(
        semantic.source_calls().0.declaration(),
        semantic.roles().n_binding,
        normal,
        result,
    )?;
    let observed = canonical
        .identity
        .read_entry_receipt(draft, &mut canonical.phis, normal, n)?;
    if observed.physical_value() != result || observed.physical_block() != normal {
        return Err("[freeze:contract][callable-loop/static-invoke-n-result-drift]".into());
    }
    let bin_value = constant::emit_integer(draft, semantic.bin_initial_i64())?;
    let bin = canonical.identity.publish_declaration_exact(
        &semantic.roles().bin_declaration,
        semantic.roles().bin_binding,
        normal,
        bin_value,
    )?;
    let bin_read =
        canonical
            .identity
            .read_entry_receipt(draft, &mut canonical.phis, normal, bin)?;
    if bin_read.physical_value() != bin_value || bin_read.physical_block() != normal {
        return Err("[freeze:contract][callable-loop/static-invoke-bin-input-drift]".into());
    }
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-invoke-function-missing]".to_owned()
        })?;
    frame_owner.validate(function)?;
    Ok(())
}

fn prepare_shell(
    input: ResolvedFunctionLoweringInputV1<'_>,
    physical_symbol: &str,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
) -> Result<MirFunction, String> {
    let ASTNode::FunctionDeclaration {
        params,
        param_decls,
        return_type_name,
        uses,
        attrs,
        ..
    } = input.source().root()
    else {
        return Err("[freeze:contract][callable-loop/static-entry-source-missing]".to_owned());
    };
    let [name] = params.as_slice() else {
        return Err("[freeze:contract][callable-loop/static-entry-parameter-count]".to_owned());
    };
    let [decl] = param_decls.as_slice() else {
        return Err("[freeze:contract][callable-loop/static-entry-declaration-count]".to_owned());
    };
    if name.is_empty() || decl.name != *name || return_type_name.is_some() {
        return Err("[freeze:contract][callable-loop/static-entry-declaration-drift]".to_owned());
    }
    let mut shell = MirFunction::new(
        FunctionSignature {
            name: physical_symbol.to_owned(),
            params: vec![formal.carrier().mir_type()],
            return_type: result_source.physical_return_type(),
            // The executable effect proof is part of the later packet slice.
            // This unpublished shell uses the conservative upper bound.
            effects: EffectMask::ALL,
        },
        BasicBlockId::new(0),
    );
    shell.metadata.declared_param_decls = vec![MirParamDecl {
        name: name.clone(),
        declared_type_name: decl.declared_type_name.clone(),
        implicit_receiver: false,
    }];
    shell.metadata.declared_return_type_name = None;
    shell.metadata.physical_param_carriers = Some(vec![formal.carrier()].into_boxed_slice());
    shell.metadata.declared_capability_uses = uses.clone();
    shell.metadata.runes = attrs.runes.clone();
    Ok(shell)
}

fn verify_entry(
    draft: &MirBuilder,
    semantic: &VerifiedStaticI64LoopSemanticV2,
    formal: &StaticI64LoopTaggedPhysicalFormalV2,
    result_source: &VerifiedStaticLoopI64ResultSourceV1,
    value: ValueId,
) -> Result<(), String> {
    let function = draft
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| {
            "[freeze:contract][callable-loop/static-entry-function-missing]".to_owned()
        })?;
    if value != ValueId::new(0)
        || function.params.as_slice() != [value]
        || function.signature.params.as_slice() != [formal.carrier().mir_type()]
        || function.signature.return_type != result_source.physical_return_type()
        || function.metadata.physical_param_carriers.as_deref() != Some(&[formal.carrier()][..])
        || semantic.roles().n_binding.owner() != formal.formal().owner()
    {
        return Err("[freeze:contract][callable-loop/static-entry-physical-drift]".to_owned());
    }
    Ok(())
}
