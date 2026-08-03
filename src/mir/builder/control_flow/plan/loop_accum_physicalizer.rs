//! Caller-zero DirectAccum physicalizer.
//!
//! This is the first consumer of a verified Recipe/JoinSig pair. It uses the
//! existing canonical CFG, function-owned Binding SSA, and one PhiTxn; it
//! does not select routes or publish a candidate module.

use super::loop_physical_input::{
    LoopPhysicalInputRejectV1, LoopPhysicalRoleV1, VerifiedLoopBindingProjectionV1,
    VerifiedLoopInputProjectionV1, VerifiedLoopPhysicalRolePlanV1,
};
use crate::mir::builder::emission::{loop_operation, phi_lifecycle::PhiTxn};
use crate::mir::builder::resolved_lowering::canonical_cfg::{
    CanonicalCfgErrorV1, CanonicalCfgSessionV1,
};
use crate::mir::builder::ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1};
use crate::mir::builder::MirBuilder;
use crate::mir::loop_recipe_contract::{
    LoopBinaryI64OpV1, LoopCompareI64OpV1, LoopConditionV1, LoopJoinEdgeRoleV1,
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
}

pub(in crate::mir::builder) fn physicalize_direct_accum_v1(
    builder: &mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
    let mut session =
        DirectAccumPhysicalizerV1::preflight(builder, input, bindings, inputs, roles)?;
    session.emit()
}

struct DirectAccumPhysicalizerV1<'a> {
    builder: &'a mut MirBuilder,
    input: VerifiedLoopPhysicalInputV1,
    bindings: VerifiedLoopBindingProjectionV1,
    inputs: VerifiedLoopInputProjectionV1,
    roles: VerifiedLoopPhysicalRolePlanV1,
    cfg: CanonicalCfgSessionV1,
    ssa: BindingSsaBuilderV1<crate::mir::builder::emission::phi_lifecycle::PhiToken>,
    phis: Option<PhiTxn>,
    values: BTreeMap<crate::mir::loop_recipe_contract::LoopValueKeyV1, ValueId>,
}

