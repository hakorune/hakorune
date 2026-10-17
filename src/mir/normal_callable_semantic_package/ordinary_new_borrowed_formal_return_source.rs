//! Lend the existing nullable-formal Return source without issuing a result.
use super::super::super::result_class_claim::BorrowedFormalReturnSourceLoanV1;
use super::*;
use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1};

impl BorrowedOrdinaryEntrySourceRefV1<'_> {
    pub(crate) fn borrowed_formal_return_uses(
        &self,
    ) -> Result<
        Vec<(
            BindingRefV1,
            BindingRefV1,
            &SourceStmtSiteV1,
            &BorrowedFormalReturnSourceLoanV1,
        )>,
        String,
    > {
        use super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
        let mut returns = Vec::new();
        for row in &self.definition.uses {
            let Use::BorrowedFormalReturn { exit, source } = &row.kind else {
                continue;
            };
            if self.definition.origins.get(&row.binding) != Some(&row.formal)
                || row.site.owner() != self.owner
                || source.value() != &row.site
                || source.formal() != row.formal
                || SourcePathV1::from_node(exit.node())
                    .child(SourcePathSegmentV1::Value)
                    .expr()
                    != *row.site.site()
                || !self.formals.contains(&(source.ordinal(), row.formal))
            {
                return Err(freeze("borrowed-formal-return/source-identity"));
            }
            returns.push((row.binding, row.formal, exit, source.as_ref()));
        }
        Ok(returns)
    }
}
