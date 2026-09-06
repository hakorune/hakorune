//! Source-issued construction obligations; not a runtime cleanup implementation.
//!
//! Eligibility is deliberately distinct from source validity. Every eligible
//! plan retains outer-storage reclamation on allocation Normal / construction
//! Fault, even when every field demand is Trivial. A store commits only on its
//! Normal edge. No MIR type, event absence or non-escape result issues this plan.

use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalObjectIdV1};
use std::collections::BTreeSet;

use crate::ast::{ASTNode, LiteralValue};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, BodyExpressionShapeV1, BodyMeReceiverV1, FunctionOwnerIdV1,
    HomeDemandV1, OwnedExprSiteV1, ResolvedAssignmentFormV1, ResolvedAssignmentSourceV1,
    ResolvedAssignmentTargetV1, ResolvedLexicalRefV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ConstructionUnavailableV1 {
    SourceRelationMissing,
    FieldContractUnsupported,
    BodyCoverageUnsupported,
    InitializationContractMissing,
    OverrideUnsupported,
}

/// Exact RHS accepted under the constructor declaration loan.
///
/// This is carried only by the existing construction-store plan to its
/// selected physical consumer. It is neither a general expression form nor a
/// second semantic receipt.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstructionStoreRhsV1 {
    LiteralI64(i64),
    Parameter {
        site: SourceExprSiteV1,
        binding: BindingRefV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionStoreV1 {
    assignment: ResolvedAssignmentSourceV1,
    field: CanonicalFieldRefV1,
    receiver_site: SourceExprSiteV1,
    receiver_binding: BindingRefV1,
    rhs: ConstructionStoreRhsV1,
}

impl ConstructionStoreV1 {
    pub(crate) const fn assignment(&self) -> &ResolvedAssignmentSourceV1 {
        &self.assignment
    }

    pub(crate) const fn field(&self) -> CanonicalFieldRefV1 {
        self.field
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) const fn rhs(&self) -> &ConstructionStoreRhsV1 {
        &self.rhs
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstructionPlanV1 {
    // Issued once by the enclosing branded semantic batch, including NoBirth.
    object: CanonicalObjectIdV1,
    field_demands: Box<[HomeDemandV1]>,
    stores: Box<[ConstructionStoreV1]>,
    // Store sites are local to this existing constructor owner. Keep it after
    // the affine New target is consumed; Box identity alone cannot qualify them.
    constructor: Option<(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)>,
}

pub(crate) type ConstructionEligibilityV1 = Result<ConstructionPlanV1, ConstructionUnavailableV1>;

impl ConstructionPlanV1 {
    pub(crate) const fn object(&self) -> CanonicalObjectIdV1 {
        self.object
    }

    pub(crate) fn constructor(
        &self,
    ) -> Option<&(crate::parser::ConstructorSourceIdV1, FunctionOwnerIdV1)> {
        self.constructor.as_ref()
    }

    pub(crate) fn field_demands(&self) -> &[HomeDemandV1] {
        &self.field_demands
    }

    pub(crate) fn stores(&self) -> &[ConstructionStoreV1] {
        &self.stores
    }

    /// Independent of field demand/count. This is an obligation, not proof
    /// that a backend already implements reclamation.
    pub(crate) const fn reclaims_unpublished_outer_storage(&self) -> bool {
        true
    }
}

/// Called only within the exact parser declaration loan at semantic issuance.
pub(super) fn issue_construction_plan(
    object_id: CanonicalObjectIdV1,
    source: &crate::parser::ParserOrdinaryBoxSourceRowV1,
    declaration: &ASTNode,
    birth: Option<(
        &crate::parser::ConstructorSourceIdV1,
        ResolvedFunctionLoweringInputV1<'_>,
    )>,
) -> ConstructionEligibilityV1 {
    use ConstructionUnavailableV1 as U;
    // Parser normalization moves defaults into Birth stores. The sealed source
    // trigger, not generated syntax or span/name heuristics, retains their role.
    if source.has_stored_field_initializer() {
        return Err(U::FieldContractUnsupported);
    }
    let ASTNode::BoxDeclaration {
        fields,
        field_decls,
        weak_fields,
        delegates,
        extends,
        invariants,
        transitions,
        type_parameters,
        is_sync,
        is_static,
        static_init,
        attrs,
        ..
    } = declaration
    else {
        return Err(U::SourceRelationMissing);
    };
    if fields.len() != field_decls.len()
        || fields
            .iter()
            .zip(field_decls)
            .any(|(name, field)| name != &field.name)
        || !weak_fields.is_empty()
        || !delegates.is_empty()
        || !extends.is_empty()
        || !invariants.is_empty()
        || !transitions.is_empty()
        || !type_parameters.is_empty()
        || *is_sync
        || *is_static
        || static_init.is_some()
        || !attrs.is_empty()
        || field_decls.iter().any(|field| {
            field.declared_type_name.as_deref() != Some("i64")
                || field.is_weak
                || field.default_value.is_some()
        })
    {
        return Err(U::FieldContractUnsupported);
    }
    let names: BTreeSet<_> = fields.iter().collect();
    if names.len() != fields.len() {
        return Err(U::SourceRelationMissing);
    }
    let mut plan = ConstructionPlanV1 {
        object: object_id,
        field_demands: vec![HomeDemandV1::Trivial; fields.len()].into_boxed_slice(),
        stores: Box::new([]),
        constructor: None,
    };
    let Some((source_id, input)) = birth else {
        return if fields.is_empty() {
            Ok(plan)
        } else {
            Err(U::InitializationContractMissing)
        };
    };
    plan.constructor = Some((source_id.clone(), input.owner()));
    let function = input.function();
    let shape = input.body_shape().ok_or(U::SourceRelationMissing)?;
    if shape.owner() != input.owner() || input.forest().owners().count() != 1 {
        return Err(U::BodyCoverageUnsupported);
    }
    let ASTNode::FunctionDeclaration {
        uses,
        contracts,
        attrs,
        ..
    } = input.source().root()
    else {
        return Err(U::SourceRelationMissing);
    };
    if !uses.is_empty() || !contracts.is_empty() || !attrs.is_empty() {
        return Err(U::BodyCoverageUnsupported);
    }
    let receivers: Vec<_> = function
        .bindings()
        .filter(|(_, row)| row.kind() == BindingKindV1::Receiver)
        .map(|(binding, _)| binding)
        .collect();
    let [receiver] = receivers.as_slice() else {
        return Err(U::SourceRelationMissing);
    };
    let body = input
        .source()
        .root_body()
        .map_err(|_| U::SourceRelationMissing)?;
    let mut stores = Vec::new();
    let mut initialized = BTreeSet::new();
    let mut statements = BTreeSet::new();
    let mut expressions = BTreeSet::new();
    for index in 0..body.statements().len() {
        let statement = input
            .source()
            .body_stmt(&body, index)
            .map_err(|_| U::SourceRelationMissing)?;
        statements.insert(statement.site().clone());
        if matches!(statement.node(), ASTNode::Return { value: None, .. })
            && index + 1 == body.statements().len()
        {
            continue;
        }
        let ASTNode::Assignment { .. } = statement.node() else {
            return Err(U::BodyCoverageUnsupported);
        };
        let mut rows = shape
            .assignment_sources()
            .iter()
            .filter(|row| row.statement_site() == statement.site());
        let row = rows.next().ok_or(U::SourceRelationMissing)?;
        if rows.next().is_some() || row.form() != ResolvedAssignmentFormV1::Plain {
            return Err(U::SourceRelationMissing);
        }
        let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver: object }) =
            function.assignment_target(row.target_site())
        else {
            return Err(U::BodyCoverageUnsupported);
        };
        let field = shape
            .expressions()
            .iter()
            .find_map(|expression| match expression {
                BodyExpressionShapeV1::FieldAccess {
                    site,
                    object: exact,
                    field,
                } if site == row.target_site() && exact == object => Some(field),
                _ => None,
            })
            .ok_or(U::SourceRelationMissing)?;
        if !shape.expressions().iter().any(|expression| {
            matches!(expression,
            BodyExpressionShapeV1::Me { site, receiver: BodyMeReceiverV1::Lexical(binding) }
                if site == object && binding == receiver)
        }) {
            return Err(U::BodyCoverageUnsupported);
        }
        let ordinal = fields
            .iter()
            .position(|name| name == field.as_ref())
            .ok_or(U::SourceRelationMissing)?;
        // Replacement requires old-value release semantics; not first-store proof.
        if !initialized.insert(ordinal) {
            return Err(U::BodyCoverageUnsupported);
        }
        let rhs = input
            .source()
            .expr_at(&OwnedExprSiteV1::new(
                input.owner(),
                row.value_site().clone(),
            ))
            .map_err(|_| U::SourceRelationMissing)?;
        let rhs = match rhs.node() {
            ASTNode::Literal {
                value: LiteralValue::Integer(value),
                ..
            } => ConstructionStoreRhsV1::LiteralI64(*value),
            ASTNode::Variable { .. } => shape
                .expressions()
                .iter()
                .find_map(|expression| match expression {
                    BodyExpressionShapeV1::Variable {
                        site,
                        resolved: ResolvedLexicalRefV1::Local(binding),
                    } if site == row.value_site()
                        && matches!(
                            function.binding(*binding).map(|record| record.kind()),
                            Some(BindingKindV1::Parameter { .. })
                        ) =>
                    {
                        Some(ConstructionStoreRhsV1::Parameter {
                            site: row.value_site().clone(),
                            binding: *binding,
                        })
                    }
                    _ => None,
                })
                .ok_or(U::BodyCoverageUnsupported)?,
            _ => return Err(U::BodyCoverageUnsupported),
        };
        expressions.extend([
            row.target_site().clone(),
            object.clone(),
            row.value_site().clone(),
        ]);
        let field = CanonicalFieldRefV1::from_declaration_ordinal(object_id, ordinal)
            .ok_or(U::SourceRelationMissing)?;
        stores.push(ConstructionStoreV1 {
            assignment: row.clone(),
            field,
            receiver_site: object.clone(),
            receiver_binding: *receiver,
            rhs,
        });
    }
    // Reject residual syntax/child owners; SequenceItem or absent Call events
    // alone do not classify ArrayLiteral, FromCall, Lambda or nested acquisition.
    if shape.statements().len() != statements.len()
        || shape
            .statements()
            .iter()
            .any(|row| !statements.contains(row.site()))
        || shape.assignment_sources().len() != stores.len()
        || function
            .expression_sites()
            .any(|site| !expressions.contains(site))
    {
        return Err(U::BodyCoverageUnsupported);
    }
    if initialized.len() != fields.len() {
        return Err(U::InitializationContractMissing);
    }
    plan.stores = stores.into_boxed_slice();
    Ok(plan)
}
