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
                let mut definitions = function
                    .blocks
                    .iter()
                    .flat_map(|(id, body)| body.all_instructions().map(move |row| (*id, row)))
                    .filter(|(_, row)| row.dst_value() == Some(local));
                if function.params.contains(&local)
                    || definitions.next() != Some((*block, instruction))
                    || definitions.next().is_some()
                {
                    return Err(freeze("initializer-expression/local-definition-identity"));
                }
                if !super::super::physical_boundary::check_binding(
                    function,
                    None,
                    *block,
                    instruction,
                )? {
                    return Err(freeze("initializer-expression/local-copy-missing"));
                }
            }
            return Ok(actual);
        }
        Ok(None)
    }
}
