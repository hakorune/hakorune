//! Branch-local Normal field readiness of the original selected allocation.
//! Each value retains its exact successful source frame; it grants no physical pin.
use super::*;
use std::rc::Rc;
use crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1;
use crate::mir::resolved_semantics::home_new_prefix::FieldWritePlanV1;

impl PrefixLocalFlow<'_> {
    /// Unknown calls may retain an exposed handle. Forget its source proof for
    /// this path, so a later scalar store cannot silently recover no-escape.
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn invalidate_dynamic_field_sources(
        &mut self, statement: &crate::mir::compiler::located::LocatedStmtV1<'_>,
    ) {
        use crate::mir::function::TypedObjectFieldStorage as Storage;
        let function = self.input.function();
        if matches!(statement.node(), ASTNode::Loop { .. }) {
            // Preissued Loop After may bypass this per-statement walk. Until
            // it carries the same mutation proof, keep Dynamic roots closed.
            for value in self.locals.values_mut() {
                if matches!(value, StoredLocal::NamedArrayHolder(source) if !source.has_element_contract()) {
                    // Keep its Home in unwind; only the local source capability is lost.
                    *value = StoredLocal::BoundValue;
                }
                if let StoredLocal::Home { construction_source, ready, .. } = value {
                    if construction_source.as_ref().and_then(|source| source.allocation_only_fields())
                        .is_some_and(|fields| fields.iter().any(|field| field.storage() == Storage::Dynamic)) {
                        *construction_source = None;
                        ready.clear();
                    }
                }
            }
            return;
        }
        let mut exposed = BTreeSet::new();
        for (_, call) in function.method_calls().filter(|(site, _)|
            function.source_site_inventory().containing_statement(site) == Some(statement.site())) {
            if let Some(root) = self.field_support_root(call.receiver_site(), true) { exposed.insert(root); }
            for argument in call.arguments() {
                if let Some(root) = self.field_support_root(argument.site(), false) { exposed.insert(root); }
            }
        }
        for (_, call) in function.direct_call_observations().filter(|(site, _)|
            function.source_site_inventory().containing_statement(site) == Some(statement.site())) {
            for argument in call.argument_sites() {
                if let Some(root) = self.field_support_root(argument, false) { exposed.insert(root); }
            }
        }
        let alias_write = self.input.body_shape().and_then(|shape| shape.assignment_sources().iter()
            .find(|row| row.statement_site() == statement.site())).and_then(|assignment|
            match function.assignment_target(assignment.target_site()) {
                Some(crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { receiver }) =>
                    self.field_support_root(receiver, true).filter(|root|
                        function.variable_ref(receiver) != Some(ResolvedLexicalRefV1::Local(*root))),
                _ => None,
            });
        for (root, value) in &mut self.locals {
            if exposed.contains(root) && matches!(value, StoredLocal::NamedArrayHolder(source) if !source.has_element_contract()) {
                *value = StoredLocal::BoundValue;
            }
            let StoredLocal::Home { construction_source, ready, .. } = value else { continue };
            if !construction_source.as_ref().and_then(|source| source.allocation_only_fields())
                .is_some_and(|fields| fields.iter().any(|field| field.storage() == Storage::Dynamic)) { continue; }
            if exposed.contains(root) { *construction_source = None; ready.clear(); }
            else if alias_write == Some(*root) { ready.clear(); }
        }
    }

    fn field_support_root(&self, site: &SourceExprSiteV1, field_receiver: bool) -> Option<BindingRefV1> {
        let mut binding = match self.input.function().variable_ref(site) {
            Some(ResolvedLexicalRefV1::Local(binding)) => binding,
            None if field_receiver => {
                let crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess { object, .. } =
                    self.input.body_shape()?.expression_shape(site)? else { return None };
                return self.field_support_root(object, true);
            },
            _ => return None,
        };
        let mut seen = BTreeSet::new();
        while seen.insert(binding) {
            match self.locals.get(&binding)? {
                StoredLocal::Home { .. } | StoredLocal::NamedArrayHolder(_) => return Some(binding),
                StoredLocal::Handle(root) | StoredLocal::FieldAlias { root, .. }
                    | StoredLocal::BorrowedArray { root, .. } => binding = *root,
                _ => return None,
            }
        }
        None
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn owned_allocation_source(
        &self, root: BindingRefV1,
    ) -> Option<(OwnedExprSiteV1, Rc<ConstructionSourceStructureV1>)> {
        let StoredLocal::Home { acquisition, construction_source: Some(source), .. } = self.locals.get(&root)? else { return None };
        source.allocation_only_fields().map(|_| (acquisition.clone(), Rc::clone(source)))
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn record_normal_field_write(
        &mut self, plan: FieldWritePlanV1,
    ) -> bool {
        let frame = plan.frame();
        let Some(local) = frame.owned_local() else { return false };
        let Some(StoredLocal::Home { acquisition, construction_source: Some(source), ready }) = self.locals.get_mut(&local.root()) else { return false };
        if acquisition != local.acquisition() || !Rc::ptr_eq(source, local.construction())
            || !(plan.owned_i64_field().map(|(field, _)| field) == Some(local.field())
                || plan.rhs().is_some_and(|rhs| rhs.owned_array().is_some() && rhs.is_consistent(frame.source()))
                    && source.allocation_only_field(local.original().write().field())
                        .is_some_and(|field| field.field() == local.field() && field.storage() == crate::mir::function::TypedObjectFieldStorage::Dynamic))
        { return false; }
        ready.insert(local.field(), plan);
        true
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn ready_field_read(
        &self, root: BindingRefV1, name: &str,
    ) -> Option<Option<FieldWritePlanV1>> {
        let Some(StoredLocal::Home { construction_source: Some(source), ready, .. }) = self.locals.get(&root) else { return Some(None) };
        if source.allocation_only_fields().is_none() { return Some(None); }
        let field = source.allocation_only_field(name)?.field();
        ready.get(&field).filter(|plan| plan.owned_i64_field().is_some()).cloned().map(Some)
    }
}
