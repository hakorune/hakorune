//! Source-bound CoreMethod call issuance for the semantic package.
//!
//! This module owns only the catalog/source target co-seal. It does not
//! select a physical route or consume Builder state.

mod collector;
mod emission;
pub(crate) use collector::NamedArrayEmissionCollectorV1;
pub(crate) use emission::{
    validate_named_array_coverage, EmittedNamedArrayRequirementV1, NamedArrayWriteEmissionPortV1,
};

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
        BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
    >,
    String,
> {
    issue_source_core_method_calls_with_v1(
        catalog,
        batch,
        selected,
        crate::mir::source_call_target::issue_source_bound_core_method_calls_v1,
    )
}

pub(crate) fn issue_source_core_method_calls_with_named_arrays_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    brands: &crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1,
) -> Result<
    BTreeMap<
        SelectedNormalCallableKeyV1,
        BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
    >,
    String,
> {
    issue_source_core_method_calls_with_v1(catalog, batch, selected, |ledger| {
        crate::mir::source_call_target::issue_source_bound_core_method_calls_with_named_arrays_v1(
            ledger,
            batch.ordinary_box_coverage(),
            brands,
        )
    })
}

fn issue_source_core_method_calls_with_v1(
    catalog: &VerifiedSourceBackedSameModuleCallableCatalogV1,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    issue: impl Fn(
        &crate::mir::resolved_semantics::CallableSemanticSourceLedgerView<'_>,
    ) -> Result<
        Box<[(SourceExprSiteV1, VerifiedSourceBoundCoreMethodCallV1)]>,
        crate::mir::source_call_target::SourceBoundCoreMethodTargetIssueV1,
    >,
) -> Result<
    BTreeMap<
        SelectedNormalCallableKeyV1,
        BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
    >,
    String,
> {
    let declarations = catalog.catalog();
    let imports = VerifiedStaticImportAliasViewV1::brand_only(declarations);
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
        let (owner, rows) = batch
            .with_lowering_input(batch_slot, |input| {
                let ledger = input
                    .forest()
                    .callable_source_ledger(input.owner())
                    .map_err(|error| format!("{error:?}"))?;
                let rows = issue(&ledger).map_err(|error| format!("{error:?}"))?;
                Ok::<_, String>((input.owner(), rows))
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
            let rows = rows
                .into_iter()
                .map(|(site, row)| {
                    if row.contract().owner() != owner || row.contract().call_site() != &site {
                        return Err(crate::mir::named_array_obligation::fault(
                            "package-source-mismatch",
                        ));
                    }
                    Ok((
                        site,
                        SelectedSourceCoreMethodCallV1 {
                            caller: catalog_key.clone(),
                            row,
                            allocation: None,
                        },
                    ))
                })
                .collect::<Result<BTreeMap<_, _>, String>>()?;
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

/// Package-owned canonical association. Builder can observe or consume it, but
/// cannot pair an arbitrary canonical key with a resolver contract.
#[derive(Debug)]
pub(crate) struct SelectedSourceCoreMethodCallV1 {
    caller: crate::mir::builder::CanonicalSameModuleCallableKeyV1,
    row: VerifiedSourceBoundCoreMethodCallV1,
    allocation: Option<crate::mir::ValueId>,
}

impl SelectedSourceCoreMethodCallV1 {
    pub(crate) fn contract(
        &self,
    ) -> &crate::mir::resolved_semantics::VerifiedResolverCoreMethodCallableContractV1 {
        self.row.contract()
    }

    pub(crate) fn require_selected(
        &self,
        key: &SelectedNormalCallableKeyV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> Result<(), String> {
        if !matches!(key, SelectedNormalCallableKeyV1::Cataloged(caller) if caller == &self.caller)
            || self.contract().owner() != owner
        {
            return Err(crate::mir::named_array_obligation::fault(
                "selected-source-mismatch",
            ));
        }
        Ok(())
    }

    /// Unconditional String rows need no retained constructor obligation.
    /// Conditional rows must instead remain owned through physical emission.
    pub(crate) fn into_unconditional_contract(
        self,
    ) -> Result<crate::mir::resolved_semantics::VerifiedResolverCoreMethodCallableContractV1, String>
    {
        if self.contract().named_array_requirement().is_some() {
            return Err(crate::mir::named_array_obligation::fault(
                "retained-source-required",
            ));
        }
        Ok(self.row.into_contract())
    }
}

#[cfg(test)]
#[path = "core_method_source_witness_tests.rs"]
pub(crate) mod test_witness;
