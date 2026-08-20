use crate::ast::ASTNode;
use crate::parser::postpass_envelope::CompletedParserPostpassV1;

/// Invocation-neutral identity retained beside one owned parsed source.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalSourceIdentityV1 {
    display_name: Box<str>,
}

impl NormalSourceIdentityV1 {
    fn new(display_name: impl Into<Box<str>>) -> Self {
        Self {
            display_name: display_name.into(),
        }
    }

    pub(crate) fn display_name(&self) -> &str {
        &self.display_name
    }
}

/// The sole input consumed by normal source-family classification.
#[derive(Debug)]
pub(crate) struct PreparedNormalSourcePlanInputV1 {
    source: NormalSourcePlanSourceV1,
    identity: NormalSourceIdentityV1,
}

#[derive(Debug)]
enum NormalSourcePlanSourceV1 {
    AstOnly(ASTNode),
    ParserBacked(CompletedParserPostpassV1),
}

impl PreparedNormalSourcePlanInputV1 {
    pub(crate) fn new(source: ASTNode, display_name: impl Into<Box<str>>) -> Self {
        Self {
            source: NormalSourcePlanSourceV1::AstOnly(source),
            identity: NormalSourceIdentityV1::new(display_name),
        }
    }

    pub(crate) fn from_parser_postpass(
        postpass: CompletedParserPostpassV1,
        display_name: impl Into<Box<str>>,
    ) -> Self {
        Self {
            source: NormalSourcePlanSourceV1::ParserBacked(postpass),
            identity: NormalSourceIdentityV1::new(display_name),
        }
    }

    pub(super) fn source(&self) -> &ASTNode {
        match &self.source {
            NormalSourcePlanSourceV1::AstOnly(source) => source,
            NormalSourcePlanSourceV1::ParserBacked(postpass) => postpass.ast(),
        }
    }

    pub(super) fn identity(&self) -> &NormalSourceIdentityV1 {
        &self.identity
    }

    pub(super) fn into_parts(self) -> (ASTNode, NormalSourceIdentityV1) {
        let source = match self.source {
            NormalSourcePlanSourceV1::AstOnly(source) => source,
            NormalSourcePlanSourceV1::ParserBacked(postpass) => postpass.into_ast(),
        };
        (source, self.identity)
    }

    pub(crate) fn has_parser_postpass(&self) -> bool {
        matches!(&self.source, NormalSourcePlanSourceV1::ParserBacked(_))
    }

    pub(crate) fn parser_postpass(&self) -> Option<&CompletedParserPostpassV1> {
        match &self.source {
            NormalSourcePlanSourceV1::AstOnly(_) => None,
            NormalSourcePlanSourceV1::ParserBacked(postpass) => Some(postpass),
        }
    }
}

/// A source-only top-level location.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalTopLevelSiteV1 {
    statement_index: usize,
}

impl NormalTopLevelSiteV1 {
    pub(super) fn new(statement_index: usize) -> Self {
        Self { statement_index }
    }

    pub(super) fn statement_index(&self) -> usize {
        self.statement_index
    }
}

/// A source-only method location inside the unique Main declaration.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct NormalMainMethodSiteV1 {
    main_statement_index: usize,
    method_key: Box<str>,
    arity: usize,
    is_static: bool,
}

impl NormalMainMethodSiteV1 {
    pub(super) fn new(
        main_statement_index: usize,
        method_key: Box<str>,
        arity: usize,
        is_static: bool,
    ) -> Self {
        Self {
            main_statement_index,
            method_key,
            arity,
            is_static,
        }
    }

    pub(super) fn method_key(&self) -> &str {
        &self.method_key
    }

    pub(super) fn main_statement_index(&self) -> usize {
        self.main_statement_index
    }

    pub(super) fn arity(&self) -> usize {
        self.arity
    }

    pub(super) fn is_static(&self) -> bool {
        self.is_static
    }
}

/// One additional callable site without callable-catalog identity.
#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalAdditionalCallableSiteV1 {
    TopLevel(NormalTopLevelSiteV1),
    MainMethod(NormalMainMethodSiteV1),
}

#[derive(Debug)]
pub(crate) struct SealedNormalScriptSourceV1 {
    input: PreparedNormalSourcePlanInputV1,
    statements: Box<[NormalTopLevelSiteV1]>,
    _seal: NormalScriptSourceSealV1,
}

#[derive(Debug)]
struct NormalScriptSourceSealV1;

impl SealedNormalScriptSourceV1 {
    pub(super) fn seal(
        input: PreparedNormalSourcePlanInputV1,
        statements: Box<[NormalTopLevelSiteV1]>,
    ) -> Self {
        Self {
            input,
            statements,
            _seal: NormalScriptSourceSealV1,
        }
    }

