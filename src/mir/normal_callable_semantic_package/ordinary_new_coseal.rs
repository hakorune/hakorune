//! Source-bound admission claims for the first Raw ordinary-`New` cohort.
//!
//! The claim is issued from the parser's final ordinary-box coverage and the
//! resolver's exact direct-local initializer relation. Builder headers, symbol
//! scans, and post-lowering target inference are deliberately outside this
//! owner.

use super::instance_construction::{ConstructionEligibilityV1, ConstructionUnavailableV1};
use crate::mir::function::ObjectDestructionDispositionV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::{cell::RefCell, collections::BTreeMap};

pub(crate) use self::birth_abi_handoff::{BirthAbiHandoffV1, BirthResultAbiV1};
use super::instance_constructor_semantic::{
    InstanceConstructorBirthLookupErrorV1, VerifiedInstanceConstructorSemanticBatchV1,
};
use super::selected_mapping::VerifiedSelectedCallableBatchMapV1;
use crate::ast::ASTNode;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::instance_constructor_abi::{
    InstanceConstructorAbiErrorV1, InstanceConstructorAbiV1,
};
use crate::mir::resolved_semantics::home_new_prefix::{
    issue_new_home_prefixes_v1, CallerNewHomePrefixV1, HomePrefixUnavailableV1,
    SelectedNewArgumentUnavailableV1, TerminalI64AddReturnV1, TerminalIntegerLiteralReturnV1, TerminalUnitReturnV1,
};
use crate::mir::resolved_semantics::DeclaredInstanceCallSemanticEffectV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, OwnedExprSiteV1, SourceBindingSiteV1, SourceExprSiteV1, SourceNodeSiteV1,
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
#[path = "ordinary_new_field_reads.rs"]
mod field_reads;
#[path = "ordinary_new_terminal_result.rs"]
mod terminal_result;
pub(crate) use terminal_result::PreparedTerminalI64AddReturnV1;
#[path = "birth_abi_handoff.rs"]
mod birth_abi_handoff;
#[path = "ordinary_new_local_commit.rs"]
mod local_commit;
#[path = "ordinary_new_terminal_home.rs"]
mod terminal_home;

pub(crate) use local_commit::{
    FinalizedRootBirthHandoffV1, FinalizedRootResultAbiV1, FinalizedRootSourceHandoffV1,
};

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
    local_commits: RefCell<BTreeMap<OwnedExprSiteV1, local_commit::NewLocalCommitV1>>,
    root_validation: RefCell<local_commit::RootNewValidation>,
    root_exit: RefCell<local_commit::RootHomeExitProgress>,
    field_reads: RefCell<BTreeMap<OwnedExprSiteV1, field_reads::FieldRead>>,
    birth_abi_handoffs: RefCell<BTreeMap<OwnedExprSiteV1, BirthAbiHandoffV1>>,
    terminal_result: Option<TerminalI64AddReturnV1>,
    terminal_unit_return: Option<TerminalUnitReturnV1>,
    terminal_integer_literal: Option<TerminalIntegerLiteralReturnV1>,
    terminal_integer_literal_value: RefCell<Option<crate::mir::ValueId>>,
    terminal_result_progress: RefCell<terminal_result::Progress>,
    root_completion: Option<
        Result<
            crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1,
            crate::mir::resolved_control_flow::FunctionCompletionVerificationErrorV1,
        >,
    >,
    // The parser-issued AppMain anchor travels with the Completion/terminal
    // source loan. It is comparison-only and never substitutes a key, name,
    // ABI, or physical root selection.
    app_main_identity: Option<crate::parser::CallableDeclarationIdentityV1>,
}

