//! Borrowed/null equality closure: the `==` operand may read a tracked
//! carrier or lent view only against the exact `ConstValue::Null`
//! producer, and each admitted use must observe exactly the operand
//! values the source draft admitted — no forged zero, no other carrier,
//! no coverage drift.
use super::*;
use crate::mir::ConstValue;

/// Records the exact `ConstValue::Null` producers a borrowed/null
/// equality row may name as its sibling.
pub(super) fn observe_null_const(
    instruction: &MirInstruction,
    null_consts: &mut BTreeSet<ValueId>,
) {
    if let MirInstruction::Const {
        dst,
        value: ConstValue::Null,
    } = instruction
    {
        null_consts.insert(*dst);
    }
}

/// A lent view may answer `==` only against the exact `ConstValue::Null`
/// producer — the sibling, never a forged zero or another borrowed
/// carrier. Each observed operand joins the formal's null-equality
/// coverage set.
pub(super) fn observe_operand(
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &BTreeMap<ValueId, BindingRefV1>,
    null_consts: &BTreeSet<ValueId>,
    lhs: &ValueId,
    rhs: &ValueId,
    uses: &mut BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
) -> Result<(), String> {
    for operand in [lhs, rhs] {
        if let Some(formal) = tracked.get(operand).or(views.get(operand)) {
            let sibling = if operand == lhs { rhs } else { lhs };
            if !null_consts.contains(sibling) {
                return Err(fault("borrowed-use/forbidden-operand"));
            }
            uses.entry(*formal).or_default().insert(*operand);
        }
    }
    Ok(())
}

/// Per-admission coverage: the observed distinct operand values must
/// equal the source draft's admission count per formal.
pub(super) fn coverage(
    uses: &BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
    admissions: &BTreeMap<BindingRefV1, usize>,
) -> Result<(), String> {
    let observed: BTreeMap<_, _> = uses
        .iter()
        .map(|(formal, values)| (*formal, values.len()))
        .collect();
    if observed != *admissions {
        return Err(fault("borrowed-use/null-coverage"));
    }
    Ok(())
}
