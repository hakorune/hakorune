//! Authority-specific sources share one original field geometry and read owner.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{LocalFieldReadRequestV1, LocalFieldReadResultV1};
use crate::mir::resolved_value_profile::expression_plan::{OriginalExpressionFieldPathV1, ExpressionPlanRootV1};
use super::super::lexical_instance_call::{ClosedNumericFieldSourceLoanV1, ExpressionObjectFieldSourceLoanV1};

#[derive(Debug, Clone)]
enum StatementFieldValueV1 {
    Object(ExpressionObjectFieldSourceLoanV1),
    Returned(super::super::result_class_claim::ReturnedFieldSourceLoanV1),
    Numeric(ClosedNumericFieldSourceLoanV1),
    /// The final Text hop of one checked field: an independent copy is acquired later.
    Text(Rc<super::super::TextFieldValueSourceV1>),
}
#[derive(Debug, Clone)]
pub(in crate::mir::normal_callable_semantic_package) struct StatementFieldPathSourceV1 {
    request: LocalFieldReadRequestV1,
    declaration: Rc<LocalFieldReadSourceV1>,
    parent: Option<Rc<Self>>,
    root: Rc<super::super::terminal_home::ExpressionFieldRootSourceV1>,
    value: StatementFieldValueV1,
}
impl StatementFieldPathSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn issue(
        request: &LocalFieldReadRequestV1, declaration: Rc<LocalFieldReadSourceV1>, parent: Option<Rc<Self>>,
        root: Option<Rc<super::super::terminal_home::ExpressionFieldRootSourceV1>>,
        returned: Option<super::super::result_class_claim::ReturnedFieldSourceLoanV1>,
        results: &super::super::lexical_instance_call::BorrowedResultCohortV1,
        ingress: &super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1,
    ) -> Result<Rc<Self>, String> {
        let path = request.path.as_ref().ok_or_else(|| fault("expression-path/original-missing"))?;
        if request.node_in_plan(path.plan()) != Some(path.node()) || request.nullable || request.formal
            || (!matches!(request.received, Some(crate::mir::resolved_semantics::home_new_prefix::ReceivedFieldReadRequestV1::StoredMapGet { .. }))
                && !matches!(path.plan().root(), ExpressionPlanRootV1::Statement {
                    role: crate::mir::resolved_value_profile::expression_root::StatementExpressionRoleV1::PrintValue, .. }))
            || declaration.expression_path.is_some() || declaration.received.is_some()
        { return Err(fault("expression-path/source-role")); }
        match (&parent, path.parent()) {
            (None, None) => {},
            (Some(parent), Some(site)) if parent.path().site() == site
                && Rc::ptr_eq(parent.path().plan(), path.plan())
                && Self::same_root(&parent.request, request)
                && matches!(parent.declaration.result(), LocalFieldReadResultV1::Alias(_)) => {},
            _ => return Err(fault("expression-path/parent-source")),
        }
        let root = match (&parent, root) {
            (Some(parent), None) => Rc::clone(&parent.root),
            (None, Some(root)) => root,
            _ => return Err(fault("expression-path/root-source")),
        };
        let value = match declaration.result() {
            LocalFieldReadResultV1::Alias(class) if path.is_intermediate() => match returned {
                Some(source) if source.read() == &path.site() && source.field() == declaration.field()
                    && source.class() == Some(class.as_ref()) && source.parent() == request.home =>
                    StatementFieldValueV1::Returned(source),
                Some(_) => return Err(fault("expression-path/returned-field-source")),
                None => StatementFieldValueV1::Object(ExpressionObjectFieldSourceLoanV1::from_declaration(
                    results, declaration.field(), class, &request.field)?),
            },
            LocalFieldReadResultV1::Scalar if !path.is_intermediate() => StatementFieldValueV1::Numeric(
                ClosedNumericFieldSourceLoanV1::from_declaration(results, ingress, declaration.field(),
                    declaration.declared_type_name(), &request.field)?),
            LocalFieldReadResultV1::Text(text) if !path.is_intermediate() => StatementFieldValueV1::Text(Rc::clone(text)),
            _ => return Err(fault("expression-path/value-role")),
        };
        let source = Self { request: request.clone(), declaration, parent, root, value };
        source.recheck_container()?;
        if source.receiver_may_be_null() && path.is_intermediate() {
            return Err(fault("expression-path/checked-object-hop-unavailable"));
        }
        Ok(Rc::new(source))
    }
    fn receiver_may_be_null(&self) -> bool {
        self.parent.as_ref().is_some_and(|parent| matches!(&parent.value,
            StatementFieldValueV1::Returned(source) if source.nullable()))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn requires_checked_scalar_v1(&self) -> bool {
        self.receiver_may_be_null() && matches!(self.value, StatementFieldValueV1::Numeric(_))
    }
    pub(in crate::mir::normal_callable_semantic_package) fn checked_parent_v1(&self) -> Option<OwnedExprSiteV1> {
        self.requires_checked_scalar_v1().then(|| self.path().parent()).flatten()
    }
    pub(in crate::mir::normal_callable_semantic_package) fn received_root_v1(&self)
        -> Option<&super::ReceivedFieldReadSourceV1> { self.root.received_source_v1() }
    fn recheck_container(&self) -> Result<(), String> {
        let expected = match &self.parent {
            Some(parent) => match parent.declaration.result() {
                LocalFieldReadResultV1::Alias(class) => Some(class.as_ref()),
                _ => return Err(fault("expression-path/parent-source")),
            },
            None => Some(self.root.class()),
        };
        let actual = match &self.value {
            StatementFieldValueV1::Object(source) => source.container_class(),
            StatementFieldValueV1::Returned(source) => {
                if source.read() != &self.path().site() || source.field() != self.declaration.field()
                    || !matches!(self.declaration.result(), LocalFieldReadResultV1::Alias(class)
                        if source.class() == Some(class.as_ref())) {
                    return Err(fault("expression-path/returned-field-drift"));
                }
                if source.field().object() != self.declaration.field().object() {
                    return Err(fault("expression-path/returned-container"));
                }
                self.root.class()
            },
            StatementFieldValueV1::Numeric(source) => source.container_class(),
            StatementFieldValueV1::Text(_) => self.root.class(),
        };
        if expected.is_some_and(|expected| expected != actual) { return Err(fault("expression-path/container-class")); }
        Ok(())
    }
    fn same_root(a: &LocalFieldReadRequestV1, b: &LocalFieldReadRequestV1) -> bool {
        a.receiver == b.receiver && a.home == b.home && a.alias_class == b.alias_class
            && a.nullable == b.nullable && a.formal == b.formal && a.received == b.received && a.alias_path == b.alias_path
    }
    fn path(&self) -> &OriginalExpressionFieldPathV1 { self.request.path.as_ref().expect("issued path") }
    fn same_source(&self, other: &Self) -> bool {
        self.request == other.request && self.declaration == other.declaration && self.root.same_source(&other.root)
            && match (&self.parent, &other.parent) {
                (None, None) => true, (Some(a), Some(b)) => a.same_source(b), _ => false,
            } && match (&self.value, &other.value) {
                (StatementFieldValueV1::Object(a), StatementFieldValueV1::Object(b)) => a.same_source(b),
                (StatementFieldValueV1::Returned(a), StatementFieldValueV1::Returned(b)) => a == b,
                (StatementFieldValueV1::Numeric(a), StatementFieldValueV1::Numeric(b)) => a.same_source(b),
                (StatementFieldValueV1::Text(a), StatementFieldValueV1::Text(b)) => a == b,
                _ => false,
            }
    }
    #[cfg(test)]
    fn recheck(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        self.recheck_in_context_v1(CallValidationContextV1::Lowering(ledger))
    }
    fn recheck_in_context_v1(&self, context: CallValidationContextV1<'_>) -> Result<(), String> {
        let ledger = context.ledger();
        let path = self.path();
        let requirement = ledger.source_field_requirement_v1(&path.site())?
            .ok_or_else(|| fault("expression-path/required-source-missing"))?;
        if !Rc::ptr_eq(requirement.plan(), path.plan()) || requirement.request() != &self.request {
            return Err(fault("expression-path/source-drift"));
        }
        if let Some(parent) = &self.parent {
            if !Rc::ptr_eq(&self.root, &parent.root) { return Err(fault("expression-path/root-source-drift")); }
            parent.recheck_in_context_v1(context)?;
        } else { self.root.recheck(ledger, &self.request, self.declaration.field(), context)?; }
        self.recheck_container()?;
        match &self.value {
            StatementFieldValueV1::Returned(source) => {
                ledger.callable_result_classes.recheck_returned_field_graph_using_v1(source, &ledger.field_write_claims,
                    &mut |site, loan| context.with_current_instance_source_v1(site, |target|
                        ledger.callable_result_classes.recheck_object_call_roots_v1(target, loan).map_err(String::from))
                        .map_err(|_| "expression-path/returned-target"))
                    .map_err(|issue| fault(issue))
            },
            StatementFieldValueV1::Object(source) => {
                let LocalFieldReadResultV1::Alias(class) = self.declaration.result() else { return Err(fault("expression-path/value-role")); };
                source.recheck(&ledger.borrowed_i64_results, self.declaration.field(), class, &self.request.field)
            },
            StatementFieldValueV1::Numeric(source) => {
                let ingress = ledger.borrowed_formal_source.as_ref().ok_or_else(|| fault("integer-source-missing"))?
                    .as_ref().map_err(Clone::clone)?;
                if !source.matches_declaration(self.declaration.field(), self.declaration.declared_type_name()) {
                    return Err(fault("expression-path/declaration-drift"));
                }
                source.recheck(&ledger.borrowed_i64_results, ingress)
            },
            StatementFieldValueV1::Text(source) => {
                let ingress = ledger.borrowed_formal_source.as_ref().ok_or_else(|| fault("text-source-missing"))?
                    .as_ref().map_err(Clone::clone)?;
                if source.field() != self.declaration.field()
                    || !source.recheck(ingress, &ledger.field_write_claims, &self.request.field) {
                    return Err(fault("expression-path/declaration-drift"));
                }
                Ok(())
            },
        }
    }
}

