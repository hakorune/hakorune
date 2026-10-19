//! Borrow the existing closed field/result proof at one original Me read.
//! No second numeric map or physical emitter is introduced here.
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::BorrowedResultCohortV1;
use super::expression::ResultExpressionSourceV1;
use super::*;
use hakorune_mir_defs::CanonicalFieldRefV1;

type Sources = BorrowedResultCohortV1;
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct IntegerFieldSourceLoanV1 {
    facts: Rc<BorrowedResultSourceFactsV1>,
    expression: Rc<ResultExpressionSourceV1>,
    field: super::expression_field::ExpressionFieldSourceLoanV1,
    receiver_declaration: (CanonicalFieldRefV1, Option<Box<str>>),
    class: Box<str>,
    name: Box<str>,
}
impl IntegerFieldSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn field(&self) -> CanonicalFieldRefV1 {
        self.field.field()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(
        &self,
        results: &Sources,
        ingress: &PreparedBorrowedFormalIngressV1,
        site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        home: BindingRefV1,
        field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        let proof = results
            .get(&site.owner())
            .ok_or_else(|| freeze("integer-field/result-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        proof.require_source_i64_v1()?;
        results.require_field_component_v1(proof.field_component.as_ref()
            .ok_or_else(|| freeze("integer-field/component-missing"))?)?;
        if !self.field.matches_declaration(self.receiver_declaration.0, self.receiver_declaration.1.as_deref()) {
            return Err(freeze("integer-field/receiver-declaration"));
        }
        if receiver != home
            || !Rc::ptr_eq(&self.facts, &proof.source_facts)
            || self.field() != field
            || proof
                .original_return
                .as_ref()
                .is_none_or(|original| original.legacy_field_receiver_v1() != Some(receiver))
            || !proof
                .expressions
                .iter()
                .any(|row| Rc::ptr_eq(row, &self.expression))
            || proof
                .field_component
                .as_ref()
                .is_none_or(|component| !component.permits(&self.expression))
            || !composition::grounded_source_results_with_ingress_v1(results, ingress)
                .contains(&site.owner())
        {
            return Err(freeze("integer-field/source-identity"));
        }
        let Some((node, source)) = self.expression.field_source(
            site.site(),
            receiver_site,
            receiver,
            &self.class,
            &self.name,
        ) else {
            return Err(freeze("integer-field/original-read"));
        };
        if !self.field.matches_source(node, &source) {
            return Err(freeze("integer-field/source-identity"));
        }
        self.field.recheck(results, ingress, self.expression.plan(), site, receiver_site, receiver, home, field)?;
        composition::source_dependencies_v1(proof, ingress)?;
        Ok(())
    }
}

pub(in crate::mir::normal_callable_semantic_package) fn lend_integer_field_v1(
    results: &Sources,
    ingress_source: &Result<PreparedBorrowedFormalIngressV1, String>,
    input: ResolvedFunctionLoweringInputV1<'_>,
    constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    site: &OwnedExprSiteV1,
    receiver_site: &SourceExprSiteV1,
    receiver: BindingRefV1,
    home: BindingRefV1,
    name: &str,
) -> Result<Option<IntegerFieldSourceLoanV1>, String> {
    let Some((entry, class)) = receiver_proof else {
        return Ok(None);
    };
    if receiver != entry
        || home != entry
        || site.owner() != input.owner()
        || !original::exact_me_receiver_v1(input, receiver_site, entry)
    {
        return Ok(None);
    }
    if original::field_return_values_v1(input, entry)?.is_empty() {
        return Ok(None);
    }
    let proof = results
        .get(&input.owner())
        .ok_or_else(|| freeze("integer-field/result-missing"))?
        .as_ref()
        .map_err(Clone::clone)?;
    // This source family is selected by its passive original Return request,
    // never the success/absence of a numeric field classification.
    let original = proof
        .original_return
        .as_ref()
        .ok_or_else(|| freeze("integer-field/original-return-missing"))?;
    if original.legacy_field_receiver_v1() != Some(entry) {
        return Err(freeze("integer-field/receiver"));
    }
    proof.require_source_i64_v1()?;
    let mut exact = proof.expressions.iter().filter_map(|expression| {
        expression
            .field_source(site.site(), receiver_site, entry, class.name(), name)
            .map(|(node, source)| (expression, node, source))
    });
    let Some((expression, _, source)) = exact.next() else {
        return Ok(None);
    };
    if exact.next().is_some() {
        return Err(freeze("integer-field/duplicate-read"));
    }
    // The SAME entry class source and constructors qualify the receiver's
    // declaration independently of the all-writer Integer proof. Keep None.
    let declaration = super::super::super::terminal_home::source_declared_field(
        constructors, class, site, name,
    ).map_err(|issue| format!("{}: {issue:?}", freeze("integer-field/receiver-declaration")))?
        .ok_or_else(|| freeze("integer-field/receiver-declaration"))?;
    let loan = IntegerFieldSourceLoanV1 {
        facts: Rc::clone(&proof.source_facts),
        expression: Rc::clone(expression),
        field: super::expression_field::ExpressionFieldSourceLoanV1::issue(
            results, ingress_source.as_ref().map_err(Clone::clone)?,
            Rc::clone(expression.plan()), Rc::clone(&source), site, receiver_site, entry, home, name,
        )?,
        receiver_declaration: (declaration.0, declaration.1.map(String::into_boxed_str)),
        class: class.name().into(),
        name: name.into(),
    };
    loan.recheck(
        results,
        ingress_source.as_ref().map_err(Clone::clone)?,
        site,
        receiver_site,
        receiver,
        home,
        loan.field(),
    )?;
    Ok(Some(loan))
}
