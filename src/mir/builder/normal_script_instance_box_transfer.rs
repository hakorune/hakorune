//! Exact installed-semantic coverage for Script-root instance Box transfer.

use std::collections::{BTreeMap, BTreeSet};

use crate::ast::ASTNode;
use crate::mir::normal_callable_semantic_package::VerifiedNormalCallableSemanticPackageV1;

use super::callable_declaration_catalog::SelectedNormalCallableSourceSiteV1;
use super::normal_instance_constructor_admission::{
    InstanceConstructorDemandRoleV1, VerifiedInstanceConstructorPhysicalSourceCohortV1,
};
use super::normal_script_program_item_admission::{
    classify_normal_script_program_item_v1, NormalScriptProgramItemAdmissionV1,
};

#[derive(Debug)]
pub(super) struct VerifiedScriptInstanceBoxTransferCohortV1 {
    statement_ordinals: BTreeSet<usize>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptInstanceBoxTransferIssueV1 {
    ProgramMissing,
    MethodCoverage,
    ConstructorCoverage,
    ForeignConstructor,
}

impl VerifiedScriptInstanceBoxTransferCohortV1 {
    pub(super) fn issue(
        source: &ASTNode,
        package: &VerifiedNormalCallableSemanticPackageV1,
    ) -> Result<Self, ScriptInstanceBoxTransferIssueV1> {
        let ASTNode::Program { statements, .. } = source else {
            return Err(ScriptInstanceBoxTransferIssueV1::ProgramMissing);
        };
        let mut method_keys = BTreeMap::<usize, BTreeMap<Box<str>, usize>>::new();
        for (_, site) in package.selected_callable_sources().entries() {
            if let SelectedNormalCallableSourceSiteV1::ProgramBoxMethod {
                statement_index,
                method_key,
            } = site
            {
                *method_keys
                    .entry(*statement_index)
                    .or_default()
                    .entry(method_key.clone())
                    .or_default() += 1;
            }
        }
        let mut constructor_keys = BTreeMap::<usize, BTreeSet<Box<str>>>::new();
        for row in package.instance_constructors().rows() {
            let ordinal = row.final_box_ordinal() as usize;
            let Some(ASTNode::BoxDeclaration { name, .. }) = statements.get(ordinal) else {
                return Err(ScriptInstanceBoxTransferIssueV1::ForeignConstructor);
            };
            if name != row.box_name()
                || !constructor_keys
                    .entry(ordinal)
                    .or_default()
                    .insert(row.key().into())
            {
                return Err(ScriptInstanceBoxTransferIssueV1::ForeignConstructor);
            }
        }

        let mut transferred = BTreeSet::new();
        for (ordinal, statement) in statements.iter().enumerate() {
            if !matches!(
                classify_normal_script_program_item_v1(statement),
                NormalScriptProgramItemAdmissionV1::InstancePrefixCompatibility
                    | NormalScriptProgramItemAdmissionV1::NonPlainInstanceFullLifecycle
            ) {
                continue;
            }
            let ASTNode::BoxDeclaration {
                methods,
                constructors,
                ..
            } = statement
            else {
                unreachable!("instance admission is a Box declaration")
            };
            let mut expected_methods = BTreeMap::<Box<str>, usize>::new();
            for entry in methods.iter_compat_name_order() {
                let ASTNode::FunctionDeclaration { name, .. } = entry.declaration() else {
                    return Err(ScriptInstanceBoxTransferIssueV1::MethodCoverage);
                };
                *expected_methods.entry(name.as_str().into()).or_default() += 1;
            }
            if method_keys.remove(&ordinal).unwrap_or_default() != expected_methods {
                return Err(ScriptInstanceBoxTransferIssueV1::MethodCoverage);
            }
            let expected_constructors = constructors
                .keys()
                .map(|key| Box::<str>::from(key.as_str()))
                .collect::<BTreeSet<_>>();
            if constructor_keys.remove(&ordinal).unwrap_or_default() != expected_constructors {
                return Err(ScriptInstanceBoxTransferIssueV1::ConstructorCoverage);
            }
            transferred.insert(ordinal);
        }
        Ok(Self {
            statement_ordinals: transferred,
        })
    }

    pub(super) fn contains(&self, statement_ordinal: usize) -> bool {
        self.statement_ordinals.contains(&statement_ordinal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_instance_constructor_admission::InstanceConstructorPhysicalSourceIssueV1;
    use crate::mir::builder::program_root_work_plan::{
        PreparedProgramRootImmediateWorkV1, PreparedProgramRootRuntimeWorkV1,
        PreparedProgramRootWorkPlanV1, ProgramRootWorkPlanAdmissionV1,
    };
    use crate::mir::resolved_semantics::{
        FunctionSemanticResolverSessionV1, ScriptRootSemanticDispositionV1,
        ScriptTransferredBoundaryV1,
    };
    use crate::parser::NyashParser;

    #[test]
    fn exact_instance_box_coverage_becomes_one_transferred_script_boundary() {
        let source = r#"
box Holder {
    init(value) { return value }
    get(value) { return value }
}
"#;
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable source");
        let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform")
        });
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must stay source-backed")
        };
        let mut resolver = FunctionSemanticResolverSessionV1::new(142).unwrap();
        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1(
            &mut resolver,
            source,
        )
        .expect("semantic package");
        let cohort =
            VerifiedScriptInstanceBoxTransferCohortV1::issue(package.source_ast(), &package)
                .expect("exact transfer cohort");
        let constructor_source_cohort = VerifiedInstanceConstructorPhysicalSourceCohortV1::issue(
            package.source_ast(),
            &package,
        )
        .expect("exact constructor source cohort");
        let ASTNode::Program { statements, .. } = package.source_ast().clone() else {
            panic!("Program source")
        };
        let plan = PreparedProgramRootWorkPlanV1::prepare_with_instance_box_transfers_and_constructor_sources(
            statements,
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
            Some(package.selected_callable_sources()),
            Some(&cohort),
            Some(&constructor_source_cohort),
        )
        .expect("physical source transfer");
        assert!(matches!(
            plan.into_parts()
                .script_root_admission
                .expect("Script window")
                .window()
                .entry_at(0)
                .expect("Box entry")
                .semantic(),
            ScriptRootSemanticDispositionV1::Transferred(
                ScriptTransferredBoundaryV1::InstanceBoxSemanticOwner
            )
        ));
    }

