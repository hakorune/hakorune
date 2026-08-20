//! Dedicated Script direct-static Recipe facts.
//!
//! This is intentionally not an extension of the scalar Script Recipe or of
//! the Loop Recipe vocabulary.  The result-publication owner is the only
//! Facts input; this module alone issues the opaque Recipe-local key.  The
//! accepted shape is deliberately narrow: the call must be the final value of
//! the Script body (a final Sequence item or the root Return value).

use std::collections::BTreeMap;

use crate::mir::resolved_semantics::{
    BodyShapeRelationV1, FunctionOwnerIdV1, SourceExprSiteV1, SourcePathSegmentV1,
    SourceStmtSiteV1, VerifiedScriptRootDemandWindowV1,
};

use super::normal_script_direct_static_result_publication_owner::VerifiedScriptDirectStaticResultPublicationOwnerV1;
use super::normal_script_source_continuation::ScriptSourceContinuationTerminalV1;
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticRecipeIssueV1 {
    SourceOwnerMismatch,
    NonFinalTerminal(SourceStmtSiteV1),
    MissingFinalValueRelation(SourceExprSiteV1),
    DuplicateFinalValueRelation(SourceExprSiteV1),
    TerminalRelationMismatch(SourceExprSiteV1),
}

/// A producer-local key.  It is not a source site, callable key, or physical
/// identifier; only this Recipe producer may issue it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct ScriptDirectStaticRecipeKeyV1(u32);

impl ScriptDirectStaticRecipeKeyV1 {
    #[cfg(test)]
    pub(super) const fn from_ordinal_for_test(ordinal: u32) -> Self {
        Self(ordinal)
    }

    pub(super) const fn ordinal(self) -> u32 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticRecipeDestinationV1 {
    FinalSequence { statement: SourceStmtSiteV1 },
    RootReturn { statement: SourceStmtSiteV1 },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticRecipeDemandV1 {
    key: ScriptDirectStaticRecipeKeyV1,
    source_owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    argument_sites: Box<[SourceExprSiteV1]>,
    result_site: SourceExprSiteV1,
    parent_relations: Box<[BodyShapeRelationV1]>,
    destination: ScriptDirectStaticRecipeDestinationV1,
    target: CanonicalSameModuleCallableKeyV1,
    representation: VerifiedCallableResultRepresentationV1,
    required_callee_i64_arguments: Box<[u32]>,
}

impl VerifiedScriptDirectStaticRecipeDemandV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        key: ScriptDirectStaticRecipeKeyV1,
        source_owner: FunctionOwnerIdV1,
        call_site: SourceExprSiteV1,
        receiver_site: SourceExprSiteV1,
        argument_sites: Box<[SourceExprSiteV1]>,
        result_site: SourceExprSiteV1,
        parent_relations: Box<[BodyShapeRelationV1]>,
        destination: ScriptDirectStaticRecipeDestinationV1,
        target: CanonicalSameModuleCallableKeyV1,
        representation: VerifiedCallableResultRepresentationV1,
        required_callee_i64_arguments: Box<[u32]>,
    ) -> Self {
        Self {
            key,
            source_owner,
            call_site,
            receiver_site,
            argument_sites,
            result_site,
            parent_relations,
            destination,
            target,
            representation,
            required_callee_i64_arguments,
        }
    }

    pub(super) const fn key(&self) -> ScriptDirectStaticRecipeKeyV1 {
        self.key
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(super) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(super) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        &self.argument_sites
    }

    pub(super) const fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(super) fn parent_relations(&self) -> &[BodyShapeRelationV1] {
        &self.parent_relations
    }

    pub(super) const fn destination(&self) -> &ScriptDirectStaticRecipeDestinationV1 {
        &self.destination
    }

    pub(super) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(super) const fn representation(&self) -> &VerifiedCallableResultRepresentationV1 {
        &self.representation
    }

