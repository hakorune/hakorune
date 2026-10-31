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
        homes: &mut Vec<LiveHomeV1>,
    ) -> bool {
        let Some((index, payload)) = self.shared_move_preflight_v1(from, to, &acquisition, homes) else { return false };
        let Some(StoredLocal::Home { construction_source: Some(construction_source), ready, .. }) = self.locals.get(&from).cloned() else { return false };
        homes.remove(index);
        self.store(from, StoredLocal::MovedOut);
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition: payload, construction_source, ready });
        homes.push(LiveHomeV1::Stable(to));
        true
    }
    pub(in super::super) fn install_shared_fresh_v1(
        &mut self, to: BindingRefV1, acquisition: super::OwnedExprSiteV1,
        payload_acquisition: super::OwnedExprSiteV1,
        construction_source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ConstructionSourceStructureV1>,
        homes: &mut Vec<LiveHomeV1>,
    ) -> bool {
        if to.owner() != self.input.owner() || acquisition.owner() != self.input.owner()
            || payload_acquisition.owner() != self.input.owner() || homes.iter().any(|home| home.binding() == to)
            || !construction_source.shared_trivial_payload_v1() { return false; }
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition, construction_source, ready: BTreeMap::new() });
        homes.push(LiveHomeV1::Stable(to));
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
        acquisition: super::OwnedExprSiteV1, homes: &mut Vec<LiveHomeV1>,
    ) -> bool {
        if from.owner() != self.input.owner() || to.owner() != self.input.owner()
            || acquisition.owner() != self.input.owner() || from == to
            || homes.iter().filter(|home| home.binding() == from).count() != 1
            || homes.iter().any(|home| home.binding() == to) { return false; }
        let Some(StoredLocal::Shared { payload_acquisition, construction_source, ready, .. }) = self.locals.get(&from).cloned() else { return false };
        self.store(to, StoredLocal::Shared { acquisition, payload_acquisition, construction_source, ready });
        homes.push(LiveHomeV1::Stable(to));
        true
    }
}
