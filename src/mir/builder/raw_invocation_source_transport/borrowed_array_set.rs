//! Located raw call lends its original source and records one physical Set.
use super::*;
use crate::mir::builder::CompletedArrayElementWriteV1;
use crate::mir::normal_callable_semantic_package::BorrowedArraySetSourceLoanV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceExprSiteV1};

impl RawInvocationChildPortV1<'_, '_> {
    pub(super) fn borrowed_array_set_source_v1(&self, method: &str, arity: usize)
        -> Result<Option<BorrowedArraySetSourceLoanV1>, String> {
        let (Some(ledger), Some(owner)) = (&self.ordinary_new_claim_ledger, self.callable_owner_v1())
            else { return Ok(None) };
        let Some(node) = self.current_source_site_v1() else { return Ok(None) };
        let call = OwnedExprSiteV1::new(owner, SourceExprSiteV1::from_node(node));
        let source = ledger.borrowed_array_set_source_at_v1(owner, &call)?;
        if source.is_some() && crate::boxes::array::ArrayMethodId::from_name_and_arity(method, arity)
            != Some(crate::boxes::array::ArrayMethodId::Set) {
            return Err("[freeze:contract][borrowed-set/call-shape]".into());
        }
        Ok(source)
    }
    pub(super) fn complete_array_set_v1(&self, builder: &mut MirBuilder,
        source: BorrowedArraySetSourceLoanV1, children: [ValueId; 3], write: CompletedArrayElementWriteV1,
    ) -> Result<(), String> {
        if self.callable_owner_v1() != Some(source.call().owner())
            || self.current_source_site_v1().as_ref() != Some(source.call().site().node()) {
            return Err("[freeze:contract][borrowed-set/call-source-drift]".into());
        }
        builder.complete_borrowed_array_set_v1(
            self.ordinary_new_claim_ledger.as_ref().ok_or("[freeze:contract][borrowed-set/ledger-missing]")?,
            source, children, write)
    }
}