impl OrdinaryNewClaimLedgerV1 {
    #[cfg(test)]
    pub(super) fn root_completion_for_test(
        &self,
    ) -> &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        self.root_completion
            .as_ref()
            .expect("selected root")
            .as_ref()
            .expect("verified completion")
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
            root_exit: RefCell::new(local_commit::RootHomeExitProgress::Unprepared),
            field_reads: RefCell::new(BTreeMap::new()),
            birth_abi_handoffs: RefCell::new(BTreeMap::new()),
            terminal_result: None,
            terminal_unit_return: None,
            terminal_integer_literal: None,
            terminal_integer_literal_value: RefCell::new(None),
            terminal_result_progress: RefCell::new(terminal_result::Progress::Pending),
            root_completion: None,
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
                    .any(|binding| !commits.values().any(|row| row.installs(*binding)))
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
            local_commit::NewLocalCommitV1::pending(
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
            ),
        );
        Ok(Some(
            claims
                .remove(site)
                .expect("claim remained present after the checked lookup"),
        ))
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.claims.borrow().is_empty()
            && self
                .local_commits
                .borrow()
                .values()
                .all(|row| row.is_complete())
            && self.root_home_exit_is_complete()
            && self.field_reads_complete()
            && self.birth_abi_handoffs.borrow().is_empty()
            && self.terminal_result_complete()
            && (self.terminal_integer_literal.is_none() || self.terminal_integer_literal_value.borrow().is_some())
    }

    pub(crate) fn terminal_i64_add_return(&self) -> Option<&TerminalI64AddReturnV1> {
        self.terminal_result.as_ref()
    }
    pub(crate) fn terminal_unit_return(&self) -> Option<&TerminalUnitReturnV1> {
        self.terminal_unit_return.as_ref()
    }
    pub(crate) fn terminal_integer_literal_return(&self) -> Option<&TerminalIntegerLiteralReturnV1> { self.terminal_integer_literal.as_ref() }
    pub(crate) fn prepare_terminal_integer_literal_return(&self, owner: crate::mir::resolved_semantics::FunctionOwnerIdV1, site: &SourceNodeSiteV1) -> Result<Option<i64>, String> {
        let Some(relation) = self.terminal_integer_literal.as_ref() else { return Ok(None); };
        let Some(Ok(completion)) = self.root_completion.as_ref() else { return Err("[freeze:contract][ordinary-new/literal-completion-missing]".into()); };
        if self.terminal_result.is_some() || self.terminal_unit_return.is_some() || relation.owner() != owner || completion.owner() != owner || completion.explicit_site() != Some(relation.return_site()) || relation.return_site().node() != site || self.terminal_integer_literal_value.borrow().is_some() { return Err("[freeze:contract][ordinary-new/literal-source-drift]".into()); }
        Ok(Some(relation.value()))
    }
    pub(crate) fn record_terminal_integer_literal_return(&self, value: crate::mir::ValueId) -> Result<(), String> {
        if self.terminal_integer_literal.is_none() || self.terminal_integer_literal_value.replace(Some(value)).is_some() { return Err("[freeze:contract][ordinary-new/literal-duplicate]".into()); }
        Ok(())
    }

    /// Consumes no source product: it only checks that the selected emitter is
    /// at the exact Completion-backed bare-return site already retained here.
    pub(crate) fn prepare_terminal_unit_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        let Some(relation) = self.terminal_unit_return.as_ref() else { return Ok(false); };
        let Some(Ok(completion)) = self.root_completion.as_ref() else {
            return Err("[freeze:contract][ordinary-new/unit-return-completion-missing]".to_owned());
        };
        if self.terminal_result.is_some()
            || relation.owner() != owner
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OrdinaryNewCoSealIssueV1 {
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
    TerminalResultFieldReadMissing {
        site: OwnedExprSiteV1,
    },
    AppMainIdentityMissing,
    AppMainIdentityDuplicate,
}

