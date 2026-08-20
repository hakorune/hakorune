//! Parser-only pure-Script cohort admission.
//!
//! This module deliberately stops before front-door identity, source rows,
//! resolver meaning, and physical lowering.  It turns the broad postpass
//! `NoBoxDeclarations` compatibility observation into a canonical admission
//! only after an exhaustive same-invocation shape check.

use super::catalog::{
    ParserCallableParameterSourceCatalogV1, ParserCallableParameterSourceDispositionV1,
};
use crate::ast::ASTNode;
use crate::parser::postpass_envelope::{CompletedParserPostpassV1, ParserPostpassProgramCohortV1};
use crate::parser::source_authority::ParserInvocationBrandV1;

#[derive(Debug)]
pub(crate) enum CanonicalScriptCohortDispositionV1 {
    NotApplicable,
    CompatibilitySource,
    Deferred,
    SourceAuthorityUnavailable,
    CohortUnresolved,
    CanonicalScriptCohortAdmitted(CanonicalScriptCohortAdmissionV1),
    IntegrityInvalid,
    ObservationIncomplete,
    NonCandidate,
    DispositionTransported,
}

/// A parser-only witness.  It contains no AST, source plan, Recipe, Join,
/// semantic target, or physical identity.
#[derive(Debug)]
pub(crate) struct CanonicalScriptCohortAdmissionV1 {
    parser_brand: ParserInvocationBrandV1,
    _seal: CanonicalScriptCohortAdmissionSealV1,
}

#[derive(Debug)]
struct CanonicalScriptCohortAdmissionSealV1;

impl CanonicalScriptCohortAdmissionV1 {
    fn issue(catalog: &ParserCallableParameterSourceCatalogV1) -> Self {
        Self {
            parser_brand: catalog.parser_brand().clone(),
            _seal: CanonicalScriptCohortAdmissionSealV1,
        }
    }

    pub(crate) fn same_parser_source(&self, other: &Self) -> bool {
        self.parser_brand.same_as(&other.parser_brand)
    }

    pub(super) fn parser_brand(&self) -> &ParserInvocationBrandV1 {
        &self.parser_brand
    }
}

pub(super) fn issue_canonical_script_cohort(
    completed: &CompletedParserPostpassV1,
    parameter_source: &ParserCallableParameterSourceDispositionV1,
) -> CanonicalScriptCohortDispositionV1 {
    let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
        return CanonicalScriptCohortDispositionV1::SourceAuthorityUnavailable;
    };

    match completed.program_cohort_for_admission() {
        ParserPostpassProgramCohortV1::NonProgram => {
            CanonicalScriptCohortDispositionV1::NotApplicable
        }
        ParserPostpassProgramCohortV1::NoBoxDeclarations => {
            classify_no_box_program(completed.ast(), catalog)
        }
        ParserPostpassProgramCohortV1::OrdinaryTopLevelBox
        | ParserPostpassProgramCohortV1::InterfaceBox
        | ParserPostpassProgramCohortV1::StaticBox
        | ParserPostpassProgramCohortV1::RecordBox
        | ParserPostpassProgramCohortV1::MixedProgram
        | ParserPostpassProgramCohortV1::TopLevelBuildGate => {
            CanonicalScriptCohortDispositionV1::CompatibilitySource
        }
    }
}

