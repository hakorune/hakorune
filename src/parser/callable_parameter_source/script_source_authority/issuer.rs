use super::super::catalog::ParserCallableParameterSourceDispositionV1;
use super::super::composite_source::ParserCompositeSourceDispositionV1;
use super::super::parser_invocation_witness::ParserInvocationWitnessV1;
use crate::parser::postpass_envelope::CompletedParserPostpassV1;
use super::model::{
    ParserNormalProgramBodySourceRowV1, ParserNormalProgramBodySyntaxKindV1,
    ParserNormalProgramSourceAuthorityDispositionV1,
    ParserNormalProgramSourceAuthorityIncompleteV1,
    ParserNormalProgramSourceAuthorityIntegrityIssueV1,
    ParserNormalProgramSourceAuthorityUnavailableV1, ParserNormalProgramSourceAuthorityV1,
};

pub(crate) fn issue_parser_normal_program_source_authority_v1(
    completed: &CompletedParserPostpassV1,
    parameter_source: &ParserCallableParameterSourceDispositionV1,
    composite: ParserCompositeSourceDispositionV1,
) -> ParserNormalProgramSourceAuthorityDispositionV1 {
    if !completed.is_source_backed() {
        return ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(
            ParserNormalProgramSourceAuthorityUnavailableV1::PostpassNotSourceBacked,
        );
    }
    let ParserCallableParameterSourceDispositionV1::Complete(catalog) = parameter_source else {
        return ParserNormalProgramSourceAuthorityDispositionV1::SourceAuthorityUnavailable(
            ParserNormalProgramSourceAuthorityUnavailableV1::ParameterSourceUnavailable,
        );
    };
    let crate::ast::ASTNode::Program { statements, .. } = completed.ast() else {
        return if composite.is_ready() {
            ParserNormalProgramSourceAuthorityDispositionV1::IntegrityInvalid(
                ParserNormalProgramSourceAuthorityIntegrityIssueV1::CompositeReadyWithoutProgramBody,
            )
        } else {
            ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(
                ParserNormalProgramSourceAuthorityIncompleteV1::ProgramBodyMissing,
            )
        };
    };
    let mut rows = Vec::with_capacity(statements.len());
    for (position, statement) in statements.iter().enumerate() {
        let Ok(position) = u32::try_from(position) else {
            return ParserNormalProgramSourceAuthorityDispositionV1::Incomplete(
                ParserNormalProgramSourceAuthorityIncompleteV1::StatementPositionOverflow,
            );
        };
        rows.push(ParserNormalProgramBodySourceRowV1::new(
            position,
            syntax_kind(statement),
        ));
    }
    ParserNormalProgramSourceAuthorityDispositionV1::Ready(
        ParserNormalProgramSourceAuthorityV1::new(
            ParserInvocationWitnessV1::from_brand(catalog.parser_brand()),
            rows.into_boxed_slice(),
            composite,
        ),
    )
}

fn syntax_kind(statement: &crate::ast::ASTNode) -> ParserNormalProgramBodySyntaxKindV1 {
    use crate::ast::ASTNode;
    use ParserNormalProgramBodySyntaxKindV1 as Kind;
    match statement {
        ASTNode::BoxDeclaration { .. } => Kind::BoxDeclaration,
        ASTNode::BuildGate { .. } => Kind::BuildGate,
        ASTNode::FunctionDeclaration { .. } => Kind::FunctionDeclaration,
        ASTNode::BrandDeclaration { .. } => Kind::BrandDeclaration,
        ASTNode::TypeAliasDeclaration { .. } => Kind::TypeAliasDeclaration,
        ASTNode::EnumDeclaration { .. } => Kind::EnumDeclaration,
        ASTNode::GlobalVar { .. } => Kind::GlobalVar,
        ASTNode::StaticConstTable { .. } => Kind::StaticConstTable,
        ASTNode::UsingStatement { .. } => Kind::UsingStatement,
        ASTNode::ImportStatement { .. } => Kind::ImportStatement,
        ASTNode::Program { .. }
        | ASTNode::Assignment { .. }
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
        | ASTNode::Call { .. } => Kind::ExecutableItem,
    }
}

pub(super) fn parser_program_body_syntax_kind(
    statement: &crate::ast::ASTNode,
) -> ParserNormalProgramBodySyntaxKindV1 {
    syntax_kind(statement)
}