pub(crate) fn issue_ordinary_new_claims_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main_identity: Option<&crate::parser::CallableDeclarationIdentityV1>,
    excluded_dynamic_batch_slot: Option<u32>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
) -> Result<OrdinaryNewClaimLedgerV1, OrdinaryNewCoSealIssueV1> {
    let app_main_batch_slot = app_main_identity
        .map(|identity| {
            let mut matches = batch
                .declarations()
                .filter(|declaration| declaration.identity().same_as(identity));
            let declaration = matches
                .next()
                .ok_or(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing)?;
            if matches.next().is_some() {
                return Err(OrdinaryNewCoSealIssueV1::AppMainIdentityDuplicate);
            }
            Ok(declaration.batch_slot())
        })
        .transpose()?;
    let mut claims = Vec::new();
    let mut root_completion = None;
    let mut root_field_reads = BTreeMap::new();
    let mut root_terminal_result = None;
    let mut root_terminal_unit_return = None;
    let mut root_terminal_integer_literal = None;
    let mut birth_abi_handoffs = BTreeMap::new();
    for declaration in batch.declarations() {
        let owner = declaration.owner();
        let batch_slot = declaration.batch_slot();
        // App Main is intentionally omitted from the generic selected-role
        // map; its exact parser identity is still admitted through the
        // source-backed batch slot supplied by the package issuer.
        let is_app_main = app_main_batch_slot == Some(batch_slot);
        if selected.role_for_batch_slot(batch_slot).is_none() && !is_app_main {
            continue;
        }
        if excluded_dynamic_batch_slot == Some(batch_slot) {
            continue;
        }
        // One source loan covers both initializer membership and binding
        // validation. Its order is not a Home availability/execution timeline.
        let (candidates, mut home_prefixes, mut argument_observations) = batch
            .with_lowering_input(batch_slot, |input| -> Result<_, OrdinaryNewCoSealIssueV1> {
                let function = input.function();
                let mut candidates = Vec::new();
                for initializer in function.expression_source().initializers() {
                    let Some(initializer_site) = initializer.initializer_site() else {
                        continue;
                    };
                    if !is_direct_local_initializer(initializer_site.node().segments()) {
                        continue;
                    }
                    let site = OwnedExprSiteV1::new(owner, initializer_site.clone());
                    let located = input.source().expr_at(&site).map_err(|_| {
                        OrdinaryNewCoSealIssueV1::SourceNavigation { site: site.clone() }
                    })?;
                    let ASTNode::New { class, arguments, field_initializers, .. } = located.node() else {
                        continue;
                    };
                    if initializer.binding().owner() != owner
                        || function.declaration_binding(initializer.declaration_site())
                            != Some(initializer.binding())
                        || !matches!(function.binding(initializer.binding()).map(|row| row.kind()),
                            Some(BindingKindV1::Local { .. }))
                    {
                        return Err(OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site });
                    }
                    candidates.push((site, class.clone().into_boxed_str(), arguments.len(),
                        initializer.binding(), initializer.declaration_site().clone(),
                        !field_initializers.is_empty()));
                }
                let selected: BTreeMap<_, _> = candidates.iter().filter(|(_, class, _, _, _, _)|
                    matches!(batch.ordinary_box_coverage().row_for(class.as_ref()), Ok(Some(_))))
                    .map(|(site, _, _, binding, _, _)| (site.clone(), *binding)).collect();
                let (home_prefixes, argument_observations) = if is_app_main && !selected.is_empty() {
                    let mut staged_reads = BTreeMap::new();
                    let mut field_is_integer = |site: &OwnedExprSiteV1, receiver_site: &SourceExprSiteV1, receiver, home, name: &str| {
                        let field = terminal_home::initialized_integer_field(
                            batch, instance_constructors, &candidates, &selected, home, name)?;
                        let Some(field) = field else { return Ok(false); };
                        if staged_reads.insert(site.clone(), field_reads::FieldRead {
                            receiver_site: receiver_site.clone(), receiver, home, field,
                            progress: field_reads::Progress::Pending,
                        }).is_some() { return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site: site.clone() }); }
                        Ok(true)
                    };
                    match crate::mir::resolved_control_flow::verify_function_completion_with_new_homes_and_argument_observations_v1(
                        input, &selected, &mut field_is_integer)? {
                        Ok((completion, prefixes, terminal_result, terminal_unit_return, terminal_integer_literal, observations)) => {
                            if matches!(completion.cleanup().terminal_homes(), Some(Ok(_))) {
                                if let Some(result) = &terminal_result {
                                    if result.owner() != input.owner()
                                        || result.field_reads().iter().any(|site|
                                            !staged_reads.contains_key(site))
                                    {
                                        return Err(
                                            OrdinaryNewCoSealIssueV1::TerminalResultFieldReadMissing {
                                                site: result.add_site().clone(),
                                            },
                                        );
                                    }
                                }
                                root_field_reads = staged_reads;
                                root_terminal_result = terminal_result;
                                root_terminal_unit_return = terminal_unit_return;
                                root_terminal_integer_literal = terminal_integer_literal;
                            }
                            root_completion = Some(Ok(completion));
                            (prefixes, observations)
                        }
                        Err(error) => {
                            root_completion = Some(Err(error));
                            (issue_new_home_prefixes_v1(input, &selected), BTreeMap::new())
                        }
                    }
                } else { (issue_new_home_prefixes_v1(input, &selected), BTreeMap::new()) };
                Ok((candidates, home_prefixes, argument_observations))
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
        for (site, class, arity, destination, declaration, has_overrides) in candidates {
            let argument_rows = argument_observations
                .remove(&site)
                .map(convert_selected_new_arguments)
                .unwrap_or_else(|| {
                    Err(SelectedNewArgumentUnavailableV1::SourceMismatch {
                        new_site: site.clone(),
                    })
                });
            let Some(box_source) = batch
                .ordinary_box_coverage()
                .row_for(class.as_ref())
                .map_err(|_| OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageDuplicate {
                    site: site.clone(),
                    class: class.clone(),
                })?
            else {
                // Builtin/plugin constructors retain their existing
                // compatibility owner.  They are deliberately outside the
                // source-backed ordinary-Box claim ledger; only an unknown
                // user Box is a coverage error here.
                if crate::box_trait::is_builtin_box(class.as_ref()) {
                    continue;
                }
                return Err(OrdinaryNewCoSealIssueV1::OrdinaryBoxCoverageMissing { site, class });
            };
            let (object, destruction) =
                instance_constructors
                    .destruction_for(box_source)
                    .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                        site: site.clone(),
                        class: class.clone(),
                        error,
                    })?;
            let construction = if has_overrides {
                Err(ConstructionUnavailableV1::OverrideUnsupported)
            } else {
                instance_constructors
                    .construction_for(box_source, arity)
                    .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                        site: site.clone(),
                        class: class.clone(),
                        error,
                    })?
                    .clone()
            };
            if matches!(&construction, Ok(plan) if plan.object() != object) {
                return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                    site,
                    class,
                    arity,
                });
            }
            let constructor =
                match instance_constructors
                    .birth_for(box_source, arity)
                    .map_err(|error| OrdinaryNewCoSealIssueV1::ConstructorLookup {
                        site: site.clone(),
                        class: class.clone(),
                        error,
                    })? {
                    Some(row) => {
                        if row.box_name() != class.as_ref()
                            || usize::try_from(row.source_arity()).ok() != Some(arity)
                        {
                            return Err(OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                                site,
                                class,
                                arity,
                            });
                        }
                        let abi = InstanceConstructorAbiV1::issue(arity).map_err(|error| {
                            OrdinaryNewCoSealIssueV1::ConstructorAbi {
                                site: site.clone(),
                                class: class.clone(),
                                error,
                            }
                        })?;
                        let target = row
                            .published_birth_key()
                            .filter(|key| {
                                key.namespace() == SameModuleCallableNamespaceV1::BirthConstructor
                                    && key.owner() == row.box_name()
                                    && key.arity() == row.source_arity()
                            })
                            .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthTargetInvalid {
                                site: site.clone(),
                                class: class.clone(),
                                arity,
                            })?
                            .clone();
                        row.birth_completion()
                            .filter(|completion| {
                                row.forest().roots() == [completion.owner()]
                                    && !completion.returns_value()
                            })
                            .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthCompletionNotUnit {
                                site: site.clone(),
                                class: class.clone(),
                            })?;
                        let effect = row
                            .birth_effect()
                            .filter(|effect| {
                                *effect == DeclaredInstanceCallSemanticEffectV1::OpaqueObservable
                            })
                            .ok_or_else(|| OrdinaryNewCoSealIssueV1::BirthEffectUnsupported {
                                site: site.clone(),
                                class: class.clone(),
                            })?;
                        let birth_abi = BirthAbiHandoffV1::issue(row, target.clone(), abi)
                            .map_err(|_| OrdinaryNewCoSealIssueV1::ConstructorRelationMismatch {
                                site: site.clone(),
                                class: class.clone(),
                                arity,
                            })?;
                        if birth_abi_handoffs.insert(site.clone(), birth_abi).is_some() {
                            return Err(OrdinaryNewCoSealIssueV1::DuplicateSite {
                                site: site.clone(),
                            });
                        }
                        OrdinaryNewConstructorDispositionV1::Birth(
                            VerifiedOrdinaryNewBirthRecipeV1 {
                                source_id: row.source_id().clone(),
                                target,
                                effect,
                                abi,
                            },
                        )
                    }
                    None => no_birth_constructor_disposition(&site, &class, arity)?,
                };
            if claims
                .iter()
                .any(|claim: &OrdinaryNewAdmissionClaimV1| claim.site == site)
            {
                return Err(OrdinaryNewCoSealIssueV1::DuplicateSite { site });
            }
            let home_prefix = home_prefixes.remove(&site).ok_or_else(|| {
                OrdinaryNewCoSealIssueV1::InitializerBindingMismatch { site: site.clone() }
            })?;
            claims.push(OrdinaryNewAdmissionClaimV1 {
                site: site.clone(),
                box_source: box_source.clone(),
                class,
                arity,
                constructor,
                destination,
                declaration,
                home_prefix,
                construction,
                object,
                destruction,
                argument_rows,
            });
        }
    }
    let names = batch
        .ordinary_box_coverage()
        .rows()
        .iter()
        .map(|row| row.name().to_owned().into_boxed_str())
        .collect();
    let mut ledger = OrdinaryNewClaimLedgerV1::issue(claims.into_boxed_slice(), names);
    ledger.root_completion = root_completion;
    ledger.field_reads = RefCell::new(root_field_reads);
    ledger.birth_abi_handoffs = RefCell::new(birth_abi_handoffs);
    ledger.terminal_result = root_terminal_result;
    ledger.terminal_unit_return = root_terminal_unit_return;
    ledger.terminal_integer_literal = root_terminal_integer_literal;
    ledger.app_main_identity = app_main_identity.cloned();
    Ok(ledger)
}

