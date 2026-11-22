//! Correspondence from completed source declarations to callable consumers.
use super::*;

use crate::mir::resolved_semantics::home_new_prefix::{
    TerminalReturnedSourceV1, TerminalValueReturnV1,
};

pub(super) fn complete_value_exits_v1(
    completion: &VerifiedFunctionCompletionV1,
    relations: &BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    mut valid: impl FnMut(&TerminalValueReturnV1) -> bool,
) -> bool {
    let exits = completion.explicit_sites();
    let unique: std::collections::BTreeSet<_> = exits.iter().collect();
    completion.returns_value()
        && completion.implicit_body_end().is_none()
        && !exits.is_empty()
        && unique.len() == exits.len() && relations.len() == exits.len()
        && exits.iter().all(|site| matches!(relations.get(site), Some(TerminalRelationV1::Value(value))
            if value.owner() == completion.owner() && value.return_site() == site && valid(value)))
}

pub(in crate::mir::normal_callable_semantic_package) fn owns_dynamic_carrier_result_v1(
    completion: &VerifiedFunctionCompletionV1,
    relations: &BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
) -> bool {
    use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1;
    completion.function_exit_contract().declared_result() == &DeclaredFunctionResultContractV1::Unannotated
        && complete_value_exits_v1(completion, relations, |value| {
            matches!(value.returned(), TerminalReturnedSourceV1::DynamicExpression(source)
                if source.owned_result_nodes().contains(&0)
                    && expression_return_source_v1(value, source, ExpressionValueClassV1::Dynamic))
        })
}

impl CallableResultContractRefV1<'_> {
    /// Dynamic provider admission uses every original Normal exit, including
    /// source-unannotated methods. The witness proves a transport, not a type.
    pub(crate) fn has_complete_i64_result_v1(&self) -> bool {
        let exits = self.completion.explicit_sites();
        let unique: std::collections::BTreeSet<_> = exits.iter().collect();
        self.declared_result_agrees_with_i64_source()
            && self.completion.owner() == self.owner
            && self.completion.returns_value()
            && self.completion.implicit_body_end().is_none()
            && !exits.is_empty()
            && unique.len() == exits.len()
            && self.terminal_relations.len() == exits.len()
            && exits.iter().all(|site| {
                self.terminal_relations.get(site).is_some_and(|row| {
                    row.owner() == self.owner && row.return_site() == site
                    && row.scalar_class() == Some(
                        crate::mir::resolved_semantics::home_new_prefix::SourceScalarKind::Integer,
                    )
                })
            })
    }
}

impl VerifiedCallableResultContractCohortV1 {
    pub(crate) fn declared_instance_keys_v1(&self) -> &[CanonicalSameModuleCallableKeyV1] {
        self.completed_context
            .as_ref()
            .map_or(&[], |(selected, _)| selected.declared_instance_keys())
    }

    pub(crate) fn dynamic_dispatch_parameters_v1(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<Vec<u32>> {
        let (selected, parameters) = self.completed_context.as_ref()?;
        let selected_key = SelectedNormalCallableKeyV1::Cataloged(key.clone());
        let batch_slot = selected.batch_slot(&selected_key)?;
        let declaration = parameters
            .iter()
            .find(|parameter| parameter.batch_slot == batch_slot)?;
        declaration
            .parameters
            .iter()
            .map(|parameter| parameter.dynamic_dispatch_wire_kind_v1())
            .collect()
    }

    /// Resolve an already-selected catalog key to its invocation-local owner.
    /// This is a correspondence lookup for physical caller attribution; it
    /// does not issue or reconstruct a source key.
    pub(crate) fn owner_for_canonical_key(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<FunctionOwnerIdV1> {
        let (selected, _) = self.completed_context.as_ref()?;
        match key.namespace() {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::FreeFunction => {
                let mut matches = self.rows.iter().filter(|row| {
                    row.top_level_input
                        .as_ref()
                        .is_some_and(|input| input.matches_canonical_key(key))
                });
                let row = matches.next()?;
                (matches.next().is_none()).then_some(row.owner())
            }
            _ => {
                let selected_key = SelectedNormalCallableKeyV1::Cataloged(key.clone());
                let batch_slot = selected.batch_slot(&selected_key)?;
                self.row(batch_slot).map(|row| row.owner())
            }
        }
    }
}
