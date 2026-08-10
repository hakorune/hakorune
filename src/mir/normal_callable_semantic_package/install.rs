//! Consuming source-backed catalog installation and scoped selected loans.

use std::collections::BTreeSet;

use crate::mir::builder::{
    CompilationContext, SameModuleCallableCatalogBrandV1, SelectedNormalCallableKeyV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::VerifiedDynamicCallableCompletionProjectionV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, HomeDemandV1};

use super::model::{
    NormalCallableDynamicProjectionV1, OwnedCallableParameterDemandDeclarationV1,
    VerifiedNormalCallableSemanticPackageV1,
};
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableSemanticPackageInstallIssueV1 {
    ForeignCatalog,
    SelectedKeyUnavailable,
    DuplicateSelectedKey,
    IncompleteSelectedCoverage,
    BatchLoan,
}

#[derive(Debug)]
pub(crate) struct InstalledNormalCallableSemanticPackageV1 {
    catalog_brand: SameModuleCallableCatalogBrandV1,
    batch: crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: VerifiedSelectedCallableBatchMapV1,
    parameter_demands: Box<[OwnedCallableParameterDemandDeclarationV1]>,
    dynamic: NormalCallableDynamicProjectionV1,
}

pub(crate) struct SelectedCallableLoweringInputRefV1<'loan> {
    source: ResolvedFunctionLoweringInputV1<'loan>,
    parameter_demands: &'loan [super::model::OwnedCallableParameterDemandV1],
    semantic: SelectedCallableSemanticRefV1<'loan>,
}

#[derive(Clone, Copy)]
pub(crate) enum SelectedCallableSemanticRefV1<'loan> {
    Ordinary,
    Dynamic {
        program: &'loan VerifiedDynamicCallableCompletionProjectionV1,
    },
}

pub(crate) struct PreparedNormalCallableSemanticPackageInstallV1<'context> {
    context: &'context mut CompilationContext,
    package: VerifiedNormalCallableSemanticPackageV1,
}

/// Exactly-once lowering surface for one installed package.
///
/// The port borrows the whole installed package and never reveals a batch
/// slot.  It records selected-key consumption and must be completed before
/// the outer lowering transaction can close.
pub(crate) struct NormalCallableSemanticPackagePortV1<'package> {
    installed: &'package InstalledNormalCallableSemanticPackageV1,
    consumed: BTreeSet<SelectedNormalCallableKeyV1>,
}

impl VerifiedNormalCallableSemanticPackageV1 {
    pub(crate) fn prepare_install<'context>(
        self,
        context: &'context mut CompilationContext,
    ) -> Result<PreparedNormalCallableSemanticPackageInstallV1<'context>, Self> {
        if !context.callable_declaration_catalog_vacant() {
            return Err(self);
        }
        Ok(PreparedNormalCallableSemanticPackageInstallV1 {
            context,
            package: self,
        })
    }
}

impl PreparedNormalCallableSemanticPackageInstallV1<'_> {
    pub(crate) fn commit(self) -> InstalledNormalCallableSemanticPackageV1 {
        let VerifiedNormalCallableSemanticPackageV1 {
            catalog,
            batch,
            selected,
            parameter_demands,
            dynamic,
        } = self.package;
        let catalog_brand = catalog.catalog().brand().clone();
        self.context
            .install_callable_declaration_catalog_preflighted(catalog.into_catalog());
        InstalledNormalCallableSemanticPackageV1 {
            catalog_brand,
            batch,
            selected,
            parameter_demands,
            dynamic,
        }
    }
}

impl InstalledNormalCallableSemanticPackageV1 {
    pub(crate) fn source_ast(&self) -> &crate::ast::ASTNode {
        self.batch.source_ast()
    }

    pub(crate) fn installed_in(&self, context: &CompilationContext) -> bool {
        context
            .callable_declaration_catalog()
            .is_ok_and(|catalog| catalog.brand().is_same(&self.catalog_brand))
    }

    pub(crate) fn begin_lowering(
        &self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if !self.installed_in(context) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::ForeignCatalog);
        }
        Ok(NormalCallableSemanticPackagePortV1 {
            installed: self,
            consumed: BTreeSet::new(),
        })
    }

    fn with_selected_lowering_input<R>(
        &self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(SelectedCallableLoweringInputRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        let batch_slot = self
            .selected
            .batch_slot(key)
            .ok_or(NormalCallableSemanticPackageInstallIssueV1::SelectedKeyUnavailable)?;
        let parameters = self
            .parameter_demands
            .iter()
            .find(|row| row.batch_slot == batch_slot)
            .map(|row| row.parameters.as_ref())
            .unwrap_or(&[]);
        let semantic = match &self.dynamic {
            NormalCallableDynamicProjectionV1::Selected {
                batch_slot: dynamic_slot,
                program,
                ..
            } if *dynamic_slot == batch_slot => SelectedCallableSemanticRefV1::Dynamic { program },
            _ => SelectedCallableSemanticRefV1::Ordinary,
        };
        self.batch
            .with_lowering_input(batch_slot, |source| {
                callback(SelectedCallableLoweringInputRefV1 {
                    source,
                    parameter_demands: parameters,
                    semantic,
                })
            })
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)
    }
}

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(crate) fn with_selected_lowering_input<R>(
        &mut self,
        key: &SelectedNormalCallableKeyV1,
        callback: impl for<'loan> FnOnce(SelectedCallableLoweringInputRefV1<'loan>) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.consumed.contains(key) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::DuplicateSelectedKey);
        }
        let result = self.installed.with_selected_lowering_input(key, callback)?;
        self.consumed.insert(key.clone());
        Ok(result)
    }

    pub(crate) fn complete(self) -> Result<(), NormalCallableSemanticPackageInstallIssueV1> {
        if self.consumed.len() != self.installed.selected.keys().len()
            || self
                .installed
                .selected
                .keys()
                .any(|key| !self.consumed.contains(key))
        {
            return Err(NormalCallableSemanticPackageInstallIssueV1::IncompleteSelectedCoverage);
        }
        Ok(())
    }
}

impl SelectedCallableLoweringInputRefV1<'_> {
    pub(crate) fn source(&self) -> ResolvedFunctionLoweringInputV1<'_> {
        self.source
    }

    pub(crate) fn parameter_demands(
        &self,
    ) -> impl ExactSizeIterator<Item = (u32, BindingRefV1, HomeDemandV1)> + '_ {
        self.parameter_demands
            .iter()
            .map(|row| (row.ordinal, row.binding, row.demand))
    }

    pub(crate) fn semantic(&self) -> SelectedCallableSemanticRefV1<'_> {
        self.semantic
    }
}
