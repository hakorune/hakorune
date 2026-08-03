//! Caller-zero DirectAccum physicalizer.
//!
//! This is the first consumer of a verified Recipe/JoinSig pair. It uses the
//! existing canonical CFG, function-owned Binding SSA, and one PhiTxn; it
//! does not select routes or publish a candidate module.

pub(in crate::mir::builder) use super::loop_accum_binding_port::DirectAccumBindingPortV1;
#[cfg(test)]
pub(in crate::mir::builder) use super::loop_accum_binding_port::RawDirectAccumBindingPort;
#[cfg(test)]
use super::loop_accum_caller_zero::{abort_caller_zero, finish_caller_zero};
use super::loop_physical_input::{
    LoopPhysicalInputRejectV1, LoopPhysicalRoleV1, VerifiedLoopBindingProjectionV1,
    VerifiedLoopInputProjectionV1, VerifiedLoopPhysicalRolePlanV1,
};
#[cfg(test)]
use crate::mir::builder::emission::phi_lifecycle::PhiToken;
use crate::mir::builder::emission::{loop_operation, phi_lifecycle::PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::{
    CanonicalCfgErrorV1, CanonicalCfgSessionV1,
};
#[cfg(test)]
use crate::mir::builder::ssa::binding::BindingSsaBuilderV1;
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopBlockKeyV1, LoopCompareI64OpV1, LoopConditionV1, LoopJoinEdgeRoleV1,
    LoopOperationV1, LoopRecipeItemV1, VerifiedLoopPhysicalInputV1,
};
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::{BasicBlockId, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopResultDispositionV1 {
    Unit,
    Value(ValueId),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhysicalSuccessReceiptV1 {
    pub(in crate::mir::builder) final_values:
        Box<[(crate::mir::loop_recipe_contract::LoopBindingKeyV1, ValueId)]>,
    pub(in crate::mir::builder) result: LoopResultDispositionV1,
}

/// Production handoff.  The loop has been emitted, but `After` remains the
/// caller's open continuation block; no function return or final-value
/// snapshot is performed here.
#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct LoopPhysicalContinuationReceiptV1 {
    pub(in crate::mir::builder) continuation_block: BasicBlockId,
    pub(in crate::mir::builder) result: LoopResultDispositionV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir::builder) enum LoopPhysicalizeErrorV1 {
    Input(LoopPhysicalInputRejectV1),
    MissingFunction,
    CurrentBlockMismatch {
        expected: BasicBlockId,
        actual: Option<BasicBlockId>,
    },
    PreheaderTerminated(BasicBlockId),
    ExistingPhysicalBlock(BasicBlockId),
    RecipeShape(&'static str),
    MissingValue(crate::mir::loop_recipe_contract::LoopValueKeyV1),
    MissingBinding(crate::mir::loop_recipe_contract::LoopBindingKeyV1),
    Operation(String),
    Cfg(CanonicalCfgErrorV1),
    Ssa(String),
    PhiAbort(String),
    #[cfg(test)]
    InjectedTestFailure(DirectAccumPhysicalizerTestFailurePointV1),
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum DirectAccumPhysicalizerTestFailurePointV1 {
    AfterHeaderCondition,
}

#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let owner = bindings.owner();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::<PhiToken>::new(owner);
    let mut port = RawDirectAccumBindingPort { ssa: &mut ssa };
    let mut phis = Some(PhiTxn::begin("loop_direct_accum_physicalizer"));
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        &mut cfg,
        &mut port,
        phis.as_mut().expect("caller-zero PHI transaction"),
    )?;
    let result = session.emit();
    match result {
        Ok(receipt) => finish_caller_zero(builder, cfg, ssa, phis, receipt),
        Err(error) => abort_caller_zero(builder, &mut phis, error),
    }
}

#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_with_test_failure(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    failure: DirectAccumPhysicalizerTestFailurePointV1,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let owner = bindings.owner();
    let mut cfg = CanonicalCfgSessionV1::new();
    let mut ssa = BindingSsaBuilderV1::<PhiToken>::new(owner);
    let mut port = RawDirectAccumBindingPort { ssa: &mut ssa };
    let mut phis = Some(PhiTxn::begin("loop_direct_accum_physicalizer"));
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        &mut cfg,
        &mut port,
        phis.as_mut().expect("caller-zero PHI transaction"),
    )?;
    session.failure = Some(failure);
    let result = session.emit();
    match result {
        Ok(receipt) => finish_caller_zero(builder, cfg, ssa, phis, receipt),
        Err(error) => abort_caller_zero(builder, &mut phis, error),
    }
}