fn convert_selected_new_arguments(
    observation: crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentObservationV1,
) -> Result<Box<[OrdinaryNewTrivialArgumentV1]>, SelectedNewArgumentUnavailableV1> {
    use crate::mir::resolved_semantics::home_new_prefix::SelectedNewArgumentKindV1 as Source;
    observation
        .arguments()
        .map(|rows| {
            rows.iter()
                .map(|row| {
                    let kind = match row.kind() {
                        Source::Integer(value) => OrdinaryNewTrivialArgumentKindV1::Integer(*value),
                        Source::Bool(value) => OrdinaryNewTrivialArgumentKindV1::Bool(*value),
                        Source::Local { binding } => {
                            OrdinaryNewTrivialArgumentKindV1::Local { binding: *binding }
                        }
                    };
                    OrdinaryNewTrivialArgumentV1::new(
                        observation.new_site().owner(),
                        observation.new_site().clone(),
                        row.ordinal(),
                        row.site().clone(),
                        kind,
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .map_err(Clone::clone)
}

fn is_direct_local_initializer(segments: &[SourcePathSegmentV1]) -> bool {
    matches!(
        segments,
        [
            SourcePathSegmentV1::Body(_),
            SourcePathSegmentV1::Initializer(_)
        ]
    )
}

fn no_birth_constructor_disposition(
    site: &OwnedExprSiteV1,
    class: &str,
    arity: usize,
) -> Result<OrdinaryNewConstructorDispositionV1, OrdinaryNewCoSealIssueV1> {
    if arity == 0 {
        return Ok(OrdinaryNewConstructorDispositionV1::NoBirthZero);
    }
    Err(OrdinaryNewCoSealIssueV1::BirthConstructorMissing {
        site: site.clone(),
        class: class.into(),
        arity,
    })
}

#[cfg(test)]
#[path = "ordinary_new_terminal_result_tests.rs"]
mod terminal_result_tests;
#[cfg(test)]
#[path = "ordinary_new_coseal_tests.rs"]
mod tests;
