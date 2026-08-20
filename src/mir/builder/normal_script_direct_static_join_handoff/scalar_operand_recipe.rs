//! AST-free scalar operand Recipe for one Script direct-static Join.
//!
//! The resolver's expression inventory is the only source of these trees.
//! This module never opens the retained AST and never creates a ValueId,
//! physical type, or call target.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::normal_script_direct_static_join_handoff::{
    VerifiedScriptDirectStaticJoinHandoffV1, VerifiedScriptDirectStaticJoinRowV1,
};
use crate::mir::builder::normal_script_direct_static_recipe::ScriptDirectStaticRecipeKeyV1;
use crate::mir::builder::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedBinaryOperatorV1, ResolvedExpressionSourceInventoryV1,
    ResolvedLiteralSourceV1, ResolvedUnaryOperatorV1, SourceExprSiteV1,
    VerifiedSemanticOwnerProductV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ScalarUnaryOperatorV1 {
    Minus,
    BitNot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum ScalarBinaryOperatorV1 {
    Add,
    Subtract,
    Multiply,
    BitAnd,
    BitOr,
    BitXor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ScalarOperandRecipeNodeV1 {
    Literal {
        site: SourceExprSiteV1,
        value: i64,
    },
    Unary {
        site: SourceExprSiteV1,
        operator: ScalarUnaryOperatorV1,
        operand: Box<Self>,
    },
    Binary {
        site: SourceExprSiteV1,
        operator: ScalarBinaryOperatorV1,
        lhs: Box<Self>,
        rhs: Box<Self>,
    },
}

impl ScalarOperandRecipeNodeV1 {
    pub(in crate::mir) const fn site(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Literal { site, .. }
            | Self::Unary { site, .. }
            | Self::Binary { site, .. } => site,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct ScalarOperandRecipeArgumentV1 {
    ordinal: u32,
    site: SourceExprSiteV1,
    tree: ScalarOperandRecipeNodeV1,
}

impl ScalarOperandRecipeArgumentV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_parts_for_test(
        ordinal: u32,
        site: SourceExprSiteV1,
        tree: ScalarOperandRecipeNodeV1,
    ) -> Self {
        Self { ordinal, site, tree }
    }

    pub(in crate::mir) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(in crate::mir) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(in crate::mir) const fn tree(&self) -> &ScalarOperandRecipeNodeV1 {
        &self.tree
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum VerifiedScriptDirectStaticScalarOperandRecipeIssueV1 {
    SourceIdentityMismatch,
    SourceOwnerMismatch,
    ScriptRootMissing,
    ScriptRootNotScript,
    MethodCallMissing(SourceExprSiteV1),
    MethodOwnerMismatch(SourceExprSiteV1),
    ReceiverSiteMismatch(SourceExprSiteV1),
    ArgumentCardinalityMismatch(SourceExprSiteV1),
    ArgumentOrdinalMismatch { site: SourceExprSiteV1, ordinal: u32 },
    ArgumentSiteMismatch { site: SourceExprSiteV1, ordinal: u32 },
    DuplicateArgumentSite(SourceExprSiteV1),
    DuplicateExpressionFact(SourceExprSiteV1),
    MissingExpressionFact(SourceExprSiteV1),
    UnsupportedLiteral(SourceExprSiteV1),
    UnsupportedUnary(SourceExprSiteV1),
    UnsupportedBinary(SourceExprSiteV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedScriptDirectStaticScalarOperandRecipeV1 {
    source_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<
        ScriptDirectStaticRecipeKeyV1,
        Box<[ScalarOperandRecipeArgumentV1]>,
    >,
}

impl VerifiedScriptDirectStaticScalarOperandRecipeV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<
            ScriptDirectStaticRecipeKeyV1,
            Box<[ScalarOperandRecipeArgumentV1]>,
        >,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    #[cfg(test)]
    pub(in crate::mir) fn with_source_identity_for_test(mut self, source_identity: usize) -> Self {
        self.source_identity = source_identity;
        self
    }

    #[cfg(test)]
    pub(in crate::mir) fn with_argument_site_for_test(
        mut self,
        key: ScriptDirectStaticRecipeKeyV1,
        site: SourceExprSiteV1,
    ) -> Self {
        if let Some(arguments) = self.rows.get_mut(&key) {
            if let Some(argument) = arguments.first_mut() {
                argument.site = site;
            }
        }
        self
    }

    pub(in crate::mir) fn issue(
        source: &VerifiedScriptSemanticSourceV1<'_>,
        join: &VerifiedScriptDirectStaticJoinHandoffV1,
    ) -> Result<Self, VerifiedScriptDirectStaticScalarOperandRecipeIssueV1> {
        if source.source() as *const _ as usize != join.source_identity() {
            return Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::SourceIdentityMismatch,
            );
        }
        let [root] = source.forest().roots() else {
            return Err(VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ScriptRootMissing);
        };
        let Some(product) = source
            .forest()
            .semantic_owner(*root)
            .and_then(VerifiedSemanticOwnerProductV1::as_script)
        else {
            return Err(VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ScriptRootNotScript);
        };
        let source_owner = product.core().data().owner;
        if source_owner != join.source_owner() {
            return Err(VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::SourceOwnerMismatch);
        }
        let inventory = product.expression_source();
        let mut rows = BTreeMap::new();
        for (key, row) in join.rows() {
            let arguments = issue_arguments(row, product, inventory)?;
            if rows.insert(*key, arguments).is_some() {
                return Err(
                    VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::DuplicateArgumentSite(
                        row.call_site().clone(),
                    ),
                );
            }
        }
        Ok(Self {
            source_owner,
            source_identity: join.source_identity(),
            rows,
        })
    }

    pub(in crate::mir) const fn source_owner(&self) -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(in crate::mir) const fn source_identity(&self) -> usize {
        self.source_identity
    }

    pub(in crate::mir) fn row(
        &self,
        key: ScriptDirectStaticRecipeKeyV1,
    ) -> Option<&[ScalarOperandRecipeArgumentV1]> {
        self.rows.get(&key).map(Box::as_ref)
    }

    pub(in crate::mir) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &ScriptDirectStaticRecipeKeyV1,
            &[ScalarOperandRecipeArgumentV1],
        ),
    > {
        self.rows.iter().map(|(key, rows)| (key, rows.as_ref()))
    }

    pub(in crate::mir) fn len(&self) -> usize {
        self.rows.len()
    }
}

fn issue_arguments(
    row: &VerifiedScriptDirectStaticJoinRowV1,
    product: &crate::mir::resolved_semantics::VerifiedResolvedScriptV1,
    inventory: &ResolvedExpressionSourceInventoryV1,
) -> Result<Box<[ScalarOperandRecipeArgumentV1]>, VerifiedScriptDirectStaticScalarOperandRecipeIssueV1>
{
    let Some((_, method)) = product
        .method_calls()
        .find(|(site, _)| *site == row.call_site())
    else {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::MethodCallMissing(
                row.call_site().clone(),
            ),
        );
    };
    if method.owner() != row.source_owner() {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::MethodOwnerMismatch(
                row.call_site().clone(),
            ),
        );
    }
    if method.receiver_site() != row.receiver_site() {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ReceiverSiteMismatch(
                row.call_site().clone(),
            ),
        );
    }
    if method.arguments().len() != row.argument_sites().len() {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ArgumentCardinalityMismatch(
                row.call_site().clone(),
            ),
        );
    }

    let mut seen = BTreeSet::new();
    let mut arguments = Vec::with_capacity(row.argument_sites().len());
    for (ordinal, (argument, expected_site)) in method
        .arguments()
        .iter()
        .zip(row.argument_sites())
        .enumerate()
    {
        let ordinal = ordinal as u32;
        if argument.ordinal() != ordinal {
            return Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ArgumentOrdinalMismatch {
                    site: row.call_site().clone(),
                    ordinal: argument.ordinal(),
                },
            );
        }
        if argument.site() != expected_site {
            return Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::ArgumentSiteMismatch {
                    site: row.call_site().clone(),
                    ordinal,
                },
            );
        }
        let tree = issue_node(inventory, expected_site, &mut seen)?;
        arguments.push(ScalarOperandRecipeArgumentV1 {
            ordinal,
            site: expected_site.clone(),
            tree,
        });
    }
    Ok(arguments.into_boxed_slice())
}

