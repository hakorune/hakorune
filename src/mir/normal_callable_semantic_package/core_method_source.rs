//! Source-bound CoreMethod call issuance for the semantic package.
//!
//! This module owns only the catalog/source target co-seal. It does not
//! select a physical route or consume Builder state.

use crate::mir::builder::{
    SelectedNormalCallableKeyV1, VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::{
    VerifiedSourceBoundCoreMethodCallV1, VerifiedSourceCallTargetCatalogV1,
    VerifiedStaticImportAliasViewV1,
};
use std::collections::BTreeMap;

pub(crate) fn issue_source_core_method_calls_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
) -> Result<
    BTreeMap<
        SelectedNormalCallableKeyV1,
        BTreeMap<SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1>,
    >,
    String,
> {
    let declarations = catalog.catalog();
    let imports = VerifiedStaticImportAliasViewV1::seal(declarations, [])
        .map_err(|error| format!("{error:?}"))?;
    let mut target_catalog = VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, [])
        .map_err(|error| format!("{error:?}"))?;
    let mut issued = BTreeMap::new();
    for key in selected.keys() {
        let SelectedNormalCallableKeyV1::Cataloged(catalog_key) = key else {
            continue;
        };
        let batch_slot = selected
            .batch_slot(key)
            .ok_or_else(|| "selected callable has no semantic batch slot".to_owned())?;
        let rows = batch
            .with_lowering_input(batch_slot, |input| {
                let ledger = input
                    .forest()
                    .callable_source_ledger(input.owner())
                    .map_err(|error| format!("{error:?}"))?;
                let rows = crate::mir::source_call_target::issue_source_bound_core_method_calls_v1(
                    &ledger,
                )
                .map_err(|error| format!("{error:?}"))?;
                Ok::<_, String>(rows)
            })
            .map_err(|error| format!("{error:?}"))??;
        target_catalog = target_catalog
            .extend_core_method_calls(catalog_key.clone(), Vec::from(rows).into_iter())
            .map_err(|error| format!("{error:?}"))?;
        let has_rows = target_catalog
            .all_rows()
            .any(|((caller, _), _)| caller == catalog_key);
        if has_rows {
            let rows = target_catalog
                .into_core_method_calls(catalog_key)
                .map_err(|error| format!("{error:?}"))?;
            issued.insert(
                SelectedNormalCallableKeyV1::Cataloged(catalog_key.clone()),
                rows,
            );
            target_catalog = VerifiedSourceCallTargetCatalogV1::seal_qualified(&imports, [])
                .map_err(|error| format!("{error:?}"))?;
        }
    }
    Ok(issued)
}
