//! Resolver-backed source Recipe row for one qualified static MethodCall.
//!
//! This row intentionally carries no target symbol or ABI. The installed Main
//! qualified-receiver relation co-seals that information later.

use crate::mir::resolved_semantics::{
    ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1, VerifiedResolvedMethodCallSourceV1,
};

use super::product::TrivialRepresentationV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedTrivialQualifiedMethodCallV1 {
    site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver: Box<str>,
    selector: Box<str>,
    arguments: Box<[SourceExprSiteV1]>,
    result: TrivialRepresentationV1,
}

impl VerifiedTrivialQualifiedMethodCallV1 {
    pub(crate) fn seal(call: &VerifiedResolvedMethodCallSourceV1) -> Result<Self, &'static str> {
        if call.receiver() != ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            || call.qualified_receiver_identity().is_none()
            || call.result_site() != call.site()
            || call.arguments().len() != call.arity() as usize
        {
            return Err("[freeze:contract][trivial_profile/qualified-method-source]");
        }
        let arguments = call
            .arguments()
            .iter()
            .enumerate()
            .map(|(index, argument)| {
                (argument.ordinal() == index as u32)
                    .then_some(argument.site().clone())
                    .ok_or("[freeze:contract][trivial_profile/qualified-method-ordinal]")
            })
            .collect::<Result<Vec<_>, _>>()?;
        let receiver = call
            .qualified_receiver_identity()
            .expect("qualified receiver checked above")
            .source_name()
            .to_owned()
            .into_boxed_str();
        if receiver.is_empty() || call.selector().is_empty() {
            return Err("[freeze:contract][trivial_profile/qualified-method-name]");
        }
        Ok(Self {
            site: call.site().clone(),
            receiver_site: call.receiver_site().clone(),
            receiver,
            selector: call.selector().to_owned().into_boxed_str(),
            arguments: arguments.into_boxed_slice(),
            result: TrivialRepresentationV1::InlineI64,
        })
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) fn receiver(&self) -> &str {
        &self.receiver
    }

    pub(crate) fn selector(&self) -> &str {
        &self.selector
    }

    pub(crate) fn arguments(&self) -> &[SourceExprSiteV1] {
        &self.arguments
    }

    pub(crate) const fn result(&self) -> TrivialRepresentationV1 {
        self.result
    }
}
