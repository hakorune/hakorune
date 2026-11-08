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

#[derive(Debug)]
pub(in crate::mir::builder) struct FunctionControlV1 {
    owner: FunctionOwnerIdV1,
    pub(in crate::mir::builder) cfg: CanonicalCfgSessionV1,
    bindings: Option<BindingSsaBuilderV1<PhiToken>>,
    transaction: Option<PhiTxn>,
}
impl FunctionControlV1 {
    fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            cfg: CanonicalCfgSessionV1::new_for_owner(owner),
            bindings: Some(BindingSsaBuilderV1::new(owner)),
            transaction: Some(PhiTxn::begin("source-loop-header-after")),
        }
    }
    pub(in crate::mir::builder) fn is_active(&self) -> bool {
        self.transaction.is_some()
    }
    pub(in crate::mir::builder) fn with_transaction<T>(
        &mut self,
        action: impl FnOnce(&mut PhiTxn) -> Result<T, String>,
    ) -> Result<T, String> {
        action(self.transaction.as_mut().ok_or_else(inactive)?)
    }
    pub(in crate::mir::builder) fn reserve(
        &mut self,
        builder: &mut MirBuilder,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), String> {
        let token = self
            .transaction
            .as_mut()
            .ok_or_else(inactive)?
            .define_provisional_phi(builder, block, value, "source-loop:before-expose")?;
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
            let bindings = self.bindings.take().ok_or_else(inactive)?;
            bindings
                .close(
                    &mut MirBindingSsaAdapterV1::new(
                        builder,
                        self.transaction.as_mut().ok_or_else(inactive)?,
                    ),
                    &witnesses,
                )
                .map_err(|error| error.to_string())?;
            MirBindingSsaAdapterV1::new(builder, self.transaction.as_mut().ok_or_else(inactive)?)
                .verify_function_phi_inputs()?;
            let function = builder
                .function_state
                .current_function
                .as_ref()
                .ok_or_else(inactive)?;
            std::mem::take(&mut self.cfg)
                .finish(function)
                .map_err(|error| error.to_string())?;
            self.transaction
                .take()
                .ok_or_else(inactive)?
                .commit(builder)
                .map_err(|error| error.to_string())
        })();
        result.map_err(|error| self.abort(builder, error))
    }
}
impl MirBuilder {
    pub(in crate::mir::builder) fn function_control_for_owner_v1(
        &mut self,
        owner: FunctionOwnerIdV1,
    ) -> Result<SharedFunctionControlV1, String> {
        let control = self
            .function_state
            .canonical_control
            .get_or_insert_with(|| Rc::new(RefCell::new(FunctionControlV1::new(owner))));
        if control.borrow().owner != owner {
            return Err("[freeze:contract][canonical-control/foreign-owner]".to_owned());
        }
        Ok(Rc::clone(control))
    }
    pub(in crate::mir::builder) fn finish_function_control_v1(&mut self) -> Result<(), String> {
        let Some(control) = self.function_state.canonical_control.take() else {
            return Ok(());
        };
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
        let (Some(input), Some(claims)) = (self.source_input, &self.ordinary_new_claim_ledger)
        else {
            return Ok(());
        };
        // This query only opens storage for an already positive selection.
        // Rejections remain owned by the original issuer at the original site.
        if input.function().loop_sites().any(|site| {
            matches!(
                claims.retained_source_loop_recipe_v2(input.owner(), site),
                Ok(Some(_))
            )
        }) {
            builder.function_control_for_owner_v1(input.owner())?;
        }
        Ok(())
    }
}
