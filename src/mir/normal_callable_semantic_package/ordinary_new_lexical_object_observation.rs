//! Original indexed caller observations corroborate slot result and route only.
use super::*;
use crate::mir::resolved_semantics::home_new_prefix::{
    LocalCallObservationV1, LocalCallResultClassV1,
};

impl OrdinaryNewClaimLedgerV1 {
    pub(super) fn checked_object_slot_observation_v1(
        &self,
        source: &LexicalInstanceCallSourceTargetV1,
        result: Option<InvokeCallResultKind>,
    ) -> Result<Option<bool>, String> {
        let Some(completion) = self.completion_index.get(&source.call_site().owner()) else {
            return Ok(None);
        };
        let completion = completion.as_ref().map_err(|issue| {
            format!("{}: {issue:?}", freeze("lexical-object/caller-completion"))
        })?;
        if completion.owner() != source.call_site().owner() {
            return Err(freeze("lexical-object/caller-completion-owner"));
        }
        let Some(flow) = completion.cleanup().root_flow() else {
            return Ok(None);
        };
        corroborate_original_object_observations_v1(source.call_site(), result, flow.local_calls())
            .map(Some)
    }
}

fn corroborate_original_object_observations_v1(
    site: &OwnedExprSiteV1,
    result: Option<InvokeCallResultKind>,
    observations: &[LocalCallObservationV1],
) -> Result<bool, String> {
    let mut matching = observations.iter().filter(|row| row.site() == site);
    let Some(observation) = matching.next() else {
        return Ok(false);
    };
    if matching.next().is_some() || observation.owner() != site.owner() {
        return Err(freeze("lexical-object/caller-observation-identity"));
    }
    if !matches!(
        observation.result(),
        LocalCallResultClassV1::Handle | LocalCallResultClassV1::Nullable
    ) || result.is_some_and(|kind| {
        !matches!(
            (observation.result(), kind),
            (LocalCallResultClassV1::Handle, InvokeCallResultKind::Handle)
                | (
                    LocalCallResultClassV1::Nullable,
                    InvokeCallResultKind::NullableHandle
                )
        )
    }) {
        return Err(freeze("lexical-object/result-observation"));
    }
    Ok(true)
}

#[cfg(test)]
#[path = "ordinary_new_lexical_object_observation_tests.rs"]
mod tests;
