//! Canonical proof for one already-defined same-block Integer operand.
//!
//! The request is deliberately neutral: it carries only an owner, a claimed
//! target block, and a physical value id supplied by a Loop receipt.  This
//! module rebinds that claim to the canonical CFG session and the actual MIR
//! definition.  It does not know Loop keys, operation classes, or ledger
//! state.

use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalOpenInstructionTargetErrorV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::type_context::TypeContext;
use crate::mir::builder::MirBuilder;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlockId, MirFunction, MirType, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalSameBlockIntegerRequestV1 {
    owner: FunctionOwnerIdV1,
    target_block: BasicBlockId,
    physical_value: ValueId,
}

impl CanonicalSameBlockIntegerRequestV1 {
    pub(in crate::mir::builder::resolved_lowering) const fn from_parts(
        owner: FunctionOwnerIdV1,
        target_block: BasicBlockId,
        physical_value: ValueId,
    ) -> Self {
        Self {
            owner,
            target_block,
            physical_value,
        }
    }

    pub(super) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) const fn target_block(self) -> BasicBlockId {
        self.target_block
    }

    pub(super) const fn physical_value(self) -> ValueId {
        self.physical_value
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) struct CanonicalPhysicalDefinitionSiteV1 {
    block: BasicBlockId,
    instruction_index: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder::resolved_lowering) enum CanonicalSameBlockIntegerRejectV1 {
    Cfg(CanonicalOpenInstructionTargetErrorV1),
    TargetOwnerMismatch,
    TargetBlockMissing,
    ParameterNotAdmitted,
    DefinitionMissing,
    DefinitionDuplicate,
    DefinitionBlockMismatch {
        expected: BasicBlockId,
        actual: BasicBlockId,
    },
    TypeUnavailable,
    TypeUnknown,
    TypeMismatch {
        found: MirType,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedCanonicalSameBlockIntegerOperandV1 {
    request: CanonicalSameBlockIntegerRequestV1,
    target: VerifiedCanonicalOpenInstructionTargetV1,
    definition: CanonicalPhysicalDefinitionSiteV1,
    _seal: CanonicalSameBlockIntegerOperandSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CanonicalSameBlockIntegerOperandSealV1;

pub(super) fn issue(
    request: CanonicalSameBlockIntegerRequestV1,
    target: VerifiedCanonicalOpenInstructionTargetV1,
    function: &MirFunction,
    type_ctx: &TypeContext,
) -> Result<VerifiedCanonicalSameBlockIntegerOperandV1, CanonicalSameBlockIntegerRejectV1> {
    if request.owner() != target.owner() || request.target_block() != target.block() {
        return Err(CanonicalSameBlockIntegerRejectV1::TargetOwnerMismatch);
    }
    let definition = issue_definition(
        function,
        type_ctx,
        request.target_block(),
        request.physical_value(),
    )?;

    Ok(VerifiedCanonicalSameBlockIntegerOperandV1 {
        request,
        target,
        definition,
        _seal: CanonicalSameBlockIntegerOperandSealV1,
    })
}

fn issue_definition(
    function: &MirFunction,
    type_ctx: &TypeContext,
    target_block: BasicBlockId,
    value: ValueId,
) -> Result<CanonicalPhysicalDefinitionSiteV1, CanonicalSameBlockIntegerRejectV1> {
    if function.get_block(target_block).is_none() {
        return Err(CanonicalSameBlockIntegerRejectV1::TargetBlockMissing);
    }
    if function.params.contains(&value) {
        return Err(CanonicalSameBlockIntegerRejectV1::ParameterNotAdmitted);
    }
    let mut definition = None;
    for block in function.block_ids() {
        let block_ref = function
            .get_block(block)
            .expect("block id came from the function");
        for (instruction_index, instruction) in block_ref.instructions.iter().enumerate() {
            if instruction.dst_value() != Some(value) {
                continue;
            }
            if definition.is_some() {
                return Err(CanonicalSameBlockIntegerRejectV1::DefinitionDuplicate);
            }
            definition = Some(CanonicalPhysicalDefinitionSiteV1 {
                block,
                instruction_index,
            });
        }
    }
    let definition = definition.ok_or(CanonicalSameBlockIntegerRejectV1::DefinitionMissing)?;
    if definition.block != target_block {
        return Err(CanonicalSameBlockIntegerRejectV1::DefinitionBlockMismatch {
            expected: target_block,
            actual: definition.block,
        });
    }
    match type_ctx.get_type(value) {
        Some(MirType::Integer) => Ok(definition),
        Some(MirType::Unknown) => Err(CanonicalSameBlockIntegerRejectV1::TypeUnknown),
        Some(found) => Err(CanonicalSameBlockIntegerRejectV1::TypeMismatch {
            found: found.clone(),
        }),
        None => Err(CanonicalSameBlockIntegerRejectV1::TypeUnavailable),
    }
}

impl<'source> super::CanonicalSsaFunctionSessionV2<'source> {
    /// Rebind one neutral Loop operand claim to this session's CFG and the
    /// unique physical MIR definition. No Loop ledger or raw-value authority
    /// is imported here.
    pub(in crate::mir::builder::resolved_lowering) fn prepare_existing_same_block_integer(
        &self,
        builder: &MirBuilder,
        request: CanonicalSameBlockIntegerRequestV1,
    ) -> Result<VerifiedCanonicalSameBlockIntegerOperandV1, CanonicalSameBlockIntegerRejectV1> {
        let function = builder.function_state.current_function.as_ref().ok_or(
            CanonicalSameBlockIntegerRejectV1::Cfg(
                CanonicalOpenInstructionTargetErrorV1::FunctionMissing,
            ),
        )?;
        let target = self
            .cfg
            .prepare_created_open_instruction_target(
                function,
                request.owner(),
                request.target_block(),
            )
            .map_err(CanonicalSameBlockIntegerRejectV1::Cfg)?;
        issue(request, target, function, &builder.function_state.type_ctx)
    }
}

impl VerifiedCanonicalSameBlockIntegerOperandV1 {
    pub(in crate::mir::builder) const fn target(self) -> VerifiedCanonicalOpenInstructionTargetV1 {
        self.target
    }

    pub(in crate::mir::builder) const fn owner(self) -> FunctionOwnerIdV1 {
        self.request.owner()
    }

    pub(in crate::mir::builder) const fn target_block(self) -> BasicBlockId {
        self.request.target_block()
    }

    pub(in crate::mir::builder) const fn physical_value(self) -> ValueId {
        self.request.physical_value()
    }

    pub(in crate::mir::builder) const fn definition_block(self) -> BasicBlockId {
        self.definition.block
    }

    pub(in crate::mir::builder) const fn definition_instruction_index(self) -> usize {
        self.definition.instruction_index
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirInstruction};

    fn function_with_blocks(count: u32) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "same_block_operand/0".to_owned(),
                params: Vec::new(),
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        for id in 1..count {
            function.add_block(BasicBlock::new(BasicBlockId::new(id)));
        }
        function
    }

    fn integer_type(value: ValueId) -> TypeContext {
        let mut type_ctx = TypeContext::new();
        type_ctx.set_type(value, MirType::Integer);
        type_ctx
    }

    #[test]
    fn unique_integer_definition_in_target_block_is_accepted() {
        let target_block = BasicBlockId::new(1);
        let value = ValueId::new(7);
        let mut function = function_with_blocks(2);
        function
            .get_block_mut(target_block)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(1),
            });

        let definition =
            issue_definition(&function, &integer_type(value), target_block, value).unwrap();
        assert_eq!(definition.block, target_block);
        assert_eq!(definition.instruction_index, 0);
    }

    #[test]
    fn definition_failures_are_typed_and_non_repairing() {
        let target_block = BasicBlockId::new(1);
        let value = ValueId::new(7);
        let mut function = function_with_blocks(3);

        assert_eq!(
            issue_definition(
                &function_with_blocks(1),
                &integer_type(value),
                target_block,
                value,
            ),
            Err(CanonicalSameBlockIntegerRejectV1::TargetBlockMissing)
        );
        assert_eq!(
            issue_definition(&function, &integer_type(value), target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::DefinitionMissing)
        );

        function
            .get_block_mut(BasicBlockId::new(0))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(1),
            });
        function
            .get_block_mut(target_block)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(2),
            });
        assert_eq!(
            issue_definition(&function, &integer_type(value), target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::DefinitionDuplicate)
        );

        let mut foreign = function_with_blocks(3);
        foreign
            .get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(1),
            });
        assert_eq!(
            issue_definition(&foreign, &integer_type(value), target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::DefinitionBlockMismatch {
                expected: target_block,
                actual: BasicBlockId::new(2),
            })
        );
    }

    #[test]
    fn parameter_and_type_failures_are_not_inferred() {
        let target_block = BasicBlockId::new(1);
        let value = ValueId::new(0);
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "same_block_operand/parameter".to_owned(),
                params: vec![MirType::Integer],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.add_block(BasicBlock::new(target_block));
        function
            .get_block_mut(target_block)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(1),
            });
        assert_eq!(
            issue_definition(&function, &integer_type(value), target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::ParameterNotAdmitted)
        );

        let value = ValueId::new(8);
        let mut typed = function_with_blocks(2);
        typed
            .get_block_mut(target_block)
            .unwrap()
            .add_instruction(MirInstruction::Const {
                dst: value,
                value: ConstValue::Integer(1),
            });
        let mut bool_ctx = TypeContext::new();
        bool_ctx.set_type(value, MirType::Bool);
        assert_eq!(
            issue_definition(&typed, &bool_ctx, target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::TypeMismatch {
                found: MirType::Bool,
            })
        );
        assert_eq!(
            issue_definition(&typed, &TypeContext::new(), target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::TypeUnavailable)
        );
        let mut unknown_ctx = TypeContext::new();
        unknown_ctx.set_type(value, MirType::Unknown);
        assert_eq!(
            issue_definition(&typed, &unknown_ctx, target_block, value),
            Err(CanonicalSameBlockIntegerRejectV1::TypeUnknown)
        );
    }
}
