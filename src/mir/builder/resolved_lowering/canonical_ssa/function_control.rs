//! Function-owned physical CFG, reaching definitions and provisional PHIs.
//! Source packets supply verified definitions; this owner never infers source
//! semantics from the emitted graph. Child loops borrow it, child functions do not.
use super::super::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::{
    emission::phi_lifecycle::{PhiToken, PhiTxn},
    ssa::binding::{BindingSsaBuilderV1, FunctionSsaKeyV1, MirBindingSsaAdapterV1},
    MirBuilder,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};
use std::{cell::RefCell, rc::Rc};

pub(in crate::mir::builder) type SharedFunctionControlV1 = Rc<RefCell<FunctionControlV1>>;

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct FunctionControlV1 {
    owner: Option<FunctionOwnerIdV1>,
    pub(in crate::mir::builder) cfg: CanonicalCfgSessionV1,
    bindings: Option<BindingSsaBuilderV1<PhiToken, FunctionSsaKeyV1>>,
    transaction: Option<PhiTxn>,
}
impl FunctionControlV1 {
    fn bind_owner(&mut self, owner: FunctionOwnerIdV1) -> Result<(), String> {
        if self.owner.is_some_and(|original| original != owner) {
            return Err("[freeze:contract][canonical-control/foreign-owner]".to_owned());
        }
        self.owner = Some(owner);
        self.cfg.bind_source_owner(owner)?;
        Ok(())
    }
    fn source_parts(
        &mut self,
    ) -> Result<
        (
            &mut BindingSsaBuilderV1<PhiToken, FunctionSsaKeyV1>,
            &mut PhiTxn,
        ),
        String,
    > {
        let owner = self.owner.ok_or_else(inactive)?;
        let transaction = self
            .transaction
            .get_or_insert_with(|| PhiTxn::begin("canonical_binding_ssa"));
        let bindings = self
            .bindings
            .get_or_insert_with(|| BindingSsaBuilderV1::with_keys(Some(owner)));
        Ok((bindings, transaction))
    }
    pub(in crate::mir::builder) fn validate_source_bindings(&mut self) -> Result<(), String> {
        self.source_parts()?
            .0
            .validate_finish()
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn read_source(
        &mut self,
        builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        deferred: Option<BasicBlockId>,
    ) -> Result<ValueId, String> {
        let (bindings, transaction) = self.source_parts()?;
        let mut adapter = match deferred {
            Some(entry) => {
                MirBindingSsaAdapterV1::new_with_deferred_reachable(builder, transaction, entry)
            }
            None => MirBindingSsaAdapterV1::new(builder, transaction),
        };
        bindings
            .read(&mut adapter, FunctionSsaKeyV1::Source(binding), block)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn seal_source(
        &mut self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
        witness: &super::super::canonical_cfg::VerifiedPredecessorsV1,
    ) -> Result<(), String> {
        let (bindings, transaction) = self.source_parts()?;
        bindings
            .seal(
                &mut MirBindingSsaAdapterV1::new(builder, transaction),
                block,
                witness,
            )
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn require_transaction_settled(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        let pending = self
            .transaction
            .as_ref()
            .is_some_and(|transaction| transaction.pending_tokens_v1().next().is_some());
        if pending {
            return self
                .transaction
                .take()
                .ok_or_else(inactive)?
                .commit(builder)
                .map_err(|error| error.to_string());
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn is_empty_for_commit(&self) -> bool {
        self.owner.is_none()
            && self.bindings.is_none()
            && self.transaction.is_none()
            && self.cfg.is_empty_for_commit()
    }
    pub(in crate::mir::builder) fn is_active(&self) -> bool {
        self.transaction.is_some()
    }
    pub(in crate::mir::builder) fn with_transaction<T>(
        &mut self,
        action: impl FnOnce(&mut PhiTxn) -> Result<T, String>,
    ) -> Result<T, String> {
        action(
            self.transaction
                .get_or_insert_with(|| PhiTxn::begin("source-loop-header-after")),
        )
    }
    pub(in crate::mir::builder) fn reserve(
        &mut self,
        builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.owner.ok_or_else(inactive)?;
        self.reserve_key(
            builder,
            FunctionSsaKeyV1::Source(binding),
            block,
            value,
            "source-loop:before-expose",
        )
    }
    fn reserve_key(
        &mut self,
        builder: &mut MirBuilder,
        key: FunctionSsaKeyV1,
        block: BasicBlockId,
        value: ValueId,
        tag: &str,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(inactive)?;
        if function.get_block(block).is_none() {
            self.cfg
                .create_block(function, block)
                .map_err(|error| error.to_string())?;
        }
        let token = self
            .transaction
            .get_or_insert_with(|| PhiTxn::begin("source-loop-header-after"))
            .define_provisional_phi(builder, block, value, tag)?;
        self.bindings
            .get_or_insert_with(|| BindingSsaBuilderV1::with_keys(self.owner))
            .adopt_provisional_phi(key, block, value, token)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn define(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.source_parts()?
            .0
            .define(FunctionSsaKeyV1::Source(binding), block, value)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn complete_blocks(
        &mut self,
        builder: &mut MirBuilder,
        blocks: &[BasicBlockId],
        deferred_entry: BasicBlockId,
    ) -> Result<(), String> {
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(inactive)?;
        let witnesses = self
            .cfg
            .predecessor_witnesses(function)
            .map_err(|error| error.to_string())?;
        let mut adapter = MirBindingSsaAdapterV1::new_with_deferred_reachable(
            builder,
            self.transaction.as_mut().ok_or_else(inactive)?,
            deferred_entry,
        );
        for block in blocks {
            let witness = witnesses
                .iter()
                .find(|row| row.block() == *block)
                .ok_or_else(|| format!("[freeze:contract][canonical-control/predecessor-witness-missing] block={block}"))?;
            self.bindings
                .as_mut()
                .ok_or_else(inactive)?
                .seal(&mut adapter, *block, witness)
                .map_err(|error| error.to_string())?;
        }
        Ok(())
    }
    pub(in crate::mir::builder) fn abort(
        &mut self,
        builder: &mut MirBuilder,
        error: String,
    ) -> String {
        match self.transaction.take() {
            Some(txn) => txn.abort_on_err(builder, error).to_string(),
            None => error,
        }
    }
    fn finish(&mut self, builder: &mut MirBuilder) -> Result<(), String> {
        self.finish_against_graph(builder, None)
    }
    fn finish_against_graph(
        &mut self,
        builder: &mut MirBuilder,
        mut projected: Option<crate::mir::MirFunction>,
    ) -> Result<(), String> {
        let result = (|| {
            let function = match projected.as_mut() {
                Some(function) => function,
                None => builder
                    .function_state
                    .current_function
                    .as_mut()
                    .ok_or_else(inactive)?,
            };
            let witnesses = self
                .cfg
                .close_function_blocks(function)
                .map_err(|error| error.to_string())?;
            if let Some(bindings) = self.bindings.take() {
                bindings
                    .close(
                        &mut MirBindingSsaAdapterV1::new(
                            builder,
                            self.transaction.as_mut().ok_or_else(inactive)?,
                        ),
                        &witnesses,
                    )
                    .map_err(|error| error.to_string())?;
                MirBindingSsaAdapterV1::new(
                    builder,
                    self.transaction.as_mut().ok_or_else(inactive)?,
                )
                .verify_function_phi_inputs()?;
            }
            let function = match projected.as_ref() {
                Some(function) => function,
                None => builder
                    .function_state
                    .current_function
                    .as_ref()
                    .ok_or_else(inactive)?,
            };
            std::mem::take(&mut self.cfg)
                .finish(function)
                .map_err(|error| error.to_string())?;
            match self.transaction.take() {
                Some(transaction) => transaction
                    .commit(builder)
                    .map_err(|error| error.to_string()),
                None => Ok(()),
            }
        })();
        result.map_err(|error| self.abort(builder, error))
    }
}
impl MirBuilder {
    pub(in crate::mir::builder) fn function_control_for_owner_v1(
        &self,
        owner: FunctionOwnerIdV1,
    ) -> Result<SharedFunctionControlV1, String> {
        let control = Rc::clone(&self.function_state.canonical_control);
        control.borrow_mut().bind_owner(owner)?;
        Ok(control)
    }
    pub(in crate::mir::builder) fn finish_projected_function_control_v1(
        &mut self,
        projected: &crate::mir::MirFunction,
    ) -> Result<(), String> {
        let control = std::mem::take(&mut self.function_state.canonical_control);
        let result = control
            .borrow_mut()
            .finish_against_graph(self, Some(projected.clone()));
        result
    }
    pub(in crate::mir::builder) fn finish_function_control_v1(&mut self) -> Result<(), String> {
        let control = std::mem::take(&mut self.function_state.canonical_control);
        let result = control.borrow_mut().finish(self);
        result
    }
}
fn inactive() -> String {
    "[freeze:contract][canonical-control/inactive]".to_owned()
}

impl crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn prepare_function_control_v1(
        &self,
        builder: &mut MirBuilder,
    ) -> Result<(), String> {
        if let Some(input) = self.source_input {
            builder.function_control_for_owner_v1(input.owner())?;
        }
        Ok(())
    }
}
