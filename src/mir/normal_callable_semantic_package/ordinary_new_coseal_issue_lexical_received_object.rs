//! Source-backed input/result checks for lexical calls that produce an object.
use super::*;
use crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::result_class_claim::OrdinaryNewResultClassClaimsV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::{
    lexical_instance_call::{PendingBorrowedFormalActualsV1, PreparedBorrowedFormalIngressV1},
    OrdinaryNewFieldWriteClaimsV1,
};

pub(super) fn handles(request: &BorrowedCallActualRequestV1<'_>) -> bool {
    matches!(
        request,
        BorrowedCallActualRequestV1::ReceivedObjectArguments(_, _)
            | BorrowedCallActualRequestV1::ReceivedFieldAliasObjectArguments { .. }
    )
}

pub(super) fn project(
    request: BorrowedCallActualRequestV1<'_>,
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    source: &Result<PreparedBorrowedFormalIngressV1, String>,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    pending: &PendingBorrowedFormalActualsV1,
    targets: &PreparedLexicalInstanceCallSourceTargetsV1,
    candidates: &[OrdinaryNewCandidate],
    classes: &OrdinaryNewResultClassClaimsV1,
    fields: &OrdinaryNewFieldWriteClaimsV1,
    site: &OwnedExprSiteV1,
) -> Result<Option<BorrowedCallArgumentsV1>, String> {
    match request {
        BorrowedCallActualRequestV1::ReceivedObjectArguments(destination, result) => {
            crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::received_producer_arguments_v1(
                batch,
                selected,
                source,
                contracts,
                pending,
                targets,
                candidates,
                classes,
                site,
                destination,
                result,
                fields,
                None,
            )
        }
        BorrowedCallActualRequestV1::ReceivedFieldAliasObjectArguments {
            receiver,
            receiver_class,
            alias_path,
            destination,
            result,
        } => crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::received_field_alias_producer_arguments_v1(
            batch,
            selected,
            source,
            contracts,
            pending,
            targets,
            candidates,
            classes,
            site,
            destination,
            result,
            fields,
            receiver,
            &receiver_class,
            alias_path,
        ),
        _ => unreachable!("received object request selected by handles"),
    }
}
