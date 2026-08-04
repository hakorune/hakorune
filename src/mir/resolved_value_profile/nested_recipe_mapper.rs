//! Same-pass Nested If facts -> isolated portable depth-one artifact.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::if_recipe_contract::{
    NestedIfAssignmentV1, NestedIfBinaryOpV1, NestedIfBindingKeyV1, NestedIfBindingV1,
    NestedIfContinuationV1, NestedIfExprKindV1, NestedIfExprV1, NestedIfJoinRowV1,
    NestedIfNodeKeyV1, NestedIfNodeV1, NestedIfRecipeArtifactV1, NestedIfRecipeProfileV1,
    NestedIfRecipeProvenanceV1, NestedIfRecipeSourceBindingV1, NestedIfRecipeV1,
    NestedIfRecipeVerifierV1, NestedIfSourceClaimRoleV1, NestedIfSourceClaimV1,
    NestedIfSourcePathStepV1, NestedIfSourcePathV1, NestedIfValueClassV1, NestedIfValueKeyV1,
    VerifiedNestedIfRecipeArtifactV1,
};
use crate::mir::resolved_semantics::{
    FunctionOriginV1, SourceExprSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

use super::nested_recipe_facts::VerifiedNestedTrivialIfRecipeFactsV1;
use super::product::{TrivialRepresentationV1, VerifiedTrivialCanonicalOwnerV1};
use super::recipe_facts::{
    TrivialRecipeBinaryOpV1, TrivialRecipeExprFactV1, TrivialRecipeExprKindV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NestedIfRecipeMapRejectV1 {
    MissingFacts,
    OwnerMismatch,
    InvalidDepthOneSource,
    BindingMismatch,
    UnsupportedRepresentation,
    MissingExpression,
    ExpressionCycle,
    UnsupportedExpression,
    SourcePathMismatch,
    Recipe(crate::mir::if_recipe_contract::NestedIfRecipeRejectReasonV1),
}

struct ExprMapper<'a> {
    facts: BTreeMap<SourceExprSiteV1, &'a TrivialRecipeExprFactV1>,
    active: BTreeSet<SourceExprSiteV1>,
    seen: BTreeMap<SourceExprSiteV1, NestedIfValueKeyV1>,
    expressions: Vec<NestedIfExprV1>,
    next_value: u32,
    binding: crate::mir::resolved_semantics::BindingRefV1,
}

impl<'a> ExprMapper<'a> {
    fn new(
        facts: BTreeMap<SourceExprSiteV1, &'a TrivialRecipeExprFactV1>,
        binding: crate::mir::resolved_semantics::BindingRefV1,
    ) -> Self {
        Self {
            facts,
            active: BTreeSet::new(),
            seen: BTreeMap::new(),
            expressions: Vec::new(),
            next_value: 1,
            binding,
        }
    }

    fn new_merge_value(&mut self) -> NestedIfValueKeyV1 {
        let key = NestedIfValueKeyV1::new(self.next_value);
        self.next_value += 1;
        key
    }

    fn visit(
        &mut self,
        site: &SourceExprSiteV1,
    ) -> Result<NestedIfValueKeyV1, NestedIfRecipeMapRejectV1> {
        if let Some(key) = self.seen.get(site).copied() {
            return Ok(key);
        }
        if !self.active.insert(site.clone()) {
            return Err(NestedIfRecipeMapRejectV1::ExpressionCycle);
        }
        let result = self.visit_uncached(site);
        self.active.remove(site);
        if let Ok(key) = result {
            self.seen.insert(site.clone(), key);
        }
        result
    }

    fn visit_uncached(
        &mut self,
        site: &SourceExprSiteV1,
    ) -> Result<NestedIfValueKeyV1, NestedIfRecipeMapRejectV1> {
        let fact = self
            .facts
            .get(site)
            .ok_or(NestedIfRecipeMapRejectV1::MissingExpression)?;
        let class = value_class(fact.representation())?;
        let kind = match fact.kind() {
            TrivialRecipeExprKindV1::Read { binding } => {
                if *binding != self.binding {
                    return Err(NestedIfRecipeMapRejectV1::BindingMismatch);
                }
                NestedIfExprKindV1::ReadBinding {
                    binding: NestedIfBindingKeyV1::new(0),
                }
            }
            TrivialRecipeExprKindV1::ConstI64 { value } => {
                NestedIfExprKindV1::ConstI64 { value: *value }
            }
            TrivialRecipeExprKindV1::ConstBool { value } => {
                NestedIfExprKindV1::ConstBool { value: *value }
            }
            TrivialRecipeExprKindV1::Binary { op, left, right } => {
                let left = self.visit(left)?;
                let right = self.visit(right)?;
                let expected = binary_class(*op);
                if expected != class {
                    return Err(NestedIfRecipeMapRejectV1::UnsupportedRepresentation);
                }
                NestedIfExprKindV1::Binary {
                    op: binary_op(*op),
                    left,
                    right,
                }
            }
            TrivialRecipeExprKindV1::DirectStaticCall => {
                return Err(NestedIfRecipeMapRejectV1::UnsupportedExpression)
            }
        };
        let key = NestedIfValueKeyV1::new(self.next_value);
        self.next_value += 1;
        self.expressions.push(NestedIfExprV1 { key, class, kind });
        Ok(key)
    }
}

pub(crate) fn map_nested_trivial_if_recipe_v1(
    profile: &VerifiedTrivialCanonicalOwnerV1,
    source_function: &VerifiedResolvedFunctionV1,
) -> Result<VerifiedNestedIfRecipeArtifactV1, NestedIfRecipeMapRejectV1> {
    if source_function.owner() != profile.owner() {
        return Err(NestedIfRecipeMapRejectV1::OwnerMismatch);
    }
    let facts = profile
        .nested_recipe_facts()
        .ok_or(NestedIfRecipeMapRejectV1::MissingFacts)?;
    let outer = facts.outer();
    let inner = facts.inner();
    let binding = facts.shared_binding();
    if !matches!(
        outer.entry_witness().representation(),
        TrivialRepresentationV1::InlineI64
    ) {
        return Err(NestedIfRecipeMapRejectV1::UnsupportedRepresentation);
    }
    if outer.entry_witness().binding() != binding
        || inner.entry_witness().binding() != binding
        || binding.owner() != profile.owner()
    {
        return Err(NestedIfRecipeMapRejectV1::BindingMismatch);
    }
    let root = root_body_index(outer.statement())?;
    let inner_prefix = inner_prefix(inner.statement(), root)?;
    let mut expression_facts = BTreeMap::new();
    for fact in facts.expressions() {
        if expression_facts.insert(fact.site().clone(), fact).is_some() {
            return Err(NestedIfRecipeMapRejectV1::MissingExpression);
        }
    }
    let mut mapper = ExprMapper::new(expression_facts, binding);
    let condition_outer = mapper.visit(outer.condition())?;
    let condition_inner = mapper.visit(inner.condition())?;
    let inner_then = mapper.visit(inner.then_assignments()[0].value())?;
    let inner_else = mapper.visit(inner.else_assignments()[0].value())?;
    let outer_else = mapper.visit(outer.else_assignments()[0].value())?;
    let _continuation = mapper.visit(facts.continuation_read())?;
    let inner_merge = mapper.new_merge_value();
    let outer_merge = mapper.new_merge_value();
    let source_binding =
        source_binding(facts, source_function.function_origin(), root, inner_prefix)?;
    let recipe = NestedIfRecipeV1 {
        nodes: vec![
            NestedIfNodeV1 {
                key: NestedIfNodeKeyV1::new(0),
                condition: condition_outer,
                then_child: Some(NestedIfNodeKeyV1::new(1)),
                then_assignments: Vec::new(),
                else_assignments: vec![NestedIfAssignmentV1 {
                    binding: NestedIfBindingKeyV1::new(0),
                    value: outer_else,
                }],
                join: NestedIfJoinRowV1 {
                    binding: NestedIfBindingKeyV1::new(0),
                    class: NestedIfValueClassV1::I64,
                    entry_value: NestedIfValueKeyV1::new(0),
                    then_value: inner_merge,
                    else_value: outer_else,
                    merge_value: outer_merge,
                },
            },
            NestedIfNodeV1 {
                key: NestedIfNodeKeyV1::new(1),
                condition: condition_inner,
                then_child: None,
                then_assignments: vec![NestedIfAssignmentV1 {
                    binding: NestedIfBindingKeyV1::new(0),
                    value: inner_then,
                }],
                else_assignments: vec![NestedIfAssignmentV1 {
                    binding: NestedIfBindingKeyV1::new(0),
                    value: inner_else,
                }],
                join: NestedIfJoinRowV1 {
                    binding: NestedIfBindingKeyV1::new(0),
                    class: NestedIfValueClassV1::I64,
                    entry_value: NestedIfValueKeyV1::new(0),
                    then_value: inner_then,
                    else_value: inner_else,
                    merge_value: inner_merge,
                },
            },
        ],
        expressions: mapper.expressions,
        bindings: vec![NestedIfBindingV1 {
            key: NestedIfBindingKeyV1::new(0),
            class: NestedIfValueClassV1::I64,
        }],
        entry_value: NestedIfValueKeyV1::new(0),
        outer_merge_value: outer_merge,
        continuation: NestedIfContinuationV1 {
            binding: NestedIfBindingKeyV1::new(0),
        },
    };
    let artifact = NestedIfRecipeArtifactV1::new(
        NestedIfRecipeProvenanceV1 {
            profile: NestedIfRecipeProfileV1::ResolvedTrivialExplicitElseDepthOne,
        },
        source_binding,
        recipe,
    );
    NestedIfRecipeVerifierV1::verify_artifact(artifact).map_err(NestedIfRecipeMapRejectV1::Recipe)
}

fn value_class(
    representation: TrivialRepresentationV1,
) -> Result<NestedIfValueClassV1, NestedIfRecipeMapRejectV1> {
    match representation {
        TrivialRepresentationV1::InlineI64 => Ok(NestedIfValueClassV1::I64),
        TrivialRepresentationV1::InlineBool => Ok(NestedIfValueClassV1::Bool),
        _ => Err(NestedIfRecipeMapRejectV1::UnsupportedRepresentation),
    }
}

fn binary_class(op: TrivialRecipeBinaryOpV1) -> NestedIfValueClassV1 {
    match op {
        TrivialRecipeBinaryOpV1::Equal
        | TrivialRecipeBinaryOpV1::NotEqual
        | TrivialRecipeBinaryOpV1::Less
        | TrivialRecipeBinaryOpV1::Greater
        | TrivialRecipeBinaryOpV1::LessEqual
        | TrivialRecipeBinaryOpV1::GreaterEqual => NestedIfValueClassV1::Bool,
        _ => NestedIfValueClassV1::I64,
    }
}

fn binary_op(op: TrivialRecipeBinaryOpV1) -> NestedIfBinaryOpV1 {
    match op {
        TrivialRecipeBinaryOpV1::Add => NestedIfBinaryOpV1::Add,
        TrivialRecipeBinaryOpV1::Subtract => NestedIfBinaryOpV1::Subtract,
        TrivialRecipeBinaryOpV1::Multiply => NestedIfBinaryOpV1::Multiply,
        TrivialRecipeBinaryOpV1::Divide => NestedIfBinaryOpV1::Divide,
        TrivialRecipeBinaryOpV1::Modulo => NestedIfBinaryOpV1::Modulo,
        TrivialRecipeBinaryOpV1::BitAnd => NestedIfBinaryOpV1::BitAnd,
        TrivialRecipeBinaryOpV1::BitOr => NestedIfBinaryOpV1::BitOr,
        TrivialRecipeBinaryOpV1::BitXor => NestedIfBinaryOpV1::BitXor,
        TrivialRecipeBinaryOpV1::Shl => NestedIfBinaryOpV1::Shl,
        TrivialRecipeBinaryOpV1::Shr => NestedIfBinaryOpV1::Shr,
        TrivialRecipeBinaryOpV1::Equal => NestedIfBinaryOpV1::Equal,
        TrivialRecipeBinaryOpV1::NotEqual => NestedIfBinaryOpV1::NotEqual,
        TrivialRecipeBinaryOpV1::Less => NestedIfBinaryOpV1::Less,
        TrivialRecipeBinaryOpV1::Greater => NestedIfBinaryOpV1::Greater,
        TrivialRecipeBinaryOpV1::LessEqual => NestedIfBinaryOpV1::LessEqual,
        TrivialRecipeBinaryOpV1::GreaterEqual => NestedIfBinaryOpV1::GreaterEqual,
    }
}

fn root_body_index(site: &SourceStmtSiteV1) -> Result<u32, NestedIfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(index)] => Ok(*index),
        _ => Err(NestedIfRecipeMapRejectV1::InvalidDepthOneSource),
    }
}

