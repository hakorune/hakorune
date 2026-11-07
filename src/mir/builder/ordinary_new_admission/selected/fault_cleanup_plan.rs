//! The one Fault-cleanup plan of a fallible operation or expression (S3b-R4..R6).
//! The plan decides which End runs on the Normal edge, which responsibilities a
//! Fault still closes, and which Fault entries remain after each Normal End;
//! the writers in `root_graph.rs` only turn a plan into blocks.
use super::*;
use crate::mir::instruction::StringInvokeOperation as Operation;

/// `normal` ends on the Normal edge in order; `fault` is the full list a Fault may
/// close; `after_normal[i]` names the `fault` entries still owed after Normal step `i`;
/// `acquisition` names the entries a Fault of the guarded operation itself closes.
#[derive(Debug, Clone, Default)]
pub(in crate::mir::builder) struct FaultCleanupPlanV1 {
    normal: Vec<InvokeOperation>,
    fault: Vec<InvokeOperation>,
    after_normal: Vec<Vec<usize>>,
    acquisition: Vec<usize>,
}

impl FaultCleanupPlanV1 {
    pub(in crate::mir::builder) fn normal(&self) -> &[InvokeOperation] {
        &self.normal
    }
    pub(in crate::mir::builder) fn fault(&self) -> &[InvokeOperation] {
        &self.fault
    }
    pub(in crate::mir::builder) fn after_normal(&self) -> &[Vec<usize>] {
        &self.after_normal
    }
    pub(in crate::mir::builder) fn acquisition(&self) -> &[usize] {
        &self.acquisition
    }

    /// S3b-R5a: temporaries of an expression newest first, then the Homes that stay owed.
    pub(in crate::mir::builder) fn owed_operations(
        acquired: &[(usize, ValueId)],
        dynamic_carriers: &[(usize, ValueId)],
        temporary_order: &[usize],
        homes: &[InvokeOperation],
    ) -> Result<Vec<InvokeOperation>, String> {
        let mut seen = std::collections::BTreeSet::new();
        if acquired.len() + dynamic_carriers.len() != temporary_order.len()
            || temporary_order.iter().any(|node| !seen.insert(*node))
        {
            return Err(freeze("expression/temporary-order"));
        }
        let mut pending = Vec::with_capacity(temporary_order.len() + homes.len());
        for node in temporary_order.iter().rev() {
            let string = acquired.iter().find(|(candidate, _)| candidate == node);
            let carrier = dynamic_carriers.iter().find(|(candidate, _)| candidate == node);
            match (string, carrier) {
                (Some((_, value)), None) => {
                    pending.push(InvokeOperation::String(Operation::End { value: *value }))
                }
                (None, Some((_, carrier))) => {
                    pending.push(InvokeOperation::DynamicCarrierEnd { carrier: *carrier })
                }
                _ => return Err(freeze("expression/temporary-coverage")),
            }
        }
        pending.extend(homes.iter().cloned());
        Ok(pending)
    }

    /// A Fault-only suffix: no Normal End, and a Fault of the guarded operation closes every entry.
    pub(in crate::mir::builder) fn unwind(operations: Vec<InvokeOperation>) -> Self {
        let acquisition = (0..operations.len()).collect();
        Self { normal: Vec::new(), fault: operations, after_normal: Vec::new(), acquisition }
    }

    /// The Fault-only suffix in front of one fallible operation of an expression.
    pub(in crate::mir::builder) fn pending(
        acquired: &[(usize, ValueId)],
        dynamic_carriers: &[(usize, ValueId)],
        temporary_order: &[usize],
        homes: &[InvokeOperation],
    ) -> Result<Self, String> {
        Ok(Self::unwind(Self::owed_operations(acquired, dynamic_carriers, temporary_order, homes)?))
    }

    /// An expression statement: the temporaries end on the Normal edge; a Fault closes the
    /// temporaries and then the Homes. `entry_fault` adds the Fault of the statement's own Invoke.
    pub(in crate::mir::builder) fn temporaries_then_homes(
        acquired: &[(usize, ValueId)],
        dynamic_carriers: &[(usize, ValueId)],
        temporary_order: &[usize],
        homes: &[InvokeOperation],
        entry_fault: bool,
    ) -> Result<Self, String> {
        let normal = Self::owed_operations(acquired, dynamic_carriers, temporary_order, &[])?;
        let fault = Self::owed_operations(acquired, dynamic_carriers, temporary_order, homes)?;
        Self::from_normal_prefix(normal, fault, if entry_fault { None } else { Some(Vec::new()) })
    }

