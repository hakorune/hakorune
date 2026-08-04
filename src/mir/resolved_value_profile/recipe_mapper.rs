//! Same-pass trivial If facts -> portable fixed-shell recipe.
//!
//! This mapper consumes an already sealed owner product. It never selects a
//! route, opens source navigation, or emits physical MIR. Missing evidence is
//! a typed rejection; it never becomes a retry.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::if_recipe_contract::{
    IfBinaryOpV1, IfBindingKeyV1, IfBindingRoleV1, IfBlockKeyV1, IfBlockRoleV1, IfCompareOpV1,
    IfContinuationV1, IfElseDispositionV1, IfItemKeyV1, IfJoinRowV1, IfOperationV1,
    IfRecipeArtifactV1, IfRecipeBindingV1, IfRecipeBlockV1, IfRecipeItemRowV1, IfRecipeProfileV1,
    IfRecipeProvenanceV1, IfRecipeRejectReasonV1, IfRecipeSourceBindingV1, IfRecipeSourceOwnerV1,
    IfRecipeValueV1, IfRecipeVerifierV1, IfSourceClaimRoleV1, IfSourceClaimV1, IfSourcePathStepV1,
    IfSourcePathV1, IfValueClassV1, IfValueKeyV1, VerifiedIfRecipeArtifactV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceBindingSiteV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::product::{
    TrivialBindingDefinitionOriginV1, TrivialRepresentationV1, VerifiedTrivialCanonicalOwnerV1,
};
use super::recipe_facts::{
    IfEntryWitnessV1, TrivialRecipeBinaryOpV1, TrivialRecipeExprFactV1, TrivialRecipeExprKindV1,
    VerifiedTrivialIfRecipeFactsV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum IfRecipeMapRejectV1 {
    MissingFacts,
    MissingEntryWitness,
    OwnerMismatch,
    EntryBindingMismatch,
    EntryClassMismatch,
    EntryDefinitionMissing,
    EntryDefinitionAfterIf,
    MissingAssignment { branch: &'static str },
    UnsupportedRepresentation,
    MissingExpression,
    CrossRegionDependency,
    ExpressionCycle,
    BindingClassMismatch,
    BindingOwnerMismatch,
    SourcePathMismatch { role: &'static str },
    ContinuationMismatch,
    Recipe(IfRecipeRejectReasonV1),
}

#[derive(Debug)]
struct MapperState<'a> {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    root_index: u32,
    expressions: BTreeMap<SourceExprSiteV1, &'a TrivialRecipeExprFactV1>,
    seen: BTreeMap<SourceExprSiteV1, (RegionV1, IfValueKeyV1)>,
    active: BTreeSet<SourceExprSiteV1>,
    values: Vec<IfRecipeValueV1>,
    inputs: Vec<IfValueKeyV1>,
    bindings: Vec<IfRecipeBindingV1>,
    binding_keys: BTreeMap<crate::mir::resolved_semantics::BindingRefV1, IfBindingKeyV1>,
    next_item: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RegionV1 {
    Condition,
    Then,
    Else,
    Continuation,
}

impl<'a> MapperState<'a> {
    fn new(
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        root_index: u32,
        expressions: BTreeMap<SourceExprSiteV1, &'a TrivialRecipeExprFactV1>,
        entry: IfEntryWitnessV1,
        entry_class: IfValueClassV1,
    ) -> Self {
        let merge_key = IfBindingKeyV1::new(0);
        Self {
            owner,
            root_index,
            expressions,
            seen: BTreeMap::new(),
            active: BTreeSet::new(),
            values: vec![IfRecipeValueV1 {
                key: IfValueKeyV1::new(0),
                class: entry_class,
            }],
            inputs: vec![IfValueKeyV1::new(0)],
            bindings: vec![IfRecipeBindingV1 {
                key: merge_key,
                role: IfBindingRoleV1::MergeTarget,
                class: entry_class,
            }],
            binding_keys: BTreeMap::from([(entry.binding(), merge_key)]),
            next_item: 0,
        }
    }

    fn ensure_binding(
        &mut self,
        binding: crate::mir::resolved_semantics::BindingRefV1,
        class: IfValueClassV1,
    ) -> Result<IfBindingKeyV1, IfRecipeMapRejectV1> {
        if binding.owner() != self.owner {
            return Err(IfRecipeMapRejectV1::BindingOwnerMismatch);
        }
        if let Some(key) = self.binding_keys.get(&binding).copied() {
            let row = &self.bindings[key.raw() as usize];
            if row.class != class {
                return Err(IfRecipeMapRejectV1::BindingClassMismatch);
            }
            return Ok(key);
        }
        let key = IfBindingKeyV1::new(self.bindings.len() as u32);
        self.bindings.push(IfRecipeBindingV1 {
            key,
            role: IfBindingRoleV1::Input,
            class,
        });
        self.binding_keys.insert(binding, key);
        Ok(key)
    }

    fn new_value(&mut self, class: IfValueClassV1, input: bool) -> IfValueKeyV1 {
        let key = IfValueKeyV1::new(self.values.len() as u32);
        self.values.push(IfRecipeValueV1 { key, class });
        if input {
            self.inputs.push(key);
        }
        key
    }

    fn push_item(&mut self, block: &mut Vec<IfRecipeItemRowV1>, operation: IfOperationV1) {
        let key = IfItemKeyV1::new(self.next_item);
        self.next_item += 1;
        block.push(IfRecipeItemRowV1 { key, operation });
    }

    fn visit(
        &mut self,
        site: &SourceExprSiteV1,
        region: RegionV1,
        block: &mut Vec<IfRecipeItemRowV1>,
    ) -> Result<IfValueKeyV1, IfRecipeMapRejectV1> {
        if let Some((old_region, key)) = self.seen.get(site).copied() {
            if old_region != region {
                return Err(IfRecipeMapRejectV1::CrossRegionDependency);
            }
            return Ok(key);
        }
        if !self.active.insert(site.clone()) {
            return Err(IfRecipeMapRejectV1::ExpressionCycle);
        }
        let result = self.visit_uncached(site, region, block);
        self.active.remove(site);
        if let Ok(key) = result {
            self.seen.insert(site.clone(), (region, key));
        }
        result
    }

    fn visit_uncached(
        &mut self,
        site: &SourceExprSiteV1,
        region: RegionV1,
        block: &mut Vec<IfRecipeItemRowV1>,
    ) -> Result<IfValueKeyV1, IfRecipeMapRejectV1> {
        let (kind, representation) = {
            let fact = self
                .expressions
                .get(site)
                .ok_or(IfRecipeMapRejectV1::MissingExpression)?;
            (fact.kind().clone(), fact.representation())
        };
        if !site_in_region(site, self.root_index, region) {
            return Err(IfRecipeMapRejectV1::CrossRegionDependency);
        }
        match kind {
            TrivialRecipeExprKindV1::Read { binding } => {
                let class = value_class(representation)?;
                let binding_key = self.ensure_binding(binding, class)?;
                let result = self.new_value(class, false);
                self.push_item(
                    block,
                    IfOperationV1::ReadBinding {
                        binding: binding_key,
                        result,
                    },
                );
                Ok(result)
            }
            TrivialRecipeExprKindV1::ConstI64 { value } => {
                let result = self.new_value(IfValueClassV1::I64, false);
                self.push_item(block, IfOperationV1::ConstI64 { result, value });
                Ok(result)
            }
            TrivialRecipeExprKindV1::ConstBool { value } => {
                let result = self.new_value(IfValueClassV1::Bool, false);
                self.push_item(block, IfOperationV1::ConstBool { result, value });
                Ok(result)
            }
            TrivialRecipeExprKindV1::Binary { op, left, right } => {
                let left = self.visit(&left, region, block)?;
                let right = self.visit(&right, region, block)?;
                let class = binary_class(op);
                if value_class(representation)? != class {
                    return Err(IfRecipeMapRejectV1::UnsupportedRepresentation);
                }
                let result = self.new_value(class, false);
                self.push_item(block, binary_operation(op, left, right, result));
                Ok(result)
            }
        }
    }
}

pub(crate) fn map_trivial_if_recipe_v1(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> Result<VerifiedIfRecipeArtifactV1, IfRecipeMapRejectV1> {
    if source_function.owner() != profile.owner() {
        return Err(IfRecipeMapRejectV1::OwnerMismatch);
    }
    let facts = profile
        .recipe_facts()
        .ok_or(IfRecipeMapRejectV1::MissingFacts)?;
    let entry = facts
        .entry_witness()
        .ok_or(IfRecipeMapRejectV1::MissingEntryWitness)?;
    let then_assignment = facts
        .then_assignment()
        .ok_or(IfRecipeMapRejectV1::MissingAssignment { branch: "then" })?;
    let explicit_else = facts.has_explicit_else();
    let else_assignment = facts.else_assignment();
    if entry.binding() != then_assignment.binding()
        || else_assignment.is_some_and(|assignment| entry.binding() != assignment.binding())
    {
        return Err(IfRecipeMapRejectV1::EntryBindingMismatch);
    }
    if entry.binding().owner() != profile.owner() {
        return Err(IfRecipeMapRejectV1::OwnerMismatch);
    }
    if facts
        .then_body()
        .is_none_or(|body| body.owner() != profile.owner())
        || (explicit_else
            && facts
                .else_body()
                .is_none_or(|body| body.owner() != profile.owner()))
        || (!explicit_else && facts.else_body().is_some())
    {
        return Err(IfRecipeMapRejectV1::OwnerMismatch);
    }
    let entry_class = value_class(entry.representation())?;
    let then_class = value_class(then_assignment.representation())?;
    let else_class = else_assignment
        .map(|assignment| value_class(assignment.representation()))
        .transpose()?;
    if entry_class != then_class || else_class.is_some_and(|class| then_class != class) {
        return Err(IfRecipeMapRejectV1::EntryClassMismatch);
    }
    verify_entry_definition(profile, entry.binding(), facts.if_site())?;
    let root_index = root_body_index(facts.if_site())?;
    let mut expressions = BTreeMap::new();
    for fact in facts.expressions() {
        if expressions.insert(fact.site().clone(), fact).is_some() {
            return Err(IfRecipeMapRejectV1::MissingExpression);
        }
    }
    let continuation_is_merge_read = matches!(
        expressions
            .get(facts.continuation_read().ok_or(IfRecipeMapRejectV1::ContinuationMismatch)?)
            .ok_or(IfRecipeMapRejectV1::MissingExpression)?
            .kind(),
        TrivialRecipeExprKindV1::Read { binding } if *binding == entry.binding()
    );
    let mut state = MapperState::new(profile.owner(), root_index, expressions, entry, entry_class);
    let mut condition_items = Vec::new();
    let condition = state.visit(facts.condition(), RegionV1::Condition, &mut condition_items)?;
    if value_class_for_key(&state.values, condition) != Some(IfValueClassV1::Bool) {
        return Err(IfRecipeMapRejectV1::UnsupportedRepresentation);
    }
    let mut then_items = Vec::new();
    let then_value = state.visit(then_assignment.value(), RegionV1::Then, &mut then_items)?;
    let merge_binding = state.binding_keys[&entry.binding()];
    state.push_item(
        &mut then_items,
        IfOperationV1::WriteBinding {
            binding: merge_binding,
            value: then_value,
        },
    );
    let mut else_items = Vec::new();
    let else_value = if let Some(else_assignment) = else_assignment {
        let value = state.visit(else_assignment.value(), RegionV1::Else, &mut else_items)?;
        state.push_item(
            &mut else_items,
            IfOperationV1::WriteBinding {
                binding: merge_binding,
                value,
            },
        );
        value
    } else {
        IfValueKeyV1::new(0)
    };
    let mut continuation_items = Vec::new();
    let continuation_site = facts
        .continuation_read()
        .ok_or(IfRecipeMapRejectV1::ContinuationMismatch)?;
    let continuation_value = state.visit(
        continuation_site,
        RegionV1::Continuation,
        &mut continuation_items,
    )?;
    if !continuation_is_merge_read || continuation_items.len() != 1 {
        return Err(IfRecipeMapRejectV1::ContinuationMismatch);
    }
    let source_binding = source_binding(
        facts,
        source_function.function_origin(),
        root_index,
        explicit_else,
    )?;
    let recipe = crate::mir::if_recipe_contract::IfRecipeV1 {
        condition_block: IfRecipeBlockV1 {
            key: IfBlockKeyV1::new(0),
            role: IfBlockRoleV1::Condition,
            items: condition_items,
        },
        then_block: IfRecipeBlockV1 {
            key: IfBlockKeyV1::new(1),
            role: IfBlockRoleV1::Then,
            items: then_items,
        },
        else_block: explicit_else.then_some(IfRecipeBlockV1 {
            key: IfBlockKeyV1::new(2),
            role: IfBlockRoleV1::Else,
            items: else_items,
        }),
        continuation_block: IfRecipeBlockV1 {
            key: IfBlockKeyV1::new(if explicit_else { 3 } else { 2 }),
            role: IfBlockRoleV1::Continuation,
            items: continuation_items,
        },
        else_disposition: if explicit_else {
            IfElseDispositionV1::Explicit
        } else {
            IfElseDispositionV1::ImplicitFallthrough
        },
        condition,
        inputs: state.inputs,
        bindings: state.bindings,
        values: state.values,
        joins: vec![IfJoinRowV1 {
            binding: merge_binding,
            class: entry_class,
            entry_value: IfValueKeyV1::new(0),
            then_value,
            else_value,
        }],
        continuation: IfContinuationV1 {
            required_read: merge_binding,
        },
    };
    let artifact = IfRecipeArtifactV1::new(
        IfRecipeProvenanceV1 {
            profile: if explicit_else {
                IfRecipeProfileV1::ResolvedTrivialExplicitElse
            } else {
                IfRecipeProfileV1::ResolvedTrivialImplicitElse
            },
        },
        source_binding,
        recipe,
    );
    IfRecipeVerifierV1::verify_artifact(artifact).map_err(IfRecipeMapRejectV1::Recipe)
}

fn value_class(
    representation: TrivialRepresentationV1,
) -> Result<IfValueClassV1, IfRecipeMapRejectV1> {
    match representation {
        TrivialRepresentationV1::InlineI64 => Ok(IfValueClassV1::I64),
        TrivialRepresentationV1::InlineBool => Ok(IfValueClassV1::Bool),
        _ => Err(IfRecipeMapRejectV1::UnsupportedRepresentation),
    }
}

fn value_class_for_key(values: &[IfRecipeValueV1], key: IfValueKeyV1) -> Option<IfValueClassV1> {
    values.get(key.raw() as usize).map(|row| row.class)
}

fn binary_operation(
    op: TrivialRecipeBinaryOpV1,
    left: IfValueKeyV1,
    right: IfValueKeyV1,
    result: IfValueKeyV1,
) -> IfOperationV1 {
    match op {
        TrivialRecipeBinaryOpV1::Equal => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::Equal,
            left,
            right,
            result,
        },
        TrivialRecipeBinaryOpV1::NotEqual => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::NotEqual,
            left,
            right,
            result,
        },
        TrivialRecipeBinaryOpV1::Less => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::Less,
            left,
            right,
            result,
        },
        TrivialRecipeBinaryOpV1::Greater => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::Greater,
            left,
            right,
            result,
        },
        TrivialRecipeBinaryOpV1::LessEqual => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::LessEqual,
            left,
            right,
            result,
        },
        TrivialRecipeBinaryOpV1::GreaterEqual => IfOperationV1::CompareI64 {
            op: IfCompareOpV1::GreaterEqual,
            left,
            right,
            result,
        },
        other => IfOperationV1::BinaryI64 {
            op: match other {
                TrivialRecipeBinaryOpV1::Add => IfBinaryOpV1::Add,
                TrivialRecipeBinaryOpV1::Subtract => IfBinaryOpV1::Subtract,
                TrivialRecipeBinaryOpV1::Multiply => IfBinaryOpV1::Multiply,
                TrivialRecipeBinaryOpV1::Divide => IfBinaryOpV1::Divide,
                TrivialRecipeBinaryOpV1::Modulo => IfBinaryOpV1::Modulo,
                TrivialRecipeBinaryOpV1::BitAnd => IfBinaryOpV1::BitAnd,
                TrivialRecipeBinaryOpV1::BitOr => IfBinaryOpV1::BitOr,
                TrivialRecipeBinaryOpV1::BitXor => IfBinaryOpV1::BitXor,
                TrivialRecipeBinaryOpV1::Shl => IfBinaryOpV1::Shl,
                TrivialRecipeBinaryOpV1::Shr => IfBinaryOpV1::Shr,
                _ => unreachable!("comparison handled above"),
            },
            left,
            right,
            result,
        },
    }
}

