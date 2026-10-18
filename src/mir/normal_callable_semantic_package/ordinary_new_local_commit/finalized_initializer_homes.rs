//! Exact installed Text Homes lent by their original initializer packet.
use super::*;

impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn finished_initializer_installs_v1(
        &self, module: &crate::mir::MirModule, function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        if !module.functions.get(&function.signature.name).is_some_and(|stored| std::ptr::eq(stored, function)) {
            return Err(freeze("finished-initializer-home/function-membership"));
        }
        if self.local_calls.values().flatten().any(|group|
            group.initializer().is_some_and(|packet| packet.frame().is_text_residence())) {
            self.validate_finalized_root_cleanup_v1(module)?;
        }
        let mut result = Vec::new();
        for (owner, groups) in &self.local_calls {
            if !groups.iter().any(|group| group.initializer().is_some_and(|packet| packet.frame().is_text_residence())) {
                continue;
            }
            self.ledger.with_finished_projection(*owner, |symbol, projection| {
                if symbol != function.signature.name { return Ok(()); }
                for group in groups {
                    let Some(packet) = group.initializer().filter(|packet| packet.frame().is_text_residence()) else { continue };
                    if packet.frame().source().owner() != *owner {
                        return Err(freeze("finished-initializer-home/source-owner"));
                    }
                    group.validate_recorded_in_context(CallValidationContextV1::Finalized(self), function, Some(projection))?;
                    self.ledger.validate_text_initializer_home_v1(packet, CallValidationContextV1::Finalized(self))?;
                    let original = packet.install_binding().ok_or_else(|| freeze("finished-initializer-home/install-missing"))?;
                    let finished = projection.binding(original.0, &original.1)?
                        .ok_or_else(|| freeze("finished-initializer-home/install-removed"))?;
                    let count = function.blocks.iter().flat_map(|(block, body)|
                        body.all_instructions().map(move |row| (*block, row)))
                        .filter(|(block, row)| *block == finished.0 && **row == finished.1).count();
                    if count != 1 || projection.recorded().iter().filter(|row| **row == finished).count() != 1
                        || result.contains(&finished) {
                        return Err(freeze("finished-initializer-home/install-membership"));
                    }
                    result.push(finished);
                }
                Ok(())
            })?;
        }
        Ok(result)
    }
}
