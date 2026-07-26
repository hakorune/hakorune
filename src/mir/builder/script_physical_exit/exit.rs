use super::{LoweredScriptTerminalV1, LoweredScriptUnitPayloadV1};
use crate::mir::raw_root_body_recipe::RawScriptUnitOriginV1;
use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirInstruction, MirType, ValueId};

/// The only accepted provisional signature state before a Script physical
/// exit is prepared. Raw and canonical Script both start from this neutral
/// contract; neither supplies a route, brand, or publication identity here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ScriptPhysicalExitOpenContractV1 {
    ProvisionalUnknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ScriptPhysicalExitErrorV1 {
    MissingCurrentFunction,
    MissingCurrentBlock,
    ProvisionalSignatureMismatch { expected: MirType, actual: MirType },
    MissingBlock(BasicBlockId),
    BlockAlreadyTerminated { block: BasicBlockId },
    UndefinedOperand { value: ValueId },
    MissingOperandType { value: ValueId },
    UnknownOperandType { value: ValueId },
    ValueExpressionCannotBeVoid { value: ValueId },
    UnitOperandMustBeVoid { value: ValueId, actual: MirType },
    UnsupportedValueType { value: ValueId, actual: MirType },
    SyntheticVoidValueIdExhausted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ScriptSourceCompletionV1 {
    Value,
    Unit { origin: RawScriptUnitOriginV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ScriptPhysicalResultV1 {
    ExistingOperand { value: ValueId, ty: MirType },
    SyntheticVoid { value: ValueId },
}

/// Prepared source-to-physical completion relation. This is intentionally
/// brand-free: source completion and physical payload are sealed before Raw or
/// canonical lifecycle owners attach their own session evidence.
#[derive(Debug)]
pub(in crate::mir) struct PreparedScriptBodyCompletionV1 {
    source: ScriptSourceCompletionV1,
    physical: PreparedScriptPhysicalResultV1,
    _seal: PreparedScriptBodyCompletionSealV1,
}

#[derive(Debug)]
struct PreparedScriptBodyCompletionSealV1;

/// Completed source-to-physical completion receipt after the sole Script
/// Return/signature commit.
#[derive(Debug)]
pub(in crate::mir) struct CompletedScriptBodyCompletionV1 {
    source: ScriptSourceCompletionV1,
    physical: ScriptPhysicalResultV1,
    _seal: CompletedScriptBodyCompletionSealV1,
}

#[derive(Debug)]
struct CompletedScriptBodyCompletionSealV1;

/// A fully borrow-checked Script exit. It holds no Raw lifecycle state and
/// performs no mutation until the consuming commit below.
#[derive(Debug)]
pub(in crate::mir) struct PreparedScriptPhysicalExitCoreV1 {
    block: BasicBlockId,
    completion: PreparedScriptBodyCompletionV1,
    _seal: PreparedScriptPhysicalExitCoreSealV1,
}

#[derive(Debug)]
pub(in crate::mir) enum PreparedScriptPhysicalResultV1 {
    ExistingOperand {
        value: ValueId,
        ty: MirType,
    },
    SyntheticVoid {
        value: ValueId,
        committed_next_value_id: u32,
    },
}

#[derive(Debug)]
struct PreparedScriptPhysicalExitCoreSealV1;

/// Result receipt after the only Script physical Return/signature mutation.
/// It deliberately does not extract or close a function session.
#[derive(Debug)]
pub(in crate::mir) struct CompletedScriptPhysicalExitCoreV1 {
    block: BasicBlockId,
    completion: CompletedScriptBodyCompletionV1,
    _seal: CompletedScriptPhysicalExitCoreSealV1,
}

#[derive(Debug)]
struct CompletedScriptPhysicalExitCoreSealV1;

pub(in crate::mir) struct ScriptPhysicalExitCommitV1;

impl PreparedScriptPhysicalExitCoreV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        terminal: LoweredScriptTerminalV1,
        open: ScriptPhysicalExitOpenContractV1,
    ) -> Result<Self, ScriptPhysicalExitErrorV1> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(ScriptPhysicalExitErrorV1::MissingCurrentFunction)?;
        let block = builder
            .function_state
            .current_block
            .ok_or(ScriptPhysicalExitErrorV1::MissingCurrentBlock)?;
        let expected = match open {
            ScriptPhysicalExitOpenContractV1::ProvisionalUnknown => MirType::Unknown,
        };
        if function.signature.return_type != expected {
            return Err(ScriptPhysicalExitErrorV1::ProvisionalSignatureMismatch {
                expected,
                actual: function.signature.return_type.clone(),
            });
        }
        let block_data = function
            .get_block(block)
            .ok_or(ScriptPhysicalExitErrorV1::MissingBlock(block))?;
        if block_data.is_terminated() {
            return Err(ScriptPhysicalExitErrorV1::BlockAlreadyTerminated { block });
        }

        let (source, physical) = match terminal {
            LoweredScriptTerminalV1::Value { value } => (
                ScriptSourceCompletionV1::Value,
                Self::prepare_value_operand(builder, function, value)?,
            ),
            LoweredScriptTerminalV1::Unit {
                origin,
                payload: LoweredScriptUnitPayloadV1::ExistingVoid { value },
            } => (
                ScriptSourceCompletionV1::Unit { origin },
                Self::prepare_unit_operand(builder, function, value)?,
            ),
            LoweredScriptTerminalV1::Unit {
                origin,
                payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
            } => {
                let value = ValueId::new(function.next_value_id);
                let committed_next_value_id = function
                    .next_value_id
                    .checked_add(1)
                    .ok_or(ScriptPhysicalExitErrorV1::SyntheticVoidValueIdExhausted)?;
                (
                    ScriptSourceCompletionV1::Unit { origin },
                    PreparedScriptPhysicalResultV1::SyntheticVoid {
                        value,
                        committed_next_value_id,
                    },
                )
            }
        };
        Ok(Self {
            block,
            completion: PreparedScriptBodyCompletionV1 {
                source,
                physical,
                _seal: PreparedScriptBodyCompletionSealV1,
            },
            _seal: PreparedScriptPhysicalExitCoreSealV1,
        })
    }

    fn prepare_value_operand(
        builder: &MirBuilder,
        function: &crate::mir::MirFunction,
        value: ValueId,
    ) -> Result<PreparedScriptPhysicalResultV1, ScriptPhysicalExitErrorV1> {
        if !crate::mir::verification::utils::compute_def_blocks(function).contains_key(&value) {
            return Err(ScriptPhysicalExitErrorV1::UndefinedOperand { value });
        }
        let ty = builder
            .value_type(value)
            .cloned()
            .ok_or(ScriptPhysicalExitErrorV1::MissingOperandType { value })?;
        if ty == MirType::Unknown {
            return Err(ScriptPhysicalExitErrorV1::UnknownOperandType { value });
        }
        if ty == MirType::Void {
            return Err(ScriptPhysicalExitErrorV1::ValueExpressionCannotBeVoid { value });
        }
        if !matches!(
            ty,
            MirType::Integer | MirType::Bool | MirType::Float | MirType::String
        ) {
            return Err(ScriptPhysicalExitErrorV1::UnsupportedValueType { value, actual: ty });
        }
        Ok(PreparedScriptPhysicalResultV1::ExistingOperand { value, ty })
    }

    fn prepare_unit_operand(
        builder: &MirBuilder,
        function: &crate::mir::MirFunction,
        value: ValueId,
    ) -> Result<PreparedScriptPhysicalResultV1, ScriptPhysicalExitErrorV1> {
        if !crate::mir::verification::utils::compute_def_blocks(function).contains_key(&value) {
            return Err(ScriptPhysicalExitErrorV1::UndefinedOperand { value });
        }
        let ty = builder
            .value_type(value)
            .cloned()
            .ok_or(ScriptPhysicalExitErrorV1::MissingOperandType { value })?;
        if ty == MirType::Unknown {
            return Err(ScriptPhysicalExitErrorV1::UnknownOperandType { value });
        }
        if ty != MirType::Void {
            return Err(ScriptPhysicalExitErrorV1::UnitOperandMustBeVoid { value, actual: ty });
        }
        Ok(PreparedScriptPhysicalResultV1::ExistingOperand { value, ty })
    }
}