fn binary_class(op: TrivialRecipeBinaryOpV1) -> IfValueClassV1 {
    match op {
        TrivialRecipeBinaryOpV1::Equal
        | TrivialRecipeBinaryOpV1::NotEqual
        | TrivialRecipeBinaryOpV1::Less
        | TrivialRecipeBinaryOpV1::Greater
        | TrivialRecipeBinaryOpV1::LessEqual
        | TrivialRecipeBinaryOpV1::GreaterEqual => IfValueClassV1::Bool,
        _ => IfValueClassV1::I64,
    }
}

fn site_in_region(site: &SourceExprSiteV1, root: u32, region: RegionV1) -> bool {
    let segments = site.node().segments();
    match (segments, region) {
        (
            [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfCondition, ..],
            RegionV1::Condition,
        ) if *index == root => true,
        (
            [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfThen(_), ..],
            RegionV1::Then,
        ) if *index == root => true,
        (
            [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfElse(_), ..],
            RegionV1::Else,
        ) if *index == root => true,
        ([SourcePathSegmentV1::Body(index), ..], RegionV1::Continuation) if *index > root => true,
        _ => false,
    }
}

fn root_body_index(site: &SourceStmtSiteV1) -> Result<u32, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index)] => Ok(*index),
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "if_node" }),
    }
}

