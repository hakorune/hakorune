use crate::ast::ASTNode;
use crate::parser::{
    AdmittedSourcePlanBoundNormalCallableSourceV1, SourcePlanBoundNormalCallableSourceV1,
};

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
    source: ASTNode,
    identity: NormalSourceIdentityV1,
}

impl PreparedNormalSourcePlanInputV1 {
    pub(crate) fn new(source: ASTNode, display_name: impl Into<Box<str>>) -> Self {
        Self {
            source,
            identity: NormalSourceIdentityV1::new(display_name),
        }
    }

    pub(super) fn source(&self) -> &ASTNode {
        &self.source
    }

    pub(super) fn identity(&self) -> &NormalSourceIdentityV1 {
        &self.identity
    }

    pub(super) fn into_parts(self) -> (ASTNode, NormalSourceIdentityV1) {
        (self.source, self.identity)
    }

    pub(crate) fn has_parser_postpass(&self) -> bool {
        false
    }
}

/// Source owner retained after one source-family decision.
///
/// `AstFixture` is the separately typed compatibility/test authority.
/// `ParserBound` is the production owner consumed from the sole parser
/// surface.  The latter can lend syntax after policy admission, but cannot
/// re-enter the AST inventory classifier.
#[derive(Debug)]
pub(super) enum NormalSourcePlanOwnerV1 {
    AstFixture(PreparedNormalSourcePlanInputV1),
    ParserBoundAdmitted {
        source: AdmittedSourcePlanBoundNormalCallableSourceV1,
        identity: NormalSourceIdentityV1,
    },
    ParserBoundRejected {
        source: SourcePlanBoundNormalCallableSourceV1,
        identity: NormalSourceIdentityV1,
    },
}

impl From<PreparedNormalSourcePlanInputV1> for NormalSourcePlanOwnerV1 {
    fn from(input: PreparedNormalSourcePlanInputV1) -> Self {
        Self::AstFixture(input)
    }
}

impl NormalSourcePlanOwnerV1 {
    pub(super) fn from_parser_bound_admitted(
        source: AdmittedSourcePlanBoundNormalCallableSourceV1,
    ) -> Self {
        let identity = NormalSourceIdentityV1::new(source.lineage().source_identity());
        Self::ParserBoundAdmitted { source, identity }
    }

    pub(super) fn from_parser_bound_rejected(
        source: SourcePlanBoundNormalCallableSourceV1,
    ) -> Self {
        let identity = NormalSourceIdentityV1::new(source.lineage().source_identity());
        Self::ParserBoundRejected { source, identity }
    }

    pub(super) fn source(&self) -> &ASTNode {
        match self {
            Self::AstFixture(input) => input.source(),
            Self::ParserBoundAdmitted { source, .. } => source.source_ast_after_policy(),
            Self::ParserBoundRejected { .. } => {
                unreachable!("rejected parser-bound owner has no AST capability")
            }
        }
    }

    pub(super) fn identity(&self) -> &NormalSourceIdentityV1 {
        match self {
            Self::AstFixture(input) => input.identity(),
            Self::ParserBoundAdmitted { identity, .. }
            | Self::ParserBoundRejected { identity, .. } => identity,
        }
    }

    pub(super) fn into_parts(self) -> (ASTNode, NormalSourceIdentityV1) {
        match self {
            Self::AstFixture(input) => input.into_parts(),
            Self::ParserBoundAdmitted { source, identity } => {
                (source.into_ast_after_policy(), identity)
            }
            Self::ParserBoundRejected { .. } => {
                unreachable!("rejected parser-bound owner has no AST extraction terminal")
            }
        }
    }

    pub(super) fn has_parser_postpass(&self) -> bool {
        matches!(
            self,
            Self::ParserBoundAdmitted { .. } | Self::ParserBoundRejected { .. }
        )
    }

    pub(super) fn parser_lineage(&self) -> Option<&crate::parser::NormalParserSourceLineageV1> {
        match self {
            Self::AstFixture(_) => None,
            Self::ParserBoundAdmitted { source, .. } => Some(source.lineage()),
            Self::ParserBoundRejected { source, .. } => Some(source.lineage()),
        }
    }

    pub(super) fn parser_invocation_witness(
        &self,
    ) -> Option<&crate::parser::ParserInvocationWitnessV1> {
        match self {
            Self::AstFixture(_) => None,
            Self::ParserBoundAdmitted { source, .. } => Some(source.invocation_witness()),
            Self::ParserBoundRejected { source, .. } => Some(source.invocation_witness()),
        }
    }

    pub(super) fn discard_after_source_plan_terminal(self) {
        match self {
            Self::AstFixture(input) => {
                let (source, identity) = input.into_parts();
                drop((source, identity));
            }
            Self::ParserBoundAdmitted { source, identity } => {
                source.discard_after_source_plan_terminal();
                drop(identity);
            }
            Self::ParserBoundRejected { source, identity } => {
                source.discard_after_source_plan_terminal();
                drop(identity);
            }
        }
    }
}

