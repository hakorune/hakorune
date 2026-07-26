//! BODY-RETURN0: the sole Raw root exit owner.
//!
//! This module owns the open-root token, the borrow-only exit plan, and the
//! one consuming commit that creates the physical Return, signature fact,
//! completion disposition, and paired witness from the same plan.

use super::module_invocation_identity::ModuleInvocationBrandV1;
use super::root_batch_slot::{RawRootBatchSlotContractV1, RawRootBatchSlotV1};
use super::root_body_completion::{
    ActiveRootBodyCompletionTrackerV1, CompletedRootBodyV1, RootBodyCompletionErrorV1,
    RootBodyResultV1,
};
use super::script_physical_exit::{
    CompletedScriptPhysicalExitCoreV1, ScriptPhysicalResultV1, ScriptSourceCompletionV1,
};
use crate::mir::raw_root_body_recipe::{
    RawRootBodyEntryContractV1, RawRootBodyRouteV1, RawRootExitPolicyV1, RawScriptUnitOriginV1,
};
use crate::mir::{
    BasicBlockId, ConstValue, MirBuilder, MirFunction, MirInstruction, MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawProvisionalReturnV1 {
    Unknown,
    FixedVoid,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawOpenRootFunctionV1 {
    route: RawRootBodyRouteV1,
    exit: RawRootExitPolicyV1,
    provisional_return: RawProvisionalReturnV1,
    _seal: RawOpenRootFunctionSealV1,
}

#[derive(Debug)]
struct RawOpenRootFunctionSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyExitSealErrorV1 {
    RootSlotMismatch,
    FunctionStateOpen,
    BuilderBlockMissing(BasicBlockId),
    RouteContractMismatch,
    ProvisionalSignatureMismatch { expected: MirType, actual: MirType },
    MissingCurrentFunction,
    MissingCurrentBlock,
    BlockAlreadyTerminated { block: BasicBlockId },
    UndefinedReturnValue { value: ValueId },
    MissingReturnType { value: ValueId },
    UnknownReturnType { value: ValueId },
    UnsupportedReturnType { value: ValueId, actual: MirType },
    TrackerNotSealable(RootBodyCompletionErrorV1),
    ScriptExitMustUseSharedKernel,
}

#[derive(Debug)]
pub(in crate::mir::builder) enum PreparedRawRootExitPlanV1 {
    AppVoid {
        block: BasicBlockId,
        discarded_tail: Option<ValueId>,
    },
}

#[derive(Debug)]
pub(in crate::mir) struct RawRootBodyExitWitnessV1 {
    brand: ModuleInvocationBrandV1,
    route: RawRootBodyRouteV1,
    disposition: RawRootBodyExitDispositionV1,
    _seal: RawRootBodyExitWitnessSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawVmUnitOriginV1 {
    EmptyBody,
    ImplicitFallthrough,
    PrintStatement,
    LocalStatement,
    AssignmentStatement,
    CompoundAssignmentStatement,
    ExplicitVoid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawVmSourceEntryDecodeKindV1 {
    Unit {
        origin: RawVmUnitOriginV1,
        requires_void: bool,
    },
    Integer,
    Bool,
    Float,
    String,
}

#[derive(Debug)]
enum RawRootBodyExitDispositionV1 {
    ScriptValue {
        block: BasicBlockId,
        value: ValueId,
        ty: MirType,
    },
    ScriptUnitValue {
        block: BasicBlockId,
        value: ValueId,
        ty: MirType,
        origin: RawScriptUnitOriginV1,
    },
    ScriptSyntheticUnit {
        block: BasicBlockId,
        returned_void: ValueId,
        origin: RawScriptUnitOriginV1,
    },
    AppVoid {
        block: BasicBlockId,
        returned_void: ValueId,
        discarded_tail: Option<ValueId>,
    },
    LegacyUnverified,
}

#[derive(Debug)]
struct RawRootBodyExitWitnessSealV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootBodyExitWitnessErrorV1 {
    ForeignBrand,
    RouteMismatch,
    CompletionMismatch,
    MissingBlock(BasicBlockId),
    SignatureMismatch {
        expected: MirType,
        actual: MirType,
    },
    ReturnMismatch {
        expected: ValueId,
        actual: Option<ValueId>,
    },
    MissingVoidConstant(ValueId),
}

impl MirBuilder {
    pub(in crate::mir::builder) fn begin_raw_root_function_v1(
        &mut self,
        slot: RawRootBatchSlotContractV1,
        entry: RawRootBodyEntryContractV1,
    ) -> Result<RawOpenRootFunctionV1, RawRootBodyExitSealErrorV1> {
        if slot != RawRootBatchSlotV1::Main.contract() {
            return Err(RawRootBodyExitSealErrorV1::RootSlotMismatch);
        }
        if self.function_state.current_function.is_some()
            || self.function_state.current_block.is_some()
        {
            return Err(RawRootBodyExitSealErrorV1::FunctionStateOpen);
        }
        let entry_block = self.next_block_id();
        let provisional_return = match entry.exit() {
            RawRootExitPolicyV1::ScriptSourceTailOrUnit => RawProvisionalReturnV1::Unknown,
            RawRootExitPolicyV1::AppFixedVoid => RawProvisionalReturnV1::FixedVoid,
        };
        let return_type = match provisional_return {
            RawProvisionalReturnV1::Unknown => MirType::Unknown,
            RawProvisionalReturnV1::FixedVoid => MirType::Void,
        };
        let signature = crate::mir::FunctionSignature {
            name: slot.symbol().to_owned(),
            params: Vec::new(),
            return_type,
            effects: crate::mir::EffectMask::PURE,
        };
        self.function_state.current_function =
            Some(self.new_function_with_metadata(signature, entry_block));
        self.function_state.current_block = Some(entry_block);
        self.function_state.frag_emit_session.reset();
        self.comp_ctx.current_slot_registry =
            Some(crate::mir::region::function_slot_registry::FunctionSlotRegistry::new());
        self.ensure_block_exists(entry_block)
            .map_err(|_| RawRootBodyExitSealErrorV1::BuilderBlockMissing(entry_block))?;
        Ok(RawOpenRootFunctionV1 {
            route: entry.route(),
            exit: entry.exit(),
            provisional_return,
            _seal: RawOpenRootFunctionSealV1,
        })
    }

    pub(in crate::mir::builder) fn prepare_raw_root_exit_v1(
        &self,
        open: &RawOpenRootFunctionV1,
        result: RootBodyResultV1,
        tracker: &ActiveRootBodyCompletionTrackerV1,
    ) -> Result<PreparedRawRootExitPlanV1, RawRootBodyExitSealErrorV1> {
        tracker
            .prepare_seal()
            .map_err(RawRootBodyExitSealErrorV1::TrackerNotSealable)?;
        let function = self
            .function_state
            .current_function
            .as_ref()
            .ok_or(RawRootBodyExitSealErrorV1::MissingCurrentFunction)?;
        let block = self
            .function_state
            .current_block
            .ok_or(RawRootBodyExitSealErrorV1::MissingCurrentBlock)?;
        let expected = match open.provisional_return {
            RawProvisionalReturnV1::Unknown => MirType::Unknown,
            RawProvisionalReturnV1::FixedVoid => MirType::Void,
        };
        if function.signature.return_type != expected {
            return Err(RawRootBodyExitSealErrorV1::ProvisionalSignatureMismatch {
                expected,
                actual: function.signature.return_type.clone(),
            });
        }
        let block_data = function
            .get_block(block)
            .ok_or(RawRootBodyExitSealErrorV1::BuilderBlockMissing(block))?;
        if block_data.is_terminated() {
            return Err(RawRootBodyExitSealErrorV1::BlockAlreadyTerminated { block });
        }
        match (open.exit, result) {
            (RawRootExitPolicyV1::ScriptSourceTailOrUnit, _) => {
                Err(RawRootBodyExitSealErrorV1::ScriptExitMustUseSharedKernel)
            }
            (RawRootExitPolicyV1::AppFixedVoid, RootBodyResultV1::Value(value)) => {
                if !crate::mir::verification::utils::compute_def_blocks(function)
                    .contains_key(&value)
                {
                    return Err(RawRootBodyExitSealErrorV1::UndefinedReturnValue { value });
                }
                Ok(PreparedRawRootExitPlanV1::AppVoid {
                    block,
                    discarded_tail: Some(value),
                })
            }
            (RawRootExitPolicyV1::AppFixedVoid, RootBodyResultV1::NoValue) => {
                Ok(PreparedRawRootExitPlanV1::AppVoid {
                    block,
                    discarded_tail: None,
                })
            }
        }
    }

    pub(in crate::mir::builder) fn commit_raw_root_exit_v1(
        &mut self,
        open: RawOpenRootFunctionV1,
        plan: PreparedRawRootExitPlanV1,
        brand: ModuleInvocationBrandV1,
    ) -> (MirFunction, RawRootBodyExitWitnessV1) {
        let (disposition, returned_void) = match plan {
            PreparedRawRootExitPlanV1::AppVoid {
                block,
                discarded_tail,
            } => {
                let function = self.function_state.current_function.as_mut().unwrap();
                let void_value = function.next_value_id();
                function.signature.return_type = MirType::Void;
                function
                    .get_block_mut(block)
                    .unwrap()
                    .add_instruction(MirInstruction::Const {
                        dst: void_value,
                        value: ConstValue::Void,
                    });
                function
                    .get_block_mut(block)
                    .unwrap()
                    .add_instruction(MirInstruction::Return {
                        value: Some(void_value),
                    });
                (
                    RawRootBodyExitDispositionV1::AppVoid {
                        block,
                        returned_void: void_value,
                        discarded_tail,
                    },
                    Some(void_value),
                )
            }
        };
        if let Some(void_value) = returned_void {
            self.function_state
                .type_ctx
                .value_types
                .insert(void_value, MirType::Void);
        }
        let route = open.route;
        let draft = self.function_state.current_function.take().unwrap();
        self.function_state.current_block = None;
        self.comp_ctx.current_slot_registry = None;
        close_raw_root_function_state_v1(self);
        (
            draft,
            RawRootBodyExitWitnessV1 {
                brand,
                route,
                disposition,
                _seal: RawRootBodyExitWitnessSealV1,
            },
        )
    }

    /// Raw lifecycle adapter after the shared Script exit kernel has already
    /// committed the only physical Return/signature. It only consumes the
    /// current Raw session and brands the resulting witness.
    pub(in crate::mir::builder) fn commit_raw_script_exit_v1(
        &mut self,
        open: RawOpenRootFunctionV1,
        completed: CompletedScriptPhysicalExitCoreV1,
        brand: ModuleInvocationBrandV1,
    ) -> (MirFunction, RawRootBodyExitWitnessV1) {
        let block = completed.block();
        let disposition = match (completed.source(), completed.physical()) {
            (
                ScriptSourceCompletionV1::Value,
                ScriptPhysicalResultV1::ExistingOperand { value, ty },
            ) => RawRootBodyExitDispositionV1::ScriptValue {
                block,
                value: *value,
                ty: ty.clone(),
            },
            (
                ScriptSourceCompletionV1::Unit { origin },
                ScriptPhysicalResultV1::ExistingOperand { value, ty },
            ) => RawRootBodyExitDispositionV1::ScriptUnitValue {
                block,
                value: *value,
                ty: ty.clone(),
                origin,
            },
            (
                ScriptSourceCompletionV1::Unit { origin },
                ScriptPhysicalResultV1::SyntheticVoid { value },
            ) => RawRootBodyExitDispositionV1::ScriptSyntheticUnit {
                block,
                returned_void: *value,
                origin,
            },
            _ => unreachable!("shared Script exit core emitted an invalid source/physical pair"),
        };
        let route = open.route;
        let draft = self.function_state.current_function.take().unwrap();
        self.function_state.current_block = None;
        self.comp_ctx.current_slot_registry = None;
        close_raw_root_function_state_v1(self);
        (
            draft,
            RawRootBodyExitWitnessV1 {
                brand,
                route,
                disposition,
                _seal: RawRootBodyExitWitnessSealV1,
            },
        )
    }
}

fn close_raw_root_function_state_v1(builder: &mut MirBuilder) {
    builder.function_state.variable_ctx = Default::default();
    builder.function_state.type_ctx = Default::default();
    builder.function_state.binding_ctx = Default::default();
    builder.function_state.resolved_binding_state = Default::default();
    builder.function_state.scope = Default::default();
    builder.function_state.compilation = Default::default();
    builder.function_state.value_origins = Default::default();
    builder.function_state.pending_phis.clear();
    builder.function_state.local_ssa_map.clear();
    builder.function_state.schedule_mat_map.clear();
    builder.function_state.pin_slot_names.clear();
    builder.function_state.frag_emit_session.reset();
    builder.function_state.return_defer_active = false;
    builder.function_state.return_defer_slot = None;
    builder.function_state.return_defer_target = None;
    builder.function_state.return_deferred_emitted = false;
    builder.function_state.in_cleanup_block = false;
    builder.function_state.cleanup_allow_return = false;
    builder.function_state.cleanup_allow_throw = false;
    builder.function_state.suppress_pin_entry_copy_next = false;
    builder.function_state.in_unified_boxcall_fallback = false;
}

impl RawRootBodyExitWitnessV1 {
    pub(in crate::mir) fn vm_decode_plan(&self) -> Result<RawVmSourceEntryDecodeKindV1, ()> {
        match &self.disposition {
            RawRootBodyExitDispositionV1::ScriptValue { ty, .. } => match ty {
                MirType::Integer => Ok(RawVmSourceEntryDecodeKindV1::Integer),
                MirType::Bool => Ok(RawVmSourceEntryDecodeKindV1::Bool),
                MirType::Float => Ok(RawVmSourceEntryDecodeKindV1::Float),
                MirType::String => Ok(RawVmSourceEntryDecodeKindV1::String),
                MirType::Void => Ok(RawVmSourceEntryDecodeKindV1::Unit {
                    origin: RawVmUnitOriginV1::ExplicitVoid,
                    requires_void: true,
                }),
                _ => Err(()),
            },
            RawRootBodyExitDispositionV1::ScriptUnitValue { origin, .. } => {
                let origin = match origin {
                    RawScriptUnitOriginV1::EmptyBody => RawVmUnitOriginV1::EmptyBody,
                    RawScriptUnitOriginV1::VoidExpression => RawVmUnitOriginV1::ExplicitVoid,
                    RawScriptUnitOriginV1::PrintStatement => RawVmUnitOriginV1::PrintStatement,
                    RawScriptUnitOriginV1::LocalStatement => RawVmUnitOriginV1::LocalStatement,
                    RawScriptUnitOriginV1::AssignmentStatement => {
                        RawVmUnitOriginV1::AssignmentStatement
                    }
                    RawScriptUnitOriginV1::CompoundAssignmentStatement => {
                        RawVmUnitOriginV1::CompoundAssignmentStatement
                    }
                };
                Ok(RawVmSourceEntryDecodeKindV1::Unit {
                    origin,
                    requires_void: false,
                })
            }
            RawRootBodyExitDispositionV1::ScriptSyntheticUnit { origin, .. } => {
                let origin = match origin {
                    RawScriptUnitOriginV1::EmptyBody => RawVmUnitOriginV1::EmptyBody,
                    RawScriptUnitOriginV1::VoidExpression => RawVmUnitOriginV1::ExplicitVoid,
                    RawScriptUnitOriginV1::PrintStatement => RawVmUnitOriginV1::PrintStatement,
                    RawScriptUnitOriginV1::LocalStatement => RawVmUnitOriginV1::LocalStatement,
                    RawScriptUnitOriginV1::AssignmentStatement => {
                        RawVmUnitOriginV1::AssignmentStatement
                    }
                    RawScriptUnitOriginV1::CompoundAssignmentStatement => {
                        RawVmUnitOriginV1::CompoundAssignmentStatement
                    }
                };
                Ok(RawVmSourceEntryDecodeKindV1::Unit {
                    origin,
                    requires_void: true,
                })
            }
            RawRootBodyExitDispositionV1::AppVoid { .. } => {
                Ok(RawVmSourceEntryDecodeKindV1::Unit {
                    origin: RawVmUnitOriginV1::ImplicitFallthrough,
                    requires_void: true,
                })
            }
            RawRootBodyExitDispositionV1::LegacyUnverified => Err(()),
        }
    }

    pub(in crate::mir::builder) fn legacy_unverified(brand: ModuleInvocationBrandV1) -> Self {
        Self {
            brand,
            route: RawRootBodyRouteV1::Script,
            disposition: RawRootBodyExitDispositionV1::LegacyUnverified,
            _seal: RawRootBodyExitWitnessSealV1,
        }
    }

    pub(in crate::mir) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(in crate::mir) const fn route(&self) -> RawRootBodyRouteV1 {
        self.route
    }

    pub(in crate::mir::builder) fn validate(
        &self,
        draft: &MirFunction,
        completion: &CompletedRootBodyV1,
        brand: ModuleInvocationBrandV1,
    ) -> Result<(), RawRootBodyExitWitnessErrorV1> {
        if self.brand != brand {
            return Err(RawRootBodyExitWitnessErrorV1::ForeignBrand);
        }
        let (block, expected_return, expected_value, void_value) = match &self.disposition {
            RawRootBodyExitDispositionV1::ScriptValue { block, value, ty } => {
                if completion.result() != RootBodyResultV1::Value(*value) {
                    return Err(RawRootBodyExitWitnessErrorV1::CompletionMismatch);
                }
                (*block, ty.clone(), Some(*value), None)
            }
            RawRootBodyExitDispositionV1::ScriptUnitValue {
                block, value, ty, ..
            } => {
                if completion.result() != RootBodyResultV1::Value(*value) {
                    return Err(RawRootBodyExitWitnessErrorV1::CompletionMismatch);
                }
                (*block, ty.clone(), Some(*value), None)
            }
            RawRootBodyExitDispositionV1::ScriptSyntheticUnit {
                block,
                returned_void,
                ..
            } => {
                if completion.result() != RootBodyResultV1::NoValue {
                    return Err(RawRootBodyExitWitnessErrorV1::CompletionMismatch);
                }
                (*block, MirType::Void, None, Some(*returned_void))
            }
            RawRootBodyExitDispositionV1::AppVoid {
                block,
                returned_void,
                ..
            } => {
                if completion.result() != RootBodyResultV1::NoValue {
                    return Err(RawRootBodyExitWitnessErrorV1::CompletionMismatch);
                }
                (*block, MirType::Void, None, Some(*returned_void))
            }
            RawRootBodyExitDispositionV1::LegacyUnverified => {
                return Err(RawRootBodyExitWitnessErrorV1::CompletionMismatch)
            }
        };
        let route_matches = match (&self.route, &self.disposition) {
            (RawRootBodyRouteV1::Script, RawRootBodyExitDispositionV1::ScriptValue { .. })
            | (RawRootBodyRouteV1::Script, RawRootBodyExitDispositionV1::ScriptUnitValue { .. })
            | (
                RawRootBodyRouteV1::Script,
                RawRootBodyExitDispositionV1::ScriptSyntheticUnit { .. },
            )
            | (RawRootBodyRouteV1::AppMain0 { .. }, RawRootBodyExitDispositionV1::AppVoid { .. }) => {
                true
            }
            _ => false,
        };
        if !route_matches {
            return Err(RawRootBodyExitWitnessErrorV1::RouteMismatch);
        }
        let body = draft
            .get_block(block)
            .ok_or(RawRootBodyExitWitnessErrorV1::MissingBlock(block))?;
        if draft.signature.return_type != expected_return {
            return Err(RawRootBodyExitWitnessErrorV1::SignatureMismatch {
                expected: expected_return,
                actual: draft.signature.return_type.clone(),
            });
        }
        let actual = match body.terminator.as_ref() {
            Some(MirInstruction::Return { value }) => *value,
            _ => None,
        };
        if actual != expected_value.or(void_value) {
            return Err(RawRootBodyExitWitnessErrorV1::ReturnMismatch {
                expected: expected_value
                    .or(void_value)
                    .unwrap_or(ValueId::new(u32::MAX)),
                actual,
            });
        }
        if let Some(void_value) = void_value {
            let has_const = body.instructions.iter().any(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Void
                    } if *dst == void_value
                )
            });
            if !has_const {
                return Err(RawRootBodyExitWitnessErrorV1::MissingVoidConstant(
                    void_value,
                ));
            }
        }
        Ok(())
    }
}