fn source_binding(
    facts: &VerifiedTrivialIfRecipeFactsV1,
    origin: FunctionOriginV1,
    root_index: u32,
    explicit_else: bool,
) -> Result<IfRecipeSourceBindingV1, IfRecipeMapRejectV1> {
    let then_assignment = facts
        .then_assignment()
        .ok_or(IfRecipeMapRejectV1::MissingAssignment { branch: "then" })?;
    let mut claims = vec![
        IfSourceClaimV1 {
            role: IfSourceClaimRoleV1::IfNode,
            path: if_node_path(facts.if_site(), root_index)?,
        },
        IfSourceClaimV1 {
            role: IfSourceClaimRoleV1::Condition,
            path: condition_path(facts.condition(), root_index)?,
        },
        IfSourceClaimV1 {
            role: IfSourceClaimRoleV1::ThenAssignment,
            path: assignment_path(then_assignment.statement(), root_index, true)?,
        },
    ];
    if explicit_else {
        let else_assignment = facts
            .else_assignment()
            .ok_or(IfRecipeMapRejectV1::MissingAssignment { branch: "else" })?;
        claims.push(IfSourceClaimV1 {
            role: IfSourceClaimRoleV1::ElseAssignment,
            path: assignment_path(else_assignment.statement(), root_index, false)?,
        });
    } else {
        claims.push(IfSourceClaimV1 {
            role: IfSourceClaimRoleV1::ImplicitBaseline,
            path: implicit_baseline_path(root_index),
        });
    }
    Ok(IfRecipeSourceBindingV1 {
        owner: IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal: origin.compilation_unit_ordinal(),
            function_ordinal: origin.function_ordinal(),
        },
        claims,
    })
}