    pub(super) fn required_callee_i64_arguments(&self) -> &[u32] {
        &self.required_callee_i64_arguments
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticRecipeV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<ScriptDirectStaticRecipeKeyV1, VerifiedScriptDirectStaticRecipeDemandV1>,
}

impl VerifiedScriptDirectStaticRecipeV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<ScriptDirectStaticRecipeKeyV1, VerifiedScriptDirectStaticRecipeDemandV1>,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(super) fn issue(
        owner: &VerifiedScriptDirectStaticResultPublicationOwnerV1,
        window: &VerifiedScriptRootDemandWindowV1,
    ) -> Result<Self, ScriptDirectStaticRecipeIssueV1> {
        let source_owner = owner.source_owner();
        let mut rows = BTreeMap::new();
        for (ordinal, (_, demand)) in owner.rows().enumerate() {
            if demand.source_owner() != source_owner {
                return Err(ScriptDirectStaticRecipeIssueV1::SourceOwnerMismatch);
            }
            let terminal = demand.terminal();
            let statement = match terminal {
                ScriptSourceContinuationTerminalV1::Sequence(statement)
                | ScriptSourceContinuationTerminalV1::Return(statement) => statement,
            };
            let Some((statement_ordinal, _)) = window
                .entries()
                .iter()
                .enumerate()
                .find(|(_, entry)| entry.site() == statement)
            else {
                return Err(ScriptDirectStaticRecipeIssueV1::NonFinalTerminal(
                    statement.clone(),
                ));
            };
            if !window.is_final_ordinal(statement_ordinal) {
                return Err(ScriptDirectStaticRecipeIssueV1::NonFinalTerminal(
                    statement.clone(),
                ));
            }

            validate_terminal_relation(
                &terminal,
                &statement,
                demand.call_site(),
                demand.parent_relations(),
            )?;

            let key = ScriptDirectStaticRecipeKeyV1(
                u32::try_from(ordinal).expect("Script Recipe row count fits u32"),
            );
            let destination = match terminal {
                ScriptSourceContinuationTerminalV1::Sequence(_) => {
                    ScriptDirectStaticRecipeDestinationV1::FinalSequence {
                        statement: statement.clone(),
                    }
                }
                ScriptSourceContinuationTerminalV1::Return(_) => {
                    ScriptDirectStaticRecipeDestinationV1::RootReturn {
                        statement: statement.clone(),
                    }
                }
            };
            let row = VerifiedScriptDirectStaticRecipeDemandV1 {
                key,
                source_owner,
                call_site: demand.call_site().clone(),
                receiver_site: demand.receiver_site().clone(),
                argument_sites: demand.argument_sites().to_vec().into_boxed_slice(),
                result_site: demand.result_site().clone(),
                parent_relations: demand.parent_relations().to_vec().into_boxed_slice(),
                destination,
                target: demand.target().clone(),
                representation: demand.representation().clone(),
                required_callee_i64_arguments: demand
                    .required_callee_i64_arguments()
                    .to_vec()
                    .into_boxed_slice(),
            };
            rows.insert(key, row);
        }
        Ok(Self {
            source_owner,
            source_identity: owner.source_identity(),
            rows,
        })
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn source_identity(&self) -> usize {
        self.source_identity
    }

    pub(super) fn demand(
        &self,
        key: ScriptDirectStaticRecipeKeyV1,
    ) -> Option<&VerifiedScriptDirectStaticRecipeDemandV1> {
        self.rows.get(&key)
    }

    pub(super) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &ScriptDirectStaticRecipeKeyV1,
            &VerifiedScriptDirectStaticRecipeDemandV1,
        ),
    > {
        self.rows.iter()
    }

    pub(super) fn len(&self) -> usize {
        self.rows.len()
    }
}

/// Validate the only terminal shapes this Recipe is allowed to consume.
///
/// A bare final expression is itself the Sequence statement node and therefore
/// has no parent relation.  A root Return owns the value through exactly one
/// `Value` relation.  In particular, a final Local/Assignment, a nested call,
/// or a control/value wrapper must not be mistaken for a Script result merely
/// because it has a `Value`-shaped parent relation.
fn validate_terminal_relation(
    terminal: &ScriptSourceContinuationTerminalV1,
    statement: &SourceStmtSiteV1,
    call_site: &SourceExprSiteV1,
    parent_relations: &[BodyShapeRelationV1],
) -> Result<(), ScriptDirectStaticRecipeIssueV1> {
    let terminal_statement = match terminal {
        ScriptSourceContinuationTerminalV1::Sequence(statement)
        | ScriptSourceContinuationTerminalV1::Return(statement) => statement,
    };
    if terminal_statement != statement {
        return Err(ScriptDirectStaticRecipeIssueV1::TerminalRelationMismatch(
            call_site.clone(),
        ));
    }

    match terminal {
        ScriptSourceContinuationTerminalV1::Sequence(_) => {
            if call_site.node() == statement.node() && parent_relations.is_empty() {
                Ok(())
            } else {
                Err(ScriptDirectStaticRecipeIssueV1::TerminalRelationMismatch(
                    call_site.clone(),
                ))
            }
        }
        ScriptSourceContinuationTerminalV1::Return(_) => {
            if parent_relations.is_empty() {
                return Err(ScriptDirectStaticRecipeIssueV1::MissingFinalValueRelation(
                    call_site.clone(),
                ));
            }
            if parent_relations.len() != 1 {
                return Err(
                    ScriptDirectStaticRecipeIssueV1::DuplicateFinalValueRelation(call_site.clone()),
                );
            }
            let relation = &parent_relations[0];
            if relation.parent() == statement.node()
                && relation.role() == &SourcePathSegmentV1::Value
                && relation.child() == call_site
            {
                Ok(())
            } else {
                Err(ScriptDirectStaticRecipeIssueV1::TerminalRelationMismatch(
                    call_site.clone(),
                ))
            }
        }
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_recipe_tests.rs"]
mod tests;
