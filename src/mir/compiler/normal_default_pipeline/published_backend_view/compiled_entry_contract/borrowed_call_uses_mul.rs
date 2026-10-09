//! Exact borrowed Mul operand closure, selected by the original source loan.
use super::*;
use crate::mir::resolved_semantics::OwnedExprSiteV1;

pub(super) struct MulOperandUse {
    pub(super) formal: BindingRefV1,
    pub(super) operand: ValueId,
    pub(super) copy: Option<Coordinate>,
    pub(super) guard: Coordinate,
}

pub(super) fn entry(
    state: &mut FunctionUses,
    source: &FinalizedRootSourceHandoffV1,
    owner: FunctionOwnerIdV1,
    function: &MirFunction,
    compare_sites: &BTreeMap<OwnedExprSiteV1, Coordinate>,
) -> Result<(), String> {
    source.with_borrowed_ordinary_muls_v1(owner, function, |loan, binding, copies| {
        let coordinate = exact_coordinate(function, binding)?;
        let MirInstruction::BinOp {
            lhs,
            rhs,
            op: crate::mir::BinaryOp::Mul,
            ..
        } = &binding.1
        else {
            return Err(fault("borrowed-use/mul-binding"));
        };
        let operands = [*lhs, *rhs];
        let views = loan.checked_views();
        if views.len() != 2 {
            return Err(fault("borrowed-use/mul-nonview-source-unproved"));
        }
        for (side, formal, guard_site) in views {
            let guard = *compare_sites
                .get(&guard_site)
                .ok_or_else(|| fault("borrowed-use/mul-guard-missing"))?;
            if !state.compare_coordinates.as_ref().is_some_and(|expected| {
                expected
                    .get(&formal)
                    .is_some_and(|uses| uses.iter().any(|(site, _)| *site == guard))
            }) {
                return Err(fault("borrowed-use/mul-guard-formal"));
            }
            let copy = copies[side]
                .as_ref()
                .map(|binding| exact_coordinate(function, binding))
                .transpose()?;
            if state
                .mul_expected
                .insert(
                    (coordinate, side),
                    MulOperandUse {
                        formal,
                        operand: operands[side],
                        copy,
                        guard,
                    },
                )
                .is_some()
            {
                return Err(fault("borrowed-use/mul-source-duplicate"));
            }
        }
        Ok(())
    })
}

pub(super) fn observe(
    state: &FunctionUses,
    tracked: &BTreeMap<ValueId, BindingRefV1>,
    views: &ViewScan,
    coordinate: Coordinate,
    operands: [ValueId; 2],
    scan: &mut Scan,
    dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
) -> Result<(), String> {
    for (side, operand) in operands.into_iter().enumerate() {
        let expected = state.mul_expected.get(&(coordinate, side));
        let formal = tracked.get(&operand).or(views.views.get(&operand));
        match (expected, formal) {
            (None, None) => continue,
            (Some(expected), Some(formal))
                if expected.formal == *formal
                    && expected.operand == operand
                    && expected.copy == views.copy_coordinates.get(&operand).copied()
                    && dominates(expected.guard.0, coordinate.0)
                    && (expected.guard.0 != coordinate.0 || expected.guard.1 < coordinate.1) =>
            {
                scan.mul_uses.insert((coordinate, side));
            }
            _ => return Err(fault("borrowed-use/mul-operand")),
        }
    }
    Ok(())
}

pub(super) fn coverage(state: &FunctionUses, scan: &Scan) -> Result<(), String> {
    if scan.mul_uses != state.mul_expected.keys().copied().collect() {
        return Err(fault("borrowed-use/mul-coverage"));
    }
    Ok(())
}

fn exact_coordinate(function: &MirFunction, binding: &Binding) -> Result<Coordinate, String> {
    let block = function
        .blocks
        .get(&binding.0)
        .ok_or_else(|| fault("borrowed-use/mul-block"))?;
    let mut rows = block
        .all_instructions()
        .enumerate()
        .filter(|(_, row)| *row == &binding.1);
    let (index, _) = rows
        .next()
        .ok_or_else(|| fault("borrowed-use/mul-missing"))?;
    if rows.next().is_some() {
        return Err(fault("borrowed-use/mul-duplicate"));
    }
    Ok((binding.0, index))
}