impl<'a> DirectAccumPhysicalizerV1<'a> {
    fn preflight(
        builder: &'a mut MirBuilder,
        input: VerifiedLoopPhysicalInputV1,
        bindings: VerifiedLoopBindingProjectionV1,
        inputs: VerifiedLoopInputProjectionV1,
        roles: VerifiedLoopPhysicalRolePlanV1,
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
        let preheader_block = function
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
                return Err(LoopPhysicalizeErrorV1::ExistingPhysicalBlock(roles.block(role)));
            }
        }
        let recipe = input.recipe().as_recipe();
        if recipe.inputs.iter().any(|value| inputs.binding_for(*value).is_none()) {
            return Err(LoopPhysicalizeErrorV1::RecipeShape("missing recipe input projection"));
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
            ssa: BindingSsaBuilderV1::new(bindings.owner()),
            phis: Some(PhiTxn::begin("loop_direct_accum_physicalizer")),
            cfg: CanonicalCfgSessionV1::new(),
            input,
            bindings,
            inputs,
            roles,
            values: BTreeMap::new(),
        })
    }

    fn emit(&mut self) -> Result<LoopPhysicalSuccessReceiptV1, LoopPhysicalizeErrorV1> {
        if let Err(error) = self.create_blocks() {
            return Err(error);
        }
        if let Err(error) = self.seed_inputs() {
            return self.abort(error);
        }
        if let Err(error) = self.emit_cfg_and_operations() {
            return self.abort(error);
        }
        let final_values = match self.capture_final_values() {
            Ok(values) => values,
            Err(error) => return self.abort(error),
        };
        if let Err(error) = self.finish_owners() {
            return self.abort(error);
        }
        let phis = self.phis.take().expect("active PHI transaction");
        if let Err(error) = phis.commit(self.builder_mut()) {
            return Err(LoopPhysicalizeErrorV1::PhiAbort(error.to_string()));
        }
        Ok(LoopPhysicalSuccessReceiptV1 {
            final_values,
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
            cfg
                .create_block(function, block)
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
            let binding = self.bindings.resolve(key).map_err(LoopPhysicalizeErrorV1::Input)?;
            self.ssa
                .define(binding, self.roles.block(LoopPhysicalRoleV1::Preheader), physical)
                .map_err(|error| LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")))?;
            self.values.insert(value, physical);
        }
        Ok(())
    }

    fn emit_cfg_and_operations(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let preheader = self.roles.block(LoopPhysicalRoleV1::Preheader);
        let header = self.roles.block(LoopPhysicalRoleV1::Header);
        let body = self.roles.block(LoopPhysicalRoleV1::Body);
        let step = self.roles.block(LoopPhysicalRoleV1::Step);
        let after = self.roles.block(LoopPhysicalRoleV1::After);
        self.emit_jump(preheader, header)?;
        self.seal(preheader)?;
        self.select(header)?;
        self.emit_header_carriers()?;
        let condition = self.block_operations(0)?;
        let condition_result = self.emit_operations(&condition)?;
        self.emit_branch(header, condition_result, body, after)?;
        self.select(body)?;
        self.seal(body)?;
        let body_operations = self.block_operations(1)?;
        self.emit_operations(&body_operations)?;
        self.emit_jump(body, step)?;
        self.select(step)?;
        self.seal(step)?;
        self.emit_jump(step, header)?;
        self.select(after)?;
        self.emit_return(after)?;
        self.seal(after)?;
        self.select(header)?;
        self.seal(header)?;
        Ok(())
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

    fn block_operations(&self, block_raw: u32) -> Result<Box<[LoopOperationV1]>, LoopPhysicalizeErrorV1> {
        let recipe = self.input.recipe().as_recipe();
        let block = recipe
            .blocks
            .iter()
            .find(|block| block.key.raw() == block_raw)
            .ok_or(LoopPhysicalizeErrorV1::RecipeShape("missing DirectAccum block"))?;
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

    fn emit_operations(&mut self, operations: &[LoopOperationV1]) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        let mut last = None;
        for operation in operations.iter().copied() {
            match operation {
                LoopOperationV1::ReadBinding { binding, result } => {
                    let binding = self.bindings.resolve(binding).map_err(LoopPhysicalizeErrorV1::Input)?;
                    let value = self.read(binding, self.current_block()?)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::ConstI64 { result, value } => {
                    let value = loop_operation::emit_const_i64(self.builder_mut(), value)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::BinaryI64 { op, left, right, result } => {
                    if !matches!(op, LoopBinaryI64OpV1::Add) {
                        return Err(LoopPhysicalizeErrorV1::RecipeShape("DirectAccum requires add"));
                    }
                    let lhs = self.value(left)?;
                    let rhs = self.value(right)?;
                    let value = loop_operation::emit_add_i64(self.builder_mut(), lhs, rhs)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::CompareI64 { op, left, right, result } => {
                    if !matches!(op, LoopCompareI64OpV1::Less) {
                        return Err(LoopPhysicalizeErrorV1::RecipeShape("DirectAccum requires less"));
                    }
                    let lhs = self.value(left)?;
                    let rhs = self.value(right)?;
                    let value = loop_operation::emit_less_i64(self.builder_mut(), lhs, rhs)
                        .map_err(LoopPhysicalizeErrorV1::Operation)?;
                    self.values.insert(result, value);
                    last = Some(value);
                }
                LoopOperationV1::WriteBinding { binding, value } => {
                    let binding = self.bindings.resolve(binding).map_err(LoopPhysicalizeErrorV1::Input)?;
                    let value = self.value(value)?;
                    self.ssa
                        .define(binding, self.current_block()?, value)
                        .map_err(|error| LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")))?;
                    last = Some(value);
                }
            }
        }
        last.ok_or(LoopPhysicalizeErrorV1::RecipeShape("empty operation block"))
    }

    fn capture_final_values(
        &mut self,
    ) -> Result<Box<[(crate::mir::loop_recipe_contract::LoopBindingKeyV1, ValueId)]>, LoopPhysicalizeErrorV1> {
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

    fn finish_owners(&mut self) -> Result<(), LoopPhysicalizeErrorV1> {
        let ssa = std::mem::replace(
            &mut self.ssa,
            BindingSsaBuilderV1::new(self.bindings.owner()),
        );
        ssa.finish()
            .map_err(|error| LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")))?;
        let cfg = std::mem::replace(&mut self.cfg, CanonicalCfgSessionV1::new());
        let function = self.function_ref()?;
        cfg.finish(function)
            .map_err(LoopPhysicalizeErrorV1::Cfg)
            .map(|_| ())
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
        let phis = self.phis.as_mut().expect("active PHI transaction");
        let builder = &mut *self.builder;
        let ssa = &mut self.ssa;
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        ssa
            .seal(&mut adapter, block, &witness)
            .map_err(|error| LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")))
    }

    fn read(&mut self, binding: BindingRefV1, block: BasicBlockId) -> Result<ValueId, LoopPhysicalizeErrorV1> {
        let phis = self.phis.as_mut().expect("active PHI transaction");
        let builder = &mut *self.builder;
        let ssa = &mut self.ssa;
        let mut adapter = MirBindingSsaAdapterV1::new(builder, phis);
        ssa
            .read(&mut adapter, binding, block)
            .map_err(|error| LoopPhysicalizeErrorV1::Ssa(format!("{error:?}")))
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

    fn emit_jump(&mut self, from: BasicBlockId, to: BasicBlockId) -> Result<(), LoopPhysicalizeErrorV1> {
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

    fn abort<T>(&mut self, error: LoopPhysicalizeErrorV1) -> Result<T, LoopPhysicalizeErrorV1> {
        let Some(phis) = self.phis.take() else {
            return Err(error);
        };
        let aborted = phis.abort_on_err(self.builder, format!("{error:?}"));
        Err(LoopPhysicalizeErrorV1::PhiAbort(aborted.to_string()))
    }
}

fn validate_direct_shape(input: &VerifiedLoopPhysicalInputV1) -> Result<(), LoopPhysicalizeErrorV1> {
    let recipe = input.recipe().as_recipe();
    let sig = input.join_sig().as_sig();
    if recipe.loops.len() != 1 || sig.loops.len() != 1 || recipe.inputs.len() != 2 {
        return Err(LoopPhysicalizeErrorV1::RecipeShape("not a singleton DirectAccum shape"));
    }
    let node = recipe
        .loops
        .first()
        .expect("singleton checked");
    if !matches!(node.condition, LoopConditionV1::Predicate { .. }) {
        return Err(LoopPhysicalizeErrorV1::RecipeShape("DirectAccum requires predicate"));
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
            return Err(LoopPhysicalizeErrorV1::RecipeShape("missing DirectAccum JoinSig edge"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::emission::loop_operation;
    use crate::mir::loop_recipe_contract::{
        direct_accum_product_for_test, VerifiedLoopPhysicalInputV1,
    };
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::{BindingId, MirBuilder};

    fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("issuer")
            .issue()
            .expect("owner")
    }

    fn roles() -> VerifiedLoopPhysicalRolePlanV1 {
        VerifiedLoopPhysicalRolePlanV1::try_new(vec![
            (LoopPhysicalRoleV1::Preheader, BasicBlockId::new(0)),
            (LoopPhysicalRoleV1::Header, BasicBlockId::new(1)),
            (LoopPhysicalRoleV1::Body, BasicBlockId::new(2)),
            (LoopPhysicalRoleV1::Step, BasicBlockId::new(3)),
            (LoopPhysicalRoleV1::After, BasicBlockId::new(4)),
        ])
        .expect("standard5 roles")
    }

    #[test]
    fn direct_accum_physicalizer_emits_through_existing_owners() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("direct_accum_physicalizer/0".to_owned());
        let initial_i = loop_operation::emit_const_i64(&mut builder, 0).expect("initial i");
        let initial_sum = loop_operation::emit_const_i64(&mut builder, 0).expect("initial sum");
        let owner = owner();
        let bindings = VerifiedLoopBindingProjectionV1::try_new(
            owner,
            vec![
                (
                    crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                    crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0)),
                ),
                (
                    crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(1),
                    crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(1)),
                ),
            ],
        )
        .expect("binding projection");
        let inputs = VerifiedLoopInputProjectionV1::try_new(
            BasicBlockId::new(0),
            vec![
                (
                    crate::mir::loop_recipe_contract::LoopValueKeyV1::new(0),
                    crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                    initial_i,
                ),
                (
                    crate::mir::loop_recipe_contract::LoopValueKeyV1::new(1),
                    crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(1),
                    initial_sum,
                ),
            ],
        )
        .expect("input projection");
        let receipt = physicalize_direct_accum_v1(
            &mut builder,
            VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
            bindings,
            inputs,
            roles(),
        )
        .expect("physicalize");
        assert_eq!(receipt.result, LoopResultDispositionV1::Unit);
        assert_eq!(receipt.final_values.len(), 2);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("function")
                .blocks
                .len(),
            5
        );
    }

    #[test]
    fn missing_preheader_input_rejects_before_block_creation() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("direct_accum_physicalizer/reject".to_owned());
        let owner = owner();
        let bindings = VerifiedLoopBindingProjectionV1::try_new(
            owner,
            vec![(
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                crate::mir::resolved_semantics::BindingRefV1::new(owner, BindingId::new(0)),
            )],
        )
        .expect("binding projection");
        let inputs = VerifiedLoopInputProjectionV1::try_new(
            BasicBlockId::new(0),
            vec![(
                crate::mir::loop_recipe_contract::LoopValueKeyV1::new(0),
                crate::mir::loop_recipe_contract::LoopBindingKeyV1::new(0),
                ValueId::new(99),
            )],
        )
        .expect("input projection");
        let error = physicalize_direct_accum_v1(
            &mut builder,
            VerifiedLoopPhysicalInputV1::from_direct_accum(direct_accum_product_for_test()),
            bindings,
            inputs,
            roles(),
        )
        .unwrap_err();
        assert!(matches!(error, LoopPhysicalizeErrorV1::RecipeShape(_)));
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("function")
                .blocks
                .len(),
            1
        );
    }
}
