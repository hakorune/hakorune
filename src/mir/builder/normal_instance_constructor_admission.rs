//! Selected-normal source/physical admission for instance constructors.
//!
//! Constructor source identity belongs to the original Program Box occurrence
//! and the parser-owned constructor-map key.  A Script plain-Box runtime prefix
//! may demand the same source row a second time, but that is not a second
//! source occurrence: each physical demand receives a fresh linear admission.

use super::calls::LegacyFunctionPendingSessionV1;
use super::module_draft_collector::FunctionDraftKeyV1;
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::MirBuilder;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalInstanceConstructorSourceKeyV1 {
    statement_index: usize,
    box_name: Box<str>,
    parser_constructor_key: Box<str>,
}

impl NormalInstanceConstructorSourceKeyV1 {
    pub(in crate::mir::builder) fn new(
        statement_index: usize,
        box_name: impl Into<Box<str>>,
        parser_constructor_key: impl Into<Box<str>>,
    ) -> Self {
        Self {
            statement_index,
            box_name: box_name.into(),
            parser_constructor_key: parser_constructor_key.into(),
        }
    }

    pub(in crate::mir::builder) const fn statement_index(&self) -> usize {
        self.statement_index
    }

    pub(in crate::mir::builder) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(in crate::mir::builder) fn parser_constructor_key(&self) -> &str {
        &self.parser_constructor_key
    }
}

/// One immutable source occurrence for every constructor row that survived the
/// parser's constructor-map normalization.  Cloning this receipt transports
/// the same source identity to Script runtime work; it does not issue another
/// source occurrence.
#[derive(Clone, Debug)]
pub(in crate::mir::builder) struct NormalInstanceConstructorSourceBatchV1 {
    sources: Box<[NormalInstanceConstructorSourceKeyV1]>,
}

impl NormalInstanceConstructorSourceBatchV1 {
    pub(in crate::mir::builder) fn new(
        statement_index: usize,
        box_name: &str,
        parser_constructor_keys: Vec<String>,
    ) -> Self {
        Self {
            sources: parser_constructor_keys
                .into_iter()
                .map(|key| {
                    NormalInstanceConstructorSourceKeyV1::new(
                        statement_index,
                        box_name.to_owned(),
                        key,
                    )
                })
                .collect(),
        }
    }

    pub(in crate::mir::builder) fn sources(&self) -> &[NormalInstanceConstructorSourceKeyV1] {
        &self.sources
    }
}

/// One physical constructor lowering demand.  Its source key deliberately
/// stays distinct from the legacy collector identity.
#[derive(Debug)]
pub(in crate::mir::builder) struct NormalInstanceConstructorDraftAdmissionV1 {
    source_key: NormalInstanceConstructorSourceKeyV1,
    physical_symbol: Box<str>,
    physical_arity: usize,
}

impl NormalInstanceConstructorDraftAdmissionV1 {
    pub(in crate::mir::builder) fn seal(
        source_key: NormalInstanceConstructorSourceKeyV1,
        normalized_parameter_count: usize,
    ) -> Self {
        let physical_symbol = format!(
            "{}.{}",
            source_key.box_name(),
            source_key.parser_constructor_key()
        )
        .into_boxed_str();
        Self {
            source_key,
            physical_symbol,
            physical_arity: normalized_parameter_count + 1,
        }
    }

    pub(in crate::mir::builder) fn source_key(&self) -> &NormalInstanceConstructorSourceKeyV1 {
        &self.source_key
    }

    pub(in crate::mir::builder) fn physical_symbol(&self) -> &str {
        &self.physical_symbol
    }

    pub(in crate::mir::builder) const fn physical_arity(&self) -> usize {
        self.physical_arity
    }

    fn into_legacy_collector_parts(self) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            source_key: _,
            physical_symbol,
            physical_arity,
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
    pub(in crate::mir::builder) fn commit_normal_instance_constructor_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: NormalInstanceConstructorDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.commit_legacy_symbol_pending(pending, admission.into_legacy_collector_parts())
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_instance_constructor_v1(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &NormalInstanceConstructorSourceKeyV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let function_name = format!(
            "{}.{}",
            source_key.box_name(),
            source_key.parser_constructor_key()
        );
        let box_name = source_key.box_name().to_owned();
        let (params, param_decls) =
            super::recursive_child_lowering::normalize_instance_box_method_input_v1(
                &function_name,
                params,
                param_decls,
            );
        let admission =
            NormalInstanceConstructorDraftAdmissionV1::seal(source_key.clone(), params.len());
        builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&box_name));
        let pending = self.capture_normalized_instance_box_method_pending_v1(
            builder,
            function_name,
            box_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )?;
        self.commit_normal_instance_constructor_pending_v1(pending, admission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;

    #[test]
    fn one_source_occurrence_materializes_two_legacy_demands() {
        let sources =
            NormalInstanceConstructorSourceBatchV1::new(7, "Page", vec!["birth/0".to_owned()]);
        let first =
            NormalInstanceConstructorDraftAdmissionV1::seal(sources.sources()[0].clone(), 0);
        let second =
            NormalInstanceConstructorDraftAdmissionV1::seal(sources.sources()[0].clone(), 0);

        assert_eq!(first.source_key().statement_index(), 7);
        assert_eq!(first.physical_symbol(), "Page.birth/0");
        assert_eq!(second.physical_arity(), 1);
        let (key, symbol, arity) = second.into_legacy_collector_parts();
        assert_eq!(
            key,
            FunctionDraftKeyV1::LegacySymbol("Page.birth/0".to_owned())
        );
        assert_eq!(symbol, "Page.birth/0");
        assert_eq!(arity, 1);
    }
}
