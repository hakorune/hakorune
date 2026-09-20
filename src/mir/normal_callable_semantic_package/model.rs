use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, CatalogedBoxMethodPhysicalHeaderProjectionV1,
    SameModuleCallableNamespaceV1, VerifiedSourceBackedDynamicCallableV1,
    VerifiedSourceBackedSameModuleCallableCatalogV1,
};
use crate::mir::callable_parameter_contract::{
    CallableParameterContractKindV1, CallableParameterDeclarationModeV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicExitTransactionCoSealV1;
use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};
use std::collections::BTreeMap;
use std::rc::Rc;

use crate::mir::source_call_target::{
    VerifiedSourceBoundCoreMethodCallV1, VerifiedStaticImportAliasViewV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum QualifiedReceiverCatalogAdmissionV1 {
    ImportedAlias,
    DirectCanonicalOwner,
}

/// Owned relation-only handoff for qualified Main method calls.
///
/// This is deliberately not a target, loan, Recipe, ABI, or physical symbol.
/// The lifecycle-owned import view is borrowed while these rows are issued;
/// only the exact relation survives in the package.
#[derive(Debug)]
pub(crate) struct VerifiedQualifiedReceiverCatalogRowV1 {
    caller: CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    receiver: Box<str>,
    canonical_owner: Box<str>,
    declaration_key: CanonicalSameModuleCallableKeyV1,
    selector: Box<str>,
    arity: u32,
    argument_sites: Box<[SourceExprSiteV1]>,
    admission: QualifiedReceiverCatalogAdmissionV1,
}

impl VerifiedQualifiedReceiverCatalogRowV1 {
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.caller
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) fn receiver(&self) -> &str {
        &self.receiver
    }

    pub(crate) fn canonical_owner(&self) -> &str {
        &self.canonical_owner
    }

    pub(crate) fn declaration_key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.declaration_key
    }

    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }

    pub(crate) const fn arity(&self) -> u32 {
        self.arity
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(crate) const fn admission(&self) -> QualifiedReceiverCatalogAdmissionV1 {
        self.admission
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedQualifiedReceiverCatalogRelationV1 {
    rows: Box<[VerifiedQualifiedReceiverCatalogRowV1]>,
    consumed: std::collections::BTreeSet<SourceExprSiteV1>,
}

#[derive(Debug)]
pub(crate) struct QualifiedReceiverCatalogTakeV1 {
    declaration_key: CanonicalSameModuleCallableKeyV1,
    argument_sites: Box<[SourceExprSiteV1]>,
}

impl VerifiedQualifiedReceiverCatalogRelationV1 {
    pub(crate) fn rows(&self) -> &[VerifiedQualifiedReceiverCatalogRowV1] {
        &self.rows
    }

    pub(crate) fn take_for_source(
        &mut self,
        caller: &CanonicalSameModuleCallableKeyV1,
        site: &SourceExprSiteV1,
        receiver: &str,
        selector: &str,
        arity: u32,
    ) -> Result<Option<QualifiedReceiverCatalogTakeV1>, Box<str>> {
        let Some(row) = self.rows.iter().find(|row| row.site() == site) else {
            return Ok(None);
        };
        if row.caller() != caller {
            return Err("[freeze:contract][mir/main-qualified-relation/caller-mismatch]".into());
        }
        if row.receiver() != receiver {
            return Err("[freeze:contract][mir/main-qualified-relation/receiver-mismatch]".into());
        }
        if row.selector() != selector || row.arity() != arity {
            return Err(
                "[freeze:contract][mir/main-qualified-relation/declaration-mismatch]".into(),
            );
        }
        if !self.consumed.insert(site.clone()) {
            return Err("[freeze:contract][mir/main-qualified-relation/already-taken]".into());
        }
        Ok(Some(QualifiedReceiverCatalogTakeV1 {
            declaration_key: row.declaration_key().clone(),
            argument_sites: row.argument_sites().to_vec().into_boxed_slice(),
        }))
    }

    pub(crate) fn finish_empty(&self) -> Result<(), Box<str>> {
        if self.consumed.len() != self.rows.len() {
            return Err(format!(
                "[freeze:contract][mir/main-qualified-relation/residual-rows] consumed={} total={}",
                self.consumed.len(),
                self.rows.len()
            )
            .into());
        }
        Ok(())
    }
}

impl QualifiedReceiverCatalogTakeV1 {
    pub(crate) fn declaration_key(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.declaration_key
    }

    pub(crate) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }
}

#[derive(Debug)]
pub(super) struct OwnedCallableParameterContractV1 {
    pub(super) ordinal: u32,
    pub(super) binding: BindingRefV1,
    pub(super) kind: CallableParameterContractKindV1,
}

#[derive(Debug)]
pub(super) struct OwnedCallableParameterContractDeclarationV1 {
    pub(super) batch_slot: u32,
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) mode: CallableParameterDeclarationModeV1,
    pub(super) parameters: Box<[OwnedCallableParameterContractV1]>,
}

