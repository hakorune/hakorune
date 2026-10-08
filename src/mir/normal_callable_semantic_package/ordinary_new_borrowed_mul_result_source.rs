//! Return classification retains the original operation product, not a type guess.
use super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1;
use super::super::super::borrowed_formal_uses::BorrowedMulSideV1;
use super::*;

pub(super) fn capture_mul_return(
    input: ResolvedFunctionLoweringInputV1<'_>,
    draft: &BorrowedFormalUsesDraftV1,
    site: &SourceExprSiteV1,
) -> Result<Option<Rc<BorrowedMulSourceV1>>, String> {
    draft
        .mul_source_at(input, &OwnedExprSiteV1::new(input.owner(), site.clone()))
        .map(|source| source.map(Rc::clone))
        .map_err(|_| freeze("borrowed-result/mul-source-identity"))
}

/// The result cohort owns the retained operation products. Its executable
/// ingress and raw call inventory remain the original owners of those facts.
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_result) fn verify_mul_returns(
    owner: FunctionOwnerIdV1,
    proof: &BorrowedI64ResultSourceV1,
    source: &PreparedBorrowedFormalIngressV1,
) -> Result<(), String> {
    let draft = source
        .definitions
        .get(&owner)
        .ok_or_else(|| freeze("borrowed-result/mul-entry-owner"))?;
    let expected: BTreeSet<_> = draft
        .uses
        .iter()
        .filter_map(|row| match &row.kind {
            BorrowedFormalUseDraftKindV1::MulOperand { binary, .. }
                if proof.returns.contains(binary) =>
            {
                Some(binary.clone())
            }
            _ => None,
        })
        .collect();
    let observed: BTreeSet<_> = proof
        .multiplications
        .iter()
        .map(|product| product.binary().clone())
        .collect();
    if observed != expected || observed.len() != proof.multiplications.len() {
        return Err(freeze("borrowed-result/mul-return-coverage"));
    }
    for product in &proof.multiplications {
        if proof.class != BorrowedResultClassV1::I64
            || product.binary().owner() != owner
            || !proof.returns.contains(product.binary())
            || !product.corroborates_retained_rows(draft)
        {
            return Err(freeze("borrowed-result/mul-source-identity"));
        }
        for side in [BorrowedMulSideV1::Left, BorrowedMulSideV1::Right] {
            let Some(child) = product.integer_call_source(side) else {
                continue;
            };
            let observation = source
                .source_incoming
                .static_observations()
                .get(child.call_site())
                .ok_or_else(|| freeze("borrowed-result/mul-static-source-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            let mut rows = source
                .source_incoming
                .exact_rows()
                .filter(|row| &row.call == child.call_site());
            let row = rows
                .next()
                .ok_or_else(|| freeze("borrowed-result/mul-static-incoming-missing"))?;
            if rows.next().is_some()
                || child.call_site().owner() != owner
                || !child.is_zeroarg_i64_v1()
                || !Rc::ptr_eq(observation, child)
                || !matches!(&row.source, BorrowedIncomingSourceV1::Static(original)
                    if Rc::ptr_eq(original, child))
                || row.callee != child.callee_owner()
                || row.source.target() != child.target()
                || row.source.target_batch_slot() != child.target_batch_slot()
                || !row.arguments.is_empty()
            {
                return Err(freeze("borrowed-result/mul-static-source-identity"));
            }
        }
    }
    Ok(())
}