    /// S3b-R6: Normal Ends over one operation. A Fault of the operation closes the Normal Ends
    /// and the unwind; a Fault after Normal also closes the cancellation of the result.
    pub(in crate::mir::builder) fn operation(
        normal_ends: Vec<InvokeOperation>,
        cancellation: &[InvokeOperation],
        unwind: Vec<InvokeOperation>,
    ) -> Result<Self, String> {
        let mut fault = normal_ends.clone();
        fault.extend_from_slice(cancellation);
        fault.extend(unwind);
        let acquisition = (0..normal_ends.len()).chain((normal_ends.len() + cancellation.len())..fault.len()).collect();
        Self::from_normal_prefix(normal_ends, fault, Some(acquisition))
    }

    /// An acquisition owes exactly one End: reject a plan that ends one value twice.
    pub(in crate::mir::builder) fn reject_duplicate_acquisition(&self) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        for operation in &self.fault {
            if let InvokeOperation::String(crate::mir::instruction::StringInvokeOperation::End { value })
                | InvokeOperation::DynamicCarrierEnd { carrier: value } = operation
            {
                if !seen.insert(*value) {
                    return Err(freeze("call-argument/duplicate-acquisition"));
                }
            }
        }
        Ok(())
    }

    /// An expression body: Normal Ends plus a replaced Home that the replacement owes first
    /// (S3b-R5). `fault` is the full list; the Fault of the guarded operation closes nothing extra.
    pub(in crate::mir::builder) fn body(
        mut normal: Vec<InvokeOperation>,
        mut fault: Vec<InvokeOperation>,
        displaced: Option<InvokeOperation>,
    ) -> Result<Self, String> {
        let mut normal_indices: Vec<_> = (0..normal.len()).collect();
        let mut displaced_after = None;
        if let Some(old) = displaced {
            let index = fault.iter().position(|operation| *operation == old)
                .ok_or_else(|| freeze("rebind-expression/old-home-missing"))?;
            if fault.iter().filter(|operation| **operation == old).count() != 1 || normal_indices.contains(&index) {
                return Err(freeze("rebind-expression/old-home-duplicate"));
            }
            if normal.is_empty() {
                normal_indices.push(index);
                normal.push(old);
            } else {
                // The installed replacement owes the displaced End before any
                // temporary's End, on Normal and on Fault alike.
                let installed = normal.len();
                fault.remove(index);
                fault.insert(0, old.clone());
                normal.insert(0, old);
                normal_indices = (0..normal.len()).collect();
                let position = (index + 1).checked_sub(installed + 2)
                    .ok_or_else(|| freeze("rebind-expression/old-home-order"))?;
                displaced_after = Some(crate::mir::normal_callable_semantic_package::displaced_replacement_fault_sequences_v2(
                    &mut fault, normal.len(), position)?);
            }
        }
        let after_normal = match displaced_after {
            Some(after) => after,
            None => (0..normal.len()).map(|step|
                crate::mir::normal_callable_semantic_package::fault_indices_after_normal_step_v2(fault.len(), &normal_indices, step))
                .collect::<Result<Vec<_>, _>>()?,
        };
        Ok(Self { normal, fault, after_normal, acquisition: Vec::new() })
    }

    /// The arguments of a `new`: the object's own cleanup first, then the argument Ends, then the Homes.
    pub(in crate::mir::builder) fn new_arguments(
        mut object_cleanup: Vec<InvokeOperation>,
        normal_ends: Vec<InvokeOperation>,
        home_unwind: Vec<InvokeOperation>,
    ) -> Result<Self, String> {
        let first = object_cleanup.len();
        object_cleanup.extend(normal_ends.iter().cloned());
        object_cleanup.extend(home_unwind);
        let indices: Vec<_> = (first..first + normal_ends.len()).collect();
        let after_normal = (0..normal_ends.len())
            .map(|step| crate::mir::normal_callable_semantic_package::fault_indices_after_normal_step_v2(
                object_cleanup.len(), &indices, step))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self { normal: normal_ends, fault: object_cleanup, after_normal, acquisition: Vec::new() })
    }

    /// A plan whose residual Fault entries were already derived from the original source
    /// (the package derives them for a source loop; this builder does not recompute them).
    pub(in crate::mir::builder) fn from_residuals(
        normal: Vec<InvokeOperation>,
        fault: Vec<InvokeOperation>,
        after_normal: Vec<Vec<usize>>,
        acquisition: Vec<usize>,
    ) -> Self {
        Self { normal, fault, after_normal, acquisition }
    }

    /// `normal` is a prefix of `fault`; S3b-R6a: each step leaves the entries after it.
    fn from_normal_prefix(normal: Vec<InvokeOperation>, fault: Vec<InvokeOperation>, acquisition: Option<Vec<usize>>) -> Result<Self, String> {
        let indices: Vec<_> = (0..normal.len()).collect();
        let after_normal = (0..normal.len())
            .map(|step| crate::mir::normal_callable_semantic_package::fault_indices_after_normal_step_v2(
                fault.len(), &indices, step))
            .collect::<Result<Vec<_>, _>>()?;
        let acquisition = acquisition.unwrap_or_else(|| (0..fault.len()).collect());
        Ok(Self { normal, fault, after_normal, acquisition })
    }
}