fn inner_prefix(site: &SourceStmtSiteV1, root: u32) -> Result<u32, NestedIfRecipeMapRejectV1> {
    match site.node().segments() {
        [SourcePathSegmentV1::Body(found), SourcePathSegmentV1::IfThen(index)]
            if *found == root =>
        {
            Ok(*index)
        }
        _ => Err(NestedIfRecipeMapRejectV1::InvalidDepthOneSource),
    }
}

fn source_path(site: &SourceStmtSiteV1) -> Result<NestedIfSourcePathV1, NestedIfRecipeMapRejectV1> {
    let steps = site
        .node()
        .segments()
        .iter()
        .map(|segment| match segment {
            SourcePathSegmentV1::Body(index) => Ok(NestedIfSourcePathStepV1::BodyItem(*index)),
            SourcePathSegmentV1::IfThen(index) => Ok(NestedIfSourcePathStepV1::IfThenItem(*index)),
            SourcePathSegmentV1::IfElse(index) => Ok(NestedIfSourcePathStepV1::IfElseItem(*index)),
            _ => Err(NestedIfRecipeMapRejectV1::SourcePathMismatch),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(NestedIfSourcePathV1 { steps })
}

fn expr_source_path(
    site: &SourceExprSiteV1,
) -> Result<NestedIfSourcePathV1, NestedIfRecipeMapRejectV1> {
    let steps = site
        .node()
        .segments()
        .iter()
        .map(|segment| match segment {
            SourcePathSegmentV1::Body(index) => Ok(NestedIfSourcePathStepV1::BodyItem(*index)),
            SourcePathSegmentV1::IfCondition => Ok(NestedIfSourcePathStepV1::IfCondition),
            SourcePathSegmentV1::IfThen(index) => Ok(NestedIfSourcePathStepV1::IfThenItem(*index)),
            SourcePathSegmentV1::IfElse(index) => Ok(NestedIfSourcePathStepV1::IfElseItem(*index)),
            SourcePathSegmentV1::Value => Ok(NestedIfSourcePathStepV1::Value),
            _ => Err(NestedIfRecipeMapRejectV1::SourcePathMismatch),
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(NestedIfSourcePathV1 { steps })
}

fn source_binding(
    facts: &VerifiedNestedTrivialIfRecipeFactsV1,
    origin: FunctionOriginV1,
    root: u32,
    inner_index: u32,
) -> Result<NestedIfRecipeSourceBindingV1, NestedIfRecipeMapRejectV1> {
    let outer = facts.outer();
    let inner = facts.inner();
    let inner_then = inner
        .then_assignments()
        .first()
        .ok_or(NestedIfRecipeMapRejectV1::SourcePathMismatch)?;
    let inner_else = inner
        .else_assignments()
        .first()
        .ok_or(NestedIfRecipeMapRejectV1::SourcePathMismatch)?;
    let outer_else = outer
        .else_assignments()
        .first()
        .ok_or(NestedIfRecipeMapRejectV1::SourcePathMismatch)?;
    let claims = vec![
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::OuterIfNode,
            path: source_path(outer.statement())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::OuterCondition,
            path: expr_source_path(outer.condition())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::InnerIfNode,
            path: source_path(inner.statement())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::InnerCondition,
            path: expr_source_path(inner.condition())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::InnerThenAssignment,
            path: source_path(inner_then.statement())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::InnerElseAssignment,
            path: source_path(inner_else.statement())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::OuterElseAssignment,
            path: source_path(outer_else.statement())?,
        },
        NestedIfSourceClaimV1 {
            role: NestedIfSourceClaimRoleV1::ContinuationRead,
            path: expr_source_path(facts.continuation_read())?,
        },
    ];
    if claims[0].path.steps != [NestedIfSourcePathStepV1::BodyItem(root)]
        || claims[2].path.steps
            != [
                NestedIfSourcePathStepV1::BodyItem(root),
                NestedIfSourcePathStepV1::IfThenItem(inner_index),
            ]
    {
        return Err(NestedIfRecipeMapRejectV1::SourcePathMismatch);
    }
    Ok(NestedIfRecipeSourceBindingV1 {
        owner: crate::mir::if_recipe_contract::IfRecipeSourceOwnerV1::FunctionBody {
            compilation_unit_ordinal: origin.compilation_unit_ordinal(),
            function_ordinal: origin.function_ordinal(),
        },
        claims,
    })
}
