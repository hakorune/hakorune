//! Selected-only outer port that loans co-sealed callable semantic sources.
//!
//! Raw/reference ports remain unchanged. Deferred selected requests use the
//! existing raw invocation port directly; only a Complete batch can construct
//! this wrapper and select `callable_semantic_root`.

use std::collections::BTreeSet;

use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::resolved_semantics::{BodyChildRoleV1, ExprChildRoleV1};
use crate::mir::{MirBuilder, ValueId};

use super::callable_declaration_catalog::SelectedNormalCallableKeyV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_callable_semantic_source::{
    VerifiedNormalCallableSemanticLoanV1, VerifiedNormalCallableSemanticSourceV1,
};
use super::normal_cataloged_box_method_admission::NormalCatalogedBoxMethodDraftAdmissionV1;
use super::normal_top_level_function_admission::NormalTopLevelFunctionDraftAdmissionV1;
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawInvocationChildPortV1, RecursiveChildLoweringPortV1,
};

pub(super) struct NormalCallableSemanticLoanPortV1<'loan, 'port, 'collector> {
    inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
    source: &'loan VerifiedNormalCallableSemanticSourceV1<'loan>,
    consumption: CallableLoanConsumptionV1,
}

struct CallableLoanConsumptionV1 {
    expected: BTreeSet<SelectedNormalCallableKeyV1>,
    consumed: BTreeSet<SelectedNormalCallableKeyV1>,
}

impl CallableLoanConsumptionV1 {
    fn new(source: &VerifiedNormalCallableSemanticSourceV1<'_>) -> Self {
        Self {
            expected: source.keys().cloned().collect(),
            consumed: BTreeSet::new(),
        }
    }

    fn consume(&mut self, key: SelectedNormalCallableKeyV1) -> Result<(), String> {
        if !self.expected.contains(&key) {
            return Err("[freeze:contract][mir/callable-semantic/missing-loan]".to_owned());
        }
        if !self.consumed.insert(key) {
            return Err("[freeze:contract][mir/callable-semantic/duplicate-loan]".to_owned());
        }
        Ok(())
    }

    fn complete(self) -> Result<(), String> {
        if self.consumed != self.expected {
            return Err("[freeze:contract][mir/callable-semantic/unconsumed-loan]".to_owned());
        }
        Ok(())
    }
}

impl<'loan, 'port, 'collector> NormalCallableSemanticLoanPortV1<'loan, 'port, 'collector> {
    pub(super) fn new(
        inner: &'loan mut RawInvocationChildPortV1<'port, 'collector>,
        source: &'loan VerifiedNormalCallableSemanticSourceV1<'loan>,
    ) -> Self {
        Self {
            inner,
            source,
            consumption: CallableLoanConsumptionV1::new(source),
        }
    }

    fn consume(
        &mut self,
        key: SelectedNormalCallableKeyV1,
    ) -> Result<VerifiedNormalCallableSemanticLoanV1<'loan>, String> {
        let loan = self.source.loan(&key)?;
        self.consumption.consume(key)?;
        Ok(loan)
    }

    pub(super) fn complete(self) -> Result<(), String> {
        self.consumption.complete()
    }

    fn with_callable_source_scope<R>(
        &mut self,
        loan: VerifiedNormalCallableSemanticLoanV1<'loan>,
        execute: impl FnOnce(
            &mut RawInvocationChildPortV1<'port, 'collector>,
            super::raw_invocation_source_transport::RawInvocationSourceTransportV1<()>,
        ) -> R,
    ) -> R {
        let script_ledger = self.inner.semantic_ledger.take();
        let result = loan.with_source_transport(|transport| execute(self.inner, transport));
        self.inner.semantic_ledger = script_ledger;
        result
    }
}

#[cfg(test)]
mod tests {
    use super::CallableLoanConsumptionV1;
    use crate::mir::builder::callable_declaration_catalog::{
        SelectedNormalCallableKeyV1, VerifiedSameModuleCallableDeclarationCatalogV1,
    };
    use crate::parser::NyashParser;
    use std::collections::BTreeSet;

    fn keys() -> Vec<SelectedNormalCallableKeyV1> {
        let program = NyashParser::parse_from_string(
            "function first() { return 1 } function second() { return 2 }",
        )
        .unwrap();
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(&program)
            .unwrap()
            .selected_source_inventory()
            .entries()
            .map(|(key, _)| key.clone())
            .collect()
    }

    fn tracker(
        expected: impl IntoIterator<Item = SelectedNormalCallableKeyV1>,
    ) -> CallableLoanConsumptionV1 {
        CallableLoanConsumptionV1 {
            expected: expected.into_iter().collect::<BTreeSet<_>>(),
            consumed: BTreeSet::new(),
        }
    }

