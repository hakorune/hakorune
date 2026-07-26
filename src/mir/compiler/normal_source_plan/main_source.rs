//! Program-owned exact Main.main/0 source relation.
//!
//! This module consumes the Main0 product emitted by the sole source-family
//! classifier. It verifies only that the sealed sites still identify the same
//! function inside the immutable owned Program.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::CallableFunctionSyntaxViewV1;

use super::product::{
    NormalMainMethodSiteV1, NormalTopLevelSiteV1, PreparedNormalSourcePlanInputV1,
    SealedNormalMainSourceV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalMainFunctionSourceErrorV1 {
    RootNotProgram,
    MainStatementMissing,
    MainStatementDrift,
    MainMethodMissing,
    MainMethodShapeDrift,
    MainMethodNameDrift,
    MainMethodStaticDrift,
    MainMethodArityDrift,
}

#[derive(Debug)]
pub(crate) struct NormalMainFunctionSourceViewV1<'src> {
    function: &'src ASTNode,
    main_statement_index: usize,
    method_key: &'src str,
}

impl<'src> NormalMainFunctionSourceViewV1<'src> {
    pub(crate) fn function(&self) -> &'src ASTNode {
        self.function
    }

    pub(crate) fn main_statement_index(&self) -> usize {
        self.main_statement_index
    }

    pub(crate) fn method_key(&self) -> &'src str {
        self.method_key
    }
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainFunctionSourceUnitV1 {
    source: SealedNormalMainSourceV1,
    _seal: VerifiedNormalMainFunctionSourceUnitSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainFunctionSourceUnitSealV1;

impl VerifiedNormalMainFunctionSourceUnitV1 {
    pub(super) fn seal(source: SealedNormalMainSourceV1) -> Self {
        Self {
            source,
            _seal: VerifiedNormalMainFunctionSourceUnitSealV1,
        }
    }

    pub(crate) fn borrow_exact_function(&self) -> NormalMainFunctionSourceViewV1<'_> {
        let Some(function) = locate_main_function(
            self.source.input(),
            self.source.main_box(),
            self.source.main_method(),
        ) else {
            unreachable!(
                "[normal-main-source/invariant] verified immutable Main relation disappeared"
            );
        };
        NormalMainFunctionSourceViewV1 {
            function,
            main_statement_index: self.source.main_box().statement_index(),
            method_key: self.source.main_method().method_key(),
        }
    }

    pub(in crate::mir) fn into_source(self) -> SealedNormalMainSourceV1 {
        self.source
    }

    #[cfg(test)]
    fn owned_program_for_test(&self) -> &ASTNode {
        self.source.input().source()
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainFunctionSourceV1 {
    owner: SealedNormalMainSourceV1,
    error: NormalMainFunctionSourceErrorV1,
}

impl RejectedNormalMainFunctionSourceV1 {
    pub(super) fn new(
        owner: SealedNormalMainSourceV1,
        error: NormalMainFunctionSourceErrorV1,
    ) -> Self {
        Self { owner, error }
    }

    pub(crate) fn error(&self) -> &NormalMainFunctionSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (SealedNormalMainSourceV1, NormalMainFunctionSourceErrorV1) {
        (self.owner, self.error)
    }
}

pub(super) fn verify_main_source_relation(
    source: &SealedNormalMainSourceV1,
) -> Result<(), NormalMainFunctionSourceErrorV1> {
    verify_main_source_parts(source.input(), source.main_box(), source.main_method())
}

pub(super) fn verify_main_source_parts(
    input: &PreparedNormalSourcePlanInputV1,
    main_box: &NormalTopLevelSiteV1,
    main_method: &NormalMainMethodSiteV1,
) -> Result<(), NormalMainFunctionSourceErrorV1> {
    let ASTNode::Program { statements, .. } = input.source() else {
        return Err(NormalMainFunctionSourceErrorV1::RootNotProgram);
    };
    let Some(statement) = statements.get(main_box.statement_index()) else {
        return Err(NormalMainFunctionSourceErrorV1::MainStatementMissing);
    };
    let ASTNode::BoxDeclaration {
        name,
        methods,
        is_static,
        ..
    } = statement
    else {
        return Err(NormalMainFunctionSourceErrorV1::MainStatementDrift);
    };
    if name != "Main"
        || !is_static
        || main_method.main_statement_index() != main_box.statement_index()
    {
        return Err(NormalMainFunctionSourceErrorV1::MainStatementDrift);
    }
    if main_method.method_key() != "main" {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodNameDrift);
    }
    let Some(function) = methods.get(main_method.method_key()) else {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodMissing);
    };
    let Some(function_view) = CallableFunctionSyntaxViewV1::from_function_ast(function) else {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodShapeDrift);
    };
    let header = function_view.header();
    if header.name() != main_method.method_key() {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodNameDrift);
    }
    if !header.is_static() || header.is_static() != main_method.is_static() {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodStaticDrift);
    }
    if !header.params().is_empty() || header.params().len() != main_method.arity() {
        return Err(NormalMainFunctionSourceErrorV1::MainMethodArityDrift);
    }
    Ok(())
}

fn locate_main_function<'src>(
    input: &'src PreparedNormalSourcePlanInputV1,
    main_box: &NormalTopLevelSiteV1,
    main_method: &NormalMainMethodSiteV1,
) -> Option<&'src ASTNode> {
    let ASTNode::Program { statements, .. } = input.source() else {
        return None;
    };
    let ASTNode::BoxDeclaration { methods, .. } = statements.get(main_box.statement_index())?
    else {
        return None;
    };
    methods.get(main_method.method_key())
}

#[cfg(test)]
#[path = "main_source_tests.rs"]
mod tests;
