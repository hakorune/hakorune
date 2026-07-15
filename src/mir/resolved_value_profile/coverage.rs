//! Exact deterministic coverage draft and terminal co-seal helpers.

use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
use crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1;
use std::collections::BTreeSet;

use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
    SourceBindingSiteV1, SourceExprSiteV1, VerifiedResolvedFunctionV1,
};

use super::direct_call::VerifiedTrivialDirectCallV1;
use super::error::TrivialProfileContractErrorV1;
use super::product::{
    TrivialBindingDefinitionOriginV1, TrivialProfileCoverageSubjectV1, TrivialRepresentationV1,
    TrivialTerminalProfileV1, VerifiedLocatedTrivialValueV1, VerifiedTrivialBindingDefinitionV1,
    VerifiedTrivialIfMergeProfileV1, VerifiedTrivialParameterEntryV1,
    VerifiedTrivialProfileCoverageV1,
};

pub(super) struct TrivialProfileDraftV1 {
    owner: FunctionOwnerIdV1,
    parameter_entries: Vec<VerifiedTrivialParameterEntryV1>,
    values: Vec<VerifiedLocatedTrivialValueV1>,
    direct_calls: Vec<VerifiedTrivialDirectCallV1>,
    definitions: Vec<VerifiedTrivialBindingDefinitionV1>,
    merge_profiles: Vec<VerifiedTrivialIfMergeProfileV1>,
    ordered_subjects: Vec<TrivialProfileCoverageSubjectV1>,
}

pub(super) struct TrivialProfilePartsV1 {
    pub(super) parameter_entries: Vec<VerifiedTrivialParameterEntryV1>,
    pub(super) values: Vec<VerifiedLocatedTrivialValueV1>,
    pub(super) direct_calls: Vec<VerifiedTrivialDirectCallV1>,
    pub(super) definitions: Vec<VerifiedTrivialBindingDefinitionV1>,
    pub(super) merge_profiles: Vec<VerifiedTrivialIfMergeProfileV1>,
    pub(super) coverage: VerifiedTrivialProfileCoverageV1,
}

pub(super) struct ResolvedFactCoverageDraftV1 {
    owner: FunctionOwnerIdV1,
    declarations: BTreeSet<SourceBindingSiteV1>,
    variable_uses: BTreeSet<SourceExprSiteV1>,
    assignments: BTreeSet<SourceExprSiteV1>,
    direct_calls: BTreeSet<SourceExprSiteV1>,
}

impl ResolvedFactCoverageDraftV1 {
    pub(super) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            declarations: BTreeSet::new(),
            variable_uses: BTreeSet::new(),
            assignments: BTreeSet::new(),
            direct_calls: BTreeSet::new(),
        }
    }

    pub(super) fn declaration_binding(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        site: &SourceBindingSiteV1,
    ) -> Result<BindingRefV1, TrivialProfileContractErrorV1> {
        let binding = product.declaration_binding(site).ok_or_else(|| {
            TrivialProfileContractErrorV1::MissingDeclarationBinding { site: site.clone() }
        })?;
        self.require_owner(binding)?;
        self.declarations.insert(site.clone());
        Ok(binding)
    }

    pub(super) fn variable_binding(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        site: &SourceExprSiteV1,
    ) -> Result<BindingRefV1, TrivialProfileContractErrorV1> {
        let reference = product.variable_ref(site).ok_or_else(|| {
            TrivialProfileContractErrorV1::MissingVariableResolution { site: site.clone() }
        })?;
        let ResolvedLexicalRefV1::Local(binding) = reference else {
            return Err(TrivialProfileContractErrorV1::NonLocalVariableResolution {
                site: site.clone(),
            });
        };
        self.require_owner(binding)?;
        self.variable_uses.insert(site.clone());
        Ok(binding)
    }

    pub(super) fn assignment_binding(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        site: &SourceExprSiteV1,
    ) -> Result<BindingRefV1, TrivialProfileContractErrorV1> {
        let resolution = product.assignment_target(site).ok_or_else(|| {
            TrivialProfileContractErrorV1::MissingAssignmentResolution { site: site.clone() }
        })?;
        let ResolvedAssignmentTargetV1::BindingRebind(binding) = resolution else {
            return Err(
                TrivialProfileContractErrorV1::NonBindingAssignmentResolution {
                    site: site.clone(),
                },
            );
        };
        self.require_owner(*binding)?;
        self.assignments.insert(site.clone());
        Ok(*binding)
    }

    pub(super) fn direct_call_target(
        &mut self,
        product: &VerifiedResolvedFunctionV1,
        site: &SourceExprSiteV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        product.direct_call_target(site).ok_or_else(|| {
            TrivialProfileContractErrorV1::MissingDirectCallResolution { site: site.clone() }
        })?;
        self.direct_calls.insert(site.clone());
        Ok(())
    }

    pub(super) fn verify(
        &self,
        product: &VerifiedResolvedFunctionV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        verify_resolved_fact_coverage_v1(
            product,
            &self.declarations,
            &self.variable_uses,
            &self.assignments,
            &self.direct_calls,
        )
    }

    fn require_owner(&self, binding: BindingRefV1) -> Result<(), TrivialProfileContractErrorV1> {
        if binding.owner() != self.owner {
            return Err(TrivialProfileContractErrorV1::ForeignBinding { binding });
        }
        Ok(())
    }
}

