//! Exact source-ID loans for selected-normal instance constructors.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::parser::ConstructorSourceIdV1;

use super::install::NormalCallableSemanticPackagePortV1;

impl NormalCallableSemanticPackagePortV1<'_> {
    pub(crate) fn with_instance_constructor_lowering_input<R>(
        &mut self,
        source_id: &ConstructorSourceIdV1,
        callback: impl for<'loan> FnOnce(ResolvedFunctionLoweringInputV1<'loan>) -> R,
    ) -> Result<R, String> {
        let row = self
            .installed
            .instance_constructors()
            .rows()
            .iter()
            .find(|row| row.source_id().same_as(source_id))
            .ok_or_else(|| {
                "[freeze:contract][mir/instance-constructor-semantic/missing-row]".to_owned()
            })?;
        let input = row.lowering_input(self.installed.source_ast())?;
        Ok(callback(input))
    }
}