fn issue_node(
    inventory: &ResolvedExpressionSourceInventoryV1,
    site: &SourceExprSiteV1,
    seen: &mut BTreeSet<SourceExprSiteV1>,
) -> Result<ScalarOperandRecipeNodeV1, VerifiedScriptDirectStaticScalarOperandRecipeIssueV1>
{
    let literal = inventory.literal(site);
    let unary = inventory.unary(site);
    let binary = inventory.binary(site);
    let fact_count = usize::from(literal.is_some())
        + usize::from(unary.is_some())
        + usize::from(binary.is_some());
    if fact_count == 0 {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::MissingExpressionFact(
                site.clone(),
            ),
        );
    }
    if fact_count != 1 || !seen.insert(site.clone()) {
        return Err(
            VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::DuplicateExpressionFact(
                site.clone(),
            ),
        );
    }
    if let Some(literal) = literal {
        return match literal {
            ResolvedLiteralSourceV1::Integer(value) => Ok(ScalarOperandRecipeNodeV1::Literal {
                site: site.clone(),
                value: *value,
            }),
            _ => Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::UnsupportedLiteral(
                    site.clone(),
                ),
            ),
        };
    }
    if let Some(unary) = unary {
        let operator = match unary.operator() {
            ResolvedUnaryOperatorV1::Minus => ScalarUnaryOperatorV1::Minus,
            ResolvedUnaryOperatorV1::BitNot => ScalarUnaryOperatorV1::BitNot,
            _ => {
                return Err(
                    VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::UnsupportedUnary(
                        site.clone(),
                    ),
                )
            }
        };
        let operand = issue_node(inventory, unary.operand(), seen)?;
        return Ok(ScalarOperandRecipeNodeV1::Unary {
            site: site.clone(),
            operator,
            operand: Box::new(operand),
        });
    }
    let binary = binary.expect("expression fact count checked above");
    let operator = match binary.operator() {
        ResolvedBinaryOperatorV1::Add => ScalarBinaryOperatorV1::Add,
        ResolvedBinaryOperatorV1::Subtract => ScalarBinaryOperatorV1::Subtract,
        ResolvedBinaryOperatorV1::Multiply => ScalarBinaryOperatorV1::Multiply,
        ResolvedBinaryOperatorV1::BitAnd => ScalarBinaryOperatorV1::BitAnd,
        ResolvedBinaryOperatorV1::BitOr => ScalarBinaryOperatorV1::BitOr,
        ResolvedBinaryOperatorV1::BitXor => ScalarBinaryOperatorV1::BitXor,
        _ => {
            return Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::UnsupportedBinary(
                    site.clone(),
                ),
            )
        }
    };
    let lhs = issue_node(inventory, binary.lhs(), seen)?;
    let rhs = issue_node(inventory, binary.rhs(), seen)?;
    Ok(ScalarOperandRecipeNodeV1::Binary {
        site: site.clone(),
        operator,
        lhs: Box::new(lhs),
        rhs: Box::new(rhs),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        ResolvedBinaryExpressionSourceV1, ResolvedExpressionSourceInventoryV1,
        ResolvedUnaryExpressionSourceV1, SourcePathSegmentV1, SourcePathV1,
    };

    fn child(site: &SourceExprSiteV1, segment: SourcePathSegmentV1) -> SourceExprSiteV1 {
        SourcePathV1::from_node(site.node()).child(segment).expr()
    }

    #[test]
    fn recursive_integer_tree_is_issued_from_resolver_facts() {
        let root = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .expr();
        let lhs = child(&root, SourcePathSegmentV1::Lhs);
        let rhs = child(&root, SourcePathSegmentV1::Rhs);
        let rhs_operand = child(&rhs, SourcePathSegmentV1::Operand);
        let inventory = ResolvedExpressionSourceInventoryV1::from_parts_for_test(
            [ResolvedBinaryExpressionSourceV1::from_parts_for_test(
                root.clone(),
                ResolvedBinaryOperatorV1::Add,
                lhs.clone(),
                rhs.clone(),
            )],
            [ResolvedUnaryExpressionSourceV1::from_parts_for_test(
                rhs.clone(),
                ResolvedUnaryOperatorV1::BitNot,
                rhs_operand.clone(),
            )],
            [
                (lhs, ResolvedLiteralSourceV1::Integer(4)),
                (rhs_operand, ResolvedLiteralSourceV1::Integer(2)),
            ],
        );

        let node = issue_node(&inventory, &root, &mut BTreeSet::new()).expect("scalar tree");
        assert!(matches!(
            node,
            ScalarOperandRecipeNodeV1::Binary {
                operator: ScalarBinaryOperatorV1::Add,
                ..
            }
        ));
    }

    #[test]
    fn typed_integer_is_rejected_before_physical_lowering() {
        let site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .expr();
        let inventory = ResolvedExpressionSourceInventoryV1::from_parts_for_test(
            [],
            [],
            [(
                site.clone(),
                ResolvedLiteralSourceV1::TypedInteger {
                    value: 1,
                    declared_type_name: "i32".into(),
                },
            )],
        );
        assert_eq!(
            issue_node(&inventory, &site, &mut BTreeSet::new()),
            Err(
                VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::UnsupportedLiteral(site)
            )
        );
    }
}
