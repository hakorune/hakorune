//! Source-bound proof for callee-required Script direct-static arguments.
//!
//! This sibling deliberately proves only the ordinals required by the sealed
//! callee result disposition.  It does not replace the all-argument scalar
//! candidate, lower a value, or select a physical route.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, ResolvedExpressionSourceInventoryV1, SourceExprSiteV1,
    VerifiedSemanticOwnerProductV1,
};

use super::scalar_operand_recipe::{
    issue_node, validate_argument_shape, ScalarOperandRecipeNodeV1,
    VerifiedScriptDirectStaticScalarOperandRecipeIssueV1,
};
use super::{
    ScriptDirectStaticRecipeKeyV1, VerifiedScriptDirectStaticJoinHandoffV1,
    VerifiedScriptDirectStaticJoinRowV1,
};
use crate::mir::builder::normal_script_semantic_source::VerifiedScriptSemanticSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum ScriptDirectStaticRequiredArgumentProofIssueV1 {
    SourceIdentityMismatch,
    ScriptRootMissing,
    ScriptRootNotScript,
    SourceOwnerMismatch,
    JoinOwnerMismatch(SourceExprSiteV1),
    DuplicateJoinKey(ScriptDirectStaticRecipeKeyV1),
    RequiredOrdinalOutOfBounds {
        site: SourceExprSiteV1,
        ordinal: u32,
    },
    UnsupportedRequiredArgument {
        site: SourceExprSiteV1,
        ordinal: u32,
        source: VerifiedScriptDirectStaticScalarOperandRecipeIssueV1,
    },
    MethodShape(VerifiedScriptDirectStaticScalarOperandRecipeIssueV1),
    CanonicalSourceRowMissing(SourceExprSiteV1),
    CanonicalSourceRowForeign(SourceExprSiteV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct RequiredArgumentProofArgumentV1 {
    ordinal: u32,
    site: SourceExprSiteV1,
    tree: ScalarOperandRecipeNodeV1,
}

impl RequiredArgumentProofArgumentV1 {
    pub(in crate::mir::builder) fn from_canonical_source(
        ordinal: u32,
        site: SourceExprSiteV1,
        tree: ScalarOperandRecipeNodeV1,
    ) -> Self {
        Self {
            ordinal,
            site,
            tree,
        }
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
pub(in crate::mir) enum ScriptDirectStaticRequiredArgumentProofDispositionV1 {
    ExactI64Empty,
    ExactI64Required(Box<[RequiredArgumentProofArgumentV1]>),
    NonExact(VerifiedCallableResultRepresentationV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct ScriptDirectStaticRequiredArgumentProofRowV1 {
    call_site: SourceExprSiteV1,
    disposition: ScriptDirectStaticRequiredArgumentProofDispositionV1,
}

impl ScriptDirectStaticRequiredArgumentProofRowV1 {
    pub(in crate::mir) const fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(in crate::mir) const fn disposition(
        &self,
    ) -> &ScriptDirectStaticRequiredArgumentProofDispositionV1 {
        &self.disposition
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedScriptDirectStaticRequiredArgumentProofV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<ScriptDirectStaticRecipeKeyV1, ScriptDirectStaticRequiredArgumentProofRowV1>,
}

impl VerifiedScriptDirectStaticRequiredArgumentProofV1 {
    pub(in crate::mir) fn issue(
        source: &VerifiedScriptSemanticSourceV1<'_>,
        join: &VerifiedScriptDirectStaticJoinHandoffV1,
    ) -> Result<Self, ScriptDirectStaticRequiredArgumentProofIssueV1> {
        if source.source() as *const _ as usize != join.source_identity() {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::SourceIdentityMismatch);
        }
        let [root] = source.forest().roots() else {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::ScriptRootMissing);
        };
        let Some(product) = source
            .forest()
            .semantic_owner(*root)
            .and_then(VerifiedSemanticOwnerProductV1::as_script)
        else {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::ScriptRootNotScript);
        };
        let source_owner = product.core().data().owner;
        if source_owner != join.source_owner() {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::SourceOwnerMismatch);
        }
        let inventory = product.expression_source();
        let mut rows = BTreeMap::new();
        for (key, row) in join.rows() {
            if row.source_owner() != source_owner {
                return Err(
                    ScriptDirectStaticRequiredArgumentProofIssueV1::JoinOwnerMismatch(
                        row.call_site().clone(),
                    ),
                );
            }
            validate_argument_shape(row, product).map_err(
                ScriptDirectStaticRequiredArgumentProofIssueV1::MethodShape,
            )?;
            let disposition = issue_row(row, inventory)?;
            if rows
                .insert(
                    *key,
                    ScriptDirectStaticRequiredArgumentProofRowV1 {
                        call_site: row.call_site().clone(),
                        disposition,
                    },
                )
                .is_some()
            {
                return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::DuplicateJoinKey(*key));
            }
        }
        Ok(Self {
            source_owner,
            source_identity: source.source() as *const _ as usize,
            rows,
        })
    }

    /// Move key-free source facts issued by A onto the Recipe-owned keys.
    /// No resolver expression inventory is consulted here.
    pub(in crate::mir::builder) fn from_canonical_source_rows(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        join: &VerifiedScriptDirectStaticJoinHandoffV1,
        mut source_rows: BTreeMap<
            SourceExprSiteV1,
            ScriptDirectStaticRequiredArgumentProofDispositionV1,
        >,
    ) -> Result<Self, ScriptDirectStaticRequiredArgumentProofIssueV1> {
        if join.source_owner() != source_owner {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::SourceOwnerMismatch);
        }
        if join.source_identity() != source_identity {
            return Err(ScriptDirectStaticRequiredArgumentProofIssueV1::SourceIdentityMismatch);
        }
        let mut rows = BTreeMap::new();
        for (key, join_row) in join.rows() {
            let Some(disposition) = source_rows.remove(join_row.call_site()) else {
                return Err(
                    ScriptDirectStaticRequiredArgumentProofIssueV1::CanonicalSourceRowMissing(
                        join_row.call_site().clone(),
                    ),
                );
            };
            if rows
                .insert(
                    *key,
                    ScriptDirectStaticRequiredArgumentProofRowV1 {
                        call_site: join_row.call_site().clone(),
                        disposition,
                    },
                )
                .is_some()
            {
                return Err(
                    ScriptDirectStaticRequiredArgumentProofIssueV1::DuplicateJoinKey(*key),
                );
            }
        }
        if let Some((site, _)) = source_rows.into_iter().next() {
            return Err(
                ScriptDirectStaticRequiredArgumentProofIssueV1::CanonicalSourceRowForeign(site),
            );
        }
        Ok(Self {
            source_owner,
            source_identity,
            rows,
        })
    }

    pub(in crate::mir) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.source_owner
    }

    pub(in crate::mir) const fn source_identity(&self) -> usize {
        self.source_identity
    }

    pub(in crate::mir) fn row(
        &self,
        key: ScriptDirectStaticRecipeKeyV1,
    ) -> Option<&ScriptDirectStaticRequiredArgumentProofRowV1> {
        self.rows.get(&key)
    }

    pub(in crate::mir) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &ScriptDirectStaticRecipeKeyV1,
            &ScriptDirectStaticRequiredArgumentProofRowV1,
        ),
    > {
        self.rows.iter()
    }

    pub(in crate::mir) fn len(&self) -> usize {
        self.rows.len()
    }
}

