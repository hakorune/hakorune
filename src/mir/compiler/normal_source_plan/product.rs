use crate::ast::ASTNode;

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

    #[cfg(test)]
    pub(super) fn method_key(&self) -> &str {
        &self.method_key
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

    #[cfg(test)]
    pub(super) fn additional_callables(&self) -> &[NormalAdditionalCallableSiteV1] {
        &self.additional_callables
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