fn classify_no_box_program(
    ast: &ASTNode,
    catalog: &ParserCallableParameterSourceCatalogV1,
) -> CanonicalScriptCohortDispositionV1 {
    let ASTNode::Program { statements, .. } = ast else {
        return CanonicalScriptCohortDispositionV1::NotApplicable;
    };

    for statement in statements {
        let state = match statement {
            ASTNode::UsingStatement { .. } | ASTNode::ImportStatement { .. } => {
                CanonicalScriptCohortDispositionV1::CohortUnresolved
            }
            ASTNode::BuildGate { .. } | ASTNode::BoxDeclaration { .. } => {
                CanonicalScriptCohortDispositionV1::CompatibilitySource
            }
            ASTNode::Program { .. } => CanonicalScriptCohortDispositionV1::CohortUnresolved,
            ASTNode::Assignment { .. }
            | ASTNode::CompoundAssignment { .. }
            | ASTNode::Print { .. }
            | ASTNode::If { .. }
            | ASTNode::Loop { .. }
            | ASTNode::LoopRange { .. }
            | ASTNode::Return { .. }
            | ASTNode::Break { .. }
            | ASTNode::Continue { .. }
            | ASTNode::Release { .. }
            | ASTNode::Nowait { .. }
            | ASTNode::TaskScope { .. }
            | ASTNode::ContextScope { .. }
            | ASTNode::FastMemRegion { .. }
            | ASTNode::AwaitExpression { .. }
            | ASTNode::QMarkPropagate { .. }
            | ASTNode::MatchExpr { .. }
            | ASTNode::EnumMatchExpr { .. }
            | ASTNode::ArrayLiteral { .. }
            | ASTNode::MapLiteral { .. }
            | ASTNode::RecordLiteral { .. }
            | ASTNode::RecordUpdate { .. }
            | ASTNode::Lambda { .. }
            | ASTNode::BlockExpr { .. }
            | ASTNode::Arrow { .. }
            | ASTNode::TryCatch { .. }
            | ASTNode::Throw { .. }
            | ASTNode::FunctionDeclaration { .. }
            | ASTNode::EnumDeclaration { .. }
            | ASTNode::BrandDeclaration { .. }
            | ASTNode::TypeAliasDeclaration { .. }
            | ASTNode::GlobalVar { .. }
            | ASTNode::StaticConstTable { .. }
            | ASTNode::Literal { .. }
            | ASTNode::Variable { .. }
            | ASTNode::UnaryOp { .. }
            | ASTNode::BinaryOp { .. }
            | ASTNode::CheckExpr { .. }
            | ASTNode::GroupedAssignmentExpr { .. }
            | ASTNode::MethodCall { .. }
            | ASTNode::FieldAccess { .. }
            | ASTNode::Index { .. }
            | ASTNode::New { .. }
            | ASTNode::This { .. }
            | ASTNode::Me { .. }
            | ASTNode::FromCall { .. }
            | ASTNode::ThisField { .. }
            | ASTNode::MeField { .. }
            | ASTNode::Local { .. }
            | ASTNode::ScopeBox { .. }
            | ASTNode::Outbox { .. }
            | ASTNode::FunctionCall { .. }
            | ASTNode::ExplicitExternCall { .. }
            | ASTNode::Call { .. } => continue,
        };
        return state;
    }

    CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(
        CanonicalScriptCohortAdmissionV1::issue(catalog),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{NyashParser, ParserBuildConfig};

    fn parse(
        source: &str,
    ) -> crate::parser::callable_parameter_source::ParsedProgramWithCallableParameterSourceV1 {
        NyashParser::parse_from_string_with_callable_parameter_source(
            source,
            ParserBuildConfig::default(),
        )
        .expect("parser source product")
    }

    #[test]
    fn pure_script_requires_the_exhaustive_shape_and_issues_once() {
        let parsed = parse("print(1)\n");
        let CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(admission) =
            parsed.canonical_script_admission()
        else {
            panic!("pure Script source should be admitted")
        };
        assert!(admission.same_parser_source(admission));
    }

    #[test]
    fn typed_admission_controls_source_disposition_not_the_old_boolean() {
        let parsed = parse("print(1)\n");
        let disposition = parsed.into_source_disposition();
        assert!(disposition.is_source_backed());
    }

    #[test]
    fn boxes_remain_compatibility_and_using_is_unresolved() {
        let boxed = parse("box Plain { run() { return 1 } }\n");
        assert!(matches!(
            boxed.canonical_script_admission(),
            CanonicalScriptCohortDispositionV1::CompatibilitySource
        ));

        let using = parse("using plain\nprint(1)\n");
        assert!(matches!(
            using.canonical_script_admission(),
            CanonicalScriptCohortDispositionV1::CohortUnresolved
        ));
    }

    #[test]
    fn independent_parser_invocations_have_distinct_admission_seals() {
        let left = parse("print(1)\n");
        let right = parse("print(1)\n");
        let CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(left) =
            left.canonical_script_admission()
        else {
            panic!("left admission")
        };
        let CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(right) =
            right.canonical_script_admission()
        else {
            panic!("right admission")
        };
        assert!(!left.same_parser_source(right));
    }
}