/// Borrowing production seam. The caller owns the function-wide CFG/SSA/PHI
/// services and decides when they are finished or committed.
#[cfg(test)]
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_borrowing(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: &mut CanonicalCfgSessionV1,
    ssa: &mut BindingSsaBuilderV1<PhiToken>,
    phis: &mut Option<PhiTxn>,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let mut port = RawDirectAccumBindingPort { ssa };
    let transaction = phis.as_mut().expect("borrowed PHI transaction");
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder,
        input,
        bindings,
        inputs,
        roles,
        cfg,
        &mut port,
        transaction,
    )?;
    let result = session.emit();
    match result {
        Ok(receipt) => Ok(receipt),
        Err(error) => abort_caller_zero(builder, phis, error),
    }
}

/// Generic production entrypoint.  The caller supplies the existing
/// function-owned identity/SSA port; this helper never creates an owner.
pub(in crate::mir::builder) fn physicalize_direct_accum_v1_with_port<P>(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: &mut CanonicalCfgSessionV1,
    port: &mut P,
    phis: &mut PhiTxn,
) -> Result<LoopPhysicalContinuationReceiptV1, LoopPhysicalizeErrorV1>
where
    P: DirectAccumBindingPortV1,
{
    let mut session = DirectAccumPhysicalizerV1::preflight(
        builder, input, bindings, inputs, roles, cfg, port, phis,
    )?;
    session.emit_inline()
}

struct DirectAccumPhysicalizerV1<'builder, 'owners, P> {
    builder: &'builder mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: &'owners mut CanonicalCfgSessionV1,
    port: &'owners mut P,
    phis: &'owners mut PhiTxn,
    values: BTreeMap<crate::mir::loop_recipe_contract::LoopValueKeyV1, ValueId>,
    #[cfg(test)]
    failure: Option<DirectAccumPhysicalizerTestFailurePointV1>,
}

