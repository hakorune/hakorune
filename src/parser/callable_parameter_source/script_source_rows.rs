//! Parser-owned AST-free rows for the admitted no-import pure-Script cohort.

use super::canonical_script_source_admission::{
    CanonicalScriptCohortAdmissionV1, CanonicalScriptCohortDispositionV1,
};
use super::catalog::ParserCallableParameterSourceCatalogV1;
use super::script_source_rows_model::{
    BrandSyntaxSnapshotV1, CanonicalScriptSourceRowsSealV1, ScriptBodyRowV1,
    ScriptBodySyntaxKindV1, ScriptDeclarationSyntaxSnapshotV1, ScriptImportConfigSnapshotV1,
    ScriptParameterSyntaxRowV1,
};
pub(crate) use super::script_source_rows_model::{
    CanonicalScriptSourceRowsDispositionV1, CanonicalScriptSourceRowsV1,
};
use crate::ast::{ASTNode, ParamDecl};
use crate::parser::postpass_envelope::CompletedParserPostpassV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ScriptRowsIssueV1 {
    NonProgramRoot,
    UnsupportedTopLevelShape,
    StatementOrdinalOverflow,
    ParameterOrdinalOverflow,
    BodyCoverageMismatch,
    CatalogBrandMismatch,
}

fn issue_rows(
    completed: &CompletedParserPostpassV1,
    catalog: &ParserCallableParameterSourceCatalogV1,
    admission: &CanonicalScriptCohortAdmissionV1,
) -> Result<CanonicalScriptSourceRowsV1, ScriptRowsIssueV1> {
    if !catalog.parser_brand().same_as(admission.parser_brand()) {
        return Err(ScriptRowsIssueV1::CatalogBrandMismatch);
    }
    let ASTNode::Program { statements, .. } = completed.ast() else {
        return Err(ScriptRowsIssueV1::NonProgramRoot);
    };
    let statement_count =
        u32::try_from(statements.len()).map_err(|_| ScriptRowsIssueV1::StatementOrdinalOverflow)?;
    let mut body_rows = Vec::with_capacity(statements.len());
    let mut declarations = Vec::new();
    let mut brands = Vec::new();
    for (ordinal, statement) in statements.iter().enumerate() {
        let ordinal =
            u32::try_from(ordinal).map_err(|_| ScriptRowsIssueV1::StatementOrdinalOverflow)?;
        let kind = match statement {
            ASTNode::FunctionDeclaration {
                name, param_decls, ..
            } => {
                declarations.push(ScriptDeclarationSyntaxSnapshotV1::new(
                    ordinal,
                    ScriptBodySyntaxKindV1::FunctionDeclaration,
                    name.clone().into_boxed_str(),
                    parameter_rows(param_decls)?,
                ));
                ScriptBodySyntaxKindV1::FunctionDeclaration
            }
            ASTNode::BrandDeclaration {
                name,
                underlying_type_name,
                ..
            } => {
                brands.push(BrandSyntaxSnapshotV1::new(
                    ordinal,
                    name.clone().into_boxed_str(),
                    underlying_type_name.clone().into_boxed_str(),
                ));
                ScriptBodySyntaxKindV1::BrandDeclaration
            }
            ASTNode::TypeAliasDeclaration { name, .. } => {
                declarations.push(ScriptDeclarationSyntaxSnapshotV1::new(
                    ordinal,
                    ScriptBodySyntaxKindV1::TypeAliasDeclaration,
                    name.clone().into_boxed_str(),
                    Box::new([]),
                ));
                ScriptBodySyntaxKindV1::TypeAliasDeclaration
            }
            ASTNode::EnumDeclaration { name, .. } => {
                declarations.push(ScriptDeclarationSyntaxSnapshotV1::new(
                    ordinal,
                    ScriptBodySyntaxKindV1::EnumDeclaration,
                    name.clone().into_boxed_str(),
                    Box::new([]),
                ));
                ScriptBodySyntaxKindV1::EnumDeclaration
            }
            ASTNode::GlobalVar { name, .. } => {
                declarations.push(ScriptDeclarationSyntaxSnapshotV1::new(
                    ordinal,
                    ScriptBodySyntaxKindV1::GlobalVar,
                    name.clone().into_boxed_str(),
                    Box::new([]),
                ));
                ScriptBodySyntaxKindV1::GlobalVar
            }
            ASTNode::StaticConstTable { name, .. } => {
                declarations.push(ScriptDeclarationSyntaxSnapshotV1::new(
                    ordinal,
                    ScriptBodySyntaxKindV1::StaticConstTable,
                    name.clone().into_boxed_str(),
                    Box::new([]),
                ));
                ScriptBodySyntaxKindV1::StaticConstTable
            }
            ASTNode::BoxDeclaration { .. }
            | ASTNode::BuildGate { .. }
            | ASTNode::UsingStatement { .. }
            | ASTNode::ImportStatement { .. }
            | ASTNode::Program { .. } => return Err(ScriptRowsIssueV1::UnsupportedTopLevelShape),
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
            | ASTNode::Call { .. } => ScriptBodySyntaxKindV1::ExecutableItem,
        };
        body_rows.push(ScriptBodyRowV1::new(ordinal, kind));
    }
    if body_rows.len() != statements.len()
        || body_rows
            .iter()
            .enumerate()
            .any(|(index, row)| match u32::try_from(index) {
                Ok(expected) => row.ordinal() != expected,
                Err(_) => true,
            })
    {
        return Err(ScriptRowsIssueV1::BodyCoverageMismatch);
    }
    Ok(CanonicalScriptSourceRowsV1 {
        parser_brand: catalog.parser_brand().clone(),
        statement_count,
        body_rows: body_rows.into_boxed_slice(),
        declarations: declarations.into_boxed_slice(),
        brands: brands.into_boxed_slice(),
        import_config: ScriptImportConfigSnapshotV1::no_imports(),
        seal: CanonicalScriptSourceRowsSealV1,
    })
}

