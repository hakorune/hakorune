//! Literal-`true` LoopTrue binding contract.
//!
//! GenericLoop keeps its condition-read carrier contract in the parent
//! handoff. This sibling admits the existing LoopTrue route's non-empty set of
//! body-read/rebind carriers and reuses the same receipt and schedule types.

use super::*;

impl<'a> CallableLoopSourceProjectionV1<'a> {
    pub(in crate::mir::builder) fn project_loop_true_disposition(
        self,
        loop_site: SourceNodeSiteV1,
    ) -> Result<CallableLoopBindingProjectionDispositionV1, String> {
        let mut receipts = Vec::new();
        for (site, binding) in self.variables {
            let Some(relative) = relative_segments(&loop_site, site) else {
                continue;
            };
            let Some(first) = relative.first() else {
                continue;
            };
            let role = match first {
                SourcePathSegmentV1::LoopBodyRoot | SourcePathSegmentV1::LoopBody(_) => {
                    CallableLoopBindingRoleV1::BodyRead
                }
                _ => continue,
            };
            receipts.push(CallableLoopBindingReceiptV1::new(
                site.clone(),
                *binding,
                role,
            ));
        }
        for (site, binding) in self.assignments {
            let Some(relative) = relative_segments(&loop_site, site) else {
                continue;
            };
            let Some(first) = relative.first() else {
                continue;
            };
            if matches!(
                first,
                SourcePathSegmentV1::LoopBodyRoot | SourcePathSegmentV1::LoopBody(_)
            ) {
                receipts.push(CallableLoopBindingReceiptV1::new(
                    site.clone(),
                    *binding,
                    CallableLoopBindingRoleV1::BodyRebind,
                ));
            }
        }
        let local_declarations = self.local_declarations(&loop_site)?;
        Ok(CallableLoopBindingProjectionDispositionV1::Ready(
            VerifiedCallableSemanticLoopBindingScheduleV1::seal_loop_true(
                self.owner,
                loop_site,
                receipts,
                local_declarations,
            )?,
        ))
    }
}

impl VerifiedCallableSemanticLoopBindingScheduleV1 {
    pub(super) fn seal_loop_true(
        owner: FunctionOwnerIdV1,
        loop_site: SourceNodeSiteV1,
        receipts: Vec<CallableLoopBindingReceiptV1>,
        local_declarations: BTreeMap<BindingRefV1, SourceBindingSiteV1>,
    ) -> Result<Self, String> {
        let iteration_locals = declarations::observed_iteration_locals(
            owner,
            &loop_site,
            &receipts,
            &local_declarations,
        )?;
        let receipts_by_binding =
            validate_projection_rows(owner, &loop_site, &receipts, &iteration_locals)?;
        let mut carrier_count = 0;
        let mut rows = Vec::with_capacity(receipts_by_binding.len());
        for (binding, receipts) in receipts_by_binding {
            let row = build_callable_loop_ready_row(binding, receipts, &iteration_locals);
            let has_read = row.receipts().iter().any(|receipt| {
                matches!(
                    receipt.role(),
                    CallableLoopBindingRoleV1::BodyRead | CallableLoopBindingRoleV1::ConditionRead
                )
            });
            let has_body_read = row
                .receipts()
                .iter()
                .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRead);
            let has_rebind = row
                .receipts()
                .iter()
                .any(|receipt| receipt.role() == CallableLoopBindingRoleV1::BodyRebind);
            if row.class() == CallableLoopReadyBindingClassV1::Carrier {
                carrier_count += 1;
            }
            if !has_read
                || (row.class() == CallableLoopReadyBindingClassV1::Carrier
                    && (!has_body_read || !has_rebind))
            {
                return Err(freeze("incomplete-loop-true-binding-coverage"));
            }
            rows.push(row);
        }
        if carrier_count == 0 {
            return Err(freeze("loop-true-carrier-cardinality-0"));
        }
        for binding in iteration_locals {
            if !rows.iter().any(|row| row.binding() == binding) {
                return Err(freeze("unconsumed-iteration-local"));
            }
        }
        Ok(Self {
            owner,
            loop_site,
            rows: rows.into_boxed_slice(),
            local_declarations,
        })
    }
}
