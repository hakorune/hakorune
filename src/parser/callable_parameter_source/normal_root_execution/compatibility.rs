use super::super::normal_source_plan_surface::ParserNormalSourcePlanSurfaceUnavailableV1;
use super::model::ParserNormalRootExecutionSourceDispositionV1;

#[derive(Debug)]
pub(crate) struct ParserNormalRootExecutionCompatibilityClosureV1 {
    _seal: ParserNormalRootExecutionCompatibilityClosureSealV1,
}

#[derive(Debug)]
struct ParserNormalRootExecutionCompatibilityClosureSealV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ParserNormalRootExecutionCompatibilityRejectV1 {
    ReadySourceCannotBecomeCompatibility,
    SourceFailureCannotBecomeCompatibility,
    IncompleteCannotBecomeCompatibility,
    IntegrityInvalidCannotBecomeCompatibility,
}

impl ParserNormalRootExecutionCompatibilityClosureV1 {
    pub(crate) fn consume_once(
        source: ParserNormalRootExecutionSourceDispositionV1,
    ) -> Result<
        Self,
        (
            ParserNormalRootExecutionSourceDispositionV1,
            ParserNormalRootExecutionCompatibilityRejectV1,
        ),
    > {
        match source {
            ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(
                ParserNormalSourcePlanSurfaceUnavailableV1::PostpassNotSourceBacked,
            ) => Ok(Self {
                _seal: ParserNormalRootExecutionCompatibilityClosureSealV1,
            }),
            source @ ParserNormalRootExecutionSourceDispositionV1::Ready(_) => Err((
                source,
                ParserNormalRootExecutionCompatibilityRejectV1::ReadySourceCannotBecomeCompatibility,
            )),
            source @ ParserNormalRootExecutionSourceDispositionV1::SourceAuthorityUnavailable(_) => {
                Err((
                    source,
                    ParserNormalRootExecutionCompatibilityRejectV1::SourceFailureCannotBecomeCompatibility,
                ))
            }
            source @ ParserNormalRootExecutionSourceDispositionV1::Incomplete(_) => Err((
                source,
                ParserNormalRootExecutionCompatibilityRejectV1::IncompleteCannotBecomeCompatibility,
            )),
            source @ ParserNormalRootExecutionSourceDispositionV1::IntegrityInvalid(_) => Err((
                source,
                ParserNormalRootExecutionCompatibilityRejectV1::IntegrityInvalidCannotBecomeCompatibility,
            )),
        }
    }
}
