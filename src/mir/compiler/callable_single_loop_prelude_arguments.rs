//! AST-free Prelude argument evidence for the first callable Loop profile.
//!
//! The issuer uses the resolver's exact argument sites and lexical BindingRefs.
//! It intentionally admits only direct variable arguments backed by caller
//! parameters with the existing exact `i64` representation.

#![cfg(test)]

use crate::ast::ASTNode;
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingRefV1, ExprChildRoleV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    SourceExprSiteV1, VerifiedCallableHeaderV1,
};

use super::callable_single_loop_recipe_coseal::VerifiedCallablePreludeV1;
use super::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PreludeArgumentRejectV1 {
    CallKindUnsupported,
    CountMismatch,
    SourceNavigation,
    NotVariable,
    MissingBinding,
    Upvar,
    ForeignBinding,
    BindingKindUnsupported,
    AbiUnsupported,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallablePreludeArgumentV1 {
    ordinal: u32,
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedCallablePreludeArgumentV1 {
    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedCallablePreludeArgumentListV1 {
    rows: Box<[VerifiedCallablePreludeArgumentV1]>,
}

impl VerifiedCallablePreludeArgumentListV1 {
    pub(crate) fn issue(
        input: ResolvedFunctionLoweringInputV1<'_>,
        prelude: &VerifiedCallablePreludeV1,
        header: &VerifiedCallableHeaderV1,
    ) -> Result<Self, PreludeArgumentRejectV1> {
        if !matches!(
            prelude.call().kind(),
            super::callable_single_loop_source_shapes::SourceCallKindV1::FreeStatic
        ) {
            return Err(PreludeArgumentRejectV1::CallKindUnsupported);
        }
        let owned_site = OwnedExprSiteV1::new(input.owner(), prelude.site().clone());
        let expression = input
            .source()
            .expr_at(&owned_site)
            .map_err(|_| PreludeArgumentRejectV1::SourceNavigation)?;
        let ASTNode::FunctionCall { arguments, .. } = expression.node() else {
            return Err(PreludeArgumentRejectV1::CallKindUnsupported);
        };
        if arguments.len() != prelude.call().argument_count() as usize
            || arguments.len() != header.signature().arity()
        {
            return Err(PreludeArgumentRejectV1::CountMismatch);
        }

        let mut rows = Vec::with_capacity(arguments.len());
        for ordinal in 0..arguments.len() {
            let ordinal =
                u32::try_from(ordinal).map_err(|_| PreludeArgumentRejectV1::CountMismatch)?;
            let argument = input
                .source()
                .child_expr_from_expr(&expression, ExprChildRoleV1::CallArgument(ordinal))
                .map_err(|_| PreludeArgumentRejectV1::SourceNavigation)?;
            if !matches!(argument.node(), ASTNode::Variable { .. }) {
                return Err(PreludeArgumentRejectV1::NotVariable);
            }
            let binding = match input.function().variable_ref(argument.site()) {
                Some(ResolvedLexicalRefV1::Local(binding)) => binding,
                Some(ResolvedLexicalRefV1::Upvar(_)) => return Err(PreludeArgumentRejectV1::Upvar),
                None => return Err(PreludeArgumentRejectV1::MissingBinding),
            };
            if binding.owner() != input.owner() {
                return Err(PreludeArgumentRejectV1::ForeignBinding);
            }
            let Some(record) = input.function().binding(binding) else {
                return Err(PreludeArgumentRejectV1::MissingBinding);
            };
            let BindingKindV1::Parameter { index } = record.kind() else {
                return Err(PreludeArgumentRejectV1::BindingKindUnsupported);
            };
            if header.signature().params().get(index as usize)
                != Some(&ExactTrivialScalarAbiV1::I64)
            {
                return Err(PreludeArgumentRejectV1::AbiUnsupported);
            }
            rows.push(VerifiedCallablePreludeArgumentV1 {
                ordinal,
                site: argument.site().clone(),
                binding,
                abi: ExactTrivialReturnAbiV1::I64,
            });
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }

    pub(crate) fn rows(&self) -> &[VerifiedCallablePreludeArgumentV1] {
        &self.rows
    }

    pub(crate) fn into_rows(self) -> Box<[VerifiedCallablePreludeArgumentV1]> {
        self.rows
    }
}