    #[test]
    fn foreign_constructor_cannot_masquerade_as_total_coverage() {
        let source = r#"
box Holder {
    init(value) { return value }
    get(value) { return value }
}
"#;
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable source");
        let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform")
        });
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must stay source-backed")
        };
        let mut resolver = FunctionSemanticResolverSessionV1::new(143).unwrap();
        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1(
            &mut resolver,
            source,
        )
        .expect("semantic package");
        let mut foreign_source = package.source_ast().clone();
        let ASTNode::Program { statements, .. } = &mut foreign_source else {
            panic!("Program source")
        };
        let ASTNode::BoxDeclaration { constructors, .. } = &mut statements[0] else {
            panic!("Box source")
        };
        let foreign = constructors.get("init/1").expect("init source").clone();
        constructors.insert("foreign/1".to_owned(), foreign);
        assert_eq!(
            VerifiedScriptInstanceBoxTransferCohortV1::issue(&foreign_source, &package)
                .expect_err("foreign constructor is not an installed semantic owner"),
            ScriptInstanceBoxTransferIssueV1::ConstructorCoverage
        );
    }

    #[test]
    fn physical_constructor_demands_retain_one_parser_source_id() {
        let source = r#"
box Holder {
    init(value) { return value }
}
"#;
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable source");
        let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform")
        });
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must stay source-backed")
        };
        let mut resolver = FunctionSemanticResolverSessionV1::new(144).unwrap();
        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1(
            &mut resolver,
            source,
        )
        .expect("semantic package");
        let physical = VerifiedInstanceConstructorPhysicalSourceCohortV1::issue(
            package.source_ast(),
            &package,
        )
        .expect("physical source cohort");
        let ASTNode::Program { statements, .. } = package.source_ast().clone() else {
            panic!("Program source")
        };
        let plan = PreparedProgramRootWorkPlanV1::prepare_with_instance_box_transfers_and_constructor_sources(
            statements,
            false,
            ProgramRootWorkPlanAdmissionV1::SelectedNormal,
            Some(package.selected_callable_sources()),
            None,
            Some(&physical),
        )
        .expect("physical source transfer");
        let parts = plan.into_parts();
        let PreparedProgramRootImmediateWorkV1::InstanceBox(immediate) = &parts.immediate[0] else {
            panic!("expected immediate instance Box")
        };
        let immediate_id = immediate
            .normal_constructor_sources()
            .expect("immediate source")
            .sources()[0]
            .source_id()
            .clone();
        assert_eq!(
            immediate
                .normal_constructor_sources()
                .expect("immediate role")
                .role(),
            InstanceConstructorDemandRoleV1::ImmediateDeclaration
        );
        let PreparedProgramRootRuntimeWorkV1::SelectedNormal(runtime) = &parts.runtime else {
            panic!("expected selected runtime")
        };
        let (runtime_sources, _) = runtime.constructor_admission_at(0).expect("runtime source");
        assert_eq!(
            runtime_sources.role(),
            InstanceConstructorDemandRoleV1::ScriptRuntimePrefix
        );
        assert!(runtime_sources.sources()[0]
            .source_id()
            .same_as(&immediate_id));
    }

    #[test]
    fn physical_constructor_cohort_rejects_foreign_constructor_row() {
        let source = r#"box Holder { init(value) { return value } }"#;
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            source,
            crate::parser::ParserBuildConfig::default(),
        )
        .expect("callable source");
        let transformed = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform")
        });
        let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) = transformed
        else {
            panic!("fixture must stay source-backed")
        };
        let mut resolver = FunctionSemanticResolverSessionV1::new(145).unwrap();
        let package = crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1(
            &mut resolver,
            source,
        )
        .expect("semantic package");
        let mut foreign = package.source_ast().clone();
        let ASTNode::Program { statements, .. } = &mut foreign else {
            panic!("Program source")
        };
        let ASTNode::BoxDeclaration { constructors, .. } = &mut statements[0] else {
            panic!("Box source")
        };
        let declaration = constructors.remove("init/1").expect("constructor");
        constructors.insert("foreign/1".to_owned(), declaration);
        assert_eq!(
            VerifiedInstanceConstructorPhysicalSourceCohortV1::issue(&foreign, &package)
                .expect_err("foreign constructor must reject"),
            InstanceConstructorPhysicalSourceIssueV1::ForeignRow
        );
    }
}