fn if_node_path(site: &SourceStmtSiteV1, root: u32) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    if root_body_index(site)? == root {
        Ok(IfSourcePathV1 {
            steps: vec![IfSourcePathStepV1::BodyItem { index: root }],
        })
    } else {
        Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "if_node" })
    }
}

fn condition_path(
    site: &SourceExprSiteV1,
    root: u32,
) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfCondition] if *index == root => {
            Ok(IfSourcePathV1 {
                steps: vec![
                    IfSourcePathStepV1::BodyItem { index: root },
                    IfSourcePathStepV1::IfCondition,
                ],
            })
        }
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch { role: "condition" }),
    }
}

fn assignment_path(
    site: &SourceStmtSiteV1,
    root: u32,
    then_branch: bool,
) -> Result<IfSourcePathV1, IfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfThen(item)]
            if then_branch && *index == root =>
        {
            Ok(IfRecipeSourcePath::then_path(root, *item))
        }
        [SourcePathSegmentV1::Body(index), SourcePathSegmentV1::IfElse(item)]
            if !then_branch && *index == root =>
        {
            Ok(IfRecipeSourcePath::else_path(root, *item))
        }
        _ => Err(IfRecipeMapRejectV1::SourcePathMismatch {
            role: if then_branch {
                "then_assignment"
            } else {
                "else_assignment"
            },
        }),
    }
}

