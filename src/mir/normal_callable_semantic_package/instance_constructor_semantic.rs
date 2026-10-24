//! Resolver-owned semantic rows for parser-issued instance constructors.

use crate::mir::function::CanonicalObjectDefinitionV1;
use hakorune_mir_defs::CanonicalObjectIdV1;
use std::collections::BTreeMap;
mod formal_contract;
mod object_definition;
#[path = "instance_construction_provider_source.rs"]
pub(super) mod provider_source;
mod source;
use super::instance_construction::{
    issue_construction_source_v1, ConstructionEligibilityV1, ConstructionPlanV1,
    ConstructionSourceEligibilityV1,
};
pub(crate) use formal_contract::{
    BirthFormalContractV1, BirthFormalDeclarationClassV1, BirthFormalPhysicalDispositionV1,
    BirthFormalUseCoverageV1,
};
pub(super) use source::capture_instance_constructor_sources_v1;

use crate::analysis::brand_program_declaration_catalog::VerifiedBrandProgramDeclarationCatalogV1;
use crate::ast::ASTNode;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::source_projection::VerifiedSourceProjectionV1;
use crate::mir::resolved_control_flow::{
    verify_function_completion_v1, FunctionCompletionVerificationErrorV1,
    VerifiedFunctionCompletionV1,
};
use crate::mir::resolved_semantics::{
    DeclaredInstanceCallSemanticEffectV1, FunctionOwnerIdV1, FunctionSemanticResolverSessionV1,
    FunctionSyntaxViewV1, ReceiverPolicyV1,
    ResolveSourceBoundSelectedCallableForestsWithBodyShapesOutcomeV1,
    SelectedCallableResolverDeferredBatchV1, SelectedCallableResolverInputV1,
    SemanticOwnerRootProfileV1, SourceBoundSelectedCallableResolverRejectV1,
    VerifiedResolvedBodyShapeInventoryV1, VerifiedSemanticOwnerForestV1,
};
use crate::parser::{
    ConstructorSourceIdV1, ConstructorSourceKindV1, ParserOrdinaryBoxSourceCoverageV1,
    ParserOrdinaryBoxSourceRowV1, VerifiedFinalCallableProgramSourceV1,
};

#[derive(Debug)]
pub(crate) enum InstanceConstructorSemanticBatchIssueV1 {
    ParserSyntax,
    SourceCoverage,
    Resolver(SourceBoundSelectedCallableResolverRejectV1),
    ResolverDeferred(SelectedCallableResolverDeferredBatchV1),
    MissingRoot,
    RootProfileMismatch,
    BodyShapeMissing,
    BodyShapeOwnerMismatch,
    BodyShapeResidual,
    Completion {
        _issue: FunctionCompletionVerificationErrorV1,
    },
    SourceProjection {
        _error: String,
    },
    ReceiverNonEscape {
        _issue: super::instance_constructor_non_escape::BirthReceiverNonEscapeIssueV1,
    },
    FormalContract {
        _issue: formal_contract::BirthFormalContractIssueV1,
    },
}

