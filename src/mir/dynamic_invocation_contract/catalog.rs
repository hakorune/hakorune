use crate::mir::builder::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::source_call_target::{
    VerifiedSourceCallTargetCatalogV1, VerifiedSourceCallTargetV1,
};

use super::VerifiedDynamicInvocationEnvelopeRefV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum DynamicInvocationEnvelopeIssueV1 {
    ForeignTargetCatalog,
    TargetOutsideDeclarationCatalog,
    MalformedDynamicTarget,
}

/// Complete route-neutral target catalog with an exact semantic envelope view
/// over every Dynamic arm.
///
/// No second row map is stored. This owner retains the original catalog and
/// derives its Dynamic rows one-to-one, so duplicate or missing envelope rows
/// are structurally impossible after successful issue.
#[derive(Debug)]
pub(crate) struct VerifiedDynamicInvocationEnvelopeCatalogV1<'catalog> {
    targets: VerifiedSourceCallTargetCatalogV1<'catalog>,
    dynamic_len: usize,
}

impl<'catalog> VerifiedDynamicInvocationEnvelopeCatalogV1<'catalog> {
    pub(crate) fn issue(
        targets: VerifiedSourceCallTargetCatalogV1<'catalog>,
        declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Result<Self, DynamicInvocationEnvelopeIssueV1> {
        if !targets.is_branded_by(declarations) {
            return Err(DynamicInvocationEnvelopeIssueV1::ForeignTargetCatalog);
        }

        let mut dynamic_len = 0usize;
        for ((caller, site), target) in targets.all_rows() {
            if declarations.declaration(caller).is_none() {
                return Err(DynamicInvocationEnvelopeIssueV1::TargetOutsideDeclarationCatalog);
            }
            let VerifiedSourceCallTargetV1::DynamicMember(target) = target else {
                continue;
            };
            if target.call_site() != site
                || target.result_site() != site
                || target.arguments().len() != target.dispatch().arity() as usize
                || target
                    .arguments()
                    .iter()
                    .enumerate()
                    .any(|(ordinal, argument)| argument.ordinal() as usize != ordinal)
            {
                return Err(DynamicInvocationEnvelopeIssueV1::MalformedDynamicTarget);
            }
            dynamic_len += 1;
        }

        Ok(Self {
            targets,
            dynamic_len,
        })
    }

    pub(crate) fn targets(&self) -> &VerifiedSourceCallTargetCatalogV1<'catalog> {
        &self.targets
    }

    pub(crate) const fn len(&self) -> usize {
        self.dynamic_len
    }

    pub(crate) const fn is_empty(&self) -> bool {
        self.dynamic_len == 0
    }

    pub(crate) fn envelopes(
        &self,
    ) -> impl Iterator<Item = VerifiedDynamicInvocationEnvelopeRefV1<'_>> {
        self.targets
            .all_rows()
            .filter_map(|((caller, site), target)| match target {
                VerifiedSourceCallTargetV1::Static(_) => None,
                VerifiedSourceCallTargetV1::DynamicMember(target) => {
                    Some(VerifiedDynamicInvocationEnvelopeRefV1 {
                        caller,
                        site,
                        target,
                    })
                }
            })
    }
}