pub(super) trait NormalSourcePlanSyntaxV1 {
    fn source_ast(&self) -> &ASTNode;
}

impl NormalSourcePlanSyntaxV1 for PreparedNormalSourcePlanInputV1 {
    fn source_ast(&self) -> &ASTNode {
        self.source()
    }
}

impl NormalSourcePlanSyntaxV1 for NormalSourcePlanOwnerV1 {
    fn source_ast(&self) -> &ASTNode {
        self.source()
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
    input: NormalSourcePlanOwnerV1,
    statements: Box<[NormalTopLevelSiteV1]>,
    _seal: NormalScriptSourceSealV1,
}

#[derive(Debug)]
struct NormalScriptSourceSealV1;

impl SealedNormalScriptSourceV1 {
    pub(super) fn seal(
        input: impl Into<NormalSourcePlanOwnerV1>,
        statements: Box<[NormalTopLevelSiteV1]>,
    ) -> Self {
        Self {
            input: input.into(),
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

    pub(super) fn into_parts(self) -> (NormalSourcePlanOwnerV1, Box<[NormalTopLevelSiteV1]>) {
        (self.input, self.statements)
    }

    pub(super) fn source_ast(&self) -> &ASTNode {
        self.input.source()
    }

    pub(crate) fn has_parser_postpass(&self) -> bool {
        self.input.has_parser_postpass()
    }

    pub(crate) fn parser_lineage(&self) -> Option<&crate::parser::NormalParserSourceLineageV1> {
        self.input.parser_lineage()
    }
}

#[derive(Debug)]
pub(crate) struct SealedNormalMainSourceV1 {
    input: NormalSourcePlanOwnerV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: NormalMainSourceSealV1,
}

#[derive(Debug)]
struct NormalMainSourceSealV1;

impl SealedNormalMainSourceV1 {
    pub(super) fn seal(
        input: impl Into<NormalSourcePlanOwnerV1>,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
    ) -> Self {
        Self {
            input: input.into(),
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

    pub(super) fn input(&self) -> &NormalSourcePlanOwnerV1 {
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
    input: NormalSourcePlanOwnerV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    additional_callables: Box<[NormalAdditionalCallableSiteV1]>,
    _seal: NormalCallableModuleSourceSealV1,
}

#[derive(Debug)]
struct NormalCallableModuleSourceSealV1;

impl SealedNormalCallableModuleSourceV1 {
    pub(super) fn seal(
        input: impl Into<NormalSourcePlanOwnerV1>,
        main_box: NormalTopLevelSiteV1,
        main_method: NormalMainMethodSiteV1,
        additional_callables: Box<[NormalAdditionalCallableSiteV1]>,
    ) -> Self {
        Self {
            input: input.into(),
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

    pub(super) fn input(&self) -> &NormalSourcePlanOwnerV1 {
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
        NormalSourcePlanOwnerV1,
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
    pub(crate) fn discard_before_dispatch(self) {
        match self {
            Self::ScalarRoot(SealedNormalScalarRootV1::Script(source)) => {
                let SealedNormalScriptSourceV1 {
                    input,
                    statements,
                    _seal,
                } = source;
                input.discard_after_source_plan_terminal();
                drop((statements, _seal));
            }
            Self::ScalarRoot(SealedNormalScalarRootV1::Main0(source)) => {
                let SealedNormalMainSourceV1 {
                    input,
                    main_box,
                    main_method,
                    _seal,
                } = source;
                input.discard_after_source_plan_terminal();
                drop((main_box, main_method, _seal));
            }
            Self::CallableModule(source) => {
                let SealedNormalCallableModuleSourceV1 {
                    input,
                    main_box,
                    main_method,
                    additional_callables,
                    _seal,
                } = source;
                input.discard_after_source_plan_terminal();
                drop((main_box, main_method, additional_callables, _seal));
            }
        }
    }

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

    pub(crate) fn parser_lineage(&self) -> Option<&crate::parser::NormalParserSourceLineageV1> {
        match self {
            Self::ScalarRoot(SealedNormalScalarRootV1::Script(source)) => source.parser_lineage(),
            Self::ScalarRoot(SealedNormalScalarRootV1::Main0(source)) => {
                source.input.parser_lineage()
            }
            Self::CallableModule(source) => source.input.parser_lineage(),
        }
    }

    pub(crate) fn parser_invocation_witness(
        &self,
    ) -> Option<&crate::parser::callable_parameter_source::ParserInvocationWitnessV1> {
        match self {
            Self::ScalarRoot(SealedNormalScalarRootV1::Script(source)) => {
                source.input.parser_invocation_witness()
            }
            Self::ScalarRoot(SealedNormalScalarRootV1::Main0(source)) => {
                source.input.parser_invocation_witness()
            }
            Self::CallableModule(source) => source.input.parser_invocation_witness(),
        }
    }
}