/// The sole owned pre-Builder semantic package for one complete callable batch.
///
/// This product is deliberately non-`Clone` and has no consuming parts API.
/// Later stages must transform it whole instead of pairing a foreign batch,
/// parameter catalog, Dynamic candidate, or private batch slot.
#[derive(Debug)]
pub(crate) struct VerifiedNormalCallableSemanticPackageV1 {
    pub(super) root_execution: NormalRootExecutionPackageStateV1,
    pub(super) catalog: VerifiedSourceBackedSameModuleCallableCatalogV1,
    pub(super) batch: VerifiedResolvedCallableSemanticBatchV1,
    pub(super) direct_call_loans: Option<super::direct_call_loan::DirectCallDispositionLoansV1>,
    pub(super) map_read_facts: super::map_read_fact::MapReadFactsV1,
    pub(super) ordinary_new_claim_ledger:
        std::rc::Rc<super::ordinary_new_coseal::OrdinaryNewClaimLedgerV1>,
    pub(super) instance_constructors:
        super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1,
    pub(super) selected: super::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    pub(super) parameter_contracts: Box<[OwnedCallableParameterContractDeclarationV1]>,
    pub(super) result_contracts: super::result_contract::VerifiedCallableResultContractCohortV1,
    pub(super) physical_signature:
        super::physical_signature::VerifiedCallablePhysicalSignatureCohortV1,
    pub(super) declared_instance_call_locators:
        super::declared_instance_locator::DeclaredInstanceCallPackageLocatorDispositionV1,
    pub(super) s6c_child: Option<super::s6c_child::VerifiedS6CSemanticChildV1>,
    pub(super) s6c_storage_header:
        Option<super::s6c_storage_header::VerifiedS6CStorageHeaderProjectionV1>,
    pub(super) physical_header: super::physical_header::VerifiedCallablePhysicalHeaderCohortV1,
    pub(super) dynamic: NormalCallableDynamicProjectionV1,
    pub(super) dynamic_physical_header: Option<CatalogedBoxMethodPhysicalHeaderProjectionV1>,
    pub(super) loop_break_source: super::loop_break_source::VerifiedLoopBreakSourcePackageV1,
    pub(super) source_core_method_calls: BTreeMap<
        crate::mir::builder::SelectedNormalCallableKeyV1,
        BTreeMap<
            crate::mir::resolved_semantics::SourceExprSiteV1,
            VerifiedSourceBoundCoreMethodCallV1,
        >,
    >,
    pub(super) app_main_qualified_receiver_catalog:
        Option<VerifiedQualifiedReceiverCatalogRelationV1>,
}

#[derive(Debug)]
pub(super) enum NormalRootExecutionPackageStateV1 {
    Prepared(crate::mir::builder::PreparedAdmittedNormalRootExpansionV1),
    MovedToLowering,
}