fn issue_row(
    row: &VerifiedScriptDirectStaticJoinRowV1,
    inventory: &ResolvedExpressionSourceInventoryV1,
) -> Result<
    ScriptDirectStaticRequiredArgumentProofDispositionV1,
    ScriptDirectStaticRequiredArgumentProofIssueV1,
> {
    if !matches!(
        row.representation(),
        VerifiedCallableResultRepresentationV1::ExactI64
    ) {
        return Ok(
            ScriptDirectStaticRequiredArgumentProofDispositionV1::NonExact(
                row.representation().clone(),
            ),
        );
    }
    if row.required_callee_i64_arguments().is_empty() {
        return Ok(ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty);
    }

    let mut seen = BTreeSet::new();
    let mut arguments = Vec::with_capacity(row.required_callee_i64_arguments().len());
    for ordinal in row.required_callee_i64_arguments() {
        let Some(site) = row.argument_sites().get(*ordinal as usize) else {
            return Err(
                ScriptDirectStaticRequiredArgumentProofIssueV1::RequiredOrdinalOutOfBounds {
                    site: row.call_site().clone(),
                    ordinal: *ordinal,
                },
            );
        };
        let tree = issue_node(inventory, site, &mut seen).map_err(|source| {
            ScriptDirectStaticRequiredArgumentProofIssueV1::UnsupportedRequiredArgument {
                site: row.call_site().clone(),
                ordinal: *ordinal,
                source,
            }
        })?;
        arguments.push(RequiredArgumentProofArgumentV1 {
            ordinal: *ordinal,
            site: site.clone(),
            tree,
        });
    }
    Ok(
        ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(
            arguments.into_boxed_slice(),
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_script_direct_static_recipe::
        ScriptDirectStaticRecipeDestinationV1;
    use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, ResolvedLiteralSourceV1, SourcePathSegmentV1, SourcePathV1,
    };

    fn row(
        argument_sites: Box<[SourceExprSiteV1]>,
        required: Box<[u32]>,
        representation: VerifiedCallableResultRepresentationV1,
    ) -> (VerifiedScriptDirectStaticJoinRowV1, SourceExprSiteV1) {
        let mut owner_issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = owner_issuer.issue().expect("source owner");
        let statement = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt();
        let call_site = SourcePathV1::from_node(statement.node()).expr();
        let receiver_site = SourcePathV1::from_node(call_site.node())
            .child(SourcePathSegmentV1::Receiver)
            .expr();
        let target =
            crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "Helpers", "run", argument_sites.len(),
            );
        let row = VerifiedScriptDirectStaticJoinRowV1::from_parts_for_test(
            ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(1),
            owner,
            call_site.clone(),
            receiver_site,
            argument_sites,
            call_site.clone(),
            Box::new([]),
            ScriptDirectStaticRecipeDestinationV1::FinalSequence { statement },
            target,
            representation,
            required,
        );
        (row, call_site)
    }

    #[test]
    fn required_proof_issues_only_required_scalar_ordinals() {
        let root = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .expr();
        let required_site = SourcePathV1::from_node(root.node())
            .child(SourcePathSegmentV1::Argument(0))
            .expr();
        let non_required_site = SourcePathV1::from_node(root.node())
            .child(SourcePathSegmentV1::Argument(1))
            .expr();
        let inventory = ResolvedExpressionSourceInventoryV1::from_parts_for_test(
            [],
            [],
            [(required_site.clone(), ResolvedLiteralSourceV1::Integer(7))],
        );
        let (row, call_site) = row(
            vec![required_site.clone(), non_required_site].into_boxed_slice(),
            vec![0].into_boxed_slice(),
            VerifiedCallableResultRepresentationV1::ExactI64,
        );
        let disposition = issue_row(&row, &inventory).expect("required scalar proof");
        let ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Required(arguments) =
            disposition
        else {
            panic!("required ordinal must be represented");
        };
        assert_eq!(arguments.len(), 1);
        assert_eq!(arguments[0].ordinal(), 0);
        assert_eq!(arguments[0].site(), &required_site);
        assert_eq!(arguments[0].tree().site(), &required_site);
        assert_eq!(row.call_site(), &call_site);
    }

    #[test]
    fn required_proof_rejects_unsupported_required_literal() {
        let site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .expr();
        let inventory = ResolvedExpressionSourceInventoryV1::from_parts_for_test(
            [],
            [],
            [(
                site.clone(),
                ResolvedLiteralSourceV1::TypedInteger {
                    value: 7,
                    declared_type_name: "i32".into(),
                },
            )],
        );
        let (row, _) = row(
            vec![site.clone()].into_boxed_slice(),
            vec![0].into_boxed_slice(),
            VerifiedCallableResultRepresentationV1::ExactI64,
        );
        assert!(matches!(
            issue_row(&row, &inventory),
            Err(
                ScriptDirectStaticRequiredArgumentProofIssueV1::UnsupportedRequiredArgument {
                    ordinal: 0,
                    source: VerifiedScriptDirectStaticScalarOperandRecipeIssueV1::
                        UnsupportedLiteral(_),
                    ..
                }
            )
        ));
    }

    #[test]
    fn empty_and_non_exact_rows_are_explicit_states() {
        let site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .expr();
        let inventory = ResolvedExpressionSourceInventoryV1::from_parts_for_test([], [], []);
        let (empty, _) = row(
            vec![site.clone()].into_boxed_slice(),
            Box::new([]),
            VerifiedCallableResultRepresentationV1::ExactI64,
        );
        assert_eq!(
            issue_row(&empty, &inventory),
            Ok(ScriptDirectStaticRequiredArgumentProofDispositionV1::ExactI64Empty)
        );

        let (nominal, _) = row(
            vec![site].into_boxed_slice(),
            Box::new([]),
            VerifiedCallableResultRepresentationV1::ExactNominalBox {
                box_name: "Token".into(),
            },
        );
        assert!(matches!(
            issue_row(&nominal, &inventory),
            Ok(ScriptDirectStaticRequiredArgumentProofDispositionV1::NonExact(
                VerifiedCallableResultRepresentationV1::ExactNominalBox { .. }
            ))
        ));
    }
}
