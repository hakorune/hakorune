//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-local initializer relation. Builder headers, symbol
//! scans, and post-lowering target inference are deliberately outside this
//! owner.

use super::instance_construction::{ConstructionEligibilityV1, ConstructionUnavailableV1};
use crate::mir::function::ObjectDestructionDispositionV1;
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
};

pub(crate) use self::birth_abi_handoff::BirthAbiHandoffV1;
use super::instance_constructor_semantic::{
    InstanceConstructorBirthLookupErrorV1, VerifiedInstanceConstructorSemanticBatchV1,
    VerifiedInstanceConstructorSemanticRowV1,
};
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instance_constructor_abi::{
    InstanceConstructorAbiErrorV1, InstanceConstructorAbiV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    CallerNewHomePrefixV1, HomePrefixUnavailableV1, ResultNewHomePrefixV1,
    SelectedNewArgumentUnavailableV1, TerminalI64AddReturnV1, TerminalI64FieldReturnV1,
    TerminalIntegerLiteralReturnV1, TerminalMapGetReturnV1, TerminalRelationV1,
    TerminalUnitReturnV1,
};
use crate::mir::resolved_semantics::DeclaredInstanceCallSemanticEffectV1;
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1,
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
#[path = "ordinary_new_field_write_claim.rs"]
mod field_write_claim;
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
#[path = "ordinary_new_lexical_instance_call.rs"]
mod lexical_instance_call;
#[path = "ordinary_new_local_commit.rs"]
mod local_commit;
pub(crate) use lexical_instance_call::LexicalInstanceCallDispositionRowV1;
pub(in crate::mir) use lexical_instance_call::{
    BorrowedFormalActualSourceV1, PreparedBorrowedFormalActualV1,
};
#[path = "ordinary_new_receiver_call_observation.rs"]
mod receiver_call_observation;
pub(crate) use receiver_call_observation::ReceiverCallClassObservationV1;
#[path = "ordinary_new_result_class_claim.rs"]
mod result_class_claim;
pub(in crate::mir::normal_callable_semantic_package) use result_class_claim::verified_value_return_sites;
pub(crate) use result_class_claim::OrdinaryNewResultClassV1;
#[path = "ordinary_new_root_instance_call.rs"]
mod root_instance_call;
#[path = "ordinary_new_terminal_access.rs"]
mod terminal_access;
#[path = "ordinary_new_terminal_home.rs"]
mod terminal_home;
pub(in crate::mir::normal_callable_semantic_package) use terminal_home::{
    entry_receiver_box_proof, receiver_scalar_field,
};
use candidate::OrdinaryNewCandidate;

pub(crate) use local_commit::{
    FinalizedBirthActualsV1, FinalizedRootResultAbiV1, FinalizedRootSourceHandoffV1,
};
pub(crate) use root_instance_call::RootInstanceCallDispositionRowV1;

