//! Source/Facts handoff for one Script direct-static Recipe.
//!
//! This is deliberately not a physical join signature.  The Recipe producer owns the
//! opaque key; this module only verifies that each key still names the exact
//! result-owner row and carries its already-sealed terminal destination.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::normal_script_direct_static_recipe::{
    ScriptDirectStaticRecipeDestinationV1, ScriptDirectStaticRecipeKeyV1,
    VerifiedScriptDirectStaticRecipeV1,
};
use crate::mir::builder::normal_script_direct_static_result_publication_owner::{
    VerifiedScriptDirectStaticResultPublicationDemandV1,
    VerifiedScriptDirectStaticResultPublicationOwnerV1,
};
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{BodyShapeRelationV1, FunctionOwnerIdV1, SourceExprSiteV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ScriptDirectStaticJoinHandoffIssueV1 {
    SourceIdentityMismatch,
    SourceOwnerMismatch,
    CardinalityMismatch,
    DuplicateCallSite(SourceExprSiteV1),
    PublicationRowMissing(SourceExprSiteV1),
    RecipeRowMissing(SourceExprSiteV1),
    RowMismatch(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct VerifiedScriptDirectStaticJoinRowV1 {
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

impl VerifiedScriptDirectStaticJoinRowV1 {
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
pub(super) struct VerifiedScriptDirectStaticJoinHandoffV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<ScriptDirectStaticRecipeKeyV1, VerifiedScriptDirectStaticJoinRowV1>,
}

impl VerifiedScriptDirectStaticJoinHandoffV1 {
    #[cfg(test)]
    pub(super) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<ScriptDirectStaticRecipeKeyV1, VerifiedScriptDirectStaticJoinRowV1>,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(super) fn issue(
        recipe: &VerifiedScriptDirectStaticRecipeV1,
        publication_owner: &VerifiedScriptDirectStaticResultPublicationOwnerV1,
    ) -> Result<Self, ScriptDirectStaticJoinHandoffIssueV1> {
        if recipe.source_identity() != publication_owner.source_identity() {
            return Err(ScriptDirectStaticJoinHandoffIssueV1::SourceIdentityMismatch);
        }
        if recipe.source_owner() != publication_owner.source_owner() {
            return Err(ScriptDirectStaticJoinHandoffIssueV1::SourceOwnerMismatch);
        }
        if recipe.len() != publication_owner.len() {
            return Err(ScriptDirectStaticJoinHandoffIssueV1::CardinalityMismatch);
        }

        let mut seen_sites = BTreeSet::new();
        let mut rows = BTreeMap::new();
        for (key, demand) in recipe.rows() {
            if !seen_sites.insert(demand.call_site().clone()) {
                return Err(ScriptDirectStaticJoinHandoffIssueV1::DuplicateCallSite(
                    demand.call_site().clone(),
                ));
            }
            let Some(owner_row) = publication_owner.demand(demand.call_site()) else {
                return Err(ScriptDirectStaticJoinHandoffIssueV1::PublicationRowMissing(
                    demand.call_site().clone(),
                ));
            };
            if !same_row(demand, owner_row) {
                return Err(ScriptDirectStaticJoinHandoffIssueV1::RowMismatch(
                    demand.call_site().clone(),
                ));
            }
            let row = VerifiedScriptDirectStaticJoinRowV1 {
                key: *key,
                source_owner: demand.source_owner(),
                call_site: demand.call_site().clone(),
                receiver_site: demand.receiver_site().clone(),
                argument_sites: demand.argument_sites().to_vec().into_boxed_slice(),
                result_site: demand.result_site().clone(),
                parent_relations: demand.parent_relations().to_vec().into_boxed_slice(),
                destination: demand.destination().clone(),
                target: demand.target().clone(),
                representation: demand.representation().clone(),
                required_callee_i64_arguments: demand
                    .required_callee_i64_arguments()
                    .to_vec()
                    .into_boxed_slice(),
            };
            if rows.insert(*key, row).is_some() {
                return Err(ScriptDirectStaticJoinHandoffIssueV1::RowMismatch(
                    demand.call_site().clone(),
                ));
            }
        }
        for (site, _) in publication_owner.rows() {
            if !seen_sites.contains(site) {
                return Err(ScriptDirectStaticJoinHandoffIssueV1::RecipeRowMissing(
                    site.clone(),
                ));
            }
        }
        Ok(Self {
            source_owner: recipe.source_owner(),
            source_identity: recipe.source_identity(),
            rows,
        })
    }

    pub(super) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(super) const fn source_identity(&self) -> usize {
        self.source_identity
    }

    pub(super) fn row(
        &self,
        key: ScriptDirectStaticRecipeKeyV1,
    ) -> Option<&VerifiedScriptDirectStaticJoinRowV1> {
        self.rows.get(&key)
    }

    pub(super) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &ScriptDirectStaticRecipeKeyV1,
            &VerifiedScriptDirectStaticJoinRowV1,
        ),
    > {
        self.rows.iter()
    }

    pub(super) fn into_site_rows(
        self,
    ) -> impl Iterator<Item = VerifiedScriptDirectStaticJoinRowV1> {
        self.rows.into_values()
    }

    pub(super) fn len(&self) -> usize {
        self.rows.len()
    }
}

fn same_row(
    recipe: &crate::mir::builder::normal_script_direct_static_recipe::VerifiedScriptDirectStaticRecipeDemandV1,
    owner: &VerifiedScriptDirectStaticResultPublicationDemandV1,
) -> bool {
    recipe.source_owner() == owner.source_owner()
        && recipe.call_site() == owner.call_site()
        && recipe.receiver_site() == owner.receiver_site()
        && recipe.argument_sites() == owner.argument_sites()
        && recipe.result_site() == owner.result_site()
        && recipe.parent_relations() == owner.parent_relations()
        && destination_matches(recipe.destination(), owner)
        && recipe.target() == owner.target()
        && recipe.representation() == owner.representation()
        && recipe.required_callee_i64_arguments() == owner.required_callee_i64_arguments()
}

fn destination_matches(
    destination: &ScriptDirectStaticRecipeDestinationV1,
    owner: &VerifiedScriptDirectStaticResultPublicationDemandV1,
) -> bool {
    match (destination, owner.terminal()) {
        (
            ScriptDirectStaticRecipeDestinationV1::FinalSequence { statement },
            crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1::Sequence(
                owner_statement,
            ),
        )
        | (
            ScriptDirectStaticRecipeDestinationV1::RootReturn { statement },
            crate::mir::builder::normal_script_source_continuation::ScriptSourceContinuationTerminalV1::Return(
                owner_statement,
            ),
        ) => statement == owner_statement,
        _ => false,
    }
}

#[cfg(test)]
#[path = "normal_script_direct_static_join_handoff_tests.rs"]
mod tests;

mod physical_input;
mod scalar_operand_recipe;

pub(in crate::mir) use physical_input::{
    VerifiedScriptDirectStaticPhysicalInputIssueV1,
    VerifiedScriptDirectStaticPhysicalInputRowV1,
    VerifiedScriptDirectStaticPhysicalInputV1,
};
pub(in crate::mir) use scalar_operand_recipe::{
    ScalarBinaryOperatorV1, ScalarOperandRecipeArgumentV1,
    ScalarOperandRecipeNodeV1, ScalarUnaryOperatorV1,
    VerifiedScriptDirectStaticScalarOperandRecipeIssueV1,
    VerifiedScriptDirectStaticScalarOperandRecipeV1,
};
