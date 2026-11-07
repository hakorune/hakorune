//! Shared handle locals: move preflight, fresh/clone installation and acquisition lookup.
use super::*;

impl<'source> PrefixLocalFlow<'source> {
    /// Preflight all source ownership checks before retaining the plan.
    pub(in super::super) fn shared_move_preflight_v1(
        &self,
        from: BindingRefV1,
        to: BindingRefV1,
        acquisition: &super::OwnedExprSiteV1,
        homes: &[LiveHomeV1],
    ) -> Option<(usize, super::OwnedExprSiteV1)> {
        if from.owner() != self.input.owner() || to.owner() != self.input.owner()
            || acquisition.owner() != self.input.owner() || from == to { return None; }
        let payload = self.home_acquisition(from)?.clone();
        if !matches!(self.locals.get(&from), Some(StoredLocal::Home { construction_source: Some(source), .. })
            if source.shared_trivial_payload_v1())
            || self.locals.iter().any(|(binding, local)| *binding != from && match local {
                StoredLocal::Handle(root) | StoredLocal::FieldAlias { root, .. }
                | StoredLocal::BorrowedArray { root, .. } => *root == from,
                _ => false,
            }) { return None; }
        let positions: Vec<_> = homes.iter().enumerate().filter(|(_, home)| home.binding() == from).map(|(i, _)| i).collect();
        if positions.len() != 1 || homes.iter().any(|home| home.binding() == to)
            || homes.iter().any(|home| home.binding() != from && self.home_acquisition(home.binding()) == Some(&payload)) {
            return None;
        }
        Some((positions[0], payload))
    }
    /// The caller retained the exact source plan and pre-operation Homes for
    /// Fault; this mutation describes only the Normal continuation.
    pub(in super::super) fn install_shared_move_v1(
        &mut self,
        from: BindingRefV1,
        to: BindingRefV1,
        acquisition: super::OwnedExprSiteV1,
        homes: &mut HomesV1,
    ) -> bool {
        let Some((index, payload)) = self.shared_move_preflight_v1(from, to, &acquisition, homes) else { return false };
        let Some(StoredLocal::Home { construction_source: Some(construction_source), ready, .. }) = self.locals.get(&from).cloned() else { return false };
        homes.remove_at(index);
        self.consume_argument_home(from);
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition: payload, construction_source, ready });
        homes.install(LiveHomeV1::Stable(to));
        true
    }
    pub(in super::super) fn install_shared_fresh_v1(
        &mut self, to: BindingRefV1, acquisition: super::OwnedExprSiteV1,
        payload_acquisition: super::OwnedExprSiteV1,
        construction_source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>,
        homes: &mut HomesV1,
    ) -> bool {
        if to.owner() != self.input.owner() || acquisition.owner() != self.input.owner()
            || payload_acquisition.owner() != self.input.owner() || homes.iter().any(|home| home.binding() == to)
            || !construction_source.shared_trivial_payload_v1() { return false; }
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition, construction_source, ready: BTreeMap::new() });
        homes.install(LiveHomeV1::Stable(to));
        true
    }

    pub(in super::super) fn shared_acquisition_v1(&self, root: BindingRefV1) -> Option<&super::OwnedExprSiteV1> {
        match self.locals.get(&root)? {
            StoredLocal::Shared { acquisition, .. } => Some(acquisition),
            _ => None,
        }
    }

    pub(in super::super) fn movable_acquisition_v1(&self, root: BindingRefV1) -> Option<&super::OwnedExprSiteV1> {
        self.home_acquisition(root).or_else(|| self.shared_acquisition_v1(root))
    }

    pub(in super::super) fn install_shared_clone_v1(
        &mut self, from: BindingRefV1, to: BindingRefV1,
        acquisition: super::OwnedExprSiteV1, homes: &mut HomesV1,
    ) -> bool {
        if from.owner() != self.input.owner() || to.owner() != self.input.owner()
            || acquisition.owner() != self.input.owner() || from == to
            || homes.iter().filter(|home| home.binding() == from).count() != 1
            || homes.iter().any(|home| home.binding() == to) { return false; }
        let Some(StoredLocal::Shared { payload_acquisition, construction_source, ready, .. }) = self.locals.get(&from).cloned() else { return false };
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition, construction_source, ready });
        homes.install(LiveHomeV1::Stable(to));
        true
    }
}

impl<'source> PrefixLocalFlow<'source> {
    pub(in super::super) fn retain_shared_clone_initializer_v1(
        &mut self, relation: &crate::mir::resolved_semantics::ResolvedInitializerRelationV1,
        from: BindingRefV1,
        shared: crate::mir::resolved_semantics::ResolvedUnaryExpressionSourceV1,
        homes: &mut HomesV1,
    ) -> bool {
        let input = self.input;
        let function = input.function();
        let to = relation.binding();
        let Some(site) = relation.initializer_site() else { return false; };
        let acquisition = super::OwnedExprSiteV1::new(input.owner(), site.clone());
        let crate::mir::resolved_semantics::SourceBindingSiteV1::Local { statement, .. } = relation.declaration_site()
            else { return false; };
        if !self.same_scope_v1(to, from, statement)
            || from == to || homes.iter().filter(|home| home.binding() == from).count() != 1
            || homes.iter().any(|home| home.binding() == to) { return false; }
        let Some(original) = self.shared_acquisition_v1(from).cloned() else { return false; };
        let plan = input.forest().callable_source_ledger(input.owner()).ok()
            .and_then(|ledger| crate::mir::resolved_value_profile::expression_plan::local_expression_source_plan_v1(&ledger, input.body_shape(), relation).ok())
            .and_then(|source| crate::mir::resolved_semantics::home_new_prefix::HomeExpressionPlanV1::shared_initializer(source,
                crate::mir::resolved_semantics::home_new_prefix::SharedPayloadSourceV1::Existing { from, acquisition: original, is_clone: true },
                shared, None, homes.to_vec().into()));
        plan.is_some_and(|plan| self.retain_expression_plan(plan))
            && self.install_shared_clone_v1(from, to, acquisition, homes)
    }
}
