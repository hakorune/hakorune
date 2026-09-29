//! Current-batch projection of the sole Home ABI issuer: entry-only loans.
//!
//! `CallableHomeAbiIssuerV1` remains the canonical issuer; this module only
//! holds the batch-facing projection the entry-flow loan amendment admits.
//! It checks parser/batch/declaration identity, instance mode, the exact
//! `Receiver` binding, and complete ordered common parameter rows before
//! lending entry demands. The loan carries no result relation and never
//! publishes a complete call-site Home ABI. Missing capability or capture
//! demands produce no loan — the named entry unavailability is preserved
//! downstream rather than filled with empty obligations.

use std::collections::BTreeSet;

use crate::mir::callable_parameter_contract::{
    CallableParameterDeclarationModeV1, VerifiedCallableParameterContractCatalogV1,
};
use crate::mir::callable_semantic_batch::{
    ResolvedCallableDeclarationModeV1, ResolvedCallableSemanticBatchLoanErrorV1,
    VerifiedResolvedCallableSemanticBatchV1,
};
use crate::mir::resolved_semantics::{
    CallableHomeAbiIssuerV1, SourceBindingSiteV1, VerifiedInstanceEntryHomeCatalogV1,
    VerifiedInstanceEntryHomeLoanV1, VerifiedInstanceEntryHomeParameterV1,
};

#[derive(Debug)]
pub(in crate::mir) enum SourceEntryHomeIssueV1 {
    BatchLoan {
        _error: ResolvedCallableSemanticBatchLoanErrorV1,
    },
    /// An `InstanceBoxMethod` declaration must carry exactly one `me`
    /// receiver site; the resolver mints it for `DeclaredInstance` bodies.
    ReceiverSiteMissing {
        _batch_slot: u32,
    },
    ReceiverSiteDuplicate {
        _batch_slot: u32,
    },
    /// The receiver site resolves to no binding, a foreign-owner binding,
    /// or a non-Receiver record — foreign evidence never becomes a loan.
    ReceiverBindingForeign {
        _batch_slot: u32,
    },
    /// The common parameter catalog must carry exactly one row per
    /// instance declaration; missing, duplicated, or mismatched rows
    /// reject before any flow sees a partial entry relation.
    ParameterRelationMissing {
        _batch_slot: u32,
    },
    ParameterRelationDuplicate {
        _batch_slot: u32,
    },
    ParameterRelationMismatch {
        _batch_slot: u32,
    },
    OwnerMismatch {
        _batch_slot: u32,
    },
}

impl CallableHomeAbiIssuerV1 {
    /// Lend entry-only Home demands for the current verified callable batch.
    ///
    /// One loan per exact `InstanceBoxMethod` declaration that carries its
    /// receiver binding, complete ordered common parameter rows, and no
    /// capture demands. Static and top-level declarations receive no loan
    /// and keep their existing route; the result relation stays unresolved.
    pub(super) fn issue_source_entry_home_catalog_v1(
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        parameter_catalog: &VerifiedCallableParameterContractCatalogV1<'_>,
    ) -> Result<VerifiedInstanceEntryHomeCatalogV1, SourceEntryHomeIssueV1> {
        let mut loans = Vec::new();
        for declaration in batch.declarations() {
            if declaration.mode() != ResolvedCallableDeclarationModeV1::InstanceBoxMethod {
                continue;
            }
            let batch_slot = declaration.batch_slot();
            let loan = batch
                .with_lowering_input(batch_slot, |input| {
                    if input.owner() != declaration.owner() {
                        return Err(SourceEntryHomeIssueV1::OwnerMismatch {
                            _batch_slot: batch_slot,
                        });
                    }
                    // Capture demands stay unsupported: no loan is lent, so
                    // the named entry unavailability is preserved.
                    if !input
                        .forest()
                        .ordered_capture_demands(input.owner())
                        .is_empty()
                    {
                        return Ok(None);
                    }
                    let function = input.function();
                    let mut receiver_sites = function
                        .declaration_sites()
                        .filter(|site| matches!(site, SourceBindingSiteV1::Receiver));
                    let Some(receiver_site) = receiver_sites.next() else {
                        return Err(SourceEntryHomeIssueV1::ReceiverSiteMissing {
                            _batch_slot: batch_slot,
                        });
                    };
                    if receiver_sites.next().is_some() {
                        return Err(SourceEntryHomeIssueV1::ReceiverSiteDuplicate {
                            _batch_slot: batch_slot,
                        });
                    }
                    let receiver = function
                        .declaration_binding(receiver_site)
                        .filter(|binding| binding.owner() == declaration.owner())
                        .ok_or(SourceEntryHomeIssueV1::ReceiverBindingForeign {
                            _batch_slot: batch_slot,
                        })?;
                    let mut contract_rows = parameter_catalog
                        .declarations()
                        .filter(|row| row.batch_slot() == batch_slot);
                    let Some(contract) = contract_rows.next() else {
                        return Err(SourceEntryHomeIssueV1::ParameterRelationMissing {
                            _batch_slot: batch_slot,
                        });
                    };
                    if contract_rows.next().is_some() {
                        return Err(SourceEntryHomeIssueV1::ParameterRelationDuplicate {
                            _batch_slot: batch_slot,
                        });
                    }
                    if contract.owner() != declaration.owner()
                        || contract.mode() != CallableParameterDeclarationModeV1::InstanceBoxMethod
                        || contract.parameters().len()
                            != usize::try_from(declaration.parameter_count()).unwrap_or(usize::MAX)
                    {
                        return Err(SourceEntryHomeIssueV1::ParameterRelationMismatch {
                            _batch_slot: batch_slot,
                        });
                    }
                    let mut ordinals = BTreeSet::new();
                    let mut parameters = Vec::with_capacity(contract.parameters().len());
                    for parameter in contract.parameters() {
                        let site = SourceBindingSiteV1::Parameter {
                            index: parameter.ordinal(),
                        };
                        if parameter.binding().owner() != declaration.owner()
                            || !ordinals.insert(parameter.ordinal())
                            || function.declaration_binding(&site) != Some(parameter.binding())
                        {
                            return Err(SourceEntryHomeIssueV1::ParameterRelationMismatch {
                                _batch_slot: batch_slot,
                            });
                        }
                        parameters.push(VerifiedInstanceEntryHomeParameterV1::issue(
                            parameter.ordinal(),
                            parameter.binding(),
                            parameter.kind(),
                            parameter.home_demand(),
                        ));
                    }
                    Ok(Some(VerifiedInstanceEntryHomeLoanV1::issue(
                        batch_slot,
                        declaration.identity().clone(),
                        declaration.owner(),
                        contract.mode(),
                        receiver,
                        parameters.into_boxed_slice(),
                    )))
                })
                .map_err(|error| SourceEntryHomeIssueV1::BatchLoan { _error: error })??;
            if let Some(loan) = loan {
                loans.push(loan);
            }
        }
        Ok(VerifiedInstanceEntryHomeCatalogV1::issue(
            loans.into_boxed_slice(),
        ))
    }
}
