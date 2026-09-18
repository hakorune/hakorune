//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-local initializer relation. Builder headers, symbol
//! scans, and post-lowering target inference are deliberately outside this
//! owner.

use super::instance_construction::{ConstructionEligibilityV1, ConstructionUnavailableV1};
use crate::mir::function::ObjectDestructionDispositionV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

pub(crate) use self::birth_abi_handoff::{BirthAbiHandoffV1, BirthResultAbiV1};
use super::instance_constructor_semantic::{
    InstanceConstructorBirthLookupErrorV1, VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instance_constructor_abi::{
    InstanceConstructorAbiErrorV1, InstanceConstructorAbiV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, HomePrefixUnavailableV1, SelectedNewArgumentUnavailableV1,
    TerminalI64AddReturnV1, TerminalI64FieldReturnV1, TerminalIntegerLiteralReturnV1,
    TerminalMapGetReturnV1, TerminalRelationV1, TerminalUnitReturnV1,
};
use crate::mir::resolved_semantics::DeclaredInstanceCallSemanticEffectV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1,
};
use crate::mir::{Effect, EffectMask};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[path = "ordinary_new_claim_access.rs"]
mod claim_access;
#[path = "ordinary_new_arguments.rs"]
mod ordinary_new_arguments;
pub(crate) use ordinary_new_arguments::{
    OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
#[path = "ordinary_new_completion_index.rs"]
mod completion_index;
#[path = "ordinary_new_completion_lookup.rs"]
mod completion_lookup;
#[path = "ordinary_new_coseal_helpers.rs"]
mod coseal_helpers;
#[path = "ordinary_new_field_reads.rs"]
mod field_reads;
// The helper owns the exact `SourcePathSegmentV1::Initializer` admission shape.
use coseal_helpers::no_birth_constructor_disposition;
#[path = "ordinary_new_coseal_issue.rs"]
mod coseal_issue;
pub(super) use coseal_issue::issue_ordinary_source_cohort_v1;
#[path = "ordinary_new_terminal_result.rs"]
mod terminal_result;
pub(crate) use terminal_result::PreparedTerminalI64AddReturnV1;
#[path = "ordinary_new_terminal_field_return.rs"]
mod terminal_field_return;
pub(crate) use terminal_field_return::PreparedTerminalI64FieldReturnV1;
#[path = "ordinary_new_terminal_map_get_return.rs"]
mod terminal_map_get_return;
pub(crate) use terminal_map_get_return::PreparedTerminalMapGetReturnV1;
#[path = "birth_abi_handoff.rs"]
mod birth_abi_handoff;
#[path = "ordinary_new_candidate.rs"]
mod candidate;
#[path = "ordinary_new_local_commit.rs"]
mod local_commit;
#[path = "ordinary_new_root_instance_call.rs"]
mod root_instance_call;
#[path = "ordinary_new_terminal_access.rs"]
mod terminal_access;
#[path = "ordinary_new_terminal_home.rs"]
mod terminal_home;
use candidate::OrdinaryNewCandidate;

pub(crate) use local_commit::{
    FinalizedBirthActualsV1, FinalizedRootResultAbiV1, FinalizedRootSourceHandoffV1,
};
pub(crate) use root_instance_call::RootInstanceCallDispositionRowV1;

#[derive(Debug)]
pub(crate) enum RootCallDispositionV1 {
    Direct(super::direct_call_loan::DirectCallDispositionRowV1),
    Instance(RootInstanceCallDispositionRowV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedOrdinaryNewBirthRecipeV1 {
    source_id: crate::parser::ConstructorSourceIdV1,
    target: CanonicalSameModuleCallableKeyV1,
    effect: DeclaredInstanceCallSemanticEffectV1,
    abi: InstanceConstructorAbiV1,
}

impl VerifiedOrdinaryNewBirthRecipeV1 {
    pub(crate) fn source_id(&self) -> &crate::parser::ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) fn target(self) -> CanonicalSameModuleCallableKeyV1 {
        self.target
    }

    pub(crate) fn target_ref(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn abi(&self) -> InstanceConstructorAbiV1 {
        self.abi
    }

    /// Explicit conservative physical policy, not an effect inferred from MIR
    /// or source event counts. Completion and FieldSet Fault remain separate.
    pub(crate) fn physical_effect_mask(&self) -> EffectMask {
        EffectMask::MUT
            .union(EffectMask::IO)
            .union(EffectMask::WRITE)
            .add(Effect::Control)
            .add(Effect::P2P)
            .add(Effect::FFI)
            .add(Effect::Panic)
            .add(Effect::Alloc)
            .add(Effect::Global)
            .add(Effect::Async)
            .add(Effect::Unsafe)
            .add(Effect::Debug)
            .add(Effect::Barrier)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum OrdinaryNewConstructorDispositionV1 {
    NoBirthZero,
    Birth(VerifiedOrdinaryNewBirthRecipeV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrdinaryNewAdmissionClaimV1 {
    site: OwnedExprSiteV1,
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    class: Box<str>,
    arity: usize,
    constructor: OrdinaryNewConstructorDispositionV1,
    destination: BindingRefV1,
    declaration: SourceBindingSiteV1,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
    construction: ConstructionEligibilityV1,
    object: CanonicalObjectIdV1,
    destruction: ObjectDestructionDispositionV1,
    argument_rows: Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OrdinaryNewClaimTakeErrorV1 {
    Unavailable,
    Mismatch,
}

#[derive(Debug)]
pub(crate) struct OrdinaryNewClaimLedgerV1 {
    claims: RefCell<BTreeMap<OwnedExprSiteV1, OrdinaryNewAdmissionClaimV1>>,
    ordinary_box_names: Box<[Box<str>]>,
    local_commits: RefCell<BTreeMap<OwnedExprSiteV1, local_commit::LocalCommitV1>>,
    root_validation: RefCell<local_commit::RootNewValidation>,
    child_physical_validation:
        RefCell<BTreeMap<FunctionOwnerIdV1, local_commit::ChildPhysicalValidation>>,
    root_exits: RefCell<BTreeMap<FunctionOwnerIdV1, local_commit::RootHomeExitProgress>>,
    // Physical bindings for the bounded source-local Call prefix. These are
    // consumed by the existing root Call entry; they do not issue a target or
    // create a second lifecycle owner.
    root_local_call_bindings: RefCell<
        BTreeMap<
            FunctionOwnerIdV1,
            Vec<(
                OwnedExprSiteV1,
                Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
            )>,
        >,
    >,
    // The local-call sites `co_seal_lifecycle` routed through the lifecycle
    // lane under each caller owner. Sealed I64 `local_calls()` rows are the
    // candidate set only: a callee with no sealed lifecycle product keeps
    // the scalar Call route and owes no binding group, so this marked subset
    // — not the whole sealed prefix — is the binding-group expectation.
    lifecycle_local_call_sites: RefCell<BTreeMap<FunctionOwnerIdV1, Vec<OwnedExprSiteV1>>>,
    root_instance_calls:
        RefCell<BTreeMap<OwnedExprSiteV1, root_instance_call::RootInstanceCallDispositionSlotV1>>,
    root_instance_call_expected: RefCell<BTreeSet<FunctionOwnerIdV1>>,
    field_reads: RefCell<BTreeMap<OwnedExprSiteV1, field_reads::FieldRead>>,
    birth_abi_handoffs: RefCell<BTreeMap<OwnedExprSiteV1, BirthAbiHandoffV1>>,
    terminal_relation: Option<TerminalRelationV1>,
    terminal_relation_index: BTreeMap<FunctionOwnerIdV1, Rc<TerminalRelationV1>>,
    terminal_integer_literal_value: RefCell<Option<crate::mir::ValueId>>,
    terminal_integer_literal_values: RefCell<BTreeMap<FunctionOwnerIdV1, crate::mir::ValueId>>,
    terminal_i64_field_value: RefCell<Option<crate::mir::ValueId>>,
    terminal_i64_field_values: RefCell<BTreeMap<FunctionOwnerIdV1, crate::mir::ValueId>>,
    terminal_result_progress: RefCell<terminal_result::Progress>,
    root_completion: Option<
        Result<
            Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
            crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1,
        >,
    >,
    completion_index: BTreeMap<
        FunctionOwnerIdV1,
        Result<
            Rc<crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1>,
            crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1,
        >,
    >,
    // The parser-issued AppMain anchor travels with the Completion/terminal
    // source loan. It is comparison-only and never substitutes a key, name,
    // ABI, or physical root selection.
    app_main_identity: Option<crate::parser::CallableDeclarationIdentityV1>,
}
impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn requires_map_lifecycle_consumer(&self) -> bool {
        fn carries_map(
            flow: &crate::mir::resolved_semantics::home_new_prefix::RootHomeFlow,
        ) -> bool {
            !flow.maps().is_empty()
                || flow.local_calls().iter().any(|call| {
                    call.result()
                    == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Map
                })
        }
        let indexed = self.completion_index.values().any(|row| {
            row.as_ref()
                .ok()
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(carries_map)
        });
        indexed
            || self
                .root_completion
                .as_ref()
                .and_then(|row| row.as_ref().ok())
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(carries_map)
    }
    #[cfg(test)]
    pub(super) fn root_completion_for_test(
        &self,
    ) -> &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        self.root_completion
            .as_ref()
            .expect("selected root")
            .as_ref()
            .expect("verified completion")
            .as_ref()
    }
    #[cfg(test)]
    pub(super) fn pending_claims_for_test(
        &self,
    ) -> std::cell::Ref<'_, BTreeMap<OwnedExprSiteV1, OrdinaryNewAdmissionClaimV1>> {
        self.claims.borrow()
    }

    pub(crate) fn issue(
        claims: Box<[OrdinaryNewAdmissionClaimV1]>,
        ordinary_box_names: Box<[Box<str>]>,
    ) -> Self {
        Self {
            claims: RefCell::new(
                claims
                    .into_vec()
                    .into_iter()
                    .map(|claim| (claim.site().clone(), claim))
                    .collect(),
            ),
            ordinary_box_names,
            local_commits: RefCell::new(BTreeMap::new()),
            root_validation: RefCell::new(local_commit::RootNewValidation::Unregistered),
            child_physical_validation: RefCell::new(BTreeMap::new()),
            root_exits: RefCell::new(BTreeMap::new()),
            root_local_call_bindings: RefCell::new(BTreeMap::new()),
            lifecycle_local_call_sites: RefCell::new(BTreeMap::new()),
            root_instance_calls: RefCell::new(BTreeMap::new()),
            root_instance_call_expected: RefCell::new(BTreeSet::new()),
            field_reads: RefCell::new(BTreeMap::new()),
            birth_abi_handoffs: RefCell::new(BTreeMap::new()),
            terminal_relation: None,
            terminal_relation_index: BTreeMap::new(),
            terminal_integer_literal_value: RefCell::new(None),
            terminal_integer_literal_values: RefCell::new(BTreeMap::new()),
            terminal_i64_field_value: RefCell::new(None),
            terminal_i64_field_values: RefCell::new(BTreeMap::new()),
            terminal_result_progress: RefCell::new(terminal_result::Progress::Pending),
            root_completion: None,
            completion_index: BTreeMap::new(),
            app_main_identity: None,
        }
    }

    pub(crate) fn try_take(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<Option<OrdinaryNewAdmissionClaimV1>, OrdinaryNewClaimTakeErrorV1> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(None);
        }
        let mut claims = self.claims.borrow_mut();
        let claim = claims
            .get(site)
            .ok_or(OrdinaryNewClaimTakeErrorV1::Unavailable)?;
        if claim.class() != class || claim.arity() != arity {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        let mut commits = self.local_commits.borrow_mut();
        if let Ok(prefix) = &claim.home_prefix {
            if prefix.destination() != claim.destination
                || prefix.required_unwind() != site
                || prefix
                    .prior_homes()
                    .iter()
                    .any(|binding| local_commit::installed_home(&commits, *binding).is_err())
            {
                return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
            }
        }
        let birth_target = match &claim.constructor {
            OrdinaryNewConstructorDispositionV1::NoBirthZero => None,
            OrdinaryNewConstructorDispositionV1::Birth(recipe) => Some(recipe.target_ref().clone()),
        };
        let birth_abi = self.birth_abi_handoffs.borrow_mut().remove(site);
        if birth_abi.as_ref().map(BirthAbiHandoffV1::target) != birth_target.as_ref() {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        commits.insert(
            site.clone(),
            local_commit::LocalCommitV1::Ordinary(local_commit::NewLocalCommitV1::pending(
                claim.destination,
                claim.declaration.clone(),
                claim.home_prefix.clone(),
                claim.box_source().clone(),
                claim.construction.clone(),
                claim.object,
                claim.destruction,
                birth_target,
                birth_abi,
                claim.argument_rows.clone(),
            )),
        );
        Ok(Some(
            claims
                .remove(site)
                .expect("claim remained present after the checked lookup"),
        ))
    }

    /// Consumes no source product: it only checks that the selected emitter is
    /// at the exact Completion-backed bare-return site already retained here.
    pub(crate) fn prepare_terminal_unit_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        if self.root_owner() != Some(owner) {
            return Ok(false);
        }
        let Some(TerminalRelationV1::Unit(relation)) = self.terminal_relation_for_owner(owner)
        else {
            return Ok(false);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(
                "[freeze:contract][ordinary-new/unit-return-completion-missing]".to_owned(),
            );
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || completion.explicit_site() != Some(relation.return_site())
            || relation.return_site().node() != site
        {
            return Err("[freeze:contract][ordinary-new/unit-return-source-drift]".to_owned());
        }
        Ok(true)
    }

    pub(crate) fn register_app_main_root(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> Result<(), String> {
        let Some(expected) = self.app_main_identity.as_ref() else {
            return Err("[freeze:contract][ordinary-new/app-main-identity-missing]".to_owned());
        };
        if !expected.same_as(identity) {
            return Err("[freeze:contract][ordinary-new/app-main-identity-mismatch]".to_owned());
        }
        self.register_new_root(owner)
    }

    /// Whether this declaration identity is the co-sealed App Main. The
    /// same-source dependency harness uses it to skip root registration
    /// for non-main owners — `register_new_root` would reject them
    /// against the root completion anyway.
    #[cfg(test)]
    pub(crate) fn is_app_main_identity(
        &self,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> bool {
        self.app_main_identity
            .as_ref()
            .is_some_and(|expected| expected.same_as(identity))
    }
}

#[derive(Debug)]
pub(crate) enum OrdinaryNewCoSealIssueV1 {
    CompletionSeed(super::physical_header::CallablePhysicalHeaderIssueV1),
    BatchLoan,
    SourceNavigation {
        site: OwnedExprSiteV1,
    },
    AllocationSiteNotDirectLocal {
        site: SourceExprSiteV1,
    },
    InitializerBindingMismatch {
        site: OwnedExprSiteV1,
    },
    OrdinaryBoxCoverageMissing {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    OrdinaryBoxCoverageDuplicate {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    ConstructorLookup {
        site: OwnedExprSiteV1,
        class: Box<str>,
        error: InstanceConstructorBirthLookupErrorV1,
    },
    ConstructorAbi {
        site: OwnedExprSiteV1,
        class: Box<str>,
        error: InstanceConstructorAbiErrorV1,
    },
    BirthTargetInvalid {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
    },
    BirthCompletionNotUnit {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    BirthEffectUnsupported {
        site: OwnedExprSiteV1,
        class: Box<str>,
    },
    ConstructorRelationMismatch {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
    },
    BirthConstructorMissing {
        site: OwnedExprSiteV1,
        class: Box<str>,
        arity: usize,
    },
    DuplicateSite {
        site: OwnedExprSiteV1,
    },
    FieldReadOwnerMismatch {
        site: OwnedExprSiteV1,
    },
    TerminalResultFieldReadMissing {
        site: OwnedExprSiteV1,
    },
    AppMainIdentityMissing,
    AppMainIdentityDuplicate,
}

#[cfg(test)]
#[path = "ordinary_new_terminal_result_tests.rs"]
mod terminal_result_tests;
#[cfg(test)]
#[path = "ordinary_new_coseal_tests.rs"]
mod tests;
