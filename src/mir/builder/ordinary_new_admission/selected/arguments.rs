//! Physical materialization of one prepared claim's argument rows. This is
//! the sole argument-value owner inside the selected `new` admission: it
//! validates the row/source relation, then emits the physical operand for
//! each kind — never a second issuer of argument meaning.
use super::freeze;
use crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use crate::mir::normal_callable_semantic_package::{
    OrdinaryNewClaimLedgerV1, OrdinaryNewTrivialArgumentKindV1, OrdinaryNewTrivialArgumentV1,
};
use crate::mir::{MirBuilder, MirInstruction, MirType, ValueId};

pub(super) fn materialize_arguments(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    arity: usize,
    rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<Vec<ValueId>, String> {
    validate_argument_rows_v1(site.owner(), site, arity, rows)?;
    rows.iter()
        .map(|row| match row.kind() {
            OrdinaryNewTrivialArgumentKindV1::Integer(value) => {
                crate::mir::builder::emission::constant::emit_integer(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Bool(value) => {
                crate::mir::builder::emission::constant::emit_bool(builder, *value)
            }
            OrdinaryNewTrivialArgumentKindV1::Null => {
                crate::mir::builder::emission::constant::emit_null(builder)
            }
            OrdinaryNewTrivialArgumentKindV1::Local { binding }
            | OrdinaryNewTrivialArgumentKindV1::Handle { binding }
            | OrdinaryNewTrivialArgumentKindV1::BoundValue { binding } => {
                let value = state
                    .value_for_exact_binding(site.owner(), *binding)
                    .map_err(|_| freeze("argument-binding-unavailable"))?;
                state
                    .observe_variable_site(row.site().node(), *binding, value)
                    .map_err(|_| freeze("argument-local-observation"))?;
                Ok(value)
            }
            OrdinaryNewTrivialArgumentKindV1::I64Field { object } => {
                // The read site is this row's exact `FieldAccess`
                // expression site; the staged issuer proof under it is
                // taken once and must resolve to the receiver's live value.
                let read_site = crate::mir::resolved_semantics::OwnedExprSiteV1::new(
                    site.owner(),
                    row.site().clone(),
                );
                let (base, field) = ledger.take_argument_field_read(
                    &read_site,
                    *object,
                    |binding, receiver_site| {
                        let value = state
                            .read_variable(receiver_site)
                            .map_err(|_| freeze("argument-field-observation"))?;
                        let expected = state
                            .value_for_exact_binding(site.owner(), binding)
                            .map_err(|_| freeze("argument-binding-unavailable"))?;
                        if value != expected {
                            return Err(freeze("argument-field-binding-drift"));
                        }
                        Ok(value)
                    },
                )?;
                let block = builder
                    .function_state
                    .current_block
                    .ok_or_else(|| freeze("no-block"))?;
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::ObjectFieldGet { dst, base, field })?;
                builder
                    .function_state
                    .type_ctx
                    .value_types
                    .insert(dst, MirType::Integer);
                ledger.record_argument_field_read(&read_site, block, dst, base, field)?;
                Ok(dst)
            }
        })
        .collect()
}

fn validate_argument_rows_v1(
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
    arity: usize,
    rows: &[OrdinaryNewTrivialArgumentV1],
) -> Result<(), String> {
    if rows.len() != arity {
        return Err(freeze("argument-row-count"));
    }
    let mut sites = std::collections::BTreeSet::new();
    for (index, row) in rows.iter().enumerate() {
        let ordinal = u32::try_from(index).map_err(|_| freeze("argument-ordinal-overflow"))?;
        if row.owner() != owner
            || row.new_site() != new_site
            || row.ordinal() != ordinal
            || !sites.insert(row.site().clone())
        {
            return Err(freeze("argument-row-drift"));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::{
        FunctionOwnerIssuerV1, SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1,
    };

    fn site(segments: Vec<SourcePathSegmentV1>) -> crate::mir::resolved_semantics::OwnedExprSiteV1 {
        let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
        crate::mir::resolved_semantics::OwnedExprSiteV1::new(
            issuer.issue().expect("owner"),
            SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments)),
        )
    }

    fn integer_row(
        new_site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        ordinal: u32,
        argument: SourceExprSiteV1,
    ) -> OrdinaryNewTrivialArgumentV1 {
        OrdinaryNewTrivialArgumentV1::new(
            owner,
            new_site.clone(),
            ordinal,
            argument,
            OrdinaryNewTrivialArgumentKindV1::Integer(1),
        )
    }

    #[test]
    fn selected_new_argument_rows_reject_malformed_source_relations() {
        let new_site = site(vec![SourcePathSegmentV1::Body(0)]);
        let argument = SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
            SourcePathSegmentV1::Body(0),
            SourcePathSegmentV1::Argument(0),
        ]));
        let valid = integer_row(&new_site, new_site.owner(), 0, argument.clone());
        assert!(
            validate_argument_rows_v1(new_site.owner(), &new_site, 1, &[])
                .unwrap_err()
                .contains("argument-row-count")
        );
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            2,
            &[
                valid.clone(),
                integer_row(&new_site, new_site.owner(), 1, argument)
            ],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        let foreign_site = site(vec![SourcePathSegmentV1::Body(1)]);
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                foreign_site.owner(),
                0,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
        assert!(validate_argument_rows_v1(
            new_site.owner(),
            &new_site,
            1,
            &[integer_row(
                &new_site,
                new_site.owner(),
                1,
                valid.site().clone()
            )],
        )
        .unwrap_err()
        .contains("argument-row-drift"));
    }
}
