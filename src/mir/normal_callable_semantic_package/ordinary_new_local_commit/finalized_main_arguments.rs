//! Same root's wrapper argv product and exact finishing bindings.
use super::*;
impl FinalizedRootSourceHandoffV1 {
    pub(in crate::mir) fn main_arguments_bindings_v1(
        &self,
        module: &crate::mir::MirModule,
        function: &MirFunction,
    ) -> Result<Vec<Binding>, String> {
        self.ledger
            .with_finished_projection(self.owner, |symbol, projection| {
                let root = module
                    .functions
                    .get(symbol)
                    .ok_or_else(|| freeze("main-arguments/root-missing"))?;
                let bindings =
                    self.ledger
                        .main_arguments_projected_bindings_v1(root, |original| {
                            let mapped =
                                projection
                                    .binding(original.0, &original.1)?
                                    .ok_or_else(|| {
                                        freeze("main-arguments/mandatory-binding-removed")
                                    })?;
                            if !projection.recorded().contains(&mapped) {
                                return Err(freeze("main-arguments/mandatory-binding-unrecorded"));
                            }
                            Ok(mapped)
                        })?;
                if std::ptr::eq(root, function) {
                    Ok(bindings)
                } else {
                    Ok(Vec::new())
                }
            })
    }
}
