//! Sole ordinary-New source claim preparation, before Home-prefix verification.
//! Selected callables and the original AppMain loan share one result draft/fixpoint.
use super::*;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::BorrowedAppMainSourceLoanV1;

pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn prepare_source_claims(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    app_main: Option<&BorrowedAppMainSourceLoanV1<'_>>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    parameter_contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<
    (
        field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        field_write_claim::OwnedFieldResidencesV1,
        result_class_claim::OrdinaryNewResultClassClaimsV1,
        BTreeMap<hakorune_mir_defs::CanonicalObjectIdV1, Option<Box<[OwnedFieldChildV1]>>>,
    ),
    OrdinaryNewCoSealIssueV1,
> {
    // Field-write claims and callable result-class claims compose
    // without any Home evidence — seal both before the verified walk so
    // that walk can mint claim-faithful `me.m(..)` observations in the
    // same sweep. Constructor (birth) rows join through the
    // program-source loan, where `lowering_input` needs the program.
    let mut field_write_draft = field_write_claim::OrdinaryNewFieldWriteClaimDraftV1::new();
    let mut result_class_draft = result_class_claim::OrdinaryNewResultClassClaimDraftV1::new();
    for declaration in batch.declarations() {
        let main = app_main.filter(|main| main.batch_slot() == declaration.batch_slot());
        let source_key = if let Some(main) = main {
            if !main.matches_function(
                declaration.owner(),
                declaration.batch_slot(),
                main.catalog_key(),
            ) {
                return Err(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing);
            }
            Some(main.catalog_key().clone())
        } else {
            selected
                .keys()
                .filter_map(|selected_key| {
                    let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                        return None;
                    };
                    (selected.batch_slot(selected_key) == Some(declaration.batch_slot()))
                        .then(|| key.clone())
                })
                .next()
        };
        // Field-write claims belong to instance boxes; the result-class
        // claim admits any cataloged key — a static-box sibling returning
        // `return new <class>` names the class for the direct-call
        // handle-result edge too.
        let owner_box = source_key
            .as_ref()
            .filter(|key| key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
            .map(|key| key.owner());
        batch
            .with_lowering_input(declaration.batch_slot(), |input| {
                field_write_draft.observe_function(
                    input.function(),
                    input.body_shape(),
                    owner_box,
                    false,
                );
                if let Some(key) = &source_key {
                    if main.is_some_and(|main| {
                        !main.matches_function(input.owner(), declaration.batch_slot(), key)
                    }) {
                        return Err(OrdinaryNewCoSealIssueV1::AppMainIdentityMissing);
                    }
                    result_class_draft.observe_function(input, key, declaration.batch_slot());
                }
                Ok(())
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
    }
    batch
        .with_normal_program_source_loan(|loan| -> Result<(), OrdinaryNewCoSealIssueV1> {
            for row in instance_constructors.rows() {
                let input = row
                    .lowering_input(loan.program())
                    .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
                field_write_draft.observe_function(
                    input.function(),
                    input.body_shape(),
                    Some(row.box_name()),
                    true,
                );
            }
            Ok(())
        })
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)??;
    let (field_write_claims, field_residences) =
        field_write_draft.finish(batch.ordinary_box_coverage());
    // A provider `new` mints no `local`-bound claim, so its child's
    // owned-field inventory would never reach the ledger through a claim.
    // Seal it here against the same residences; unproven children stay
    // explicitly unadmitted for every cleanup consumer.
    let mut owned_field_children = BTreeMap::new();
    seal_provider_owned_children_v1(
        batch,
        instance_constructors,
        &field_residences,
        &mut owned_field_children,
    )?;
    let mut callable_result_classes = result_class_draft.finish(
        batch.ordinary_box_coverage(),
        batch,
        selected,
        app_main,
        &field_write_claims,
        parameter_contracts,
    );
    result_class_claim::attach_source_child_relations_v1(
        &mut callable_result_classes,
        batch,
        selected,
        instance_constructors,
        &field_write_claims,
        parameter_contracts,
    );
    Ok((
        field_write_claims,
        field_residences,
        callable_result_classes,
        owned_field_children,
    ))
}
