//! Source-only field leaf shared by Return and statement-expression adapters.
//! Receiver class/Home qualification and the complete enclosing frame stay with
//! their existing issuer; this loan never grants ownership or execution.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::DeclaredFieldValueSourceV1;
use crate::mir::resolved_value_profile::expression_plan::ExpressionSourcePlanV1;
use hakorune_mir_defs::CanonicalFieldRefV1;

/// Closed numeric value evidence, independent of a direct or nested receiver.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct ClosedNumericFieldSourceLoanV1 {
    component: Rc<composition::ClosedFieldComponentV1>,
    source: Rc<DeclaredFieldValueSourceV1>,
    name: Box<str>,
    container: Box<str>,
}
impl ClosedNumericFieldSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_declaration(
        results: &BorrowedResultCohortV1, ingress: &PreparedBorrowedFormalIngressV1,
        field: CanonicalFieldRefV1, declared: Option<&str>, name: &str,
    ) -> Result<Self, String> {
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        let mut sources = context.sources().values().filter(|row| row.field() == field);
        let source = sources.next().ok_or_else(|| freeze("integer-field/source-missing"))?;
        if sources.next().is_some() { return Err(freeze("integer-field/source-ambiguous")); }
        let loan = Self::prepare(results, Rc::clone(source), name)?;
        loan.recheck(results, ingress)?;
        if !loan.matches_declaration(field, declared) { return Err(freeze("integer-field/receiver-declaration")); }
        Ok(loan)
    }
    fn prepare(results: &BorrowedResultCohortV1,
        source: Rc<DeclaredFieldValueSourceV1>, name: &str) -> Result<Self, String> {
        let component = results.field_component.as_ref().map_err(Clone::clone)?.as_ref()
            .ok_or_else(|| freeze("integer-field/component-missing"))?;
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        let mut keys = context.sources().iter().filter(|(_, row)| Rc::ptr_eq(row, &source));
        let (key, _) = keys.next().ok_or_else(|| freeze("integer-field/source-missing"))?;
        if keys.next().is_some() || key.1.as_ref() != name { return Err(freeze("integer-field/source-ambiguous")); }
        Ok(Self { component: Rc::clone(component), source, name: name.into(), container: key.0.clone() })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(&self,
        results: &BorrowedResultCohortV1, ingress: &PreparedBorrowedFormalIngressV1) -> Result<(), String> {
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        if !context.sources().get(&(self.container.clone(), self.name.clone()))
            .is_some_and(|row| Rc::ptr_eq(row, &self.source)) { return Err(freeze("integer-field/container-source")); }
        composition::recheck_field_source_v1(results, ingress, &self.component, &self.source, &self.name)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn container_class(&self) -> &str { &self.container }
    pub(in crate::mir::normal_callable_semantic_package) fn field(&self) -> CanonicalFieldRefV1 { self.source.field() }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_declaration(&self, field: CanonicalFieldRefV1, declared: Option<&str>) -> bool {
        self.field() == field && self.source.declared_type_name() == declared
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.component, &other.component) && Rc::ptr_eq(&self.source, &other.source)
            && self.name == other.name && self.container == other.container
    }
}

/// Original all-writer class evidence for an intermediate object field.
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct ExpressionObjectFieldSourceLoanV1 {
    source: Rc<crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::ObjectFieldClassSourceV1>,
    key: (Box<str>, Box<str>),
}
impl ExpressionObjectFieldSourceLoanV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_declaration(
        results: &BorrowedResultCohortV1, field: CanonicalFieldRefV1, class: &str, name: &str,
    ) -> Result<Self, String> {
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        let mut rows = context.object_sources().iter().filter(|((_, actual), row)| actual.as_ref() == name && row.field() == Some(field));
        let (key, source) = rows.next().ok_or_else(|| freeze("expression-path/object-source-missing"))?;
        if rows.next().is_some() || source.class() != class { return Err(freeze("expression-path/object-source-drift")); }
        Ok(Self { source: Rc::clone(source), key: key.clone() })
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck(&self, results: &BorrowedResultCohortV1,
        field: CanonicalFieldRefV1, class: &str, name: &str) -> Result<(), String> {
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        if self.key.1.as_ref() != name || self.source.field() != Some(field) || self.source.class() != class
            || !context.object_sources().get(&self.key).is_some_and(|row| Rc::ptr_eq(row, &self.source)) {
            return Err(freeze("expression-path/object-source-drift"));
        }
        Ok(())
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.source, &other.source) && self.key == other.key
    }
    pub(in crate::mir::normal_callable_semantic_package) fn container_class(&self) -> &str { &self.key.0 }
}

