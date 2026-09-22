//! Existing isolated lowering tests have no installed catalog. Their witness
//! supports unconditional String contracts only; it cannot mint Array authority.
use super::*;

pub(crate) fn unconditional_test_rows(
    rows: Box<[(SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)]>,
) -> BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1> {
    rows.into_vec()
        .into_iter()
        .map(|(site, row)| {
            assert!(
                row.contract().named_array_requirement().is_none(),
                "conditional Array evidence requires the real package issuer"
            );
            (site, SelectedSourceCoreMethodCallV1 {
            caller: crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "IsolatedSourceWitness", "unconditional", 0),
            row,
        })
        })
        .collect()
}
