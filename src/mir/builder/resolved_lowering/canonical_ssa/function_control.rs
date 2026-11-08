//! Function-owned physical CFG, reaching definitions and provisional PHIs.
//! Source packets supply verified definitions; this owner never infers source
//! semantics from the emitted graph. Child loops borrow it, child functions do not.
use super::super::canonical_cfg::CanonicalCfgSessionV1;
use crate::mir::builder::{
    emission::phi_lifecycle::{PhiToken, PhiTxn},
    ssa::binding::{BindingSsaBuilderV1, MirBindingSsaAdapterV1},
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
    bindings: Option<BindingSsaBuilderV1<PhiToken>>,
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
        let owner = self.owner.ok_or_else(inactive)?;
        let token = self
            .transaction
            .get_or_insert_with(|| PhiTxn::begin("source-loop-header-after"))
            .define_provisional_phi(builder, block, value, "source-loop:before-expose")?;
        self.bindings
            .get_or_insert_with(|| BindingSsaBuilderV1::new(owner));
        self.bindings
            .as_mut()
            .ok_or_else(inactive)?
            .adopt_provisional_phi(binding, block, value, token)
            .map_err(|error| error.to_string())
    }
    pub(in crate::mir::builder) fn define(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        self.bindings
            .as_mut()
            .ok_or_else(inactive)?
            .define(binding, block, value)
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
        let result = (|| {
            let function = builder
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(inactive)?;
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
            let function = builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(inactive)?;
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
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Result<SharedFunctionControlV1, String> {
        let control = Rc::clone(&self.function_state.canonical_control);
        control.borrow_mut().bind_owner(owner)?;
        Ok(control)
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