#[derive(Debug)]
pub(crate) enum RootCallDispositionV1 {
    Direct(super::direct_call_loan::DirectCallDispositionRowV1),
    Instance(RootInstanceCallDispositionRowV1),
    Lexical(std::rc::Rc<local_commit::EmittedLexicalCallProjectionV1>),
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

/// One sealed owned field child of an object, in declaration order.
/// `Array` children release through `OwnedFieldResidenceRelease`;
/// `Object` children release through `OwnedObjectFieldRelease`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OwnedFieldChildV1 {
    pub(crate) field: CanonicalFieldRefV1,
    pub(crate) kind: OwnedFieldChildKindV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OwnedFieldChildKindV1 {
    Array,
    /// The child object's canonical identity; S0 admits only
    /// `PlainI64NoHook` children (the helper-teardown bound is later).
    Object(CanonicalObjectIdV1),
}

/// The sealed `new` evidence both claim families carry identically — one
/// shared core so the site/class/constructor/construction/object rows exist
/// once, never as duplicated fields to keep in sync.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrdinaryNewClaimCoreV1 {
    site: OwnedExprSiteV1,
    box_source: crate::parser::ParserOrdinaryBoxSourceRowV1,
    class: Box<str>,
    arity: usize,
    constructor: OrdinaryNewConstructorDispositionV1,
    construction: ConstructionEligibilityV1,
    object: CanonicalObjectIdV1,
    destruction: ObjectDestructionDispositionV1,
    argument_rows: Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1>,
    /// Owned field residences in declaration order; `Some` only when
    /// every residence-capable field's birth-side provider is sealed.
    children: Option<Box<[OwnedFieldChildV1]>>,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrdinaryNewAdmissionClaimV1 {
    core: OrdinaryNewClaimCoreV1,
    destination: BindingRefV1,
    declaration: SourceBindingSiteV1,
    home_prefix: Result<CallerNewHomePrefixV1, HomePrefixUnavailableV1>,
}

/// Bounded return-position claim for `return new <class>(...)`. The fresh
/// object's ownership transfers to the caller at the Return edge, so the
/// claim carries no destination binding or local declaration — the
/// destination-less `ResultNewHomePrefixV1` stands in their place. The
/// emitted object value must land on `Return { value }` exactly; the
/// caller-side Handle result ABI is a separate downstream family.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct OrdinaryNewResultClaimV1 {
    core: OrdinaryNewClaimCoreV1,
    home_prefix: Result<ResultNewHomePrefixV1, HomePrefixUnavailableV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OrdinaryNewClaimTakeErrorV1 {
    Unavailable,
    Mismatch,
}

#[derive(Debug)]
pub(crate) struct OrdinaryNewClaimLedgerV1 {
    claims: RefCell<BTreeMap<OwnedExprSiteV1, OrdinaryNewAdmissionClaimV1>>,
    // Return-position `new` claims keyed by the exact `new` site — the
    // bounded transfer-to-caller family inside this same ledger, not a
    // parallel issuer.
    result_claims: RefCell<BTreeMap<OwnedExprSiteV1, OrdinaryNewResultClaimV1>>,
    ordinary_box_names: Box<[Box<str>]>,
    local_commits: RefCell<BTreeMap<OwnedExprSiteV1, local_commit::LocalCommitV1>>,
    root_validation: RefCell<local_commit::RootNewValidation>,
    child_physical_validation:
        RefCell<BTreeMap<FunctionOwnerIdV1, local_commit::ChildPhysicalValidation>>,
    // One exit progress row per (owner, source exit statement site): a
    // function with several `return` statements carries independent evidence
    // and one exit's row can never satisfy another.
    root_exits: RefCell<
        BTreeMap<(FunctionOwnerIdV1, SourceStmtSiteV1), local_commit::RootHomeExitProgress>,
    >,
    // Physical bindings for the bounded source-local Call prefix. These are
    // consumed by the existing root Call entry; they do not issue a target or
    // create a second lifecycle owner.
    root_local_call_bindings:
        RefCell<BTreeMap<FunctionOwnerIdV1, Vec<local_commit::RootLocalCallBindingGroupV1>>>,
    // The local-call sites `co_seal_lifecycle` routed through the lifecycle
    // lane under each caller owner. Sealed I64 `local_calls()` rows are the
    // candidate set only: a callee with no sealed lifecycle product keeps
    // the scalar Call route and owes no binding group, so this marked subset
    // — not the whole sealed prefix — is the binding-group expectation.
    lifecycle_local_call_sites: RefCell<BTreeMap<FunctionOwnerIdV1, Vec<OwnedExprSiteV1>>>,
    // Physical bindings for the source-issued typed Map read chain. The
    // selected caller records these only after the typed Invoke and its
    // Normal/Fault projections are emitted.
    map_read_bindings: RefCell<
        BTreeMap<
            FunctionOwnerIdV1,
            Vec<(
                OwnedExprSiteV1,
                Vec<(crate::mir::BasicBlockId, crate::mir::MirInstruction)>,
            )>,
        >,
    >,
    root_instance_calls:
        RefCell<BTreeMap<OwnedExprSiteV1, root_instance_call::RootInstanceCallDispositionSlotV1>>,
    lexical_source_targets:
        Option<lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1>,
    // Source-only transport closure. Errors are retained for the selected
    // prefix callback; neither Ok nor Err installs or changes a carrier.
    borrowed_formal_source:
        Option<Result<lexical_instance_call::PreparedBorrowedFormalIngressV1, String>>,
    borrowed_formal_actuals: lexical_instance_call::PendingBorrowedFormalActualsV1,
    // Existing formal values joined to the borrowed source projection. This
    // correspondence does not install a carrier or authorize a backend.
    borrowed_i64_results: BTreeMap<
        FunctionOwnerIdV1,
        Result<lexical_instance_call::BorrowedI64ResultSourceV1, String>,
    >,
    borrowed_entry_values: RefCell<
        BTreeMap<
            FunctionOwnerIdV1,
            lexical_instance_call::BorrowedOrdinaryEntryPhysicalV1,
        >,
    >,
    lexical_instance_calls: RefCell<
        BTreeMap<OwnedExprSiteV1, lexical_instance_call::LexicalInstanceCallDispositionSlotV1>,
    >,
    root_instance_call_expected: RefCell<BTreeSet<FunctionOwnerIdV1>>,
    field_reads: RefCell<BTreeMap<OwnedExprSiteV1, field_reads::FieldRead>>,
    // Argument-position `receiver.field` reads keyed by the exact
    // `FieldAccess` argument expression site — issued only for argument
    // rows the sealed observation proved `I64Field`, consumed once by the
    // selected `new` admission's argument materialization.
    argument_field_reads: RefCell<BTreeMap<OwnedExprSiteV1, field_reads::ArgumentFieldRead>>,
    // Local-initializer `receiver.field` reads keyed by the exact
    // `FieldAccess` initializer expression site — issued by the sealed
    // `local_read_field` proof and consumed once by the raw field-read
    // interception inside the claimed body's local statement lowering.
    local_field_reads: RefCell<BTreeMap<OwnedExprSiteV1, field_reads::LocalFieldRead>>,
    birth_abi_handoffs: RefCell<BTreeMap<OwnedExprSiteV1, BirthAbiHandoffV1>>,
    // Destination-less verified `Birth` recipes for `new` sites outside the
    // local-commit claim lane (non-`[Body, Initializer]` positions). An entry
    // admits only the typed `Callee::BirthConstructor` edge at that site; it
    // issues no destination, home, or lifecycle authority.
    birth_site_index:
        RefCell<BTreeMap<OwnedExprSiteV1, (VerifiedOrdinaryNewBirthRecipeV1, BirthAbiHandoffV1)>>,
    /// Emitted provider `new` Birth edges recorded by the construction-store
    /// emitter; the final root handoff folds them into `birth_actuals` with
    /// the same per-site exclusivity as claim rows.
    provider_births: RefCell<Vec<local_commit::ProviderBirthRecordV1>>,
    // Field-class provenance: `(owning box, field)` claims a class only
    // when every package write to that field is an attributed `me.` write
    // storing `new` of one agreed ordinary box. Read-only after issuance;
    // missing/ambiguous writers simply produce no row.
    field_write_claims: field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    // Owned field children per canonical object, issued once with the
    // claims: `Some` means every residence-capable declared field has a
    // sealed birth-side residence and `None` means the destruction
    // disposition requires children the package never proved. Objects
    // outside the owned-field dispositions carry no row at all.
    pub(super) owned_field_children:
        BTreeMap<CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
    // Callable result-class provenance: a selected callable claims a class
    // only when its sealed body ends in a `return` and every `return` row
    // constructs `new` of one agreed ordinary box. Read-only after
    // issuance; non-uniform evidence simply produces no row.
    callable_result_classes: result_class_claim::OrdinaryNewResultClassClaimsV1,
    // Claim-faithful `local x = me.m(..)` call-result observations, keyed
    // by the exact call site. Minted by the deferred pass after the
    // result-class fixpoint; `NullableObject` evidence never authorizes
    // Handle behavior.
    receiver_call_observations:
        BTreeMap<OwnedExprSiteV1, receiver_call_observation::ReceiverCallClassObservationV1>,
    // Terminal relations are keyed by the exact source exit statement site:
    // the App Main root map is owner-implied (App Main only), while children
    // keep `(owner -> site -> relation)` in the index. One exit's evidence
    // is never borrowed for another.
    terminal_relation: BTreeMap<SourceStmtSiteV1, TerminalRelationV1>,
    terminal_relation_index:
        BTreeMap<FunctionOwnerIdV1, Rc<BTreeMap<SourceStmtSiteV1, TerminalRelationV1>>>,
    terminal_integer_literal_value: RefCell<BTreeMap<SourceStmtSiteV1, crate::mir::ValueId>>,
    terminal_integer_literal_values:
        RefCell<BTreeMap<(FunctionOwnerIdV1, SourceStmtSiteV1), crate::mir::ValueId>>,
    terminal_i64_field_value: RefCell<BTreeMap<SourceStmtSiteV1, crate::mir::ValueId>>,
    terminal_i64_field_values:
        RefCell<BTreeMap<(FunctionOwnerIdV1, SourceStmtSiteV1), crate::mir::ValueId>>,
    terminal_result_progress: RefCell<BTreeMap<SourceStmtSiteV1, terminal_result::Progress>>,
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
#[path = "ordinary_new_ledger.rs"]
mod ledger;

#[derive(Debug)]
pub(crate) enum OrdinaryNewCoSealIssueV1 {
    CompletionSeed(super::physical_header::CallablePhysicalHeaderIssueV1),
    RootTerminalSource(HomePrefixUnavailableV1),
    BorrowedFormalIngress {
        site: OwnedExprSiteV1,
        issue: String,
    },
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
    /// A sealed `I64Field` argument row reached the ledger without the
    /// staged issuer proof at its exact `FieldAccess` site.
    ArgumentFieldReadMissing {
        site: OwnedExprSiteV1,
    },
    AppMainIdentityMissing,
    AppMainIdentityDuplicate,
}

#[cfg(test)]
#[path = "ordinary_new_lexical_instance_call_tests.rs"]
mod lexical_instance_call_tests;
#[cfg(test)]
#[path = "ordinary_new_terminal_result_tests.rs"]
mod terminal_result_tests;
#[cfg(test)]
#[path = "ordinary_new_coseal_tests.rs"]
mod tests;

pub(in crate::mir) use local_commit::{
    EmittedLexicalCallProjectionV1, FinalizedLexicalCallContextV1, LexicalCallArgumentProjectionV1,
    PreparedLexicalCallProjectionV1,
};
