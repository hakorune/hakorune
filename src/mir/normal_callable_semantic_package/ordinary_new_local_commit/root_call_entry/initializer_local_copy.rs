//! Shared completed Local-copy receipt validation for every initializer purpose.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Same completed Local Copy law for scalar/Text and nonowning get purposes.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn validate_initializer_local_copy_v1(
        &self,
        context: CallValidationContextV1<'_>,
        function: &MirFunction,
        finishing: Option<&super::super::physical_boundary::FinishedBindings>,
        value: ValueId,
        local: ValueId,
        copy: Option<&Binding>,
    ) -> Result<Option<Binding>, String> {
        if let Some(copy) = copy {
            if !matches!(copy.1, MirInstruction::Copy { dst, src } if dst == local && src == value)
            {
                return Err(freeze("initializer-expression/local-copy-receipt"));
            }
            let actual = match (finishing, context) {
                (Some(boundary), CallValidationContextV1::Finalized(_)) => {
                    boundary.relation_binding(function, copy)?
                }
                (Some(boundary), CallValidationContextV1::Lowering(_)) => {
                    if boundary.captured_definition(local)? != copy {
                        return Err(freeze("initializer-expression/local-copy-capture"));
                    }
                    None
                }
                (None, _) => Some(copy.clone()),
            };
            if let Some((block, instruction)) = &actual {
                self.with_function_geometry_v1(function, |geometry| {
                    if !geometry.matches_exact_definition(*block, instruction, true) {
                        return Err(freeze("initializer-expression/local-definition-identity"));
                    }
                    Ok(())
                })?;
            }
            return Ok(actual);
        }
        Ok(None)
    }
}