impl<'builder, 'owners, P> DirectAccumPhysicalizerV1<'builder, 'owners, P>
where
    P: DirectAccumBindingPortV1,
{
    fn preflight(
        builder: &'builder mut MirBuilder,
        input: VerifiedLoopPhysicalInputV1,
        bindings: VerifiedLoopBindingProjectionV1,
        inputs: VerifiedLoopInputProjectionV1,
        roles: VerifiedLoopPhysicalRolePlanV1,
        cfg: &'owners mut CanonicalCfgSessionV1,
        port: &'owners mut P,
        phis: &'owners mut PhiTxn,
    ) -> Result<Self, LoopPhysicalizeErrorV1> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        let preheader = roles.block(LoopPhysicalRoleV1::Preheader);
        if inputs.preheader() != preheader {
            return Err(LoopPhysicalizeErrorV1::CurrentBlockMismatch {
                expected: preheader,
                actual: Some(inputs.preheader()),
            });
        }
        if builder.function_state.current_block != Some(preheader) {
            return Err(LoopPhysicalizeErrorV1::CurrentBlockMismatch {
                expected: preheader,
                actual: builder.function_state.current_block,
            });
        }
        let preheader_block =
            function
                .get_block(preheader)
                .ok_or(LoopPhysicalizeErrorV1::Input(
                    LoopPhysicalInputRejectV1::MissingPhysicalRole(LoopPhysicalRoleV1::Preheader),
                ))?;
        if preheader_block.is_terminated() {
            return Err(LoopPhysicalizeErrorV1::PreheaderTerminated(preheader));
        }
        for role in [
            LoopPhysicalRoleV1::Header,
            LoopPhysicalRoleV1::Body,
            LoopPhysicalRoleV1::Step,
            LoopPhysicalRoleV1::After,
        ] {
            if function.get_block(roles.block(role)).is_some() {
                return Err(LoopPhysicalizeErrorV1::ExistingPhysicalBlock(
                    roles.block(role),
                ));
            }
        }
        let recipe = input.recipe().as_recipe();
        if recipe
            .inputs
            .iter()
            .any(|value| inputs.binding_for(*value).is_none())
        {
            return Err(LoopPhysicalizeErrorV1::RecipeShape(
                "missing recipe input projection",
            ));
        }
        for value in recipe.inputs.iter().copied() {
            let (_, physical) = inputs.binding_for(value).expect("preflight checked");
            if function.defined_values().contains(&physical)
                && builder.function_state.type_ctx.get_type(physical) == Some(&MirType::Integer)
            {
                continue;
            }
            return Err(LoopPhysicalizeErrorV1::MissingValue(value));
        }
        validate_direct_shape(&input)?;
        Ok(Self {
            builder,
            port,
            phis,
            cfg,
            input,
            bindings,
            inputs,
            roles,
            values: BTreeMap::new(),
            #[cfg(test)]
            failure: None,
        })
    }

    fn emit(&mut self) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
        let continuation = self.emit_inline()?;
        self.close_after()?;
        let final_values = self.capture_final_values()?;
        Ok(LoopPhysicalSuccessReceiptV1 {
            final_values,
            result: continuation.result,
        })
    }

    fn emit_inline(&mut self) -> Result<LoopPhysicalContinuationReceiptV1, LoopPhysicalizeErrorV1> {
        self.create_blocks()?;
        self.seed_inputs()?;
        self.emit_cfg_and_operations_inline()?;
        Ok(LoopPhysicalContinuationReceiptV1 {
            continuation_block: self.roles.block(LoopPhysicalRoleV1::After),
            result: LoopResultDispositionV1::Unit,
        })
    }

    fn create_blocks(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let block_ids = [
            self.roles.block(LoopPhysicalRoleV1::Header),
            self.roles.block(LoopPhysicalRoleV1::Body),
            self.roles.block(LoopPhysicalRoleV1::Step),
            self.roles.block(LoopPhysicalRoleV1::After),
        ];
        let cfg = &self.cfg;
        let builder = &mut *self.builder;
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        for block in block_ids {
            cfg.create_block(function, block)
                .map_err(LoopPhysicalizeErrorV1::Cfg)?;
        }
        Ok(())
    }

    fn seed_inputs(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        for value in self.input.recipe().as_recipe().inputs.iter().copied() {
            let (key, physical) = self
                .inputs
                .binding_for(value)
                .ok_or(LoopPhysicalizeErrorV1::MissingValue(value))?;
            let binding = self
                .bindings
                .resolve(key)
                .map_err(LoopPhysicalizeErrorV1::Input)?;
            let preheader = self.roles.block(LoopPhysicalRoleV1::Preheader);
            let builder = &mut *self.builder;
            self.port
                .seed_input(builder, binding, preheader, physical)
                .map_err(LoopPhysicalizeErrorV1::Ssa)?;
            self.values.insert(value, physical);
        }
        Ok(())
    }

    fn emit_cfg_and_operations_inline(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let preheader = self.roles.block(LoopPhysicalRoleV1::Preheader);
        let header = self.roles.block(LoopPhysicalRoleV1::Header);
        let body = self.roles.block(LoopPhysicalRoleV1::Body);
        let step = self.roles.block(LoopPhysicalRoleV1::Step);
        let after = self.roles.block(LoopPhysicalRoleV1::After);
        self.emit_jump(preheader, header)?;
        self.seal(preheader)?;
        self.select(header)?;
        self.emit_header_carriers()?;
        let (condition_block, body_block) = self.direct_recipe_blocks()?;
        let condition = self.block_operations(condition_block)?;
        let condition_result = self.emit_operations(&condition)?;
        #[cfg(test)]
        self.inject_failure_if_requested(
            DirectAccumPhysicalizerTestFailurePointV1::AfterHeaderCondition,
        )?;
        self.emit_branch(header, condition_result, body, after)?;
        self.select(body)?;
        self.seal(body)?;
        let body_operations = self.block_operations(body_block)?;
        self.emit_operations(&body_operations)?;
        self.emit_jump(body, step)?;
        self.select(step)?;
        self.seal(step)?;
        self.emit_jump(step, header)?;
        self.select(header)?;
        self.seal(header)?;
        self.select(after)?;
        Ok(())
    }

    fn close_after(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let after = self.roles.block(LoopPhysicalRoleV1::After);
        self.select(after)?;
        self.emit_return(after)?;
        self.seal(after)
    }

    fn emit_header_carriers(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let carriers = self
            .input
            .join_sig()
            .as_sig()
            .loops
            .first()
            .ok_or(LoopPhysicalizeErrorV1::RecipeShape("missing root JoinSig"))?;
        let carriers = carriers.carriers.clone();
        for carrier in carriers.iter() {
            let binding = self
                .bindings
                .resolve(carrier.binding)
                .map_err(LoopPhysicalizeErrorV1::Input)?;
            let value = self.read(binding, self.roles.block(LoopPhysicalRoleV1::Header))?;
            loop_operation::publish_i64_value(self.builder_mut(), value)
                .map_err(LoopPhysicalizeErrorV1::Operation)?;
        }
        Ok(())
    }

    fn direct_recipe_blocks(
        &self,
    ) -> Result<(LoopBlockKeyV1, LoopBlockKeyV1), LoopPhysicalizeErrorV1> {
        let node = self.input.recipe().as_recipe().loops.first().ok_or(
            LoopPhysicalizeErrorV1::RecipeShape("missing DirectAccum loop"),
        )?;
        let condition = match node.condition {
            LoopConditionV1::Predicate { block, .. } => block,
            LoopConditionV1::Always => {
                return Err(LoopPhysicalizeErrorV1::RecipeShape(
                    "DirectAccum requires predicate",
                ))
            }
        };
        Ok((condition, node.body))
    }

    fn block_operations(
        &self,
        block_key: LoopBlockKeyV1,
    ) -> Result<Box<[LoopOperationV1]>, LoopPhysicalizeErrorV1> {
        let recipe = self.input.recipe().as_recipe();
        let block = recipe
            .blocks
            .iter()
            .find(|block| block.key == block_key)
            .ok_or(LoopPhysicalizeErrorV1::RecipeShape(
                "missing DirectAccum block",
            ))?;
        block
            .items
            .iter()
            .map(|item_key| {
                let item = recipe
                    .items
                    .iter()
                    .find(|row| row.key == *item_key)
                    .ok_or(LoopPhysicalizeErrorV1::RecipeShape("missing recipe item"))?;
                match item.item {
                    LoopRecipeItemV1::Operation { operation } => Ok(operation),
                    _ => Err(LoopPhysicalizeErrorV1::RecipeShape(
                        "DirectAccum block contains control item",
                    )),
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
    }

    fn emit_operations(
        &mut self,
        operations: &[LoopOperationV1],
    ) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        let mut last = None;
        for operation in operations.iter().copied() {
            match operation {
                LoopOperationV1::ReadBinding { binding, result } => {
                    let recipe_binding = binding;
                    let binding = self
                        .bindings
                        .resolve(recipe_binding)
                        .map_err(LoopPhysicalizeErrorV1::Input)?;
                    let block = self.current_block()?;
                    let value = self.read_effect(recipe_binding, binding, block)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::ConstI64 { result, value } => {
                    let value = loop_operation::emit_const_i64(self.builder_mut(), value)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::BinaryI64 {
                    op,
                    left,
                    right,
                    result,
                } => {
                    if !matches!(op, LoopBinaryI64OpV1::Add) {
                        return Err(LoopPhysicalizeErrorV1::RecipeShape(
                            "DirectAccum requires add",
                        ));
                    }
                    let lhs = self.value(left)?;
                    let rhs = self.value(right)?;
                    let value = loop_operation::emit_add_i64(self.builder_mut(), lhs, rhs)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::CompareI64 {
                    op,
                    left,
                    right,
                    result,
                } => {
                    if !matches!(op, LoopCompareI64OpV1::Less) {
                        return Err(LoopPhysicalizeErrorV1::RecipeShape(
                            "DirectAccum requires less",
                        ));
                    }
                    let lhs = self.value(left)?;
                    let rhs = self.value(right)?;
                    let value = loop_operation::emit_less_i64(self.builder_mut(), lhs, rhs)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::WriteBinding { binding, value } => {
                    let recipe_binding = binding;
                    let binding = self
                        .bindings
                        .resolve(recipe_binding)
                        .map_err(LoopPhysicalizeErrorV1::Input)?;
                    let value = self.value(value)?;
                    let block = self.current_block()?;
                    self.write_effect(recipe_binding, binding, block, value)?;
                    last = Some(value);
                }
            }
        }
        last.ok_or(LoopPhysicalizeErrorV1::RecipeShape("empty operation block"))
    }

    fn capture_final_values(
        &mut self,
    ) -> Result<
        Box<[(crate::mir::loop_recipe_contract::LoopBindingKeyV1, ValueId)]>,
        LoopPhysicalizeErrorV1,
    > {
        let after = self.roles.block(LoopPhysicalRoleV1::After);
        let carriers = self
            .input
            .join_sig()
            .as_sig()
            .loops
            .first()
            .ok_or(LoopPhysicalizeErrorV1::RecipeShape("missing root JoinSig"))?;
        let carriers = carriers.carriers.clone();
        carriers
            .iter()
            .map(|carrier| {
                let binding = self
                    .bindings
                    .resolve(carrier.binding)
                    .map_err(LoopPhysicalizeErrorV1::Input)?;
                self.read(binding, after)
                    .map(|value| (carrier.binding, value))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Vec::into_boxed_slice)
    }

    fn seal(&mut self, block: BasicBlockId) -> Result<(), LoopPhysicalizeErrorV1> {
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        let witness = self
            .cfg
            .seal_block(function, block)
            .map_err(LoopPhysicalizeErrorV1::Cfg)?;
        let phis = &mut *self.phis;
        let builder = &mut *self.builder;
        self.port
            .seal(builder, phis, block, &witness)
            .map_err(LoopPhysicalizeErrorV1::Ssa)
    }

    fn read(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        let phis = &mut *self.phis;
        let builder = &mut *self.builder;
        self.port
            .read_binding(builder, phis, binding, block)
            .map_err(LoopPhysicalizeErrorV1::Ssa)
    }

    fn read_effect(
        &mut self,
        recipe_binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        let phis = &mut *self.phis;
        let builder = &mut *self.builder;
        let value = match (
            block == self.roles.block(LoopPhysicalRoleV1::Header),
            block == self.roles.block(LoopPhysicalRoleV1::Body),
            recipe_binding.raw(),
        ) {
            (true, false, 0) => self
                .port
                .read_condition_induction(builder, phis, binding, block),
            (false, true, 1) => self
                .port
                .read_update_accumulator(builder, phis, binding, block),
            (false, true, 0) => self.port.read_step_induction(builder, phis, binding, block),
            _ => {
                return Err(LoopPhysicalizeErrorV1::RecipeShape(
                    "DirectAccum read role is not canonical",
                ))
            }
        };
        value.map_err(LoopPhysicalizeErrorV1::Ssa)
    }

    fn write_effect(
        &mut self,
        recipe_binding: crate::mir::loop_recipe_contract::LoopBindingKeyV1,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), LoopPhysicalizeErrorV1> {
        let result = match (
            block == self.roles.block(LoopPhysicalRoleV1::Body),
            recipe_binding.raw(),
        ) {
            (true, 1) => self.port.write_update_accumulator(binding, block, value),
            (true, 0) => self.port.write_step_induction(binding, block, value),
            _ => {
                return Err(LoopPhysicalizeErrorV1::RecipeShape(
                    "DirectAccum write role is not canonical",
                ))
            }
        };
        result.map_err(LoopPhysicalizeErrorV1::Ssa)
    }

    fn value(
        &self,
        key: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    ) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        self.values
            .get(&key)
            .copied()
            .ok_or(LoopPhysicalizeErrorV1::MissingValue(key))
    }

    fn select(&mut self, block: BasicBlockId) -> Result<(), LoopPhysicalizeErrorV1> {
        let cfg = &self.cfg;
        let builder = &mut *self.builder;
        cfg.select_block(builder, block)
            .map_err(LoopPhysicalizeErrorV1::Cfg)
    }

    fn emit_jump(
        &mut self,
        from: BasicBlockId,
        to: BasicBlockId,
    ) -> Result<(), LoopPhysicalizeErrorV1> {
        let cfg = &self.cfg;
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        cfg.emit_jump(function, from, to)
            .map_err(LoopPhysicalizeErrorV1::Cfg)
    }

    fn emit_branch(
        &mut self,
        from: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Result<(), LoopPhysicalizeErrorV1> {
        let cfg = &self.cfg;
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        cfg.emit_branch(function, from, condition, then_block, else_block)
            .map_err(LoopPhysicalizeErrorV1::Cfg)
    }

    fn emit_return(&mut self, block: BasicBlockId) -> Result<(), LoopPhysicalizeErrorV1> {
        let cfg = &self.cfg;
        let function = self
            .builder
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)?;
        cfg.emit_return(function, block, None)
            .map_err(LoopPhysicalizeErrorV1::Cfg)
    }

    fn current_block(&self) -> Result<BasicBlockId, LoopPhysicalizeErrorV1> {
        self.builder_ref()
            .function_state
            .current_block
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)
    }

    fn function_ref(&self) -> Result<&crate::mir::MirFunction, LoopPhysicalizeErrorV1> {
        self.builder_ref()
            .function_state
            .current_function
            .as_ref()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)
    }

    fn function_mut(&mut self) -> Result<&mut crate::mir::MirFunction, LoopPhysicalizeErrorV1> {
        self.builder_mut()
            .function_state
            .current_function
            .as_mut()
            .ok_or(LoopPhysicalizeErrorV1::MissingFunction)
    }

    fn builder_ref(&self) -> &MirBuilder {
        self.builder
    }

    fn builder_mut(&mut self) -> &mut MirBuilder {
        self.builder
    }

    #[cfg(test)]
    fn inject_failure_if_requested(
        &self,
        point: DirectAccumPhysicalizerTestFailurePointV1,
    ) -> Result<(), LoopPhysicalizeErrorV1> {
        if self.failure == Some(point) {
            return Err(LoopPhysicalizeErrorV1::InjectedTestFailure(point));
        }
        Ok(())
    }
}

fn validate_direct_shape(
    input: &VerifiedLoopPhysicalInputV1,
) -> Result<(), LoopPhysicalizeErrorV1> {
    let recipe = input.recipe().as_recipe();
    let sig = input.join_sig().as_sig();
    if recipe.loops.len() != 1 || sig.loops.len() != 1 || recipe.inputs.len() != 2 {
        return Err(LoopPhysicalizeErrorV1::RecipeShape(
            "not a singleton DirectAccum shape",
        ));
    }
    let node = recipe.loops.first().expect("singleton checked");
    if !matches!(node.condition, LoopConditionV1::Predicate { .. }) {
        return Err(LoopPhysicalizeErrorV1::RecipeShape(
            "DirectAccum requires predicate",
        ));
    }
    let row = sig.loops.first().expect("singleton checked");
    let roles = row.edges.iter().map(|edge| edge.role).collect::<Vec<_>>();
    for required in [
        LoopJoinEdgeRoleV1::Enter,
        LoopJoinEdgeRoleV1::PredicateTrue,
        LoopJoinEdgeRoleV1::PredicateFalse,
        LoopJoinEdgeRoleV1::Backedge,
    ] {
        if !roles.contains(&required) {
            return Err(LoopPhysicalizeErrorV1::RecipeShape(
                "missing DirectAccum JoinSig edge",
            ));
        }
    }
    Ok(())
}