impl TrivialProfileDraftV1 {
    pub(super) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            parameter_entries: Vec::new(),
            values: Vec::new(),
            direct_calls: Vec::new(),
            definitions: Vec::new(),
            merge_profiles: Vec::new(),
            ordered_subjects: Vec::new(),
        }
    }

    pub(super) fn record_parameter_entry(
        &mut self,
        site: SourceBindingSiteV1,
        binding: BindingRefV1,
        formal_index: u32,
        source_name: String,
        abi: ExactTrivialParameterAbiV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        let expected_site = SourceBindingSiteV1::Parameter {
            index: formal_index,
        };
        if site != expected_site
            || formal_index as usize != self.parameter_entries.len()
            || abi != ExactTrivialParameterAbiV1::I64
        {
            return Err(
                TrivialProfileContractErrorV1::ParameterEntryContractMismatch { formal_index },
            );
        }
        let representation = TrivialRepresentationV1::InlineI64;
        self.record_definition(
            binding,
            TrivialBindingDefinitionOriginV1::Declaration(site.clone()),
            representation,
        )?;
        self.parameter_entries
            .push(VerifiedTrivialParameterEntryV1::new(
                site,
                binding,
                formal_index,
                source_name,
                abi,
                representation,
            ));
        Ok(())
    }

    pub(super) fn record_value(
        &mut self,
        site: SourceExprSiteV1,
        representation: TrivialRepresentationV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        self.record_subject(TrivialProfileCoverageSubjectV1::Value(site.clone()))?;
        self.values
            .push(VerifiedLocatedTrivialValueV1::new(site, representation));
        Ok(())
    }

    pub(super) fn record_direct_call(
        &mut self,
        row: VerifiedTrivialDirectCallV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        self.record_subject(TrivialProfileCoverageSubjectV1::DirectCall(
            row.site().clone(),
        ))?;
        self.direct_calls.push(row);
        Ok(())
    }

    pub(super) fn record_definition(
        &mut self,
        binding: BindingRefV1,
        origin: TrivialBindingDefinitionOriginV1,
        representation: TrivialRepresentationV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        self.require_owner(binding)?;
        self.record_subject(TrivialProfileCoverageSubjectV1::Definition {
            binding,
            origin: origin.clone(),
        })?;
        self.definitions
            .push(VerifiedTrivialBindingDefinitionV1::new(
                binding,
                origin,
                representation,
            ));
        Ok(())
    }

    pub(super) fn record_merge_profile(
        &mut self,
        statement: crate::mir::resolved_semantics::SourceStmtSiteV1,
        binding: BindingRefV1,
        representation: TrivialRepresentationV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        self.require_owner(binding)?;
        self.record_subject(TrivialProfileCoverageSubjectV1::IfMergeProfile {
            statement: statement.clone(),
            binding,
        })?;
        self.merge_profiles
            .push(VerifiedTrivialIfMergeProfileV1::new(
                statement,
                binding,
                representation,
            ));
        Ok(())
    }

    pub(super) fn record_subject(
        &mut self,
        subject: TrivialProfileCoverageSubjectV1,
    ) -> Result<(), TrivialProfileContractErrorV1> {
        if self.ordered_subjects.contains(&subject) {
            return Err(TrivialProfileContractErrorV1::DuplicateCoverage { subject });
        }
        self.ordered_subjects.push(subject);
        Ok(())
    }

    pub(super) fn finish(self) -> TrivialProfilePartsV1 {
        TrivialProfilePartsV1 {
            parameter_entries: self.parameter_entries,
            values: self.values,
            direct_calls: self.direct_calls,
            definitions: self.definitions,
            merge_profiles: self.merge_profiles,
            coverage: VerifiedTrivialProfileCoverageV1::from_verified_order(self.ordered_subjects),
        }
    }

    fn require_owner(&self, binding: BindingRefV1) -> Result<(), TrivialProfileContractErrorV1> {
        if binding.owner() != self.owner {
            return Err(TrivialProfileContractErrorV1::ForeignBinding { binding });
        }
        Ok(())
    }
}

