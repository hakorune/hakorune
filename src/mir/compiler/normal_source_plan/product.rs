use crate::ast::ASTNode;

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

#[derive(Debug)]
pub(crate) struct SealedNormalMainSourceV1 {
    input: ASTNode,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: NormalMainSourceSealV1,
}

#[derive(Debug)]
struct NormalMainSourceSealV1;

impl SealedNormalMainSourceV1 {
    pub(super) fn seal(
        input: ASTNode,
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

    pub(super) fn input(&self) -> &ASTNode {
        &self.input
    }

    pub(super) fn main_box(&self) -> &NormalTopLevelSiteV1 {
        &self.main_box
    }

    pub(super) fn main_method(&self) -> &NormalMainMethodSiteV1 {
        &self.main_method
    }
}