#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct ExpressionFieldSourceLoanV1 {
    value: ClosedNumericFieldSourceLoanV1,
    plan: Rc<ExpressionSourcePlanV1>,
    node: usize,
    site: OwnedExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver: BindingRefV1,
    home: BindingRefV1,
}
impl ExpressionFieldSourceLoanV1 {
    /// Pair the common source proof with the separately qualified receiver
    /// declaration. Canonical coordinates alone never prove receiver class.
    pub(in crate::mir::normal_callable_semantic_package) fn from_read(
        results: &BorrowedResultCohortV1,
        ingress: &PreparedBorrowedFormalIngressV1,
        plan: Rc<ExpressionSourcePlanV1>,
        request: &crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadRequestV1,
        declaration: &crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1,
    ) -> Result<Self, String> {
        let context = results.field_candidates.as_ref().map_err(Clone::clone)?;
        let mut sources = context.sources().values().filter(|row| row.field() == declaration.field());
        let source = sources.next().ok_or_else(|| freeze("integer-field/source-missing"))?;
        if sources.next().is_some() { return Err(freeze("integer-field/source-ambiguous")); }
        let loan = Self::issue(results, ingress, plan, Rc::clone(source),
            &request.site, &request.receiver_site, request.receiver, request.home, &request.field)?;
        loan.recheck_read(results, ingress, &request.site, &request.receiver_site,
            request.receiver, request.home, declaration)?;
        Ok(loan)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn matches_plan(&self, plan: &Rc<ExpressionSourcePlanV1>) -> bool {
        Rc::ptr_eq(&self.plan, plan)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn same_source(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.plan, &other.plan) && self.value.same_source(&other.value) && self.node == other.node
            && self.site == other.site && self.receiver_site == other.receiver_site
            && self.receiver == other.receiver && self.home == other.home
    }
    pub(in crate::mir::normal_callable_semantic_package) fn recheck_read(
        &self, results: &BorrowedResultCohortV1, ingress: &PreparedBorrowedFormalIngressV1,
        site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1, home: BindingRefV1,
        declaration: &crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1,
    ) -> Result<(), String> {
        if !matches!(declaration.result(), crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar)
            || !self.matches_declaration(declaration.field(), declaration.declared_type_name()) {
            return Err(freeze("integer-field/receiver-declaration"));
        }
        self.recheck(results, ingress, &self.plan, site, receiver_site, receiver, home, declaration.field())
    }

    pub(super) fn issue(
        results: &BorrowedResultCohortV1,
        ingress: &PreparedBorrowedFormalIngressV1,
        plan: Rc<ExpressionSourcePlanV1>,
        source: Rc<DeclaredFieldValueSourceV1>,
        site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        home: BindingRefV1,
        name: &str,
    ) -> Result<Self, String> {
        let value = ClosedNumericFieldSourceLoanV1::prepare(results, source, name)?;
        let node = plan.field_read_node_v1(site.site(), receiver_site, receiver, name)
            .ok_or_else(|| freeze("integer-field/original-read"))?;
        let loan = Self {
            value, plan, node,
            site: site.clone(), receiver_site: receiver_site.clone(), receiver, home,
        };
        loan.recheck(results, ingress, &loan.plan, site, receiver_site, receiver, home, loan.field())?;
        Ok(loan)
    }
    pub(super) fn field(&self) -> CanonicalFieldRefV1 { self.value.field() }
    /// Agreement with a separately qualified receiver's original declaration.
    /// Absent annotations stay absent; this comparison grants no Integer/Home.
    pub(super) fn matches_declaration(&self, field: CanonicalFieldRefV1, declared: Option<&str>) -> bool {
        self.value.matches_declaration(field, declared)
    }
    pub(super) fn matches_source(&self, node: usize, source: &Rc<DeclaredFieldValueSourceV1>) -> bool {
        self.node == node && Rc::ptr_eq(&self.value.source, source)
    }
    pub(super) fn recheck(
        &self,
        results: &BorrowedResultCohortV1,
        ingress: &PreparedBorrowedFormalIngressV1,
        plan: &Rc<ExpressionSourcePlanV1>,
        site: &OwnedExprSiteV1,
        receiver_site: &SourceExprSiteV1,
        receiver: BindingRefV1,
        home: BindingRefV1,
        field: CanonicalFieldRefV1,
    ) -> Result<(), String> {
        if !Rc::ptr_eq(plan, &self.plan) || self.plan.owner() != site.owner()
            || receiver.owner() != site.owner() || home.owner() != site.owner()
            || self.site != *site || self.receiver_site != *receiver_site
            || self.receiver != receiver || self.home != home || self.field() != field
        { return Err(freeze("integer-field/source-identity")); }
        if self.plan.field_read_node_v1(site.site(), receiver_site, receiver, &self.value.name) != Some(self.node) {
            return Err(freeze("integer-field/original-read"));
        }
        self.value.recheck(results, ingress)
    }
}
