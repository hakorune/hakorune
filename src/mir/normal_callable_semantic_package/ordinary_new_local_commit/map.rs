//! Physical Map progress in the existing local ledger; Completion owns meaning.
use super::*;
use crate::mir::instruction::MapInvokeOperation as Map;
use crate::mir::resolved_semantics::home_new_prefix::{MapHomeEntry, MapHomeFlow};
use crate::mir::resolved_semantics::ResolvedInitializerRelationV1;

#[derive(Debug)]
pub(in crate::mir::normal_callable_semantic_package) struct MapLocalProgress {
    pub(super) binding: BindingRefV1,
    pub(super) declaration: SourceBindingSiteV1,
    progress: MapProgress,
}
#[derive(Debug)]
enum MapProgress {
    Emitting,
    Emitted {
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
        phase: MapPhase,
    },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MapPhase {
    ExpressionCompleted,
    Installed,
    Checked,
}
impl MapLocalProgress {
    pub(super) fn initializer(&self) -> Option<ValueId> {
        match self.progress {
            MapProgress::Emitted {
                result,
                phase: MapPhase::ExpressionCompleted,
                ..
            } => Some(result),
            _ => None,
        }
    }
    pub(super) fn local(&self) -> Option<ValueId> {
        match self.progress {
            MapProgress::Emitted {
                result,
                phase: MapPhase::Installed | MapPhase::Checked,
                ..
            } => Some(result),
            _ => None,
        }
    }
    pub(super) fn install(&mut self, local: ValueId) {
        match &mut self.progress {
            MapProgress::Emitted { result, phase, .. }
                if *result == local && *phase == MapPhase::ExpressionCompleted =>
            {
                *phase = MapPhase::Installed
            }
            _ => unreachable!("Map local batch preflight"),
        }
    }
    pub(super) fn mark_checked(&mut self) {
        match &mut self.progress {
            MapProgress::Emitted { phase, .. } if *phase != MapPhase::ExpressionCompleted => {
                *phase = MapPhase::Checked
            }
            _ => unreachable!("Map emission batch validation"),
        }
    }
    pub(super) fn is_complete(&self) -> bool {
        matches!(
            self.progress,
            MapProgress::Emitted {
                phase: MapPhase::Checked,
                ..
            }
        )
    }
    pub(super) fn checked_bindings(&self) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        match &self.progress {
            MapProgress::Emitted {
                bindings,
                phase: MapPhase::Checked,
                ..
            } => Ok(bindings),
            _ => Err(freeze("artifact-map-unchecked")),
        }
    }
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn has_map_source(&self, site: &OwnedExprSiteV1) -> bool {
        self.root_completion
            .as_ref()
            .and_then(|c| c.as_ref().ok())
            .and_then(|c| c.cleanup().root_flow())
            .is_some_and(|flow| flow.maps().iter().any(|row| row.site() == site))
    }
    pub(crate) fn map_flow(&self, site: &OwnedExprSiteV1) -> Result<&MapHomeFlow, String> {
        let completion = self
            .root_completion
            .as_ref()
            .and_then(|c| c.as_ref().ok())
            .filter(|c| c.owner() == site.owner())
            .ok_or_else(|| freeze("map-completion"))?;
        let flow = completion
            .cleanup()
            .root_flow()
            .ok_or_else(|| freeze("map-root-flow"))?;
        let mut found = flow.maps().iter().filter(|m| m.site() == site);
        let map = found
            .next()
            .and_then(|m| m.complete())
            .ok_or_else(|| freeze("map-source-unavailable"))?;
        if found.next().is_some() {
            return Err(freeze("map-source-duplicate"));
        }
        Ok(map)
    }
    pub(in crate::mir::normal_callable_semantic_package) fn map_demands_consumed(&self) -> bool {
        let Some(flow) = self
            .root_completion
            .as_ref()
            .and_then(|c| c.as_ref().ok())
            .and_then(|c| c.cleanup().root_flow())
        else {
            return true;
        };
        let rows = self.local_commits.borrow();
        flow.maps().iter().all(|m| {
            m.complete().is_some()
                && matches!(rows.get(m.site()), Some(LocalCommitV1::Map(row)) if row.is_complete())
        })
    }
    pub(crate) fn begin_map_emission(
        &self,
        site: &OwnedExprSiteV1,
        relation: &ResolvedInitializerRelationV1,
    ) -> Result<(), String> {
        let flow = self.map_flow(site)?;
        if relation.binding() != flow.destination()
            || relation.initializer_site() != Some(site.site())
            || !matches!(
                relation.declaration_site(),
                SourceBindingSiteV1::Local { .. }
            )
        {
            return Err(freeze("map-initializer-source-drift"));
        }
        let mut rows = self.local_commits.borrow_mut();
        if rows.contains_key(site) {
            return Err(freeze("map-duplicate-emission"));
        }
        for binding in flow.allocation_fault() {
            if !installed_home(&rows, binding).is_ok_and(LocalCommitV1::end_available) {
                return Err(freeze("map-prior-home-unavailable"));
            }
        }
        for entry in flow.entries() {
            let (acquisition, binding) = entry.transfer_home()
                .ok_or_else(|| freeze("map-value-consumer-missing"))?;
            let row = rows
                .get(acquisition)
                .and_then(LocalCommitV1::ordinary)
                .filter(|row| row.installs(binding))
                .ok_or_else(|| freeze("map-candidate-not-installed"))?;
            if row.destruction != super::super::ObjectDestructionDispositionV1::PlainI64NoHook {
                return Err(freeze("map-candidate-end-unavailable"));
            }
        }
        rows.insert(
            site.clone(),
            LocalCommitV1::Map(MapLocalProgress {
                binding: relation.binding(),
                declaration: relation.declaration_site().clone(),
                progress: MapProgress::Emitting,
            }),
        );
        Ok(())
    }
    pub(crate) fn map_candidate_object(
        &self,
        entry: &MapHomeEntry,
        value: ValueId,
    ) -> Result<CanonicalObjectIdV1, String> {
        let (acquisition, binding) = entry.transfer_home()
            .ok_or_else(|| freeze("map-value-consumer-missing"))?;
        let rows = self.local_commits.borrow();
        let row = rows
            .get(acquisition)
            .and_then(LocalCommitV1::ordinary)
            .filter(|row| row.installs(binding) && row.emission.local() == Some(value))
            .ok_or_else(|| freeze("map-candidate-value-drift"))?;
        Ok(row.object)
    }
    pub(crate) fn map_outer_operations(
        &self,
        site: &OwnedExprSiteV1,
        installed: usize,
    ) -> Result<Vec<InvokeOperation>, String> {
        let flow = self.map_flow(site)?;
        let rows = self.local_commits.borrow();
        flow.outer_after_installs(installed)
            .ok_or_else(|| freeze("map-prefix-range"))?
            .map(|binding| {
                installed_home(&rows, binding)
                    .map(LocalCommitV1::end_operation)
                    .map_err(|_| freeze("map-outer-home-missing"))
            })
            .collect()
    }
    pub(crate) fn record_map_emission(
        &self,
        site: &OwnedExprSiteV1,
        result: ValueId,
        bindings: Vec<(BasicBlockId, MirInstruction)>,
    ) -> Result<(), String> {
        let mut rows = self.local_commits.borrow_mut();
        let Some(LocalCommitV1::Map(row)) = rows.get_mut(site) else {
            return Err(freeze("map-record-without-begin"));
        };
        if !matches!(row.progress, MapProgress::Emitting) || bindings.is_empty() {
            return Err(freeze("map-record-state"));
        }
        row.progress = MapProgress::Emitted {
            result,
            bindings,
            phase: MapPhase::ExpressionCompleted,
        };
        Ok(())
    }
    pub(crate) fn map_initializer_matches(
        &self,
        site: &OwnedExprSiteV1,
        binding: BindingRefV1,
        value: ValueId,
    ) -> bool {
        matches!(self.local_commits.borrow().get(site), Some(LocalCommitV1::Map(row))
            if row.binding == binding && row.initializer() == Some(value))
    }
    pub(crate) fn is_installed_map_binding(&self, binding: BindingRefV1, value: ValueId) -> bool {
        self.local_commits.borrow().values().any(|row|
            matches!(row, LocalCommitV1::Map(map) if map.binding == binding && map.local() == Some(value)))
    }
    pub(super) fn validate_map_emission(
        &self,
        site: &OwnedExprSiteV1,
        function: &MirFunction,
        projection: Option<&super::physical_boundary::FinishedBindings>,
    ) -> Result<(), String> {
        let flow = self.map_flow(site)?;
        let rows = self.local_commits.borrow();
        let Some(LocalCommitV1::Map(row)) = rows.get(site) else {
            return Err(freeze("map-progress-missing"));
        };
        if row.binding != flow.destination() || row.local().is_none() {
            return Err(freeze("map-local-incomplete"));
        }
        let MapProgress::Emitted {
            result, bindings, ..
        } = &row.progress
        else {
            return Err(freeze("map-emission-incomplete"));
        };
        let keys: Vec<_> = bindings
            .iter()
            .filter_map(|(_, i)| match i {
                MirInstruction::Invoke {
                    operation: InvokeOperation::Map(Map::PrepareKey { utf8 }),
                    ..
                } => Some(utf8.as_str()),
                _ => None,
            })
            .collect();
        if keys != flow.entries().iter().map(|e| e.key()).collect::<Vec<_>>() {
            return Err(freeze("map-key-order"));
        }
        let installs: Vec<_> = bindings
            .iter()
            .filter_map(|(_, i)| match i {
                MirInstruction::Invoke {
                    operation:
                        InvokeOperation::Map(Map::InstallIndexed {
                            map, object, value, ..
                        }),
                    ..
                } => Some((*map, *object, *value)),
                _ => None,
            })
            .collect();
        if installs.len() != flow.entries().len() {
            return Err(freeze("map-install-count"));
        }
        for (entry, (map, object, value)) in flow.entries().iter().zip(installs) {
            if map != *result || self.map_candidate_object(entry, value)? != object {
                return Err(freeze("map-install-source-drift"));
            }
        }
        for (id, expected) in bindings {
            if !super::physical_boundary::check_binding(function, projection, *id, expected)? {
                return Err(freeze("map-emission-binding-drift"));
            }
        }
        Ok(())
    }
}