fn implicit_baseline_path(root: u32) -> IfSourcePathV1 {
    IfSourcePathV1 {
        steps: vec![
            IfSourcePathStepV1::BodyItem { index: root },
            IfSourcePathStepV1::IfImplicitBaseline,
        ],
    }
}

struct IfRecipeSourcePath;

impl IfRecipeSourcePath {
    fn then_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfThenItem { index: item },
            ],
        }
    }

    fn else_path(root: u32, item: u32) -> IfSourcePathV1 {
        IfSourcePathV1 {
            steps: vec![
                IfSourcePathStepV1::BodyItem { index: root },
                IfSourcePathStepV1::IfElseItem { index: item },
            ],
        }
    }
}

fn verify_entry_definition(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    binding: crate::mir::resolved_semantics::BindingRefV1,
    if_site: &SourceStmtSiteV1,
) -> Result<(), IfRecipeMapRejectV1> {
    let mut found = false;
    let mut found_after = false;
    for definition in profile.definitions() {
        if definition.binding() != binding {
            continue;
        }
        match definition.origin() {
            TrivialBindingDefinitionOriginV1::Declaration(site) => match site {
                SourceBindingSiteV1::Parameter { .. } | SourceBindingSiteV1::Receiver => {
                    found = true;
                }
                SourceBindingSiteV1::Local { statement, .. }
                | SourceBindingSiteV1::Outbox { statement, .. }
                | SourceBindingSiteV1::Nowait { statement } => {
                    if statement.node().segments() < if_site.node().segments() {
                        found = true;
                    } else {
                        found_after = true;
                    }
                }
                _ => {}
            },
            TrivialBindingDefinitionOriginV1::Assignment(_) => {}
        }
    }
    if found {
        Ok(())
    } else if found_after {
        Err(IfRecipeMapRejectV1::EntryDefinitionAfterIf)
    } else {
        Err(IfRecipeMapRejectV1::EntryDefinitionMissing)
    }
}