    /// Consume the exact Script source family into the shared source-result
    /// recipe. The recipe owns all Script-tail classification; this source
    /// plan never exposes a bare AST for a second classifier.
    pub(crate) fn prepare_script_recipe(
        self,
    ) -> Result<
        super::script_recipe::VerifiedNormalScriptRecipeV1,
        super::script_recipe::RejectedNormalScriptRecipeV1,
    > {
        super::script_recipe::prepare(self)
    }

    pub(super) fn into_parts(
        self,
    ) -> (PreparedNormalSourcePlanInputV1, Box<[NormalTopLevelSiteV1]>) {
        (self.input, self.statements)
    }

    pub(super) fn source_ast(&self) -> &ASTNode {
        self.input.source()
    }

    pub(crate) fn has_parser_postpass(&self) -> bool {
        self.input.has_parser_postpass()
    }

    pub(crate) fn parser_postpass(
        &self,
    ) -> Option<&crate::parser::postpass_envelope::CompletedParserPostpassV1> {
        self.input.parser_postpass()
    }
}

#[derive(Debug)]
pub(crate) struct SealedNormalMainSourceV1 {
    input: PreparedNormalSourcePlanInputV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: NormalMainSourceSealV1,
}

#[derive(Debug)]
struct NormalMainSourceSealV1;

impl SealedNormalMainSourceV1 {
    pub(super) fn seal(
        input: PreparedNormalSourcePlanInputV1,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
    ) -> Self {
        Self {
            input,
            main_box,
            main_method,
            _seal: NormalMainSourceSealV1,
        }
    }

    pub(crate) fn prepare_function_source(
        self,
    ) -> Result<
        super::main_source::VerifiedNormalMainFunctionSourceUnitV1,
        super::main_source::RejectedNormalMainFunctionSourceV1,
    > {
        match super::main_source::verify_main_source_relation(&self) {
            Ok(()) => Ok(super::main_source::VerifiedNormalMainFunctionSourceUnitV1::seal(self)),
            Err(error) => Err(super::main_source::RejectedNormalMainFunctionSourceV1::new(
                self, error,
            )),
        }
    }

    pub(super) fn input(&self) -> &PreparedNormalSourcePlanInputV1 {
        &self.input
    }

    pub(super) fn main_box(&self) -> &NormalTopLevelSiteV1 {
        &self.main_box
    }

    pub(super) fn main_method(&self) -> &NormalMainMethodSiteV1 {
        &self.main_method
    }
}

#[derive(Debug)]
pub(crate) struct SealedNormalCallableModuleSourceV1 {
    input: PreparedNormalSourcePlanInputV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    additional_callables: Box<[NormalAdditionalCallableSiteV1]>,
    _seal: NormalCallableModuleSourceSealV1,
}

#[derive(Debug)]
struct NormalCallableModuleSourceSealV1;

impl SealedNormalCallableModuleSourceV1 {
    pub(super) fn seal(
        input: PreparedNormalSourcePlanInputV1,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
        additional_callables: Box<[NormalAdditionalCallableSiteV1]>,
    ) -> Self {
        Self {
            input,
            main_box,
            main_method,
            additional_callables,
            _seal: NormalCallableModuleSourceSealV1,
        }
    }

    pub(crate) fn prepare_callable_source(
        self,
    ) -> Result<
        super::callable_source::VerifiedNormalCallableSourceUnitV1,
        super::callable_source::RejectedNormalCallableSourceV1,
    > {
        super::callable_source::prepare(self)
    }

    pub(super) fn additional_callables(&self) -> &[NormalAdditionalCallableSiteV1] {
        &self.additional_callables
    }

    pub(super) fn input(&self) -> &PreparedNormalSourcePlanInputV1 {
        &self.input
    }

    pub(super) fn main_box(&self) -> &NormalTopLevelSiteV1 {
        &self.main_box
    }

    pub(super) fn main_method(&self) -> &NormalMainMethodSiteV1 {
        &self.main_method
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PreparedNormalSourcePlanInputV1,
        NormalTopLevelSiteV1,
        NormalMainMethodSiteV1,
        Box<[NormalAdditionalCallableSiteV1]>,
    ) {
        (
            self.input,
            self.main_box,
            self.main_method,
            self.additional_callables,
        )
    }
}

#[derive(Debug)]
pub(crate) enum SealedNormalScalarRootV1 {
    Script(SealedNormalScriptSourceV1),
    Main0(SealedNormalMainSourceV1),
}

#[derive(Debug)]
pub(crate) enum SealedNormalSourcePlanV1 {
    ScalarRoot(SealedNormalScalarRootV1),
    CallableModule(SealedNormalCallableModuleSourceV1),
}

impl SealedNormalSourcePlanV1 {
    pub(crate) fn has_parser_postpass(&self) -> bool {
        match self {
            Self::ScalarRoot(SealedNormalScalarRootV1::Script(source)) => {
                source.has_parser_postpass()
            }
            Self::ScalarRoot(SealedNormalScalarRootV1::Main0(source)) => {
                source.input.has_parser_postpass()
            }
            Self::CallableModule(source) => source.input.has_parser_postpass(),
        }
    }
}