#[derive(Debug, Clone)]
pub(super) enum FieldPathSourceV1 {
    Return(crate::mir::normal_callable_semantic_package::ResultPathFieldSourceLoanV1),
    Print(Rc<StatementFieldPathSourceV1>),
}
impl PartialEq for FieldPathSourceV1 {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Return(a), Self::Return(b)) => a == b,
            (Self::Print(a), Self::Print(b)) => a.same_source(b), _ => false,
        }
    }
}
impl Eq for FieldPathSourceV1 {}
impl FieldPathSourceV1 {
    pub(super) fn site(&self) -> OwnedExprSiteV1 { match self { Self::Return(a) => a.site(), Self::Print(a) => a.path().site() } }
    pub(super) fn receiver_site(&self) -> &SourceExprSiteV1 { match self { Self::Return(a) => a.receiver_site(), Self::Print(a) => a.path().receiver_site() } }
    pub(super) fn root(&self) -> BindingRefV1 { match self { Self::Return(a) => a.root(), Self::Print(a) => a.path().root() } }
    pub(super) fn home(&self) -> BindingRefV1 { match self { Self::Return(a) => a.root(), Self::Print(a) => a.request.home } }
    pub(super) fn parent(&self) -> Option<OwnedExprSiteV1> { match self { Self::Return(a) => a.parent(), Self::Print(a) => a.path().parent() } }
    pub(super) fn matches_parent(&self, parent: Option<&Self>) -> bool {
        match self {
            Self::Print(source) => match (&source.parent, parent) {
                (None, None) => true,
                (Some(expected), Some(Self::Print(actual))) => Rc::ptr_eq(expected, actual),
                _ => false,
            },
            Self::Return(source) => source.parent() == parent.map(Self::site),
        }
    }
    pub(super) fn descriptor(&self) -> Result<(CanonicalFieldRefV1, LocalFieldReadResultV1, Option<&str>), String> {
        match self { Self::Return(a) => a.descriptor(), Self::Print(a) => Ok((a.declaration.field(), a.declaration.result().clone(), a.declaration.declared_type_name())) }
    }
    pub(super) fn result_source_for_recheck_v1(&self, ledger: &OrdinaryNewClaimLedgerV1)
        -> Result<Option<crate::mir::normal_callable_semantic_package::CallableScalarResultSourceLoanV1>, String> {
        match self {
            Self::Return(source) => {
                let result = ledger.callable_scalar_result_source_v1(source.site().owner())?
                    .ok_or_else(|| fault("result-path/result-missing"))?;
                Ok(Some(result))
            }
            Self::Print(_) => Ok(None),
        }
    }
    pub(super) fn recheck_using_result_v1(&self, ledger: &OrdinaryNewClaimLedgerV1,
        context: CallValidationContextV1<'_>,
        result: Option<&crate::mir::normal_callable_semantic_package::CallableScalarResultSourceLoanV1>,
    ) -> Result<(), String> {
        context.require_ledger(ledger)?;
        match self {
            Self::Return(source) => {
                let result = result.filter(|result| result.owner() == source.site().owner())
                    .ok_or_else(|| fault("result-path/result-missing"))?;
                source.recheck(result)
            }
            Self::Print(source) => source.recheck_in_context_v1(context),
        }
    }
}