#[derive(Debug)]
pub(crate) struct InstanceConstructorSourceDataV1 {
    source_id: ConstructorSourceIdV1,
    box_source: ParserOrdinaryBoxSourceRowV1,
    object: CanonicalObjectIdV1,
    published_birth_key: Option<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1>,
    final_box_ordinal: u32,
    box_name: Box<str>,
    key: Box<str>,
    kind: ConstructorSourceKindV1,
    source_arity: u32,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    body_shapes: BTreeMap<FunctionOwnerIdV1, VerifiedResolvedBodyShapeInventoryV1>,
    birth_completion: Option<VerifiedFunctionCompletionV1>,
    birth_effect: Option<DeclaredInstanceCallSemanticEffectV1>,
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticRowV1 {
    source: InstanceConstructorSourceDataV1,
    construction: ConstructionEligibilityV1,
    construction_source: ConstructionSourceEligibilityV1,
    provider_static_arguments: Box<[std::rc::Rc<provider_source::ProviderStaticArgumentSourceV1>]>,
    formal_contracts: Box<[BirthFormalContractV1]>,
}

#[derive(Debug)]
pub(super) struct CapturedInstanceConstructorSourcesV1 {
    rows: Box<[InstanceConstructorSourceDataV1]>,
    box_sources: ParserOrdinaryBoxSourceCoverageV1,
    object_sources: Box<[(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)]>,
    declarations: Box<[object_definition::ObjectDeclarationSourceV1]>,
}

impl std::ops::Deref for VerifiedInstanceConstructorSemanticRowV1 {
    type Target = InstanceConstructorSourceDataV1;
    fn deref(&self) -> &Self::Target {
        &self.source
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedInstanceConstructorSemanticBatchV1 {
    rows: Box<[VerifiedInstanceConstructorSemanticRowV1]>,
    box_sources: ParserOrdinaryBoxSourceCoverageV1,
    object_sources: Box<[(ParserOrdinaryBoxSourceRowV1, CanonicalObjectIdV1)]>,
    object_definitions: std::cell::RefCell<Option<Box<[CanonicalObjectDefinitionV1]>>>,
    no_birth_construction: Vec<(
        ParserOrdinaryBoxSourceRowV1,
        ConstructionSourceEligibilityV1,
        ConstructionEligibilityV1,
    )>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InstanceConstructorBirthLookupErrorV1 {
    SourceArityOverflow,
    DuplicateBirth,
    ParentSourceMismatch,
    BirthArityMismatch,
    ObjectDefinitionsTransferred,
    ObjectDefinitionMissing,
}

impl VerifiedInstanceConstructorSemanticBatchV1 {
    pub(super) fn finish_execution_plans_v1(
        &mut self,
        ledger: &mut super::OrdinaryNewClaimLedgerV1,
    ) -> Result<(), String> {
        for row in self.rows.iter_mut() {
            use super::instance_construction::ConstructionUnavailableV1 as U;
            if row.kind != ConstructorSourceKindV1::Birth { continue; }
            let qualified = if row.source_arity == 0 && row.construction == Err(U::BodyCoverageUnsupported)
                && row.construction_source.as_ref().is_ok_and(|source| source.steps().iter().any(|step|
                    matches!(step, super::instance_construction::ConstructionSourceStepV1::StoredChildCall(_)))) {
                ConstructionPlanV1::qualify_ordered_unit_calls_v1(&row.construction_source,
                    |source, site| ledger.constructor_unit_step_ready_v1(source, site))?
            } else if row.construction == Err(U::FieldContractUnsupported) {
                let definitions = self.object_definitions.borrow();
                let definitions = definitions.as_deref().ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
                let definition = definitions.get(row.object().declaration_index() as usize)
                    .ok_or("[freeze:contract][constructor-unit/new-source-drift]")?;
                let declarations = definition.local_fields_for_layout()
                    .map_err(|_| "[freeze:contract][constructor-unit/new-source-drift]")?;
                // Fixed Array residences never acquire a Dynamic Text meaning.
                if row.construction_source.as_ref().is_ok_and(|source| source.stores().iter().any(|store|
                    definition.is_array_field(store.field().declaration_ordinal() as usize)
                        && store.mode().is_err())) { continue; }
                ConstructionPlanV1::qualify_source_dynamic_stores_v1(&row.construction_source, declarations,
                    &self.object_sources, definitions,
                    |store| {
                        let mode = ledger.constructor_dynamic_store_mode_v1(row, store);
                        mode
                    },
                    |store| ledger.constructor_text_store_sum_v1(row, store))
            } else { continue; };
            if let Err(reason) = &qualified {
            }
            let Ok(plan) = qualified else {
                continue;
            };
            // Receiver selection already installed canonical provider End
            // inventory in the same ledger; never issue it again here.
            for store in plan.stores() {
                if let super::instance_construction::ConstructionStoreRhsV1::ProviderConstruction {
                    object: Some(child), owned_fields, ..
                } = store.rhs() {
                    if !owned_fields.is_empty() {
                        let children = ledger.owned_field_children_for(child.object())
                            .flatten().ok_or("[freeze:contract][constructor-unit/provider-end-missing]")?;
                        if children != owned_fields.as_ref() {
                            return Err("[freeze:contract][constructor-unit/provider-end-drift]".into());
                        }
                    }
                }
            }
            ledger.install_late_constructor_plan_v1(row, &plan)?;
            row.construction = Ok(plan);
        }
        Ok(())
    }

    pub(crate) fn rows(&self) -> &[VerifiedInstanceConstructorSemanticRowV1] {
        &self.rows
    }

    /// Published `birth` caller key for one exact constructor source id.
    /// The key is sealed by selection; a constructor without a published
    /// `birth` contributes no provider loan.
    pub(crate) fn published_birth_key(
        &self,
        source_id: &ConstructorSourceIdV1,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        self.rows
            .iter()
            .find(|row| row.source_id().same_as(source_id))
            .and_then(|row| row.published_birth_key())
    }

    pub(crate) fn birth_for(
        &self,
        box_source: &ParserOrdinaryBoxSourceRowV1,
        source_arity: usize,
    ) -> Result<
        Option<&VerifiedInstanceConstructorSemanticRowV1>,
        InstanceConstructorBirthLookupErrorV1,
    > {
        let source_arity = u32::try_from(source_arity)
            .map_err(|_| InstanceConstructorBirthLookupErrorV1::SourceArityOverflow)?;
        if !matches!(self.box_sources.row_for(box_source.name()),
            Ok(Some(own)) if own.same_source_as(box_source))
        {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        if self.rows.iter().any(|row| {
            row.final_box_ordinal as usize == box_source.final_box_ordinal()
                && !row.box_source.same_source_as(box_source)
        }) {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        let mut matches = self.rows.iter().filter(|row| {
            row.box_source.same_source_as(box_source)
                && row.kind == ConstructorSourceKindV1::Birth
                && row.source_arity == source_arity
        });
        let Some(row) = matches.next() else {
            if self.rows.iter().any(|row| {
                row.box_source.same_source_as(box_source)
                    && row.kind == ConstructorSourceKindV1::Birth
            }) {
                return Err(InstanceConstructorBirthLookupErrorV1::BirthArityMismatch);
            }
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(InstanceConstructorBirthLookupErrorV1::DuplicateBirth);
        }
        Ok(Some(row))
    }

    /// The single `birth` row for one exact Box source row, regardless of
    /// source arity. `DuplicateBirth` and `ParentSourceMismatch` keep the
    /// same fail-fast boundary as `birth_for`.
    pub(crate) fn birth_row_for(
        &self,
        source: &ParserOrdinaryBoxSourceRowV1,
    ) -> Result<
        Option<&VerifiedInstanceConstructorSemanticRowV1>,
        InstanceConstructorBirthLookupErrorV1,
    > {
        if !matches!(self.box_sources.row_for(source.name()),
            Ok(Some(own)) if own.same_source_as(source))
        {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        if self.rows.iter().any(|row| {
            row.final_box_ordinal as usize == source.final_box_ordinal()
                && !row.box_source.same_source_as(source)
        }) {
            return Err(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch);
        }
        let mut matches = self.rows.iter().filter(|row| {
            row.box_source.same_source_as(source) && row.kind == ConstructorSourceKindV1::Birth
        });
        let Some(row) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err(InstanceConstructorBirthLookupErrorV1::DuplicateBirth);
        }
        Ok(Some(row))
    }

    /// Lends the same declaration-issued source, including unavailable execution.
    pub(crate) fn construction_source_for(
        &self,
        parent: &ParserOrdinaryBoxSourceRowV1,
        arity: usize,
    ) -> Result<&ConstructionSourceEligibilityV1, InstanceConstructorBirthLookupErrorV1> {
        if let Some(row) = self.birth_for(parent, arity)? {
            return Ok(&row.construction_source);
        }
        if arity != 0 {
            return Err(InstanceConstructorBirthLookupErrorV1::BirthArityMismatch);
        }
        self.no_birth_construction
            .iter()
            .find(|(own, _, _)| own.same_source_as(parent))
            .map(|(_, source, _)| source)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }

    pub(crate) fn construction_for(
        &self,
        parent: &ParserOrdinaryBoxSourceRowV1,
        arity: usize,
    ) -> Result<&ConstructionEligibilityV1, InstanceConstructorBirthLookupErrorV1> {
        if let Some(row) = self.birth_for(parent, arity)? {
            return Ok(&row.construction);
        }
        self.no_birth_construction
            .iter()
            .find(|(own, _, _)| own.same_source_as(parent))
            .map(|(_, _, plan)| plan)
            .ok_or(InstanceConstructorBirthLookupErrorV1::ParentSourceMismatch)
    }
}

impl VerifiedInstanceConstructorSemanticRowV1 {
    pub(crate) fn construction(&self) -> &ConstructionEligibilityV1 {
        &self.construction
    }
    pub(crate) fn construction_source(&self) -> &ConstructionSourceEligibilityV1 {
        &self.construction_source
    }

    pub(crate) fn formal_contracts(&self) -> &[BirthFormalContractV1] {
        &self.formal_contracts
    }
}

impl InstanceConstructorSourceDataV1 {
    pub(super) fn box_source_v1(&self) -> &ParserOrdinaryBoxSourceRowV1 {
        &self.box_source
    }

    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn published_birth_key(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        self.published_birth_key.as_ref()
    }

    pub(crate) fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }

    pub(crate) const fn final_box_ordinal(&self) -> u32 {
        self.final_box_ordinal
    }

    pub(crate) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(crate) fn key(&self) -> &str {
        &self.key
    }

    pub(crate) const fn kind(&self) -> ConstructorSourceKindV1 {
        self.kind
    }

    pub(crate) const fn source_arity(&self) -> u32 {
        self.source_arity
    }

    pub(crate) fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    /// Borrows this row's sealed body-shape inventory for provider joins.
    /// Single-root rows only; a multi-root forest yields `None`.
    pub(crate) fn body_shape(&self) -> Option<&VerifiedResolvedBodyShapeInventoryV1> {
        let [root] = self.forest.roots() else {
            return None;
        };
        self.body_shapes.get(root)
    }

    pub(crate) fn birth_completion(&self) -> Option<&VerifiedFunctionCompletionV1> {
        self.birth_completion.as_ref()
    }

    pub(crate) const fn birth_effect(&self) -> Option<DeclaredInstanceCallSemanticEffectV1> {
        self.birth_effect
    }

    /// Borrow another original owner using the SAME constructor forest and
    /// retained shape. The ordinary root input still verifies Birth completion.
    pub(crate) fn lowering_input_for_owner<'a>(
        &'a self,
        source: &'a ASTNode,
        owner: FunctionOwnerIdV1,
    ) -> Result<ResolvedFunctionLoweringInputV1<'a>, String> {
        let root = self.lowering_input(source)?;
        let shape = self.body_shapes.get(&owner).ok_or_else(|| {
            "[freeze:contract][mir/instance-constructor-semantic/owner-shape]".to_owned()
        })?;
        root.for_owner(owner, shape).map_err(|error| {
            format!("[freeze:contract][mir/instance-constructor-semantic/owner-input] {error:?}")
        })
    }

    pub(crate) fn lowering_input<'a>(
        &'a self,
        source: &'a ASTNode,
    ) -> Result<ResolvedFunctionLoweringInputV1<'a>, String> {
        let ASTNode::Program { statements, .. } = source else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/program]".to_owned());
        };
        let Some(ASTNode::BoxDeclaration {
            name, constructors, ..
        }) = statements.get(self.final_box_ordinal as usize)
        else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/box]".to_owned());
        };
        if name != self.box_name.as_ref() {
            return Err("[freeze:contract][mir/instance-constructor-semantic/box-name]".to_owned());
        }
        let Some(function) = constructors.get(self.key.as_ref()) else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/key]".to_owned());
        };
        let input = ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            function,
            &self.forest,
            &self.projection,
        )
        .map_err(|error| {
            format!("[freeze:contract][mir/instance-constructor-semantic/input] {error:?}")
        })?;
        let [root] = self.forest.roots() else {
            return Err("[freeze:contract][mir/instance-constructor-semantic/root]".to_owned());
        };
        if !std::ptr::eq(
            self.projection
                .owner_root(function, *root)
                .map_err(|error| error.to_string())?,
            function,
        ) {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/root-identity]".to_owned(),
            );
        }
        let shape = self.body_shapes.get(root).ok_or_else(|| {
            "[freeze:contract][mir/instance-constructor-semantic/body-shape]".to_owned()
        })?;
        if shape.owner() != *root
            || shape.body_root() != &input.function().root_profile().body_root()
        {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/body-shape-owner]".to_owned(),
            );
        }
        if self.kind == ConstructorSourceKindV1::Birth
            && self.birth_completion().map(|completion| completion.owner()) != Some(*root)
        {
            return Err(
                "[freeze:contract][mir/instance-constructor-semantic/completion-owner]".to_owned(),
            );
        }
        Ok(input.with_body_shape(shape))
    }
}

