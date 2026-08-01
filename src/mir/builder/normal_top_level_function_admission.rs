//! Selected-normal source/physical admission for one top-level function.
//!
//! A top-level declaration occurrence is not a Box method.  Its source key is
//! therefore local to the Program work plan, while its physical draft contract
//! deliberately preserves the legacy `name/arity` collector identity.

use super::callable_declaration_catalog::SelectedTopLevelFunctionKeyV1;
use super::calls::LegacyFunctionPendingSessionV1;
use super::module_draft_collector::{FunctionDraftKeyV1, ModuleDraftAdmissionErrorV1};
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::MirBuilder;

/// One declaration occurrence in the source-order Program statement vector.
///
/// `statement_index` distinguishes source declarations that project to the
/// same legacy physical symbol.  It is not a collector key and never changes
/// legacy replacement behavior.
/// Cloning transports the same Program occurrence into the recursive source
/// carrier; it does not issue another source identity.
/// One selected-normal top-level declaration paired with its legacy physical
/// draft relation.  It owns no AST, Builder borrow, or collector state.
#[derive(Debug)]
pub(in crate::mir::builder) struct NormalTopLevelFunctionDraftAdmissionV1 {
    source_key: SelectedTopLevelFunctionKeyV1,
    physical_symbol: Box<str>,
    physical_arity: usize,
    _seal: NormalTopLevelFunctionDraftAdmissionSealV1,
}

#[derive(Debug)]
struct NormalTopLevelFunctionDraftAdmissionSealV1;

impl NormalTopLevelFunctionDraftAdmissionV1 {
    pub(in crate::mir::builder) fn from_catalog_key(
        source_key: SelectedTopLevelFunctionKeyV1,
    ) -> Self {
        let physical_symbol = format!(
            "{}/{}",
            source_key.declared_name(),
            source_key.declared_arity()
        )
        .into_boxed_str();
        let physical_arity = source_key.declared_arity();
        Self {
            source_key,
            physical_symbol,
            physical_arity,
            _seal: NormalTopLevelFunctionDraftAdmissionSealV1,
        }
    }

    pub(in crate::mir::builder) fn source_key(&self) -> &SelectedTopLevelFunctionKeyV1 {
        &self.source_key
    }

    pub(in crate::mir::builder) fn physical_symbol(&self) -> &str {
        &self.physical_symbol
    }

    pub(in crate::mir::builder) const fn physical_arity(&self) -> usize {
        self.physical_arity
    }

    pub(in crate::mir::builder) fn into_legacy_collector_parts(
        self,
    ) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            source_key: _,
            physical_symbol,
            physical_arity,
            _seal: _,
        } = self;
        let symbol = physical_symbol.into_string();
        (
            FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
            symbol,
            physical_arity,
        )
    }
}

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn commit_normal_top_level_function_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.commit_legacy_symbol_pending(pending, admission.into_legacy_collector_parts())
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_top_level_function_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let source_root =
            super::raw_invocation_source_transport::RawInvocationRootLineageV1::TopLevel(
                admission.source_key().clone(),
            );
        self.lower_normal_top_level_function_with_source_v1(
            builder,
            admission,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
            super::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
                (),
                source_root,
            ),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_top_level_function_with_source_v1(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
        source_transport: super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        if params.len() != admission.physical_arity() {
            return Err(ModuleLoweringPortChildErrorV1::Admission(
                ModuleDraftAdmissionErrorV1::ArityMismatch {
                    symbol: admission.physical_symbol().to_owned(),
                    expected: admission.physical_arity(),
                    actual: params.len(),
                },
            ));
        }
        let function_name = admission.physical_symbol().to_owned();
        builder.observe_legacy_method_lowering_v1(&function_name, &body, None);
        let pending = super::raw_invocation_source_transport::RawSourceTransportPortV1::
            with_source_transport_v1(
                self,
                source_transport,
                |port, ()| {
                    port.capture_static_box_method_pending_v1(
                        builder,
                        function_name,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )
                },
            )?;
        self.commit_normal_top_level_function_pending_v1(pending, admission)
    }
}