pub(super) fn verify_terminal_completion_co_seal_v1(
    owner: FunctionOwnerIdV1,
    terminal: &TrivialTerminalProfileV1,
    completion: &VerifiedFunctionCompletionV1,
) -> Result<(), TrivialProfileContractErrorV1> {
    let matches = match terminal {
        TrivialTerminalProfileV1::ExplicitValue { statement, .. } => {
            completion.explicit_site() == Some(statement) && completion.returns_value()
        }
        TrivialTerminalProfileV1::ExplicitNoValue { statement } => {
            completion.explicit_site() == Some(statement) && !completion.returns_value()
        }
        TrivialTerminalProfileV1::ImplicitNoValue { body, body_end } => completion
            .implicit_body_end()
            .is_some_and(|(actual_body, actual_end)| {
                actual_body == body && actual_end == *body_end
            }),
    };
    if !matches || completion.owner() != owner {
        return Err(TrivialProfileContractErrorV1::TerminalCardinality);
    }
    Ok(())
}

pub(super) fn verify_resolved_fact_coverage_v1(
    product: &VerifiedResolvedFunctionV1,
    declarations: &BTreeSet<SourceBindingSiteV1>,
    variable_uses: &BTreeSet<SourceExprSiteV1>,
    assignments: &BTreeSet<SourceExprSiteV1>,
    direct_calls: &BTreeSet<SourceExprSiteV1>,
) -> Result<(), TrivialProfileContractErrorV1> {
    let expected_declarations = product
        .declaration_sites()
        .cloned()
        .collect::<BTreeSet<_>>();
    if expected_declarations != *declarations {
        let (missing, extra) = set_difference(&expected_declarations, declarations);
        return Err(
            TrivialProfileContractErrorV1::DeclarationFactCoverageMismatch { missing, extra },
        );
    }

    let expected_uses = product
        .variable_refs()
        .map(|(site, _)| site.clone())
        .collect::<BTreeSet<_>>();
    if expected_uses != *variable_uses {
        let (missing, extra) = set_difference(&expected_uses, variable_uses);
        return Err(TrivialProfileContractErrorV1::VariableFactCoverageMismatch { missing, extra });
    }

    let expected_assignments = product
        .assignment_targets()
        .map(|(site, _)| site.clone())
        .collect::<BTreeSet<_>>();
    if expected_assignments != *assignments {
        let (missing, extra) = set_difference(&expected_assignments, assignments);
        return Err(
            TrivialProfileContractErrorV1::AssignmentFactCoverageMismatch { missing, extra },
        );
    }
    let expected_calls = product
        .direct_call_targets()
        .map(|(site, _)| site.clone())
        .collect::<BTreeSet<_>>();
    if expected_calls != *direct_calls {
        let (missing, extra) = set_difference(&expected_calls, direct_calls);
        return Err(
            TrivialProfileContractErrorV1::DirectCallFactCoverageMismatch { missing, extra },
        );
    }
    Ok(())
}

fn set_difference<T: Clone + Ord>(
    expected: &BTreeSet<T>,
    visited: &BTreeSet<T>,
) -> (Box<[T]>, Box<[T]>) {
    (
        expected
            .difference(visited)
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
        visited
            .difference(expected)
            .cloned()
            .collect::<Vec<_>>()
            .into_boxed_slice(),
    )
}
