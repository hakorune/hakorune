use crate::ast::ASTNode;
use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::source_call_target::{
    StaticImportAliasViewErrorV1, VerifiedStaticImportAliasViewV1,
};

use super::legacy_static_import_snapshot::CompilerSuppliedStaticImportSnapshotV1;
use super::lowering_input::{LegacyModuleLoweringInputV1, LegacyModuleOriginV1};

/// One complete Legacy source request before route selection.
///
/// This disconnected owner carries syntax provenance and exactly one typed
/// compiler-supplied import snapshot. It owns no Builder or retry authority.
#[derive(Debug)]
pub(super) struct LegacyWholeSourceCompileRequestV1 {
    input: LegacyModuleLoweringInputV1,
    imports: CompilerSuppliedStaticImportSnapshotV1,
    diagnostic_source_hint: Option<Box<str>>,
}

impl LegacyWholeSourceCompileRequestV1 {
    pub(super) const fn new(
        input: LegacyModuleLoweringInputV1,
        imports: CompilerSuppliedStaticImportSnapshotV1,
        diagnostic_source_hint: Option<Box<str>>,
    ) -> Self {
        Self {
            input,
            imports,
            diagnostic_source_hint,
        }
    }

    pub(super) const fn ast(&self) -> &ASTNode {
        self.input.ast()
    }

    pub(super) const fn origin(&self) -> LegacyModuleOriginV1 {
        self.input.origin()
    }

    pub(super) fn diagnostic_source_hint(&self) -> Option<&str> {
        self.diagnostic_source_hint.as_deref()
    }

    pub(super) fn verify_alias_view<'catalog>(
        &self,
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Result<VerifiedStaticImportAliasViewV1<'catalog>, StaticImportAliasViewErrorV1> {
        self.imports.verify_alias_view(declarations)
    }

    pub(super) const fn imports_are_explicit(&self) -> bool {
        self.imports.is_explicit()
    }

    pub(super) fn import_count(&self) -> usize {
        self.imports.len()
    }

    pub(super) fn import_entries(&self) -> impl Iterator<Item = (&str, &str)> {
        self.imports.entries()
    }

    pub(super) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, Span};

    use super::super::legacy_static_import_snapshot::CompilerSuppliedStaticImportSnapshotV1;
    use super::super::lowering_input::{LegacyModuleLoweringInputV1, LegacyModuleOriginV1};
    use super::LegacyWholeSourceCompileRequestV1;

    #[test]
    fn request_retains_exact_input_origin_and_one_import_snapshot() {
        let request = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::bare_ast(ASTNode::Program {
                statements: Vec::new(),
                span: Span::unknown(),
            }),
            CompilerSuppliedStaticImportSnapshotV1::explicit([(
                "Alias".to_owned(),
                "Owner".to_owned(),
            )])
            .unwrap(),
            Some("fixture.hako".into()),
        );
        assert_eq!(request.origin(), LegacyModuleOriginV1::BareAst);
        assert!(matches!(request.ast(), ASTNode::Program { .. }));
        assert_eq!(request.diagnostic_source_hint(), Some("fixture.hako"));
        request.discard();
    }

    #[test]
    fn compatibility_origins_remain_explicitly_distinct() {
        let syntax = || ASTNode::Program {
            statements: Vec::new(),
            span: Span::unknown(),
        };
        let program = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::program_v0_compatibility(syntax()),
            CompilerSuppliedStaticImportSnapshotV1::none(),
            None,
        );
        let repl = LegacyWholeSourceCompileRequestV1::new(
            LegacyModuleLoweringInputV1::repl_compatibility(syntax()),
            CompilerSuppliedStaticImportSnapshotV1::none(),
            None,
        );
        assert_eq!(
            program.origin(),
            LegacyModuleOriginV1::ProgramV0Compatibility
        );
        assert_eq!(repl.origin(), LegacyModuleOriginV1::ReplCompatibility);
    }
}