impl ScriptPhysicalExitCommitV1 {
    /// Sole physical Script Return/signature writer. All failure-prone checks
    /// and synthetic ValueId allocation happen in `prepare`.
    pub(in crate::mir::builder) fn commit_projected(
        builder: &mut MirBuilder,
        prepared: PreparedScriptPhysicalExitCoreV1,
    ) -> CompletedScriptPhysicalExitCoreV1 {
        let PreparedScriptPhysicalExitCoreV1 {
            block,
            completion,
            _seal: _,
        } = prepared;
        let PreparedScriptBodyCompletionV1 {
            source,
            physical,
            _seal: _,
        } = completion;
        let (physical, return_type) = match physical {
            PreparedScriptPhysicalResultV1::ExistingOperand { value, ty } => (
                ScriptPhysicalResultV1::ExistingOperand {
                    value,
                    ty: ty.clone(),
                },
                ty,
            ),
            PreparedScriptPhysicalResultV1::SyntheticVoid {
                value,
                committed_next_value_id,
            } => {
                let function = builder.function_state.current_function.as_mut().unwrap();
                function.next_value_id = committed_next_value_id;
                function
                    .get_block_mut(block)
                    .unwrap()
                    .add_instruction(MirInstruction::Const {
                        dst: value,
                        value: ConstValue::Void,
                    });
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(value, MirType::Void);
                (
                    ScriptPhysicalResultV1::SyntheticVoid { value },
                    MirType::Void,
                )
            }
        };
        let returned_value = match &physical {
            ScriptPhysicalResultV1::ExistingOperand { value, .. }
            | ScriptPhysicalResultV1::SyntheticVoid { value } => *value,
        };
        let function = builder.function_state.current_function.as_mut().unwrap();
        function.signature.return_type = return_type;
        function
            .get_block_mut(block)
            .unwrap()
            .add_instruction(MirInstruction::Return {
                value: Some(returned_value),
            });
        CompletedScriptPhysicalExitCoreV1 {
            block,
            completion: CompletedScriptBodyCompletionV1 {
                source,
                physical,
                _seal: CompletedScriptBodyCompletionSealV1,
            },
            _seal: CompletedScriptPhysicalExitCoreSealV1,
        }
    }
}