    #[test]
    fn callable_loan_consumption_rejects_missing_duplicate_and_unconsumed_rows() {
        let keys = keys();
        let mut missing = tracker([keys[0].clone()]);
        assert!(missing
            .consume(keys[1].clone())
            .unwrap_err()
            .contains("missing-loan"));

        let mut duplicate = tracker([keys[0].clone()]);
        duplicate.consume(keys[0].clone()).unwrap();
        assert!(duplicate
            .consume(keys[0].clone())
            .unwrap_err()
            .contains("duplicate-loan"));

        assert!(tracker([keys[0].clone()])
            .complete()
            .unwrap_err()
            .contains("unconsumed-loan"));
        let mut complete = tracker([keys[0].clone()]);
        complete.consume(keys[0].clone()).unwrap();
        complete.complete().unwrap();
    }
}

impl RecursiveChildLoweringPortV1 for NormalCallableSemanticLoanPortV1<'_, '_, '_> {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        input: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        self.inner.lower_body(builder, input)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.inner.lower_statement(builder, input)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        input: ASTNode,
    ) -> Result<ValueId, String> {
        self.inner.lower_expression(builder, input)
    }

    fn prepare_expression_child_source_v1(
        &self,
        parent: &ASTNode,
        role: ExprChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner.prepare_expression_child_source_v1(parent, role)
    }

    fn prepare_body_child_source_v1(
        &self,
        parent: &ASTNode,
        role: BodyChildRoleV1,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner.prepare_body_child_source_v1(parent, role)
    }

    fn prepare_body_statement_source_v1(
        &self,
        statement: &ASTNode,
        index: usize,
    ) -> Result<PreparedRawChildSourceV1, String> {
        self.inner
            .prepare_body_statement_source_v1(statement, index)
    }

    fn with_prepared_child_source_v1<R>(
        &mut self,
        prepared: PreparedRawChildSourceV1,
        execute: impl FnOnce(&mut Self) -> R,
    ) -> R {
        match prepared {
            PreparedRawChildSourceV1::Preserve => execute(self),
            PreparedRawChildSourceV1::Exact(source) => {
                let parent = self.inner.active_source.replace(source);
                let result = execute(self);
                self.inner.active_source = parent;
                result
            }
        }
    }
}

impl RawBoxMethodChildPortV1 for NormalCallableSemanticLoanPortV1<'_, '_, '_> {
    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String> {
        self.inner.lower_static_main_box(builder, box_name, methods)
    }

    fn lower_nested_box_method(
        &mut self,
        builder: &mut MirBuilder,
        input: super::nested_box_method_source::NestedBoxMethodLoweringInputV1,
    ) -> Result<(), String> {
        self.inner.lower_nested_box_method(builder, input)
    }
}

impl RootCallableCapturePortV1 for NormalCallableSemanticLoanPortV1<'_, '_, '_> {
    #[allow(clippy::too_many_arguments)]
    fn lower_normal_instance_constructor(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &super::normal_instance_constructor_admission::NormalInstanceConstructorSourceKeyV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.inner
            .lower_normal_instance_constructor_v1(
                builder,
                source_key,
                params,
                param_decls,
                return_type_name,
                body,
                uses,
                attrs,
            )
            .map_err(|error| error.to_string())
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_normal_top_level_function(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalTopLevelFunctionDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        let key = SelectedNormalCallableKeyV1::TopLevel(admission.source_key().clone());
        let loan = self.consume(key)?;
        self.with_callable_source_scope(loan, |inner, transport| {
            inner
                .lower_normal_top_level_function_with_source_v1(
                    builder,
                    admission,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    transport,
                )
                .map_err(|error| error.to_string())
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_static_box_method(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        let source_key = admission.source_key().clone();
        let loan = self.consume(SelectedNormalCallableKeyV1::Cataloged(source_key.clone()))?;
        self.with_callable_source_scope(loan, |inner, transport| {
            inner
                .lower_normal_cataloged_static_box_method_with_source_v1(
                    builder,
                    admission,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    transport,
                )
                .map_err(|error| error.to_string())
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn lower_cataloged_instance_box_method(
        &mut self,
        builder: &mut MirBuilder,
        admission: NormalCatalogedBoxMethodDraftAdmissionV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        let source_key = admission.source_key().clone();
        let loan = self.consume(SelectedNormalCallableKeyV1::Cataloged(source_key.clone()))?;
        self.with_callable_source_scope(loan, |inner, transport| {
            inner
                .lower_normal_cataloged_instance_box_method_with_source_v1(
                    builder,
                    admission,
                    params,
                    param_decls,
                    return_type_name,
                    body,
                    uses,
                    attrs,
                    transport,
                )
                .map_err(|error| error.to_string())
        })
    }
}