#[derive(Debug)]
pub(super) enum NormalCallableDynamicProjectionV1 {
    ValidUnselected,
    Selected {
        batch_slot: u32,
        _owner: FunctionOwnerIdV1,
        source: Rc<VerifiedSourceBackedDynamicCallableV1>,
        program: VerifiedDynamicExitTransactionCoSealV1,
        result: Option<crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1>,
    },
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir) enum NormalCallableDynamicProjectionRefV1<'package> {
    ValidUnselected,
    Selected {
        program: &'package VerifiedDynamicExitTransactionCoSealV1,
    },
}

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(crate) fn issue_app_main_qualified_receiver_catalog_relation(
        &self,
        imports: &VerifiedStaticImportAliasViewV1<'_>,
    ) -> Result<Option<VerifiedQualifiedReceiverCatalogRelationV1>, Box<str>> {
        let Some(app_main) = self.catalog.catalog().source_backed_app_main() else {
            return Ok(None);
        };
        if !imports.is_branded_by(self.catalog.catalog()) {
            return Err("[freeze:contract][mir/main-import-view/catalog-brand]".into());
        }
        let Some((main_slot, _callable_index)) = self.batch.main_callable_index() else {
            return Ok(None);
        };
        let caller = app_main.catalog_key().clone();
        let mut rows = Vec::new();
        self.batch
            .with_lowering_input(main_slot, |input| {
                let ledger = input
                    .forest()
                    .callable_source_ledger(input.owner())
                    .map_err(|error| format!("[mir/main-import-view/ledger] {error:?}"))?;
                for (site, call) in ledger.method_calls() {
                    if call.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound {
                        continue;
                    }
                    let Some(identity) = call.qualified_receiver_identity() else {
                        return Err(
                            "[freeze:contract][mir/main-import-view/missing-receiver-identity]"
                                .to_owned(),
                        );
                    };
                    let receiver = identity.source_name();
                    if receiver.is_empty() || matches!(receiver, "__mir__" | "__repl" | "mem") {
                        return Err(
                            "[freeze:contract][mir/main-import-view/reserved-receiver]".to_owned()
                        );
                    }
                    let (admission, canonical_owner) = match imports.canonical_owner(receiver) {
                        Some(owner) => (QualifiedReceiverCatalogAdmissionV1::ImportedAlias, owner),
                        None => (
                            QualifiedReceiverCatalogAdmissionV1::DirectCanonicalOwner,
                            receiver,
                        ),
                    };
                    let Some(declaration) = self.catalog.catalog().declaration_for(
                        SameModuleCallableNamespaceV1::StaticBoxMethod,
                        canonical_owner,
                        call.selector(),
                        usize::try_from(call.arity()).map_err(|_| {
                            "[freeze:contract][mir/main-import-view/arity-overflow]".to_owned()
                        })?,
                    ) else {
                        return Err(
                            "[freeze:contract][mir/main-import-view/declaration-mismatch]"
                                .to_owned(),
                        );
                    };
                    if declaration.key().namespace()
                        != SameModuleCallableNamespaceV1::StaticBoxMethod
                        || declaration.key().owner() != canonical_owner
                        || declaration.key().name() != call.selector()
                        || declaration.key().arity() != call.arity()
                    {
                        return Err(
                            "[freeze:contract][mir/main-import-view/declaration-shape]".to_owned()
                        );
                    }
                    let selected_key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(
                        declaration.key().clone(),
                    );
                    let Some(batch_slot) = self.selected.batch_slot(&selected_key) else {
                        return Err(
                            "[freeze:contract][mir/main-import-view/selected-header-missing]"
                                .to_owned(),
                        );
                    };
                    let Some(header) = self.physical_header.row(batch_slot, &self.result_contracts)
                    else {
                        return Err(
                            "[freeze:contract][mir/main-import-view/selected-header-missing]"
                                .to_owned(),
                        );
                    };
                    let exact_i64_params = self
                        .parameter_contracts
                        .iter()
                        .find(|row| row.batch_slot == batch_slot)
                        .is_some_and(|row| {
                            row.parameters.iter().all(|parameter| {
                                parameter.kind
                                    == CallableParameterContractKindV1::ExactTrivial(
                                        ExactTrivialParameterAbiV1::I64,
                                    )
                            })
                        });
                    if header.result() != ExactTrivialScalarAbiV1::I64 || !exact_i64_params {
                        return Err(
                            "[freeze:contract][mir/main-import-view/selected-header-not-exact-i64]"
                                .to_owned(),
                        );
                    }
                    let argument_sites = call
                        .arguments()
                        .iter()
                        .enumerate()
                        .map(|(index, argument)| {
                            if argument.ordinal() != index as u32 {
                                return Err(
                                    "[freeze:contract][mir/main-import-view/argument-ordinal]"
                                        .to_owned(),
                                );
                            }
                            Ok(argument.site().clone())
                        })
                        .collect::<Result<Vec<_>, String>>()?;
                    if argument_sites.len() != call.arity() as usize {
                        return Err(
                            "[freeze:contract][mir/main-import-view/argument-cardinality]"
                                .to_owned(),
                        );
                    }
                    rows.push(VerifiedQualifiedReceiverCatalogRowV1 {
                        caller: caller.clone(),
                        site: site.clone(),
                        receiver: receiver.into(),
                        canonical_owner: canonical_owner.into(),
                        declaration_key: declaration.key().clone(),
                        selector: call.selector().into(),
                        arity: call.arity(),
                        argument_sites: argument_sites.into_boxed_slice(),
                        admission,
                    });
                }
                Ok::<_, String>(())
            })
            .map_err(|error| format!("[mir/main-import-view/batch] {error:?}"))??;
        Ok(Some(VerifiedQualifiedReceiverCatalogRelationV1 {
            rows: rows.into_boxed_slice(),
            consumed: std::collections::BTreeSet::new(),
        }))
    }

    pub(crate) fn retain_app_main_qualified_receiver_catalog(
        &mut self,
        relation: VerifiedQualifiedReceiverCatalogRelationV1,
    ) -> Result<(), Box<str>> {
        if self
            .app_main_qualified_receiver_catalog
            .replace(relation)
            .is_some()
        {
            return Err("[freeze:contract][mir/main-import-view/duplicate-relation]".into());
        }
        Ok(())
    }

    pub(crate) fn map_read_facts(&self) -> &super::map_read_fact::MapReadFactsV1 {
        &self.map_read_facts
    }

    pub(in crate::mir) fn take_root_execution(
        &mut self,
    ) -> Result<crate::mir::builder::PreparedAdmittedNormalRootExpansionV1, ()> {
        match std::mem::replace(
            &mut self.root_execution,
            NormalRootExecutionPackageStateV1::MovedToLowering,
        ) {
            NormalRootExecutionPackageStateV1::Prepared(root) => Ok(root),
            NormalRootExecutionPackageStateV1::MovedToLowering => Err(()),
        }
    }

    pub(crate) fn source_ast(&self) -> &crate::ast::ASTNode {
        self.batch.source_ast()
    }

    pub(crate) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.batch.with_normal_program_source_loan(callback)
    }

    /// Borrow the declaration catalog that is owned by this same source
    /// package.  Lookup issuers use this accessor together with the package's
    /// HRTB source loan, so a foreign catalog cannot be supplied independently.
    pub(crate) fn declaration_catalog(
        &self,
    ) -> &crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1 {
        self.catalog.catalog()
    }

    pub(crate) fn instance_constructors(
        &self,
    ) -> &super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticBatchV1 {
        &self.instance_constructors
    }

    pub(super) fn declared_instance_call_locators(
        &self,
    ) -> &super::declared_instance_locator::DeclaredInstanceCallPackageLocatorDispositionV1 {
        &self.declared_instance_call_locators
    }

    #[cfg(test)]
    pub(crate) fn batch(&self) -> &VerifiedResolvedCallableSemanticBatchV1 {
        &self.batch
    }

    #[cfg(test)]
    pub(crate) fn loop_break_source_row_count(&self) -> usize {
        self.loop_break_source.rows().len()
    }

    #[cfg(test)]
    pub(crate) fn loop_break_source_candidate_count(&self) -> usize {
        self.loop_break_source.candidate_count()
    }

    #[cfg(test)]
    pub(crate) fn has_direct_call_loan(&self) -> bool {
        self.direct_call_loans.is_some()
    }

    pub(crate) fn selected_callable_sources(
        &self,
    ) -> &crate::mir::builder::VerifiedSelectedNormalCallableSourceInventoryV1 {
        self.catalog.catalog().selected_source_inventory()
    }

    #[cfg(test)]
    pub(crate) fn parameter_declaration_count(&self) -> usize {
        self.parameter_contracts.len()
    }

    #[cfg(test)]
    pub(crate) fn parameter_count(&self) -> usize {
        self.parameter_contracts
            .iter()
            .map(|row| row.parameters.len())
            .sum()
    }

    #[cfg(test)]
    pub(crate) fn physical_signature(
        &self,
    ) -> &super::physical_signature::VerifiedCallablePhysicalSignatureCohortV1 {
        &self.physical_signature
    }

    #[cfg(test)]
    pub(in crate::mir) fn dynamic_projection(&self) -> NormalCallableDynamicProjectionRefV1<'_> {
        match &self.dynamic {
            NormalCallableDynamicProjectionV1::ValidUnselected => {
                NormalCallableDynamicProjectionRefV1::ValidUnselected
            }
            NormalCallableDynamicProjectionV1::Selected { program, .. } => {
                NormalCallableDynamicProjectionRefV1::Selected { program }
            }
        }
    }
}
