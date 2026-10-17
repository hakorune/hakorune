//! The first original variable read transfers its receipt with its value.
use super::*;
use crate::mir::normal_callable_semantic_package::BorrowedFieldWriteSourceLoanV1 as Source;
use crate::mir::builder::ExactLexicalReadV1;
use crate::mir::resolved_semantics::SourceStmtSiteV1;

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn borrowed_field_write_source_v1(&self) -> Result<Option<Source>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1()) else { return Ok(None) };
        let node = self.current_source_site_v1().ok_or("[freeze:contract][borrowed-field-write/statement-site]")?;
        ledger.borrowed_field_write_source_at_v1(owner, &SourceStmtSiteV1::from_node(node))
    }
    pub(super) fn read_borrowed_field_write_source_v1(&self, source: &Source, receiver: bool, input: ASTNode,
    ) -> Result<Rc<ExactLexicalReadV1>, String> {
        let (binding, _, site) = if receiver { source.receiver() } else { source.value() };
        if !matches!(input, ASTNode::Variable { .. }) || self.callable_owner_v1() != Some(site.owner())
            || self.current_source_site_v1().as_ref() != Some(site.site().node())
        { return Err("[freeze:contract][borrowed-field-write/read-source]".into()); }
        let state = self.callable_ledger.as_ref().ok_or("[freeze:contract][borrowed-field-write/state]")?;
        let read = state.borrow_mut().read_variable_with_source_v1(site.site().node())?;
        read.value_for(site.owner(), site.site().node(), binding).map_err(|issue| issue.to_string())?;
        Ok(Rc::new(read))
    }
    pub(super) fn emit_borrowed_field_write_source_v1(&self, builder: &mut MirBuilder,
        source: Source, field: &str, reads: [Rc<ExactLexicalReadV1>; 2],
    ) -> Result<(), String> {
        let node = self.current_source_site_v1().ok_or("[freeze:contract][borrowed-field-write/statement-site]")?;
        if self.callable_owner_v1() != Some(source.frame().source().owner())
            || &SourceStmtSiteV1::from_node(node) != source.frame().source().root().statement()
        { return Err("[freeze:contract][borrowed-field-write/statement-drift]".into()); }
        let ledger = self.ordinary_new_claim_ledger.as_ref().ok_or("[freeze:contract][borrowed-field-write/ledger]")?;
        let state = self.callable_ledger.as_ref().ok_or("[freeze:contract][borrowed-field-write/state]")?;
        super::super::ordinary_new_admission::selected::borrowed_field_write::emit(builder, &mut state.borrow_mut(), ledger, source, field, reads)
    }
}