impl LocalFieldReadSourceV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn statement_path(&self) -> Option<&Rc<StatementFieldPathSourceV1>> {
        match &self.expression_path { Some(FieldPathSourceV1::Print(source)) => Some(source), _ => None }
    }
    pub(in crate::mir::normal_callable_semantic_package) fn with_statement_path(
        declaration: &Rc<Self>, path: Rc<StatementFieldPathSourceV1>,
    ) -> Rc<Self> {
        let mut source = declaration.as_ref().clone();
        source.expression_path = Some(FieldPathSourceV1::Print(path));
        Rc::new(source)
    }
}

impl OrdinaryNewClaimLedgerV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn validate_statement_field_root_read_v1(
        &self, site: &OwnedExprSiteV1, read: &crate::mir::builder::ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<(), String> {
        context.require_ledger(self)?;
        let rows = self.local_field_reads.borrow();
        let row = rows.get(site).ok_or_else(|| fault("expression-path/root-read-missing"))?;
        self.recheck_result_path_field_v1(site, row, context)?;
        let source = row.source.statement_path().ok_or_else(|| fault("expression-path/root-source-missing"))?;
        if source.parent.is_some() { return Err(fault("expression-path/root-read-parent")); }
        source.root.validate_read_v1(read, context)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn stored_get_field_root_source_v1(
        &self, site: &OwnedExprSiteV1, read: &crate::mir::builder::ExactLexicalReadV1,
        context: CallValidationContextV1<'_>,
    ) -> Result<Option<(
        Rc<crate::mir::resolved_semantics::home_new_prefix::MapFieldGetLocalV1>,
        Rc<crate::mir::resolved_semantics::home_new_prefix::SourceNullExclusionV1>,
    )>, String> {
        self.validate_statement_field_root_read_v1(site, read, context)?;
        let rows = self.local_field_reads.borrow();
        let source = rows.get(site).and_then(|row| row.source.statement_path())
            .ok_or_else(|| fault("expression-path/root-source-missing"))?;
        Ok(source.root.stored_get_v1().map(|(frame, exclusion)| (Rc::clone(frame), Rc::clone(exclusion))))
    }
    pub(in crate::mir) fn statement_field_path_v1(&self, site: &OwnedExprSiteV1, context: CallValidationContextV1<'_>)
        -> Result<Option<OriginalExpressionFieldPathV1>, String> {
        let rows = self.local_field_reads.borrow();
        let Some(row) = rows.get(site) else { return Ok(None); };
        let Some(source) = row.source.statement_path() else { return Ok(None); };
        self.recheck_result_path_field_v1(site, row, context)?;
        Ok(Some(source.path().clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nested_print_entry_and_alias_roots_retain_original_source() {
        for text in [
            "box Leaf { n: i64 birth() { me.n = 7 } }
             box Wrapper { child: Leaf birth(child) { me.child = child } }
             box Maker { make(flag: i64) { if flag == 0 { return null } return new Leaf() }
                 wrap(flag: i64) { local h = me.make(flag) return new Wrapper(h) } }
             static box Main { main() { local maker = new Maker() local w = maker.wrap(1)
                 print(\"n=\" + w.child.n) return 0 } }",
            "box Leaf { n: i64 birth() { me.n = 7 } }
             box Parent { child: Leaf birth() { me.child = new Leaf() }
                 read() { print(\"n=\" + me.child.n) return me.child.n } }
             static box Main { main() { local p = new Parent() print(p.read()) return 0 } }",
            "box Leaf { n: i64 birth() { me.n = 7 } }
             box Parent { child: Leaf birth() { me.child = new Leaf() } }
             box Outer { child: Parent birth(child) { me.child = child } }
             static box Main { main() { local p = new Parent() local outer = new Outer(p)
                 local alias = outer.child print(\"n=\" + alias.child.n) return 0 } }",
        ] {
            let mut package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
            let foreign = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(text).unwrap();
            let ledger = &package.ordinary_new_claim_ledger;
            let paths: Vec<_> = ledger.local_field_reads.borrow().values()
                .filter_map(|row| row.source.statement_path().cloned()).collect();
            assert_eq!(paths.len(), 2, "original nested path must be qualified: {text}");
            for source in &paths { source.recheck(ledger).unwrap(); }
            if paths.iter().any(|source| matches!(source.value, StatementFieldValueV1::Returned(_))) {
                assert_eq!(paths.iter().filter(|source| source.requires_checked_scalar_v1()).count(), 1,
                    "maybe-null returned child requires the same parent's checked scalar hop");
                let ledger = Rc::get_mut(&mut package.ordinary_new_claim_ledger).unwrap();
                let retained = std::mem::take(&mut ledger.field_write_claims);
                assert!(paths.iter().any(|source| source.recheck(ledger).is_err()),
                    "returned path cannot outlive its original writer proof");
                ledger.field_write_claims = retained;
            }
            let ledger = &package.ordinary_new_claim_ledger;
            let root = paths.iter().find(|source| source.parent.is_none()).unwrap();
            if let Some(alias) = &root.request.alias_path {
                let acquisition = match &alias.root_source {
                    crate::mir::resolved_semantics::home_new_prefix::FieldAliasRootSourceV1::OwnedHome { acquisition } => acquisition,
                    crate::mir::resolved_semantics::home_new_prefix::FieldAliasRootSourceV1::InstanceEntry { .. } => {
                        panic!("this fixture's alias is rooted in an owned Home")
                    }
                };
                let foreign_source = foreign.ordinary_new_claim_ledger.claims.borrow().values()
                    .find(|row| row.class() == "Outer").unwrap().box_source().clone();
                let retained = {
                    let mut rows = ledger.claims.borrow_mut();
                    std::mem::replace(&mut rows.get_mut(acquisition).unwrap().core.box_source, foreign_source)
                };
                assert!(root.recheck(ledger).is_err(), "alias cannot borrow a replacement New source anchor");
                ledger.claims.borrow_mut().get_mut(acquisition).unwrap().core.box_source = retained;
            } else {
                let mut changed = root.as_ref().clone();
                changed.root = foreign.ordinary_new_claim_ledger.local_field_reads.borrow().values()
                    .find_map(|row| row.source.statement_path().map(|source| Rc::clone(&source.root))).unwrap();
                assert!(changed.recheck(ledger).is_err(), "Me root must keep its original entry source");
            }
            root.recheck(ledger).unwrap();
        }
    }

    #[test]
    fn nested_print_paths_reject_same_class_field_and_parent_source_substitution() {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            "box Leaf { n: i64 birth() { me.n = 7 } }
             box OtherLeaf { n: i64 birth() { me.n = 9 } }
             box Parent { left: Leaf right: Leaf birth() { me.left = new Leaf() me.right = new Leaf() } }
             static box Main { main() { local p = new Parent() print(\"n=\" + p.right.n) return 0 } }").unwrap();
        let ledger = &package.ordinary_new_claim_ledger;
        let ingress = ledger.borrowed_formal_source.as_ref().unwrap().as_ref().unwrap();
        let (site, original) = ledger.local_field_reads.borrow().iter().find_map(|(site, row)|
            row.source.statement_path().filter(|path| path.path().is_intermediate())
                .map(|source| (site.clone(), Rc::clone(source)))).unwrap();
        original.recheck(ledger).unwrap();
        let left = ledger.field_write_claims.object_sources().get(&("Parent".into(), "left".into())).unwrap();
        let declaration = LocalFieldReadSourceV1::from_declaration(left.field().unwrap(),
            LocalFieldReadResultV1::Alias("Leaf".into()), Some("Leaf".into()));
        assert!(StatementFieldPathSourceV1::issue(&original.request, Rc::clone(&declaration), None,
            Some(Rc::clone(&original.root)), None, &ledger.borrowed_i64_results, ingress).is_err(), "a sibling name cannot qualify the original right path");
        let mut swapped = original.as_ref().clone();
        swapped.declaration = declaration;
        swapped.value = StatementFieldValueV1::Object(ExpressionObjectFieldSourceLoanV1::from_declaration(
            &ledger.borrowed_i64_results, left.field().unwrap(), "Leaf", "left").unwrap());
        assert!(swapped.recheck(ledger).is_err(), "independent source re-demand rejects the same-class sibling");
        let terminal = ledger.local_field_reads.borrow().values().find_map(|row| row.source.statement_path()
            .filter(|path| !path.path().is_intermediate()).cloned()).unwrap();
        let foreign = ledger.field_write_claims.value_source("OtherLeaf", "n").unwrap().as_ref().unwrap();
        let foreign_declaration = LocalFieldReadSourceV1::from_declaration(foreign.field(), LocalFieldReadResultV1::Scalar, Some("i64".into()));
        assert!(StatementFieldPathSourceV1::issue(&terminal.request, Rc::clone(&foreign_declaration),
            terminal.parent.clone(), None, None, &ledger.borrowed_i64_results, ingress).is_err(),
            "same-name numeric field from another parent class cannot qualify the original path");
        let mut swapped = terminal.as_ref().clone();
        swapped.declaration = foreign_declaration;
        swapped.value = StatementFieldValueV1::Numeric(ClosedNumericFieldSourceLoanV1::from_declaration(
            &ledger.borrowed_i64_results, ingress, foreign.field(), Some("i64"), "n").unwrap());
        assert!(swapped.recheck(ledger).is_err(), "independent re-demand checks parent/container class");
        let mut rows = ledger.local_field_reads.borrow_mut();
        ledger.validate_result_path_fields_present_v1(site.owner(), &rows).unwrap();
        let row = rows.get_mut(&site).unwrap();
        let retained = Rc::clone(&row.source);
        let replacement = Rc::new(original.as_ref().clone());
        row.source = LocalFieldReadSourceV1::with_statement_path(&original.declaration, replacement);
        assert!(ledger.validate_result_path_fields_present_v1(site.owner(), &rows).is_err(),
            "child must retain the SAME current parent source, not an equal replacement");
        rows.get_mut(&site).unwrap().source = retained;
        ledger.validate_result_path_fields_present_v1(site.owner(), &rows).unwrap();
    }
}
