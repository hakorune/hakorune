//! Program-owned exact Main.main/0 source relation.
//!
//! The tests verify that the sealed sites identify the exact function
//! inside the immutable owned Program.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::CallableFunctionSyntaxViewV1;

use super::product::{NormalMainMethodSiteV1, NormalTopLevelSiteV1, SealedNormalMainSourceV1};

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
pub(super) struct NormalMainFunctionSourceViewV1<'src> {
    function: &'src ASTNode,
    main_statement_index: usize,
    method_key: &'src str,
}

impl<'src> NormalMainFunctionSourceViewV1<'src> {
    pub(super) fn function(&self) -> &'src ASTNode {
        self.function
    }

    pub(super) fn main_statement_index(&self) -> usize {
        self.main_statement_index
    }

    pub(super) fn method_key(&self) -> &'src str {
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

    pub(super) fn borrow_exact_function(&self) -> NormalMainFunctionSourceViewV1<'_> {
        borrow_exact_main_function_v1(
            self.source.input(),
            self.source.main_box(),
            self.source.main_method(),
        )
        .unwrap_or_else(|_| {
            unreachable!(
                "[normal-main-source/invariant] verified immutable Main relation disappeared"
            )
        })
    }


    #[cfg(test)]
    fn owned_program_for_test(&self) -> &ASTNode {
        self.source.input()
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

}

pub(super) fn verify_main_source_relation(
    source: &SealedNormalMainSourceV1,
) -> Result<(), NormalMainFunctionSourceErrorV1> {
    verify_main_source_parts(source.input(), source.main_box(), source.main_method())
}

pub(super) fn verify_main_source_parts(
    input: &ASTNode,
    main_box: &NormalTopLevelSiteV1,
    main_method: &NormalMainMethodSiteV1,
) -> Result<(), NormalMainFunctionSourceErrorV1> {
    let ASTNode::Program { statements, .. } = input else {
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
    let Some(function) = methods.get_declaration(main_method.method_key()) else {
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

pub(super) fn borrow_exact_main_function_v1<'src>(
    input: &'src ASTNode,
    main_box: &'src NormalTopLevelSiteV1,
    main_method: &'src NormalMainMethodSiteV1,
) -> Result<NormalMainFunctionSourceViewV1<'src>, NormalMainFunctionSourceErrorV1> {
    verify_main_source_parts(input, main_box, main_method)?;
    let function = locate_main_function(input, main_box, main_method)
        .ok_or(NormalMainFunctionSourceErrorV1::MainMethodMissing)?;
    Ok(NormalMainFunctionSourceViewV1 {
        function,
        main_statement_index: main_box.statement_index(),
        method_key: main_method.method_key(),
    })
}

fn locate_main_function<'src>(
    input: &'src ASTNode,
    main_box: &NormalTopLevelSiteV1,
    main_method: &NormalMainMethodSiteV1,
) -> Option<&'src ASTNode> {
    let ASTNode::Program { statements, .. } = input else {
        return None;
    };
    let ASTNode::BoxDeclaration { methods, .. } = statements.get(main_box.statement_index())?
    else {
        return None;
    };
    methods.get_declaration(main_method.method_key())
}

#[cfg(test)]
#[path = "main_source_tests.rs"]
mod tests;
