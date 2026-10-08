//! Borrow the exact returned-New source before or after its affine take.
//! This lends retained source fields, never physical completion permission.
use super::*;

pub(in crate::mir::normal_callable_semantic_package) struct ResultNewSourceRefV1<'a> {
    pub(in crate::mir::normal_callable_semantic_package) site: &'a OwnedExprSiteV1,
    pub(in crate::mir::normal_callable_semantic_package) class: &'a str,
    pub(in crate::mir::normal_callable_semantic_package) arity: usize,
    pub(in crate::mir::normal_callable_semantic_package) object: CanonicalObjectIdV1,
    pub(in crate::mir::normal_callable_semantic_package) destruction:
        super::super::ObjectDestructionDispositionV1,
    pub(in crate::mir::normal_callable_semantic_package) construction:
        &'a super::super::ConstructionEligibilityV1,
    pub(in crate::mir::normal_callable_semantic_package) prefix:
        &'a Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
    pub(in crate::mir::normal_callable_semantic_package) arguments: &'a Result<
        Box<[super::super::OrdinaryNewTrivialArgumentV1]>,
        crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentUnavailableV1,
    >,
    pub(in crate::mir::normal_callable_semantic_package) children:
        Option<&'a [super::super::OwnedFieldChildV1]>,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn with_result_new_source_v1<T>(
        &self,
        site: &OwnedExprSiteV1,
        read: impl FnOnce(ResultNewSourceRefV1<'_>) -> Result<T, String>,
    ) -> Result<Option<T>, String> {
        let claims = self.result_claims.borrow();
        let commits = self.local_commits.borrow();
        let source = match (claims.get(site), commits.get(site)) {
            (Some(_), Some(_)) => return Err(freeze("result-source/duplicate-store")),
            (Some(claim), None) => {
                if claim.site() != site || claim.class() != claim.box_source().name() {
                    return Err(freeze("result-source/claim-identity"));
                }
                ResultNewSourceRefV1 {
                    site: claim.site(),
                    class: claim.class(),
                    arity: claim.arity(),
                    object: claim.object(),
                    destruction: claim.core.destruction(),
                    construction: claim.construction(),
                    prefix: &claim.home_prefix,
                    arguments: &claim.core.argument_rows,
                    children: claim.core.children(),
                }
            }
            (None, Some(LocalCommitV1::Result(row))) => {
                if &row.site != site {
                    return Err(freeze("result-source/commit-identity"));
                }
                ResultNewSourceRefV1 {
                    site: &row.site,
                    class: row.box_source.name(),
                    arity: row.arity,
                    object: row.object,
                    destruction: row.destruction,
                    construction: &row.construction,
                    prefix: &row.home_prefix,
                    arguments: &row.argument_rows,
                    children: row.children.as_deref(),
                }
            }
            (None, Some(_)) => return Err(freeze("result-source/commit-kind")),
            (None, None) => return Ok(None),
        };
        read(source).map(Some)
    }
}