fn parameter_rows(
    declarations: &[ParamDecl],
) -> Result<Box<[ScriptParameterSyntaxRowV1]>, ScriptRowsIssueV1> {
    let mut rows = Vec::with_capacity(declarations.len());
    for (ordinal, declaration) in declarations.iter().enumerate() {
        let ordinal =
            u32::try_from(ordinal).map_err(|_| ScriptRowsIssueV1::ParameterOrdinalOverflow)?;
        rows.push(ScriptParameterSyntaxRowV1::new(
            ordinal,
            declaration.name.clone().into_boxed_str(),
            declaration
                .declared_type_name
                .clone()
                .map(String::into_boxed_str),
        ));
    }
    Ok(rows.into_boxed_slice())
}

pub(super) fn issue_canonical_script_source_rows(
    completed: &CompletedParserPostpassV1,
    catalog: &ParserCallableParameterSourceCatalogV1,
    admission: &CanonicalScriptCohortDispositionV1,
) -> CanonicalScriptSourceRowsDispositionV1 {
    match admission {
        CanonicalScriptCohortDispositionV1::NotApplicable => {
            CanonicalScriptSourceRowsDispositionV1::NotApplicable
        }
        CanonicalScriptCohortDispositionV1::CompatibilitySource => {
            CanonicalScriptSourceRowsDispositionV1::CompatibilitySource
        }
        CanonicalScriptCohortDispositionV1::Deferred => {
            CanonicalScriptSourceRowsDispositionV1::Deferred
        }
        CanonicalScriptCohortDispositionV1::CohortUnresolved => {
            CanonicalScriptSourceRowsDispositionV1::CohortUnresolved
        }
        CanonicalScriptCohortDispositionV1::SourceAuthorityUnavailable => {
            CanonicalScriptSourceRowsDispositionV1::SourceAuthorityUnavailable
        }
        CanonicalScriptCohortDispositionV1::CanonicalScriptCohortAdmitted(admission) => {
            match issue_rows(completed, catalog, admission) {
                Ok(rows) => CanonicalScriptSourceRowsDispositionV1::HandoffReady(rows),
                Err(ScriptRowsIssueV1::CatalogBrandMismatch) => {
                    CanonicalScriptSourceRowsDispositionV1::IntegrityInvalid
                }
                Err(ScriptRowsIssueV1::NonProgramRoot)
                | Err(ScriptRowsIssueV1::UnsupportedTopLevelShape)
                | Err(ScriptRowsIssueV1::StatementOrdinalOverflow)
                | Err(ScriptRowsIssueV1::ParameterOrdinalOverflow)
                | Err(ScriptRowsIssueV1::BodyCoverageMismatch) => {
                    CanonicalScriptSourceRowsDispositionV1::ObservationIncomplete
                }
            }
        }
        CanonicalScriptCohortDispositionV1::IntegrityInvalid => {
            CanonicalScriptSourceRowsDispositionV1::IntegrityInvalid
        }
        CanonicalScriptCohortDispositionV1::ObservationIncomplete => {
            CanonicalScriptSourceRowsDispositionV1::ObservationIncomplete
        }
        CanonicalScriptCohortDispositionV1::NonCandidate => {
            CanonicalScriptSourceRowsDispositionV1::NonCandidate
        }
        CanonicalScriptCohortDispositionV1::DispositionTransported => {
            CanonicalScriptSourceRowsDispositionV1::DispositionTransported
        }
    }
}
