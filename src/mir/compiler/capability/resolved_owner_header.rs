//! First-family owner/header seal before the selected plan is consumed.
//!
//! This product is deliberately independent from the exact-i64 callable
//! header.  The general first-family profile admits zero-arity functions and
//! does not own an exact scalar ABI contract.

use crate::mir::resolved_semantics::{
    CallableHeaderSyntaxViewV1, CanonicalCallableSymbolV1, FunctionOwnerIdV1,
};

use super::{CanonicalFirstFamilyPlanBrandV1, CanonicalFirstFamilyPlanV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedOwnerHeaderFamilyV1 {
    TrivialBindingSsa,
    CurrentCanonicalAPlus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolvedOwnerHeaderSealErrorV1 {
    SourceRootMustBeFunction,
    SourceNameContainsPhysicalSeparator {
        name: Box<str>,
    },
    OwnerMismatch {
        input: FunctionOwnerIdV1,
        source: FunctionOwnerIdV1,
        resolved: FunctionOwnerIdV1,
    },
    ForeignPlan {
        expected_family: ResolvedOwnerHeaderFamilyV1,
        expected_owner: FunctionOwnerIdV1,
        actual_family: ResolvedOwnerHeaderFamilyV1,
        actual_owner: FunctionOwnerIdV1,
    },
}

/// Owned, construction-only snapshot of one selected first-family header.
///
/// No public constructor exists.  The parent capability module is the only
/// production owner allowed to combine the plan brand and resolved input.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedOwnerHeaderV1 {
    brand: CanonicalFirstFamilyPlanBrandV1,
    owner: FunctionOwnerIdV1,
    symbol: CanonicalCallableSymbolV1,
    arity: usize,
    _seal: ResolvedOwnerHeaderSealV1,
}

#[derive(Debug)]
struct ResolvedOwnerHeaderSealV1;

impl VerifiedResolvedOwnerHeaderV1 {
    pub(in crate::mir::compiler) fn seal(
        brand: CanonicalFirstFamilyPlanBrandV1,
        plan: &CanonicalFirstFamilyPlanV1<'_>,
    ) -> Result<Self, ResolvedOwnerHeaderSealErrorV1> {
        Self::seal_input(brand, plan.function_input())
    }

    pub(in crate::mir::compiler) fn seal_input(
        brand: CanonicalFirstFamilyPlanBrandV1,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Self, ResolvedOwnerHeaderSealErrorV1> {
        let owner = input.owner();
        let source_owner = input.source().owner();
        let resolved_owner = input.function().owner();
        if owner != source_owner || owner != resolved_owner {
            return Err(ResolvedOwnerHeaderSealErrorV1::OwnerMismatch {
                input: owner,
                source: source_owner,
                resolved: resolved_owner,
            });
        }
        let header = CallableHeaderSyntaxViewV1::from_function_ast(input.source().root())
            .ok_or(ResolvedOwnerHeaderSealErrorV1::SourceRootMustBeFunction)?;
        if header.name().contains('/') {
            return Err(
                ResolvedOwnerHeaderSealErrorV1::SourceNameContainsPhysicalSeparator {
                    name: header.name().into(),
                },
            );
        }
        let arity = header.params().len();
        Ok(Self {
            brand,
            owner,
            symbol: CanonicalCallableSymbolV1::from_name_arity(header.name(), arity),
            arity,
            _seal: ResolvedOwnerHeaderSealV1,
        })
    }

    pub(crate) const fn family(&self) -> ResolvedOwnerHeaderFamilyV1 {
        self.brand.family()
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn symbol(&self) -> &CanonicalCallableSymbolV1 {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) fn require_same_plan(
        &self,
        plan: &CanonicalFirstFamilyPlanV1<'_>,
    ) -> Result<(), ResolvedOwnerHeaderSealErrorV1> {
        let actual = plan.brand();
        let actual_owner = plan.function_input().owner();
        if self.brand == actual && self.owner == actual_owner {
            return Ok(());
        }
        Err(ResolvedOwnerHeaderSealErrorV1::ForeignPlan {
            expected_family: self.family(),
            expected_owner: self.owner,
            actual_family: actual.family(),
            actual_owner,
        })
    }
}
