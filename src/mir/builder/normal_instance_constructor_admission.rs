//! Selected-normal source/physical admission for instance constructors.
//!
//! Constructor source identity belongs to the original Program Box occurrence
//! and the parser-owned constructor-map key.  A Script plain-Box runtime prefix
//! may demand the same source row a second time, but that is not a second
//! source occurrence: each physical demand receives a fresh linear admission.

use super::calls::LegacyFunctionPendingSessionV1;
use super::module_draft_collector::FunctionDraftKeyV1;
use super::module_lowering_invocation::{ModuleLoweringPortChildErrorV1, ModuleLoweringPortV1};
use super::recursive_child_lowering::RawInvocationChildPortV1;
use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl};
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;
use crate::mir::MirBuilder;
use crate::parser::ConstructorSourceIdV1;

#[path = "normal_instance_constructor_demand_manifest.rs"]
mod demand_manifest;
pub(super) use demand_manifest::{
    InstanceConstructorDemandExpectationV1, InstanceConstructorDemandManifestBuilderV1,
    InstanceConstructorDemandRoleV1, InstanceConstructorDemandTicketV1,
    VerifiedInstanceConstructorPhysicalDemandManifestV1,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct VerifiedInstanceConstructorPhysicalSourceCohortV1 {
    rows: Box<[VerifiedInstanceConstructorPhysicalSourceRowV1]>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct VerifiedInstanceConstructorPhysicalSourceRowV1 {
    source_id: ConstructorSourceIdV1,
    final_box_ordinal: u32,
    box_name: Box<str>,
    key: Box<str>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum InstanceConstructorPhysicalSourceIssueV1 {
    ProgramMissing,
    ForeignRow,
    DuplicateSourceId,
}

impl VerifiedInstanceConstructorPhysicalSourceCohortV1 {
    pub(super) fn issue(
        source: &ASTNode,
        package: &VerifiedNormalCallableSemanticPackageV1,
    ) -> Result<Self, InstanceConstructorPhysicalSourceIssueV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(InstanceConstructorPhysicalSourceIssueV1::ProgramMissing);
        };
        let mut rows = Vec::with_capacity(package.instance_constructors().rows().len());
        for semantic in package.instance_constructors().rows() {
            if rows
                .iter()
                .any(|row: &VerifiedInstanceConstructorPhysicalSourceRowV1| {
                    row.source_id.same_as(semantic.source_id())
                })
            {
                return Err(InstanceConstructorPhysicalSourceIssueV1::DuplicateSourceId);
            }
            let Some(ASTNode::BoxDeclaration {
                name, constructors, ..
            }) = statements.get(semantic.final_box_ordinal() as usize)
            else {
                return Err(InstanceConstructorPhysicalSourceIssueV1::ForeignRow);
            };
            if name != semantic.box_name() || !constructors.contains_key(semantic.key()) {
                return Err(InstanceConstructorPhysicalSourceIssueV1::ForeignRow);
            }
            let Some(ASTNode::FunctionDeclaration { .. }) = constructors.get(semantic.key()) else {
                return Err(InstanceConstructorPhysicalSourceIssueV1::ForeignRow);
            };
            rows.push(VerifiedInstanceConstructorPhysicalSourceRowV1 {
                source_id: semantic.source_id().clone(),
                final_box_ordinal: semantic.final_box_ordinal(),
                box_name: semantic.box_name().into(),
                key: semantic.key().into(),
            });
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    fn row_for(
        &self,
        statement_index: usize,
        box_name: &str,
        key: &str,
    ) -> Result<&VerifiedInstanceConstructorPhysicalSourceRowV1, String> {
        let ordinal = u32::try_from(statement_index)
            .map_err(|_| "[freeze:contract][mir/instance-constructor-source/ordinal]".to_owned())?;
        let mut matches = self.rows.iter().filter(|row| {
            row.final_box_ordinal == ordinal
                && row.box_name.as_ref() == box_name
                && row.key.as_ref() == key
        });
        let row = matches.next().ok_or_else(|| {
            "[freeze:contract][mir/instance-constructor-source/missing]".to_owned()
        })?;
        if matches.next().is_some() {
            return Err("[freeze:contract][mir/instance-constructor-source/duplicate]".to_owned());
        }
        Ok(row)
    }

    pub(super) fn validate_program(&self, statements: &[ASTNode]) -> Result<(), String> {
        let mut expected = 0usize;
        for (statement_index, statement) in statements.iter().enumerate() {
            let ASTNode::BoxDeclaration {
                is_static: false,
                name,
                constructors,
                ..
            } = statement
            else {
                continue;
            };
            for (key, declaration) in constructors {
                if !matches!(declaration, ASTNode::FunctionDeclaration { .. }) {
                    return Err(
                        "[freeze:contract][mir/instance-constructor-source/non-function]"
                            .to_owned(),
                    );
                }
                self.row_for(statement_index, name, key)?;
                expected += 1;
            }
        }
        if expected != self.rows.len() {
            return Err("[freeze:contract][mir/instance-constructor-source/count]".to_owned());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(in crate::mir::builder) struct NormalInstanceConstructorSourceKeyV1 {
    source_id: ConstructorSourceIdV1,
    statement_index: usize,
    box_name: Box<str>,
    parser_constructor_key: Box<str>,
}

impl NormalInstanceConstructorSourceKeyV1 {
    fn from_physical_source(
        source_id: ConstructorSourceIdV1,
        statement_index: usize,
        box_name: &str,
        parser_constructor_key: &str,
    ) -> Self {
        Self {
            source_id,
            statement_index,
            box_name: box_name.into(),
            parser_constructor_key: parser_constructor_key.into(),
        }
    }

    pub(in crate::mir::builder) const fn statement_index(&self) -> usize {
        self.statement_index
    }

    pub(in crate::mir::builder) fn box_name(&self) -> &str {
        &self.box_name
    }

    pub(in crate::mir::builder) fn parser_constructor_key(&self) -> &str {
        &self.parser_constructor_key
    }

    pub(in crate::mir::builder) fn source_id(&self) -> &ConstructorSourceIdV1 {
        &self.source_id
    }
}

/// One immutable source occurrence for every constructor row that survived the
/// parser's constructor-map normalization.  Cloning this receipt transports
/// the same source identity to Script runtime work; it does not issue another
/// source occurrence. Physical demand tickets are move-only and are consumed
/// by the selected-normal adapter.
#[derive(Debug)]
pub(in crate::mir::builder) struct NormalInstanceConstructorSourceBatchV1 {
    sources: Box<[NormalInstanceConstructorSourceKeyV1]>,
    tickets: Box<[InstanceConstructorDemandTicketV1]>,
    role: InstanceConstructorDemandRoleV1,
}

impl NormalInstanceConstructorSourceBatchV1 {
    pub(in crate::mir::builder) fn from_physical_cohort(
        statement_index: usize,
        box_name: &str,
        parser_constructor_keys: impl IntoIterator<Item = String>,
        cohort: &VerifiedInstanceConstructorPhysicalSourceCohortV1,
        role: InstanceConstructorDemandRoleV1,
    ) -> Result<Self, String> {
        let mut sources = Vec::new();
        for key in parser_constructor_keys {
            let row = cohort.row_for(statement_index, box_name, &key)?;
            sources.push(NormalInstanceConstructorSourceKeyV1::from_physical_source(
                row.source_id.clone(),
                statement_index,
                row.box_name.as_ref(),
                row.key.as_ref(),
            ));
        }
        let tickets = sources
            .iter()
            .map(|source| InstanceConstructorDemandTicketV1::issue(source.source_id(), role))
            .collect();
        Ok(Self {
            sources: sources.into_boxed_slice(),
            tickets,
            role,
        })
    }

    pub(in crate::mir::builder) fn sources(&self) -> &[NormalInstanceConstructorSourceKeyV1] {
        &self.sources
    }

    pub(in crate::mir::builder) const fn role(&self) -> InstanceConstructorDemandRoleV1 {
        self.role
    }

    pub(in crate::mir::builder) fn demand_expectations(
        &self,
    ) -> Vec<InstanceConstructorDemandExpectationV1> {
        self.sources
            .iter()
            .map(|source| {
                InstanceConstructorDemandExpectationV1::new(source.source_id(), self.role)
            })
            .collect()
    }

    pub(in crate::mir::builder) fn into_ticketed_sources(
        self,
    ) -> Result<
        Vec<(
            NormalInstanceConstructorSourceKeyV1,
            InstanceConstructorDemandTicketV1,
        )>,
        String,
    > {
        let Self {
            sources,
            tickets,
            role: _,
        } = self;
        if sources.len() != tickets.len() {
            return Err(
                "[freeze:contract][mir/instance-constructor-demand/source-ticket-count]".to_owned(),
            );
        }
        Ok(sources
            .into_vec()
            .into_iter()
            .zip(tickets.into_vec())
            .collect())
    }

    #[cfg(test)]
    pub(super) fn for_test(
        statement_index: usize,
        box_name: &str,
        parser_constructor_keys: impl IntoIterator<Item = String>,
        role: InstanceConstructorDemandRoleV1,
    ) -> Self {
        let sources = parser_constructor_keys
            .into_iter()
            .enumerate()
            .map(|(ordinal, key)| {
                NormalInstanceConstructorSourceKeyV1::from_physical_source(
                    ConstructorSourceIdV1::test_new(ordinal as u32),
                    statement_index,
                    box_name,
                    &key,
                )
            })
            .collect::<Vec<_>>();
        let tickets = sources
            .iter()
            .map(|source| InstanceConstructorDemandTicketV1::issue(source.source_id(), role))
            .collect();
        Self {
            sources: sources.into_boxed_slice(),
            tickets,
            role,
        }
    }
}

/// One physical constructor lowering demand.  Its source key deliberately
/// stays distinct from the legacy collector identity.
#[derive(Debug)]
pub(in crate::mir::builder) struct NormalInstanceConstructorDraftAdmissionV1 {
    source_key: NormalInstanceConstructorSourceKeyV1,
    physical_symbol: Box<str>,
    physical_arity: usize,
}

impl NormalInstanceConstructorDraftAdmissionV1 {
    pub(in crate::mir::builder) fn seal(
        source_key: NormalInstanceConstructorSourceKeyV1,
        normalized_parameter_count: usize,
    ) -> Self {
        let physical_symbol = format!(
            "{}.{}",
            source_key.box_name(),
            source_key.parser_constructor_key()
        )
        .into_boxed_str();
        Self {
            source_key,
            physical_symbol,
            physical_arity: normalized_parameter_count + 1,
        }
    }

    pub(in crate::mir::builder) fn source_key(&self) -> &NormalInstanceConstructorSourceKeyV1 {
        &self.source_key
    }

    pub(in crate::mir::builder) fn physical_symbol(&self) -> &str {
        &self.physical_symbol
    }

    pub(in crate::mir::builder) const fn physical_arity(&self) -> usize {
        self.physical_arity
    }

    fn into_legacy_collector_parts(self) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            source_key: _,
            physical_symbol,
            physical_arity,
        } = self;
        let symbol = physical_symbol.into_string();
        (
            FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
            symbol,
            physical_arity,
        )
    }
}

impl ModuleLoweringPortV1<'_> {
    pub(in crate::mir::builder) fn commit_normal_instance_constructor_pending(
        &mut self,
        pending: LegacyFunctionPendingSessionV1<'_>,
        admission: NormalInstanceConstructorDraftAdmissionV1,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        self.commit_legacy_symbol_pending(pending, admission.into_legacy_collector_parts())
    }
}

impl RawInvocationChildPortV1<'_, '_> {
    #[allow(clippy::too_many_arguments)]
    pub(in crate::mir::builder) fn lower_normal_instance_constructor_v1(
        &mut self,
        builder: &mut MirBuilder,
        source_key: &NormalInstanceConstructorSourceKeyV1,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), ModuleLoweringPortChildErrorV1> {
        let function_name = format!(
            "{}.{}",
            source_key.box_name(),
            source_key.parser_constructor_key()
        );
        let box_name = source_key.box_name().to_owned();
        let (params, param_decls) =
            super::recursive_child_lowering::normalize_instance_box_method_input_v1(
                &function_name,
                params,
                param_decls,
            );
        let admission =
            NormalInstanceConstructorDraftAdmissionV1::seal(source_key.clone(), params.len());
        let source_root =
            super::raw_invocation_source_transport::RawInvocationRootLineageV1::InstanceConstructor(
                source_key.clone(),
            );
        builder.observe_legacy_method_lowering_v1(&function_name, &body, Some(&box_name));
        let pending = super::raw_invocation_source_transport::RawSourceTransportPortV1::
            with_source_transport_v1(
                self,
                super::raw_invocation_source_transport::RawInvocationSourceTransportV1::root(
                    (),
                    source_root,
                ),
                |port, ()| {
                    port.capture_normalized_instance_box_method_pending_v1(
                        builder,
                        function_name,
                        box_name,
                        params,
                        param_decls,
                        return_type_name,
                        body,
                        uses,
                        attrs,
                    )
                },
            )?;
        self.commit_normal_instance_constructor_pending_v1(pending, admission)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::module_draft_collector::FunctionDraftKeyV1;

    #[test]
    fn one_source_occurrence_materializes_two_legacy_demands() {
        let source_id = ConstructorSourceIdV1::test_new(0);
        let source_key = NormalInstanceConstructorSourceKeyV1::from_physical_source(
            source_id.clone(),
            7,
            "Page",
            "birth/0",
        );
        let first = NormalInstanceConstructorDraftAdmissionV1::seal(source_key.clone(), 0);
        let second = NormalInstanceConstructorDraftAdmissionV1::seal(source_key, 0);

        assert!(first.source_key().source_id().same_as(&source_id));
        assert_eq!(first.source_key().statement_index(), 7);
        assert_eq!(first.physical_symbol(), "Page.birth/0");
        assert_eq!(second.physical_arity(), 1);
        let (key, symbol, arity) = second.into_legacy_collector_parts();
        assert_eq!(
            key,
            FunctionDraftKeyV1::LegacySymbol("Page.birth/0".to_owned())
        );
        assert_eq!(symbol, "Page.birth/0");
        assert_eq!(arity, 1);
    }

    #[test]
    fn physical_demand_manifest_rejects_duplicate_or_swapped_role_ticket() {
        let batch = NormalInstanceConstructorSourceBatchV1::for_test(
            7,
            "Page",
            ["birth/0".to_owned()],
            InstanceConstructorDemandRoleV1::ImmediateDeclaration,
        );
        let mut builder = InstanceConstructorDemandManifestBuilderV1::default();
        builder.issue_batch(&batch).expect("first role ticket");
        assert_eq!(
            builder
                .issue_batch(&batch)
                .expect_err("duplicate role ticket"),
            "[freeze:contract][mir/instance-constructor-demand/duplicate-ticket]"
        );

        let manifest = builder.finish();
        let swapped = vec![InstanceConstructorDemandExpectationV1::new(
            &ConstructorSourceIdV1::test_new(0),
            InstanceConstructorDemandRoleV1::ScriptRuntimePrefix,
        )];
        assert_eq!(
            manifest
                .validate_exact(&swapped)
                .expect_err("swapped role must not satisfy manifest"),
            "[freeze:contract][mir/instance-constructor-demand/coverage]"
        );
    }
}
