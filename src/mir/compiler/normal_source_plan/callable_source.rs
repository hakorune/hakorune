//! One-Program normal Main-plus-top-level-helper source owner.
//!
//! This layer consumes source-family evidence only. It issues no callable
//! identities, resolves no bodies, and opens no Builder/backend state.

use crate::mir::resolved_semantics::{
    CallableModuleHeaderSyntaxErrorV1, SourceCallableDeclarationSiteV1,
    VerifiedCallableHeaderSourceUnitV1,
};

use super::main_source::{verify_main_source_parts, NormalMainFunctionSourceErrorV1};
use super::product::{
    NormalAdditionalCallableSiteV1, NormalMainMethodSiteV1, NormalSourceIdentityV1,
    NormalTopLevelSiteV1, SealedNormalCallableModuleSourceV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalCallableSourceStageV1 {
    MainRelation,
    HelperSiteProjection,
    HeaderSourceUnit,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum NormalCallableSourceErrorV1 {
    MainRelation(NormalMainFunctionSourceErrorV1),
    MainMethodHelperUnsupported { method_key: Box<str> },
    HeaderSourceUnit(CallableModuleHeaderSyntaxErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalCallableSourceUnitV1 {
    source: VerifiedCallableHeaderSourceUnitV1,
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    _seal: VerifiedNormalCallableSourceUnitSealV1,
}

#[derive(Debug)]
struct VerifiedNormalCallableSourceUnitSealV1;

impl VerifiedNormalCallableSourceUnitV1 {
    pub(crate) fn helper_sites(&self) -> &[SourceCallableDeclarationSiteV1] {
        self.source.declaration_sites()
    }

    pub(crate) fn helper_source(&self) -> &VerifiedCallableHeaderSourceUnitV1 {
        &self.source
    }

    pub(crate) fn main_statement_index(&self) -> usize {
        self.main_box.statement_index()
    }

    pub(crate) fn main_method_key(&self) -> &str {
        self.main_method.method_key()
    }

    pub(crate) fn source_identity(&self) -> &str {
        self.identity.display_name()
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalCallableSourceV1 {
    owner: SealedNormalCallableModuleSourceV1,
    stage: NormalCallableSourceStageV1,
    error: NormalCallableSourceErrorV1,
}

impl RejectedNormalCallableSourceV1 {
    pub(crate) const fn stage(&self) -> NormalCallableSourceStageV1 {
        self.stage
    }

    pub(crate) fn error(&self) -> &NormalCallableSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

pub(super) fn prepare(
    owner: SealedNormalCallableModuleSourceV1,
) -> Result<VerifiedNormalCallableSourceUnitV1, RejectedNormalCallableSourceV1> {
    if let Err(error) =
        verify_main_source_parts(owner.input(), owner.main_box(), owner.main_method())
    {
        return Err(reject(
            owner,
            NormalCallableSourceStageV1::MainRelation,
            NormalCallableSourceErrorV1::MainRelation(error),
        ));
    }

    let mut helper_sites = Vec::with_capacity(owner.additional_callables().len());
    for site in owner.additional_callables() {
        match site {
            NormalAdditionalCallableSiteV1::TopLevel(site) => {
                let helper = match SourceCallableDeclarationSiteV1::from_statement_index(
                    site.statement_index(),
                ) {
                    Ok(helper) => helper,
                    Err(error) => {
                        return Err(reject(
                            owner,
                            NormalCallableSourceStageV1::HelperSiteProjection,
                            NormalCallableSourceErrorV1::HeaderSourceUnit(error),
                        ))
                    }
                };
                helper_sites.push(helper);
            }
            NormalAdditionalCallableSiteV1::MainMethod(site) => {
                let method_key: Box<str> = site.method_key().into();
                return Err(reject(
                    owner,
                    NormalCallableSourceStageV1::HelperSiteProjection,
                    NormalCallableSourceErrorV1::MainMethodHelperUnsupported { method_key },
                ));
            }
        }
    }
    let helper_sites = helper_sites.into_boxed_slice();
    if let Err(error) = VerifiedCallableHeaderSourceUnitV1::validate_exact_sites(
        owner.input().source(),
        &helper_sites,
    ) {
        return Err(reject(
            owner,
            NormalCallableSourceStageV1::HeaderSourceUnit,
            NormalCallableSourceErrorV1::HeaderSourceUnit(error),
        ));
    }

    let (input, main_box, main_method, _) = owner.into_parts();
    let (program, identity) = input.into_parts();
    let source = VerifiedCallableHeaderSourceUnitV1::seal_exact_sites(program, helper_sites)
        .unwrap_or_else(|error| {
            unreachable!(
                "[normal-callable-source/invariant] prevalidated immutable sites drifted: {error:?}"
            )
        });
    Ok(VerifiedNormalCallableSourceUnitV1 {
        source,
        identity,
        main_box,
        main_method,
        _seal: VerifiedNormalCallableSourceUnitSealV1,
    })
}

fn reject(
    owner: SealedNormalCallableModuleSourceV1,
    stage: NormalCallableSourceStageV1,
    error: NormalCallableSourceErrorV1,
) -> RejectedNormalCallableSourceV1 {
    RejectedNormalCallableSourceV1 {
        owner,
        stage,
        error,
    }
}

#[cfg(test)]
#[path = "callable_source_tests.rs"]
mod tests;
