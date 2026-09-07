//! Sole physical consumer of selected Array Local release/control plans.
use super::*;
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::instruction::InvokeOperation;
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{ArrayElementWriteKind, ArrayWriteProducerKind, BasicBlock, MirType};
use std::collections::BTreeMap;

pub(in crate::mir::builder) struct ArrayLocalEmissionInput {
    pub recipe: ArrayLocalRecipeV1,
    pub frame: ValueId,
    pub homes: BTreeMap<BindingRefV1, ValueId>,
}

pub(in crate::mir::builder) fn emit_literal<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    elements: Vec<ASTNode>,
    input: ArrayLocalEmissionInput,
) -> Result<(ValueId, String, ArrayLiteralEmission), String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let ArrayLocalEmissionInput {
        recipe,
        frame,
        homes,
    } = input;
    if recipe.elements().len() != elements.len() {
        return Err(fault("recipe-child-count"));
    }
    let allocation = builder.next_value_id();
    // Both cleanup plans are bound before issuing the first operation. The
    // allocation-failure plan cannot refer to the not-yet-acquired residence.
    let allocation_fault = cleanup(builder, frame, recipe.allocation_fault(), &homes, None)?;
    let acquired_fault = cleanup(
        builder,
        frame,
        recipe.acquired_fault(),
        &homes,
        Some(allocation),
    )?;
    let allocation_control = invoke(
        builder,
        frame,
        allocation_fault.block,
        InvokeOperation::IntrinsicArrayNew,
    )?;
    builder.emit_instruction(MirInstruction::InvokeNormalResult {
        dst: allocation,
        invoke_block: allocation_control.origin,
    })?;
    let allocation_site = last_instruction(builder)?.0;
    // One-way compatibility observations for existing Local/type metadata.
    builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .insert(allocation, "ArrayBox".into());
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(allocation, MirType::Box("ArrayBox".into()));
    builder
        .comp_ctx
        .type_registry
        .record_newbox(allocation, "ArrayBox".into());
    builder
        .comp_ctx
        .type_registry
        .record_type(allocation, MirType::Box("ArrayBox".into()));
    let claim = format!("typed-array:local-literal:{}", allocation.0);
    let claim_control = invoke(
        builder,
        frame,
        acquired_fault.block,
        InvokeOperation::ArrayStateContractClaim {
            contract_id: claim.clone(),
            array: allocation,
        },
    )?;
    let mut outputs = Vec::new();
    let mut element_types = Vec::new();
    for element in elements {
        let value = drive_legacy_expression_v1(builder, port, element)?;
        let (definition_site, definition) = last_instruction(builder)?;
        let definition = definition.clone();
        element_types.push(
            builder
                .function_state
                .type_ctx
                .value_types
                .get(&value)
                .cloned(),
        );
        let write = builder.next_array_write_site_id();
        let control = invoke(
            builder,
            frame,
            acquired_fault.block,
            InvokeOperation::ArrayElementWrite {
                site_id: write,
                kind: ArrayElementWriteKind::LiteralAppend,
                producer: ArrayWriteProducerKind::Literal,
                receiver: allocation,
                index: None,
                value,
            },
        )?;
        outputs.push(ArrayElementEmission {
            value,
            definition,
            definition_site,
            write,
            control,
        });
    }
    crate::mir::builder::types::array_element::record_array_literal_elements(
        builder,
        allocation,
        &element_types,
    );
    let entry = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| fault("function-missing"))?
        .entry_block;
    Ok((
        allocation,
        claim.clone(),
        ArrayLiteralEmission {
            recipe,
            entry,
            frame,
            allocation,
            allocation_site,
            allocation_control,
            allocation_fault,
            acquired_fault,
            claim,
            claim_control,
            elements: outputs,
        },
    ))
}

fn invoke(
    builder: &mut MirBuilder,
    frame: ValueId,
    fault_block: BasicBlockId,
    operation: InvokeOperation,
) -> Result<ArrayInvokeSite, String> {
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| fault("block-missing"))?;
    let normal = builder.next_block_id();
    builder.emit_instruction(MirInstruction::Invoke {
        operation,
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: fault_block,
    })?;
    builder.start_new_block(normal)?;
    Ok(ArrayInvokeSite {
        origin,
        normal,
        fault: fault_block,
    })
}

fn cleanup(
    builder: &mut MirBuilder,
    frame: ValueId,
    roles: &[ArrayReleaseRoleV1],
    homes: &BTreeMap<BindingRefV1, ValueId>,
    incomplete: Option<ValueId>,
) -> Result<ArrayCleanupEmission, String> {
    let releases = roles
        .iter()
        .map(|role| {
            let value = match role {
                ArrayReleaseRoleV1::IncompleteResidence => incomplete,
                ArrayReleaseRoleV1::Home(binding) => homes.get(binding).copied(),
            }
            .ok_or_else(|| fault("release-binding-missing"))?;
            Ok((role.clone(), value))
        })
        .collect::<Result<Vec<_>, String>>()?;
    let block = builder.next_block_id();
    let mut body = BasicBlock::new(block);
    for (_, value) in &releases {
        body.add_instruction(MirInstruction::ArrayResidenceRelease { value: *value });
    }
    body.set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
    builder
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| fault("function-missing"))?
        .add_block(body);
    Ok(ArrayCleanupEmission { block, releases })
}
