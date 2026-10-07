//! Final operand closure over original entry/Copy/Forwarded loans.
//! These temporary sets corroborate one publication; they issue no authority.
#[path = "borrowed_call_uses_null_compare.rs"]
mod null_compare;
#[path = "borrowed_call_uses_object_field.rs"]
mod object_field;
#[path = "borrowed_call_uses_closure.rs"]
mod closure;
use object_field::path_dominates;
use super::*;
use crate::mir::compiler::normal_default_pipeline::published_backend_view::physical_program::PublishedLifecyclePhysicalFunctionV1;
use crate::mir::normal_callable_semantic_package::{
    BorrowedFormalActualSourceV1, FinalizedRootSourceHandoffV1, PreparedBorrowedFormalActualV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, MirFunction};

type Coordinate = (BasicBlockId, usize);
type Binding = (BasicBlockId, MirInstruction);

#[derive(Default)]
struct FunctionUses {
    roots: BTreeMap<ValueId, BindingRefV1>,
    copies: BTreeMap<ValueId, (Binding, Option<Coordinate>)>,
    arguments: BTreeMap<(BasicBlockId, usize, usize), BindingRefV1>,
    /// Checked-compare view admissions per formal, proved by the original
    /// source draft. Each admitted operand use must be observed through
    /// exactly one physical instruction coordinate and operand side per use.
    /// Shared carriers retain distinct source uses in both scans.
    compare_admissions: BTreeMap<BindingRefV1, usize>,
    /// Expected occurrences lent by the independently checked original Compare.
    compare_coordinates: Option<BTreeMap<BindingRefV1, BTreeSet<(Coordinate, usize)>>>,
    /// Dominated `+` operand admissions per formal under the same source
    /// draft; each admits exactly one distinct operand value per scan.
    add_admissions: BTreeMap<BindingRefV1, usize>,
    /// Dominated `.set` element-value admissions per formal under the same
    /// source draft; each admits exactly one distinct operand value per scan.
    set_admissions: BTreeMap<BindingRefV1, usize>,
    /// Dominated `new`-argument admissions per formal under the same source
    /// draft; each admits exactly one distinct operand value per scan.
    ctor_admissions: BTreeMap<BindingRefV1, usize>,
    /// Null-equality operand admissions per formal under the same source
    /// draft; each admits exactly one distinct operand value per scan.
    null_admissions: BTreeMap<BindingRefV1, usize>,
    /// Dominated `formal.field` admissions per formal under the same
    /// source draft; each admits exactly one `object_field_get` row.
    field_admissions: BTreeMap<BindingRefV1, usize>,
    /// Sealed `formal -> canonical object` the physical
    /// `object_field_get` must name exactly.
    object_views: BTreeMap<BindingRefV1, hakorune_mir_defs::CanonicalObjectIdV1>,
}

#[derive(Default)]
struct Scan {
    definitions: BTreeMap<ValueId, usize>,
    coordinates: BTreeSet<Coordinate>,
    arguments: BTreeSet<(BasicBlockId, usize, usize)>,
    /// Exact operand occurrences, including both sides of shared carriers.
    compare_uses: BTreeMap<BindingRefV1, BTreeSet<(Coordinate, usize)>>,
    /// Distinct operand values serving each formal's dominated `+` view.
    add_uses: BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
    /// Distinct element values serving each formal's dominated `.set` view.
    set_uses: BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
    /// Distinct argument values serving each formal's dominated `new`-arg
    /// view.
    ctor_uses: BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
    /// Distinct operand values serving each formal's null-equality view.
    null_uses: BTreeMap<BindingRefV1, BTreeSet<ValueId>>,
    /// Physical `object_field_get` rows serving each formal's guarded read.
    field_uses: BTreeMap<BindingRefV1, usize>,
}

