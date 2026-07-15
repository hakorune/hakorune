//! Co-sealed pre-Builder profile row for one exact trivial direct call.

use crate::mir::canonical_direct_call_contract::{
    VerifiedDirectCallEffectV1, VerifiedTrivialDirectCallTargetV1,
};
use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, SourceExprSiteV1, VerifiedCallableIndexV1, VerifiedResolvedFunctionV1,
};

use super::error::TrivialProfileContractErrorV1;
use super::product::TrivialRepresentationV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialDirectCallV1 {
    site: SourceExprSiteV1,
    target: VerifiedTrivialDirectCallTargetV1,
    arguments: Box<[SourceExprSiteV1]>,
    result: TrivialRepresentationV1,
    effect: VerifiedDirectCallEffectV1,
}

impl VerifiedTrivialDirectCallV1 {
    pub(super) fn seal(
        owner: FunctionOwnerIdV1,
        site: SourceExprSiteV1,
        source_name: &str,
        arguments: Vec<SourceExprSiteV1>,
        resolved: &VerifiedResolvedFunctionV1,
        index: &VerifiedCallableIndexV1,
    ) -> Result<Self, TrivialProfileContractErrorV1> {
        let target = resolved.direct_call_target(&site).ok_or_else(|| {
            TrivialProfileContractErrorV1::MissingDirectCallResolution { site: site.clone() }
        })?;
        let arity = u32::try_from(arguments.len()).map_err(|_| {
            TrivialProfileContractErrorV1::DirectCallHeaderMismatch { site: site.clone() }
        })?;
        let header = index
            .resolve_free_static_source_call(source_name, arity)
            .map_err(
                |_| TrivialProfileContractErrorV1::DirectCallHeaderMismatch { site: site.clone() },
            )?;
        if index.len() != 1
            || resolved.owner() != owner
            || header.callable() != target.callable()
            || header.callable().owner() != owner
            || header.signature().arity() != arguments.len()
        {
            return Err(TrivialProfileContractErrorV1::DirectCallTargetMismatch { site });
        }
        Ok(Self {
            site,
            target: VerifiedTrivialDirectCallTargetV1::from_header(header),
            arguments: arguments.into_boxed_slice(),
            result: TrivialRepresentationV1::InlineI64,
            effect: VerifiedDirectCallEffectV1::ConservativeBarrier,
        })
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &VerifiedTrivialDirectCallTargetV1 {
        &self.target
    }

    pub(crate) fn arguments(&self) -> &[SourceExprSiteV1] {
        &self.arguments
    }

    pub(crate) const fn result(&self) -> TrivialRepresentationV1 {
        self.result
    }

    pub(crate) const fn effect(&self) -> VerifiedDirectCallEffectV1 {
        self.effect
    }
}
