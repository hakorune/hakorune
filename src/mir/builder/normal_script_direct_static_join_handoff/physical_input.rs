//! AST-free physical input assembled from one existing Join row and its
//! source-issued scalar operand Recipe.

use std::collections::BTreeMap;

use crate::mir::builder::normal_script_direct_static_recipe::ScriptDirectStaticRecipeKeyV1;
use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};

use super::scalar_operand_recipe::{
    ScalarOperandRecipeArgumentV1, VerifiedScriptDirectStaticScalarOperandRecipeV1,
};
use super::{VerifiedScriptDirectStaticJoinHandoffV1, VerifiedScriptDirectStaticJoinRowV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum VerifiedScriptDirectStaticPhysicalInputIssueV1 {
    SourceIdentityMismatch,
    SourceOwnerMismatch,
    CardinalityMismatch,
    OperandRowMissing(ScriptDirectStaticRecipeKeyV1),
    ArgumentCardinalityMismatch(ScriptDirectStaticRecipeKeyV1),
    ArgumentSiteMismatch {
        key: ScriptDirectStaticRecipeKeyV1,
        ordinal: u32,
    },
    ArgumentOrdinalMismatch {
        key: ScriptDirectStaticRecipeKeyV1,
        ordinal: u32,
    },
    DuplicateKey(ScriptDirectStaticRecipeKeyV1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedScriptDirectStaticPhysicalInputRowV1 {
    key: ScriptDirectStaticRecipeKeyV1,
    join: VerifiedScriptDirectStaticJoinRowV1,
    arguments: Box<[ScalarOperandRecipeArgumentV1]>,
}

impl VerifiedScriptDirectStaticPhysicalInputRowV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_parts_for_test(
        key: ScriptDirectStaticRecipeKeyV1,
        join: VerifiedScriptDirectStaticJoinRowV1,
        arguments: Box<[ScalarOperandRecipeArgumentV1]>,
    ) -> Self {
        Self {
            key,
            join,
            arguments,
        }
    }

    pub(in crate::mir) const fn key(&self) -> ScriptDirectStaticRecipeKeyV1 {
        self.key
    }

    pub(in crate::mir) const fn source_owner(&self) -> FunctionOwnerIdV1 {
        self.join.source_owner()
    }

    pub(in crate::mir) const fn call_site(&self) -> &SourceExprSiteV1 {
        self.join.call_site()
    }

    pub(in crate::mir) fn argument_sites(&self) -> &[SourceExprSiteV1] {
        self.join.argument_sites()
    }

    pub(in crate::mir) const fn target(
        &self,
    ) -> &crate::mir::builder::CanonicalSameModuleCallableKeyV1 {
        self.join.target()
    }

    pub(in crate::mir) const fn representation(&self) -> &VerifiedCallableResultRepresentationV1 {
        self.join.representation()
    }

    pub(in crate::mir) fn arguments(&self) -> &[ScalarOperandRecipeArgumentV1] {
        &self.arguments
    }

    pub(in crate::mir) fn destination(
        &self,
    ) -> &crate::mir::builder::normal_script_direct_static_recipe::ScriptDirectStaticRecipeDestinationV1
    {
        self.join.destination()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) struct VerifiedScriptDirectStaticPhysicalInputV1 {
    source_owner: FunctionOwnerIdV1,
    source_identity: usize,
    rows: BTreeMap<
        ScriptDirectStaticRecipeKeyV1,
        VerifiedScriptDirectStaticPhysicalInputRowV1,
    >,
}

impl VerifiedScriptDirectStaticPhysicalInputV1 {
    #[cfg(test)]
    pub(in crate::mir) fn from_parts_for_test(
        source_owner: FunctionOwnerIdV1,
        source_identity: usize,
        rows: BTreeMap<
            ScriptDirectStaticRecipeKeyV1,
            VerifiedScriptDirectStaticPhysicalInputRowV1,
        >,
    ) -> Self {
        Self {
            source_owner,
            source_identity,
            rows,
        }
    }

    pub(in crate::mir) fn issue(
        join: &VerifiedScriptDirectStaticJoinHandoffV1,
        operands: &VerifiedScriptDirectStaticScalarOperandRecipeV1,
    ) -> Result<Self, VerifiedScriptDirectStaticPhysicalInputIssueV1> {
        if join.source_identity() != operands.source_identity() {
            return Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::SourceIdentityMismatch);
        }
        if join.source_owner() != operands.source_owner() {
            return Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::SourceOwnerMismatch);
        }
        if join.len() != operands.len() {
            return Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::CardinalityMismatch);
        }
        let mut rows = BTreeMap::new();
        for (key, join_row) in join.rows() {
            let Some(arguments) = operands.row(*key) else {
                return Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::OperandRowMissing(
                    *key,
                ));
            };
            if arguments.len() != join_row.argument_sites().len() {
                return Err(
                    VerifiedScriptDirectStaticPhysicalInputIssueV1::ArgumentCardinalityMismatch(
                        *key,
                    ),
                );
            }
            for (ordinal, (argument, expected_site)) in arguments
                .iter()
                .zip(join_row.argument_sites())
                .enumerate()
            {
                let ordinal = ordinal as u32;
                if argument.ordinal() != ordinal {
                    return Err(
                        VerifiedScriptDirectStaticPhysicalInputIssueV1::ArgumentOrdinalMismatch {
                            key: *key,
                            ordinal: argument.ordinal(),
                        },
                    );
                }
                if argument.site() != expected_site {
                    return Err(
                        VerifiedScriptDirectStaticPhysicalInputIssueV1::ArgumentSiteMismatch {
                            key: *key,
                            ordinal,
                        },
                    );
                }
            }
            let row = VerifiedScriptDirectStaticPhysicalInputRowV1 {
                key: *key,
                join: join_row.clone(),
                arguments: arguments.to_vec().into_boxed_slice(),
            };
            if rows.insert(*key, row).is_some() {
                return Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::DuplicateKey(
                    *key,
                ));
            }
        }
        Ok(Self {
            source_owner: join.source_owner(),
            source_identity: join.source_identity(),
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
    ) -> Option<&VerifiedScriptDirectStaticPhysicalInputRowV1> {
        self.rows.get(&key)
    }

    pub(in crate::mir) fn rows(
        &self,
    ) -> impl Iterator<
        Item = (
            &ScriptDirectStaticRecipeKeyV1,
            &VerifiedScriptDirectStaticPhysicalInputRowV1,
        ),
    > {
        self.rows.iter()
    }

    pub(in crate::mir) fn len(&self) -> usize {
        self.rows.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_script_direct_static_recipe::{
        ScriptDirectStaticRecipeDestinationV1, ScriptDirectStaticRecipeKeyV1,
    };
    use crate::mir::builder::normal_script_direct_static_join_handoff::ScalarOperandRecipeNodeV1;
    use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1};
    use std::collections::BTreeMap;

    fn fixture() -> (
        VerifiedScriptDirectStaticJoinHandoffV1,
        VerifiedScriptDirectStaticScalarOperandRecipeV1,
        ScriptDirectStaticRecipeKeyV1,
        SourceExprSiteV1,
    ) {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        let owner = issuer.issue().expect("source owner");
        let statement = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(0))
            .stmt();
        let call_site = SourcePathV1::from_node(statement.node()).expr();
        let receiver_site = SourcePathV1::from_node(call_site.node())
            .child(SourcePathSegmentV1::Receiver)
            .expr();
        let argument_site = SourcePathV1::from_node(call_site.node())
            .child(SourcePathSegmentV1::Argument(0))
            .expr();
        let key = ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(0);
        let target = crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Helpers", "run", 1,
        );
        let join_row = VerifiedScriptDirectStaticJoinRowV1::from_parts_for_test(
            key,
            owner,
            call_site.clone(),
            receiver_site,
            vec![argument_site.clone()].into_boxed_slice(),
            call_site.clone(),
            Box::new([]),
            ScriptDirectStaticRecipeDestinationV1::FinalSequence { statement },
            target,
            VerifiedCallableResultRepresentationV1::ExactI64,
            Box::new([]),
        );
        let handoff = VerifiedScriptDirectStaticJoinHandoffV1::from_parts_for_test(
            owner,
            41,
            BTreeMap::from([(key, join_row)]),
        );
        let argument = ScalarOperandRecipeArgumentV1::from_parts_for_test(
            0,
            argument_site.clone(),
            ScalarOperandRecipeNodeV1::Literal {
                site: argument_site.clone(),
                value: 7,
            },
        );
        let operands = VerifiedScriptDirectStaticScalarOperandRecipeV1::from_parts_for_test(
            owner,
            41,
            BTreeMap::from([(key, vec![argument].into_boxed_slice())]),
        );
        (handoff, operands, key, argument_site)
    }

    #[test]
    fn physical_input_preserves_join_key_and_operand_site() {
        let (handoff, operands, key, argument_site) = fixture();
        let input = VerifiedScriptDirectStaticPhysicalInputV1::issue(&handoff, &operands)
            .expect("physical input");
        let row = input.row(key).expect("one physical row");
        assert_eq!(row.key(), key);
        assert_eq!(row.arguments()[0].site(), &argument_site);
        assert_eq!(row.arguments()[0].ordinal(), 0);
    }

    #[test]
    fn physical_input_rejects_identity_and_site_drift_before_lowering() {
        let (handoff, operands, _key, _) = fixture();
        let operands = operands.with_source_identity_for_test(42);
        assert_eq!(
            VerifiedScriptDirectStaticPhysicalInputV1::issue(&handoff, &operands),
            Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::SourceIdentityMismatch)
        );

        let (handoff, operands, key, _) = fixture();
        let wrong_site = SourcePathV1::program_body()
            .child(SourcePathSegmentV1::ProgramBody(1))
            .expr();
        let operands = operands.with_argument_site_for_test(key, wrong_site);
        assert_eq!(
            VerifiedScriptDirectStaticPhysicalInputV1::issue(&handoff, &operands),
            Err(VerifiedScriptDirectStaticPhysicalInputIssueV1::ArgumentSiteMismatch {
                key,
                ordinal: 0,
            })
        );
    }
}