/// The sole physical projections a lent view may take: an untracked operand
/// of an admitted instruction kind whose single definition is a `Copy`
/// straight from a tracked carrier, plus the checked-compare sites that
/// dominate later `+` uses per formal.
#[derive(Default)]
struct ViewScan {
    views: BTreeMap<ValueId, BindingRefV1>,
    compares: BTreeMap<BindingRefV1, Vec<Coordinate>>,
    /// Values whose single definition is the exact `ConstValue::Null`
    /// producer; only a `borrowed_null_compare` sibling may reference them.
    null_consts: BTreeSet<ValueId>,
    /// Non-null successor blocks per formal, anchored by each borrowed
    /// null compare's consuming `if` branch.
    nonnull_successors: BTreeMap<BindingRefV1, Vec<BasicBlockId>>,
}

#[derive(Default)]
pub(super) struct BorrowedCallUses {
    functions: BTreeMap<String, FunctionUses>,
}

impl BorrowedCallUses {
    pub(super) fn entry(
        &mut self,
        source: &FinalizedRootSourceHandoffV1,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(function.signature.name.clone())
            .or_default();
        for (_, formal, value) in source.borrowed_ordinary_entry_values_v1(owner)?.iter() {
            if function
                .params
                .iter()
                .filter(|parameter| *parameter == value)
                .count()
                != 1
            {
                return Err(fault("borrowed-use/entry-parameter"));
            }
            if let Some(previous) = state.roots.insert(*value, *formal) {
                if previous != *formal {
                    return Err(fault("borrowed-use/conflicting-root"));
                }
            }
        }
        source.with_borrowed_ordinary_alias_copies_v1(owner, function, |_, _, _, _, _, copies| {
            for original in copies {
                let coordinate =
                    source.borrowed_ordinary_alias_copy_coordinate_v1(owner, function, original)?;
                state.copy(original, coordinate)?;
            }
            Ok(())
        })?;
        for (_, formal, _) in source
            .borrowed_ordinary_compare_uses_v1(owner, function)?
            .iter()
        {
            *state.compare_admissions.entry(*formal).or_default() += 1;
        }
        let mut expected = BTreeMap::<_, BTreeSet<_>>::new();
        source.with_borrowed_ordinary_compares_v1(owner, function, |loan, _, binding| {
            let block = function.blocks.get(&binding.0)
                .ok_or_else(|| fault("borrowed-use/compare-block"))?;
            let mut rows = block.all_instructions().enumerate()
                .filter(|(_, row)| **row == binding.1);
            let (index, _) = rows.next().ok_or_else(|| fault("borrowed-use/compare-missing"))?;
            if rows.next().is_some() { return Err(fault("borrowed-use/compare-duplicate")); }
            for (formal, side) in loan.operand_formals() {
                if !expected.entry(formal).or_default().insert(((binding.0, index), side)) {
                    return Err(fault("borrowed-use/compare-source-duplicate"));
                }
            }
            Ok(())
        })?;
        state.compare_coordinates = Some(expected);
        for (_, formal, _) in source
            .borrowed_ordinary_add_uses_v1(owner, function)?
            .iter()
        {
            *state.add_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_array_element_uses_v1(owner, function)?
            .iter()
        {
            *state.set_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _, _) in source
            .borrowed_ordinary_new_argument_uses_v1(owner, function)?
            .iter()
        {
            *state.ctor_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_null_compare_uses_v1(owner, function)?
            .iter()
        {
            *state.null_admissions.entry(*formal).or_default() += 1;
        }
        for (_, formal, _) in source
            .borrowed_ordinary_field_read_uses_v1(owner, function)?
            .iter()
        {
            *state.field_admissions.entry(*formal).or_default() += 1;
        }
        state.object_views =
            source.borrowed_ordinary_formal_object_views_v1(owner, function)?;
        Ok(())
    }

    pub(super) fn call(
        &mut self,
        caller: &MirFunction,
        coordinate: Coordinate,
        actuals: &[PreparedBorrowedFormalActualV1],
        copies: &[(Binding, Coordinate)],
    ) -> Result<(), String> {
        let state = self
            .functions
            .entry(caller.signature.name.clone())
            .or_default();
        for (original, finished) in copies {
            state.copy(original, Some(*finished))?;
        }
        for actual in actuals {
            if let BorrowedFormalActualSourceV1::Forwarded { formal, .. } = actual.source {
                let key = (coordinate.0, coordinate.1, actual.ordinal as usize);
                if let Some(previous) = state.arguments.insert(key, formal) {
                    if previous != formal {
                        return Err(fault("borrowed-use/conflicting-argument"));
                    }
                }
            }
        }
        Ok(())
    }
}

impl FunctionUses {
    fn copy(&mut self, original: &Binding, coordinate: Option<Coordinate>) -> Result<(), String> {
        let MirInstruction::Copy { dst, .. } = original.1 else {
            return Err(fault("borrowed-use/copy-instruction"));
        };
        let proof = (original.clone(), coordinate);
        if let Some(previous) = self.copies.insert(dst, proof.clone()) {
            if previous != proof {
                return Err(fault("borrowed-use/conflicting-copy"));
            }
        }
        Ok(())
    }

    fn tracked(&self) -> Result<BTreeMap<ValueId, BindingRefV1>, String> {
        let mut tracked = self.roots.clone();
        let mut remaining: BTreeSet<_> = self.copies.keys().copied().collect();
        while !remaining.is_empty() {
            let mut progressed = false;
            for dst in remaining.clone() {
                let MirInstruction::Copy { src, .. } = self.copies[&dst].0 .1 else {
                    unreachable!()
                };
                let Some(formal) = tracked.get(&src).copied() else {
                    continue;
                };
                if tracked.insert(dst, formal).is_some() {
                    return Err(fault("borrowed-use/copy-redefinition"));
                }
                remaining.remove(&dst);
                progressed = true;
            }
            if !progressed {
                return Err(fault("borrowed-use/unrooted-copy"));
            }
        }
        Ok(tracked)
    }

    /// A lent view copy is the sole physical projection the envelopes lend:
    /// an untracked operand of a `Compare` or `BinOp{Add}` whose single
    /// definition is a `Copy` straight from a tracked carrier. The
    /// projection itself never joins the tracked set — a read through any
    /// other instruction stays a forbidden operand, and each `+` use must
    /// additionally sit inside the admitted compare's dominance cone.
    fn scan_views(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        indexed: &[(Coordinate, &MirInstruction)],
    ) -> Result<ViewScan, String> {
        let mut defs: BTreeMap<ValueId, &MirInstruction> = BTreeMap::new();
        for (_, instruction) in indexed {
            if let Some(dst) = instruction.dst_value() {
                if defs.insert(dst, instruction).is_some() {
                    return Err(fault("borrowed-use/view-definition-duplicate"));
                }
            }
        }
        let mut views = BTreeMap::new();
        for (_, instruction) in indexed {
            let operands: Vec<ValueId> = match instruction {
                MirInstruction::Compare { lhs, rhs, .. } => vec![*lhs, *rhs],
                MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                    ..
                } => vec![*lhs, *rhs],
                MirInstruction::ArrayElementWrite {
                    kind: crate::mir::ArrayElementWriteKind::Set,
                    value,
                    ..
                } => vec![*value],
                MirInstruction::Invoke {
                    operation: InvokeOperation::Call { call, .. },
                    ..
                }
                | MirInstruction::Call(call)
                    if matches!(call.callee, Callee::BirthConstructor { .. }) =>
                {
                    call.args.clone()
                }
                _ => continue,
            };
            for operand in operands {
                if tracked.contains_key(&operand) {
                    continue;
                }
                let Some(MirInstruction::Copy { src, .. }) = defs.get(&operand) else {
                    continue;
                };
                let Some(formal) = tracked.get(src) else {
                    continue;
                };
                if let Some(previous) = views.insert(operand, *formal) {
                    if previous != *formal {
                        return Err(fault("borrowed-use/view-conflict"));
                    }
                }
            }
        }
        let mut compares: BTreeMap<BindingRefV1, Vec<Coordinate>> = BTreeMap::new();
        let mut null_consts = BTreeSet::new();
        for (coordinate, instruction) in indexed {
            null_compare::observe_null_const(instruction, &mut null_consts);
            match instruction {
                // Only the admitted checked compare (`>` + Normal-Integer
                // proof) anchors `+`/`.set`/`new` dominance cones; a null
                // equality supplies non-null only, never the Integer lane.
                MirInstruction::Compare {
                    op: crate::mir::CompareOp::Gt,
                    lhs,
                    rhs,
                    ..
                } => {
                    for operand in [lhs, rhs] {
                        if let Some(formal) = tracked.get(operand).or(views.get(operand)) {
                            compares.entry(*formal).or_default().push(*coordinate);
                        }
                    }
                }
                _ => {}
            }
        }
        let nonnull_successors =
            object_field::collect_nonnull_successors(tracked, &views, &null_consts, indexed);
        Ok(ViewScan {
            views,
            compares,
            null_consts,
            nonnull_successors,
        })
    }

    fn instruction(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        views: &ViewScan,
        coordinate: Coordinate,
        instruction: &MirInstruction,
        definitions: &mut Scan,
        dominates: &dyn Fn(BasicBlockId, BasicBlockId) -> bool,
    ) -> Result<(), String> {
        if !definitions.coordinates.insert(coordinate) {
            return Err(fault("borrowed-use/coordinate-duplicate"));
        }
        if let Some(dst) = instruction
            .dst_value()
            .filter(|dst| tracked.contains_key(dst))
        {
            let Some((original, Some(expected))) = self.copies.get(&dst) else {
                return Err(fault("borrowed-use/root-redefinition"));
            };
            if coordinate != *expected || instruction != &original.1 {
                return Err(fault("borrowed-use/copy-definition"));
            }
            *definitions.definitions.entry(dst).or_default() += 1;
        }
        match instruction {
            MirInstruction::Copy { dst, src } if tracked.contains_key(src) => {
                let view = views.views.get(dst) == tracked.get(src);
                if !view
                    && !self.copies.get(dst).is_some_and(|(original, expected)| {
                        original.1 == *instruction && *expected == Some(coordinate)
                    })
                {
                    return Err(fault("borrowed-use/unproved-copy"));
                }
            }
            MirInstruction::Call(call)
                if matches!(call.callee, Callee::BirthConstructor { .. }) =>
            {
                // Bare BirthConstructor calls carry the same dominated
                // `new`-argument rule as the invoke form.
                let mut forbidden = false;
                call.callee
                    .for_each_value_operand(|value| {
                        forbidden |=
                            tracked.contains_key(&value) || views.views.contains_key(&value);
                    });
                if forbidden {
                    return Err(fault("borrowed-use/callee-operand"));
                }
                for value in &call.args {
                    if let Some(formal) = tracked.get(value).or(views.views.get(value)) {
                        let dominated = views.compares.get(formal).is_some_and(|sites| {
                            sites.iter().any(|&(block, index)| {
                                dominates(block, coordinate.0)
                                    && (block != coordinate.0 || index < coordinate.1)
                            })
                        });
                        if !dominated {
                            return Err(fault("borrowed-use/undominated-view"));
                        }
                        definitions
                            .ctor_uses
                            .entry(*formal)
                            .or_default()
                            .insert(*value);
                    }
                }
            }
            MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, .. },
                fault_frame,
                ..
            } => {
                let mut forbidden = tracked.contains_key(fault_frame)
                    || views.views.contains_key(fault_frame);
                call.callee.for_each_value_operand(|value| {
                    forbidden |=
                        tracked.contains_key(&value) || views.views.contains_key(&value);
                });
                if forbidden {
                    return Err(fault("borrowed-use/callee-or-fault-frame"));
                }
                if matches!(call.callee, Callee::BirthConstructor { .. }) {
                    // A `new <Child>(...)` argument may read the same lent
                    // view only inside the admitted compare's dominance cone;
                    // the receiver and sibling scalar actuals never carry a
                    // borrowed lane.
                    for value in &call.args {
                        if let Some(formal) =
                            tracked.get(value).or(views.views.get(value))
                        {
                            let dominated = views.compares.get(formal).is_some_and(|sites| {
                                sites.iter().any(|&(block, index)| {
                                    dominates(block, coordinate.0)
                                        && (block != coordinate.0 || index < coordinate.1)
                                })
                            });
                            if !dominated {
                                return Err(fault("borrowed-use/undominated-view"));
                            }
                            definitions
                                .ctor_uses
                                .entry(*formal)
                                .or_default()
                                .insert(*value);
                        }
                    }
                } else {
                    for (ordinal, value) in call.args.iter().enumerate() {
                        match (
                            tracked.get(value),
                            self.arguments.get(&(coordinate.0, coordinate.1, ordinal)),
                        ) {
                            (Some(formal), Some(expected)) if formal == expected => {
                                if self
                                    .copies
                                    .get(value)
                                    .is_some_and(|(_, coordinate)| coordinate.is_none())
                                {
                                    return Err(fault("borrowed-use/omitted-copy-argument"));
                                }
                                definitions
                                    .arguments
                                    .insert((coordinate.0, coordinate.1, ordinal));
                            }
                            (None, None) => {}
                            _ => return Err(fault("borrowed-use/argument")),
                        }
                    }
                    if self.arguments.keys().any(|(block, index, ordinal)| {
                        (*block, *index) == coordinate && *ordinal >= call.args.len()
                    }) {
                        return Err(fault("borrowed-use/argument-missing"));
                    }
                }
            }
            MirInstruction::Compare {
                op: crate::mir::CompareOp::Gt,
                lhs,
                rhs,
                ..
            } => {
                // A tracked operand is legal only through an admitted
                // checked-compare view; the draft count closes coverage.
                for (side, operand) in [lhs, rhs].into_iter().enumerate() {
                    if let Some(formal) = tracked.get(operand).or(views.views.get(operand)) {
                        definitions
                            .compare_uses
                            .entry(*formal)
                            .or_default()
                            .insert((coordinate, side));
                    }
                }
            }
            MirInstruction::Compare {
                op: crate::mir::CompareOp::Eq,
                lhs,
                rhs,
                ..
            } => {
                // A lent view may answer `==` only against the exact
                // `ConstValue::Null` producer — the sibling, never a
                // forged zero or another borrowed carrier.
                null_compare::observe_operand(
                    tracked,
                    &views.views,
                    &views.null_consts,
                    lhs,
                    rhs,
                    &mut definitions.null_uses,
                )?;
            }
            MirInstruction::Compare { lhs, rhs, .. } => {
                // `!=`, `<`, `<=`, `>=` and general tagged equality stay
                // outside the admitted envelopes.
                if [lhs, rhs].iter().any(|operand| {
                    tracked.contains_key(operand) || views.views.contains_key(operand)
                }) {
                    return Err(fault("borrowed-use/forbidden-operand"));
                }
            }
            MirInstruction::ObjectFieldGet { base, field, .. } => {
                // A borrowed formal's guarded field read: dominated by its
                // admitted null compare's non-null successor and naming
                // exactly the sealed object view — untracked bases fall
                // through to their own lanes.
                object_field::observe_operand(
                    tracked,
                    views,
                    &self.object_views,
                    base,
                    field,
                    coordinate,
                    &mut definitions.field_uses,
                    dominates,
                )?;
            }
            MirInstruction::BinOp {
                op: crate::mir::BinaryOp::Add,
                lhs,
                rhs,
                ..
            } => {
                // A `+` operand may read the same lent view only inside the
                // admitted compare's dominance cone — a same-block use must
                // also order after the compare's site check.
                for operand in [lhs, rhs] {
                    if let Some(formal) = tracked.get(operand).or(views.views.get(operand)) {
                        let dominated = views.compares.get(formal).is_some_and(|sites| {
                            sites.iter().any(|&(block, index)| {
                                dominates(block, coordinate.0)
                                    && (block != coordinate.0 || index < coordinate.1)
                            })
                        });
                        if !dominated {
                            return Err(fault("borrowed-use/undominated-view"));
                        }
                        definitions
                            .add_uses
                            .entry(*formal)
                            .or_default()
                            .insert(*operand);
                    }
                }
            }
            MirInstruction::ArrayElementWrite {
                kind: crate::mir::ArrayElementWriteKind::Set,
                receiver,
                index,
                value,
                ..
            } => {
                // A `.set` element value may read the same lent view only
                // inside the admitted compare's dominance cone; the receiver
                // and index operands never carry a borrowed lane.
                if tracked.contains_key(receiver)
                    || views.views.contains_key(receiver)
                    || index.is_some_and(|index| {
                        tracked.contains_key(&index) || views.views.contains_key(&index)
                    })
                {
                    return Err(fault("borrowed-use/forbidden-operand"));
                }
                if let Some(formal) = tracked.get(value).or(views.views.get(value)) {
                    let dominated = views.compares.get(formal).is_some_and(|sites| {
                        sites.iter().any(|&(block, index)| {
                            dominates(block, coordinate.0)
                                && (block != coordinate.0 || index < coordinate.1)
                        })
                    });
                    if !dominated {
                        return Err(fault("borrowed-use/undominated-view"));
                    }
                    definitions
                        .set_uses
                        .entry(*formal)
                        .or_default()
                        .insert(*value);
                }
            }
            _ if instruction.used_values().iter().any(|value| {
                tracked.contains_key(value) || views.views.contains_key(value)
            }) =>
            {
                return Err(fault("borrowed-use/forbidden-operand"));
            }
            _ => {}
        }
        Ok(())
    }

    fn definitions(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        params: &[ValueId],
        definitions: &Scan,
    ) -> Result<(), String> {
        if definitions.arguments != self.arguments.keys().copied().collect() {
            return Err(fault("borrowed-use/argument-coverage"));
        }
        let compare_uses: BTreeMap<_, _> = definitions
            .compare_uses
            .iter()
            .map(|(formal, values)| (*formal, values.len()))
            .collect();
        if compare_uses != self.compare_admissions
            || self.compare_coordinates.as_ref().is_some_and(|expected|
                *expected != definitions.compare_uses)
        {
            return Err(fault("borrowed-use/compare-coverage"));
        }
        let add_uses: BTreeMap<_, _> = definitions
            .add_uses
            .iter()
            .map(|(formal, values)| (*formal, values.len()))
            .collect();
        if add_uses != self.add_admissions {
            return Err(fault("borrowed-use/add-coverage"));
        }
        let set_uses: BTreeMap<_, _> = definitions
            .set_uses
            .iter()
            .map(|(formal, values)| (*formal, values.len()))
            .collect();
        if set_uses != self.set_admissions {
            return Err(fault("borrowed-use/set-coverage"));
        }
        let ctor_uses: BTreeMap<_, _> = definitions
            .ctor_uses
            .iter()
            .map(|(formal, values)| (*formal, values.len()))
            .collect();
        if ctor_uses != self.ctor_admissions {
            return Err(fault("borrowed-use/ctor-coverage"));
        }
        null_compare::coverage(&definitions.null_uses, &self.null_admissions)?;
        object_field::coverage(&definitions.field_uses, &self.field_admissions)?;
        for value in tracked.keys() {
            let parameter_count = params
                .iter()
                .filter(|parameter| *parameter == value)
                .count();
            let definition_count = definitions.definitions.get(value).copied().unwrap_or(0);
            let expected = match self.copies.get(value) {
                Some((_, coordinate)) => (0, usize::from(coordinate.is_some())),
                None => (1, 0),
            };
            if (parameter_count, definition_count) != expected {
                return Err(fault("borrowed-use/definition-coverage"));
            }
        }
        Ok(())
    }
}

#[cfg(test)]
#[path = "borrowed_call_uses_tests.rs"]
mod tests;

// Lend the original-source proof only through the production projection checker.
// Tests cannot substitute the expected name or tracked bindings.
#[cfg(test)]
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn projection_fixture(
) -> (
    MirFunction,
    impl for<'a> Fn(&[PublishedLifecyclePhysicalFunctionV1<'a>]) -> Result<(), String>,
) {
    let (state, function, _) = tests::fixture();
    let tracked = state.tracked().unwrap();
    let name = function.signature.name.clone();
    (function, move |rows| {
        state.verify_published(&tracked, &name, rows)
    })
}