impl CapturedInstanceConstructorSourcesV1 {
    pub(super) fn rows_for_batch(
        &self,
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    ) -> Result<&[InstanceConstructorSourceDataV1], InstanceConstructorSemanticBatchIssueV1> {
        if &self.box_sources != batch.ordinary_box_coverage() {
            return Err(InstanceConstructorSemanticBatchIssueV1::SourceCoverage);
        }
        Ok(&self.rows)
    }

    pub(super) fn seal(
        self,
        batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
        static_claims: &super::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1,
        field_writers: &std::rc::Rc<super::ordinary_new_coseal::OriginalFieldWriterInventoryV1>,
    ) -> Result<VerifiedInstanceConstructorSemanticBatchV1, InstanceConstructorSemanticBatchIssueV1>
    {
        self.rows_for_batch(batch)?;
        let Self {
            rows: sources,
            box_sources,
            object_sources,
            declarations,
        } = self;
        let mut birth_rows = BTreeMap::<usize, std::collections::BTreeSet<u32>>::new();
        for row in &sources {
            if row.kind == ConstructorSourceKindV1::Birth {
                birth_rows
                    .entry(row.box_source.final_box_ordinal())
                    .or_default()
                    .insert(row.source_arity);
            }
        }
        batch.with_normal_program_source_loan(|loan| {
            let mut object_definitions = Vec::with_capacity(declarations.len());
            for (declaration, (parent, object)) in declarations.into_vec().into_iter().zip(&object_sources) {
                let fixed = field_writers.qualify_fixed_array_fields_v1(*object, parent.name(), declaration.fields(), &sources, loan.program())
                    .map_err(|_error| InstanceConstructorSemanticBatchIssueV1::SourceProjection { _error })?;
                let definition = declaration.seal(fixed);
                definition.validate_fixed_array_fields(*object)
                    .map_err(|_error| InstanceConstructorSemanticBatchIssueV1::SourceProjection { _error })?;
                object_definitions.push(definition);
            }
            let mut no_birth_construction = Vec::new();
            for (parent, object) in &object_sources {
                if sources.iter().any(|row| row.kind == ConstructorSourceKindV1::Birth
                    && row.box_source.same_source_as(parent)) { continue; }
                let source = batch.with_ordinary_box_syntax(parent, |declaration|
                    issue_construction_source_v1(*object, parent, declaration, None, None,
                        &birth_rows, &object_sources, &object_definitions))
                    .map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                let plan = ConstructionPlanV1::without_child_calls(&source);
                no_birth_construction.push((parent.clone(), source, plan));
            }
            let mut rows = Vec::with_capacity(sources.len());
            for source in sources.into_vec() {
                let input = source.lowering_input(loan.program()).map_err(|error|
                    InstanceConstructorSemanticBatchIssueV1::SourceProjection { _error: error })?;
                let parent = source.box_source.clone();
                let (construction_source, construction, provider_static_arguments, formal_contracts) = batch.with_ordinary_box_syntax(&parent, |declaration| -> Result<_, InstanceConstructorSemanticBatchIssueV1> {
                    let provider_static_arguments = if source.kind == ConstructorSourceKindV1::Birth {
                        let caller = source.published_birth_key.as_ref()
                            .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                        provider_source::issue_provider_static_sources_v1(
                            &source.source_id, source.object, declaration, input, caller, static_claims)
                    } else { Box::new([]) };
                    let construction_source = if source.kind == ConstructorSourceKindV1::Birth {
                        let caller = source.published_birth_key.as_ref()
                            .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?;
                        issue_construction_source_v1(source.object, &source.box_source, declaration,
                            Some((&source.source_id, input)), Some((caller, provider_static_arguments.as_ref())),
                            &birth_rows, &object_sources, &object_definitions)
                    } else {
                        Err(super::instance_construction::ConstructionUnavailableV1::BodyCoverageUnsupported)
                    };
                    let construction = ConstructionPlanV1::without_child_calls(&construction_source);
                    let formal_contracts = if source.kind == ConstructorSourceKindV1::Birth {
                        formal_contract::issue_birth_formal_contracts(input.source().root(), &source, &construction_source,
                            object_definitions.get(source.object.declaration_index() as usize)
                                .ok_or(InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?)
                            .map_err(|issue| InstanceConstructorSemanticBatchIssueV1::FormalContract { _issue: issue })?
                    } else { Box::new([]) };
                    Ok((construction_source, construction, provider_static_arguments, formal_contracts))
                }).map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)??;
                rows.push(VerifiedInstanceConstructorSemanticRowV1 {
                    source, construction_source, construction, provider_static_arguments, formal_contracts,
                });
            }
            Ok(VerifiedInstanceConstructorSemanticBatchV1 {
                rows: rows.into_boxed_slice(), box_sources, object_sources,
                object_definitions: std::cell::RefCell::new(Some(object_definitions.into_boxed_slice())),
                no_birth_construction,
            })
        }).map_err(|_| InstanceConstructorSemanticBatchIssueV1::SourceCoverage)?
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod construction_tests;
