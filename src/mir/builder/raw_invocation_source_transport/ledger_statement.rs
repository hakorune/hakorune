//! Statements the claim ledger already decided are emitted here, not in the port impl.
use super::*;

impl RawInvocationChildPortV1<'_, '_> {
    /// Emits one ledger-decided statement (field Map install, local Map store, field Array append).
    /// `None` means the ledger has no decision for the current statement.
    pub(super) fn emit_ledger_statement_v1(
        &mut self,
        builder: &mut MirBuilder,
    ) -> Result<Option<ValueId>, String> {
        let (Some(ledger), Some(state), Some(source_input), Some(node)) = (
            &self.ordinary_new_claim_ledger,
            &self.callable_ledger,
            self.source_input,
            self.current_source_site_v1(),
        ) else {
            return Ok(None);
        };
        let site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            source_input.owner(),
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(node),
        );
        use crate::mir::builder::ordinary_new_admission::selected::{
            field_array_append, map_field_install, map_local_store,
        };
        if let Some(source) = ledger.map_field_install_source_v1(&site)? {
            map_field_install::emit(builder, &mut state.borrow_mut(), ledger, source)?;
        } else if let Some((store, map_site)) = ledger.map_local_store_at_v1(&site)? {
            map_local_store::emit(builder, &mut state.borrow_mut(), ledger, &store, &map_site)?;
        } else if let Some(source) = ledger.field_array_text_append_at_v1(&site)? {
            let frame = source
                .text_frame()
                .ok_or("[freeze:contract][field-array-append/original-call]")?;
            ledger.stage_source_array_receiver_v1(
                source_input,
                frame.source().receiver(),
                frame.source().field_name(),
            )?;
            field_array_append::emit_text(builder, &mut state.borrow_mut(), ledger, source)?;
        } else {
            return Ok(None);
        }
        crate::mir::builder::emission::constant::emit_void(builder).map(Some)
    }
}
