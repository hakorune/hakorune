//! Shared read-only Home preparation for raw and selected root exits.
//! Selected Static retains the exact order until collector admission succeeds.
use super::*;

pub(super) enum RootHomePreparationV1 {
    NotSelected,
    Unavailable,
    Prepared {
        order: RootHomeCleanupOrderV1,
        direct_object: bool,
    },
}

pub(crate) struct PreparedRootHomeOrderV1 {
    pub(super) owner: FunctionOwnerIdV1,
    pub(super) site: SourceStmtSiteV1,
    pub(super) order: RootHomeCleanupOrderV1,
}

impl PreparedRootHomeOrderV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.site
    }
}

pub(crate) struct PreparedRootHomeGraphPlanV1 {
    pub(crate) operations: Vec<RootHomeReleaseOriginV1>,
    pub(crate) faults: RootHomeFaultSequencesV1,
    pub(crate) order: PreparedRootHomeOrderV1,
}

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn preview_root_home_exit_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<RootHomePreparationV1, String> {
        if !self.object_return_construction_ready_v1(owner)? {
            return Ok(RootHomePreparationV1::NotSelected);
        }
        let Some(completion) = self.completion_for_owner(owner) else {
            return Ok(RootHomePreparationV1::NotSelected);
        };
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(RootHomePreparationV1::NotSelected);
        };
        if !self.root_home_flow_ready_v1(completion)? {
            return Ok(RootHomePreparationV1::NotSelected);
        }
        let exit = SourceStmtSiteV1::from_node(site.clone());
        if !completion.explicit_sites().contains(&exit) {
            return Err(freeze("root-exit-site-mismatch"));
        }
        let Some(row) = self.normal_exit_projection_v1(owner, &exit)? else {
            return Ok(RootHomePreparationV1::NotSelected);
        };
        let rows = self.local_commits.borrow();
        let Some(mut order) = RootHomeCleanupOrderV1::from_home_end_plans(
            &rows,
            &exit,
            row.fault_homes(),
            row.homes(),
        )? else {
            return Ok(RootHomePreparationV1::Unavailable);
        };
        let direct = self.verified_direct_object_return_source_v1(owner, &exit)?;
        if let Some(view) = &direct {
            let Some(source) = view.cleanup_source() else {
                return Ok(RootHomePreparationV1::Unavailable);
            };
            order.retain_direct_source(source)?;
        } else if let Some(source) = self.verified_terminal_call_source_v1(owner, &exit)? {
            if let Some(loan) = source.legacy_terminal().and_then(|terminal| terminal.source_scalar()) {
                if self.callable_scalar_result_source_v1(loan.source().callee_owner())?.is_none() {
                    return Err(freeze("scalar-result-source-missing"));
                }
                order.retain_scalar_source(exit.clone(), loan.clone())?;
            }
        }
        order.stored_map_get = self.stored_map_get_return_source_v1(owner, &exit)?;
        Ok(RootHomePreparationV1::Prepared {
            order,
            direct_object: direct.is_some(),
        })
    }

    pub(crate) fn prepared_root_home_graph_plan_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
    ) -> Result<PreparedRootHomeGraphPlanV1, String> {
        if self.root_exits.borrow().get(&(owner, site.clone().into())).is_some_and(|progress| {
            !matches!(progress, RootHomeExitProgress::Unprepared)
        }) {
            return Err(freeze("selected-static-root-already-prepared"));
        }
        let RootHomePreparationV1::Prepared {
            order,
            direct_object: false,
        } = self.preview_root_home_exit_v1(owner, site.node())?
        else {
            return Err(freeze("selected-static-root-home-unavailable"));
        };
        if self
            .call_source_completion_for_owner_at(owner, site)
            .is_some_and(|(_, terminal)| !terminal.arguments().is_empty())
        {
            return Err(freeze("selected-static-root-arguments-unsupported"));
        }
        let faults = RootHomeFaultSequencesV1 {
            full: order.full().to_vec(),
            acquisition: order.acquisition_fault_indices().to_vec(),
            after_normal: (0..order.normal().len())
                .map(|index| order.fault_indices_after_normal_step(index))
                .collect::<Result<_, _>>()?,
        };
        Ok(PreparedRootHomeGraphPlanV1 {
            operations: order.normal(),
            faults,
            order: PreparedRootHomeOrderV1 {
                owner,
                site: site.clone(),
                order,
            },
        })
    }
}