impl CompletedScriptPhysicalExitCoreV1 {
    pub(in crate::mir::builder) const fn block(&self) -> BasicBlockId {
        self.block
    }

    pub(in crate::mir::builder) const fn source(&self) -> ScriptSourceCompletionV1 {
        self.completion.source
    }

    pub(in crate::mir::builder) fn physical(&self) -> &ScriptPhysicalResultV1 {
        &self.completion.physical
    }

    pub(in crate::mir::builder) fn completion(&self) -> &CompletedScriptBodyCompletionV1 {
        &self.completion
    }
}

impl CompletedScriptBodyCompletionV1 {
    pub(in crate::mir::builder) const fn source(&self) -> ScriptSourceCompletionV1 {
        self.source
    }

    pub(in crate::mir::builder) fn physical(&self) -> &ScriptPhysicalResultV1 {
        &self.physical
    }
}

impl PreparedScriptBodyCompletionV1 {
    pub(in crate::mir::builder) const fn source(&self) -> ScriptSourceCompletionV1 {
        self.source
    }

    pub(in crate::mir::builder) fn physical(&self) -> &PreparedScriptPhysicalResultV1 {
        &self.physical
    }
}

impl PreparedScriptPhysicalExitCoreV1 {
    pub(in crate::mir::builder) fn completion(&self) -> &PreparedScriptBodyCompletionV1 {
        &self.completion
    }
}
