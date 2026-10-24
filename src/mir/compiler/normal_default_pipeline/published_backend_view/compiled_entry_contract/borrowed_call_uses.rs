//! Final operand closure over original entry/Copy/Forwarded loans.
//! These temporary sets corroborate one publication; they issue no authority.
#[path = "borrowed_call_uses_formal_return.rs"]
mod formal_return;
#[cfg(test)]
pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) use formal_return::verify_original_formal_return;
#[path = "borrowed_call_uses_integer_return.rs"]
mod integer_return;
#[path = "borrowed_call_uses_null_compare.rs"]
mod null_compare;
#[path = "borrowed_call_uses_object_field.rs"]
mod object_field;
#[path = "borrowed_call_uses_mul.rs"]
mod mul;
#[path = "borrowed_call_uses_closure.rs"]
mod closure;
#[path = "borrowed_call_uses_guarded_arguments.rs"]
mod guarded_arguments;
#[path = "borrowed_call_uses_arguments.rs"]
mod arguments;
#[path = "borrowed_call_uses_field_write.rs"]
mod field_write;
#[path = "borrowed_call_uses_array_set.rs"]
mod array_set;
use object_field::path_dominates;
use super::*;
use crate::mir::compiler::normal_default_pipeline::published_backend_view::physical_program::PublishedLifecyclePhysicalFunctionV1;
use crate::mir::normal_callable_semantic_package::{
    BorrowedFormalActualSourceV1, FinalizedRootSourceHandoffV1, PreparedBorrowedFormalActualV1, BorrowedFormalDomainV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, MirFunction};

type Coordinate = (BasicBlockId, usize);
type Binding = (BasicBlockId, MirInstruction);

#[derive(Default)]
struct FunctionUses {
    finite_input: bool,
    condition_bindings: BTreeMap<Coordinate, MirInstruction>,
    domains: BTreeMap<ValueId, BorrowedFormalDomainV1>,
    text_arguments: BTreeMap<(BasicBlockId, usize, usize), crate::mir::instruction::StringInvokeOperand>,
    owned_field_writes: BTreeMap<Coordinate, field_write::OwnedFieldWriteUse>,
    field_writes: BTreeMap<Coordinate, field_write::FieldWriteUse>,
    array_sets: BTreeMap<Coordinate, array_set::ArraySetUse>,
    guarded_arguments: guarded_arguments::GuardedArguments,
    integer_returns: BTreeMap<Coordinate, integer_return::IntegerReturnUse>,
    formal_returns: BTreeMap<Coordinate, formal_return::FormalReturnUse>,
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
    mul_expected: BTreeMap<(Coordinate, usize), mul::MulOperandUse>,
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
    owned_field_writes: BTreeSet<Coordinate>,
    field_writes: BTreeSet<Coordinate>,
    array_sets: BTreeSet<Coordinate>,
    integer_returns: BTreeSet<Coordinate>,
    formal_returns: BTreeSet<Coordinate>,
    definitions: BTreeMap<ValueId, usize>,
    coordinates: BTreeSet<Coordinate>,
    arguments: BTreeSet<(BasicBlockId, usize, usize)>,
    /// Exact operand occurrences, including both sides of shared carriers.
    compare_uses: BTreeMap<BindingRefV1, BTreeSet<(Coordinate, usize)>>,
    mul_uses: BTreeSet<(Coordinate, usize)>,
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
    copy_coordinates: BTreeMap<ValueId, Coordinate>,
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

impl FunctionUses {
    fn scalar_operands_v1(&self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        instruction: &MirInstruction, accepts: impl Fn(&BorrowedFormalDomainV1) -> bool,
    ) -> bool {
        instruction.used_values().iter().all(|value| {
            let Some(formal) = tracked.get(value).or(views.views.get(value)) else { return true };
            let mut roots = self.roots.iter().filter(|(_, root)| *root == formal).peekable();
            roots.peek().is_some() && roots.all(|(value, _)| self.domains.get(value).is_some_and(&accepts))
        })
    }

    fn condition_operand_v1(&self, tracked: &BTreeMap<ValueId, BindingRefV1>, views: &ViewScan,
        coordinate: Coordinate, instruction: &MirInstruction,
    ) -> bool {
        if !self.finite_input || self.condition_bindings.get(&coordinate) != Some(instruction) { return false; }
        // Raw tag equality must consume the exact null-use admission even for
        // a signed-only/Bool-only incoming SUM. Scalar permission cannot skip it.
        if matches!(instruction, MirInstruction::Compare { op: crate::mir::CompareOp::Eq, lhs, rhs, .. }
            if views.null_consts.contains(lhs) || views.null_consts.contains(rhs)) { return false; }
        match instruction {
            MirInstruction::Branch { .. } => self.scalar_operands_v1(tracked, views, instruction, BorrowedFormalDomainV1::bool_only),
            MirInstruction::BinOp { .. } => self.scalar_operands_v1(tracked, views, instruction, BorrowedFormalDomainV1::signed_only),
            MirInstruction::Copy { .. } | MirInstruction::Phi { .. } | MirInstruction::Compare { .. } =>
                self.scalar_operands_v1(tracked, views, instruction, |domain| domain.signed_only() || domain.bool_only()),
            _ => false,
        }
    }

    fn has_source_integer_compare(&self, coordinate: Coordinate) -> bool {
        self.compare_coordinates.as_ref().is_some_and(|rows| {
            rows.values().any(|uses| uses.iter().any(|(found, _)| *found == coordinate))
        })
    }
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
        integer_return::guards(self, indexed)?;
        let mut defs: BTreeMap<ValueId, (Coordinate, &MirInstruction)> = BTreeMap::new();
        for (coordinate, instruction) in indexed {
            if let Some(dst) = instruction.dst_value() {
                if defs.insert(dst, (*coordinate, instruction)).is_some() {
                    return Err(fault("borrowed-use/view-definition-duplicate"));
                }
            }
        }
        let mut views = BTreeMap::new();
        let mut copy_coordinates = BTreeMap::new();
        for (coordinate, instruction) in indexed {
            let operands: Vec<ValueId> = match instruction {
                MirInstruction::Compare { lhs, rhs, .. } => vec![*lhs, *rhs],
                MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                    ..
                } => vec![*lhs, *rhs],
                MirInstruction::BinOp {
                    op: crate::mir::BinaryOp::Mul,
                    lhs,
                    rhs,
                    ..
                } if self.mul_expected.contains_key(&(*coordinate, 0))
                    || self.mul_expected.contains_key(&(*coordinate, 1)) => vec![*lhs, *rhs],
                MirInstruction::ArrayElementWrite {
                    kind: crate::mir::ArrayElementWriteKind::Set,
                    value,
                    ..
                } => vec![*value],
                MirInstruction::Invoke { operation: InvokeOperation::Birth(call), .. } => {
                    call.arguments.iter().filter_map(|argument| match argument {
                        crate::mir::instruction::BirthArgumentV1::Value(value) => Some(*value), _ => None,
                    }).collect()
                }
                MirInstruction::Call(call)
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
                let Some((copy_coordinate, MirInstruction::Copy { src, .. })) = defs.get(&operand) else {
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
                copy_coordinates.insert(operand, *copy_coordinate);
            }
        }
        let mut compares: BTreeMap<BindingRefV1, Vec<Coordinate>> = BTreeMap::new();
        let mut null_consts = BTreeSet::new();
        for (coordinate, instruction) in indexed {
            null_compare::observe_null_const(instruction, &mut null_consts);
            match instruction {
                // Only a source-proven Integer comparison anchors the numeric
                // dominance cone; a null
                // equality supplies non-null only, never the Integer lane.
                MirInstruction::Compare {
                    op,
                    lhs,
                    rhs,
                    ..
                } if matches!(op, crate::mir::CompareOp::Gt | crate::mir::CompareOp::Le)
                    || self.has_source_integer_compare(*coordinate) => {
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
            copy_coordinates,
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
        if self.condition_operand_v1(tracked, views, coordinate, instruction) { return Ok(()); }
        if field_write::observe(self, tracked, views, coordinate, instruction, definitions, dominates)? { return Ok(()); }
        if array_set::observe(self, tracked, views, coordinate, instruction, definitions, dominates)? { return Ok(()); }
        match instruction {
            MirInstruction::Copy { dst, src } if tracked.contains_key(src) => {
                let view = views.views.get(dst) == tracked.get(src);
                if !view
                    && !self.copies.get(dst).is_some_and(|(original, expected)| {
                        original.1 == *instruction && *expected == Some(coordinate)
                    })
                {
                    if crate::config::env::builder_mir_compile_trace() {
                        eprintln!(
                            "[borrowed-use/copy-trace] coordinate={coordinate:?} instruction={instruction:?} formal={:?} retained={:?}",
                            tracked.get(src),
                            self.copies.get(dst)
                        );
                    }
                    return Err(format!("{} coordinate={coordinate:?} instruction={instruction:?}",
                        fault("borrowed-use/unproved-copy")));
                }
            }
            MirInstruction::Invoke { operation: InvokeOperation::String(operation), fault_frame, .. } => {
                self.verify_text_operation_v1(tracked, views, operation, *fault_frame)?;
            }
            MirInstruction::Invoke { operation: InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::FieldHas { base, key, .. }
                | crate::mir::instruction::MapInvokeOperation::FieldGetIndexed { base, key, .. }), fault_frame, .. } => {
                self.verify_text_operands_v1(tracked, views, &[key], &[*base], *fault_frame)?;
            }
            MirInstruction::Invoke { operation: InvokeOperation::ArrayElementWrite {
                kind: crate::mir::ArrayElementWriteKind::Push, receiver, index: None,
                value: crate::mir::instruction::ArrayElementWriteValueV1::Text(operand), ..
            }, fault_frame, .. } => {
                // The source-backed append copies Text into its own element;
                // the input remains readonly and grants no receiver/frame use.
                self.verify_text_operands_v1(tracked, views, &[operand], &[*receiver], *fault_frame)?;
            }
            MirInstruction::Invoke { operation: InvokeOperation::Birth(call), fault_frame, .. } => {
                if tracked.contains_key(fault_frame) || views.views.contains_key(fault_frame)
                    || tracked.contains_key(&call.receiver) || views.views.contains_key(&call.receiver) {
                    return Err(fault("borrowed-use/callee-or-fault-frame"));
                }
                for argument in &call.arguments {
                    if let crate::mir::instruction::BirthArgumentV1::Text(operand) = argument {
                        self.verify_text_operands_v1(tracked, views, &[operand], &[call.receiver], *fault_frame)?;
                        continue;
                    }
                    let crate::mir::instruction::BirthArgumentV1::Value(value) = argument else {
                        return Err(fault("borrowed-use/callee-operand"));
                    };
                    if let Some(formal) = tracked.get(value).or(views.views.get(value)) {
                        if !views.compares.get(formal).is_some_and(|sites| sites.iter().any(|&(block, index)|
                            dominates(block, coordinate.0) && (block != coordinate.0 || index < coordinate.1))) {
                            return Err(fault("borrowed-use/undominated-view"));
                        }
                        definitions.ctor_uses.entry(*formal).or_default().insert(*value);
                    }
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
                    return Err(fault("borrowed-use/callee-or-fault-frame"));
                }
                for (ordinal, value) in call.args.iter().enumerate() {
                    match (
                        tracked.get(value),
                        self.arguments.get(&(coordinate.0, coordinate.1, ordinal)),
                    ) {
                        (Some(formal), Some(expected)) if formal == expected => {
                            guarded_arguments::observe(self, coordinate, ordinal, *formal, dominates)?;
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
                        _ => return Err(format!("{} coordinate={coordinate:?} ordinal={ordinal} value={value:?} tracked={:?} expected={:?} instruction={instruction:?}",
                            fault("borrowed-use/argument"), tracked.get(value),
                            self.arguments.get(&(coordinate.0, coordinate.1, ordinal)))),
                    }
                }
                if self.arguments.keys().any(|(block, index, ordinal)| {
                    (*block, *index) == coordinate && *ordinal >= call.args.len()
                }) {
                    return Err(fault("borrowed-use/argument-missing"));
                }
            }
            MirInstruction::Compare {
                op,
                lhs,
                rhs,
                ..
            } if matches!(op, crate::mir::CompareOp::Gt | crate::mir::CompareOp::Le)
                || self.has_source_integer_compare(coordinate) => {
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
            MirInstruction::BinOp {
                op: crate::mir::BinaryOp::Mul,
                lhs,
                rhs,
                ..
            } if self.mul_expected.contains_key(&(coordinate, 0))
                || self.mul_expected.contains_key(&(coordinate, 1)) => {
                mul::observe(self, tracked, views, coordinate, [*lhs, *rhs], definitions, dominates)?;
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
            MirInstruction::Return { value } if self.integer_returns.contains_key(&coordinate) => {
                integer_return::observe(self, tracked, coordinate, value, definitions, dominates)?;
            }
            MirInstruction::Return { .. } if self.formal_returns.contains_key(&coordinate) => {
                formal_return::observe(self, tracked, coordinate, instruction, definitions)?;
            }
            // SSA liveness metadata is not an executable borrowed payload use.
            MirInstruction::KeepAlive { .. } => {},
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
        integer_return::coverage(self, definitions)?;
        field_write::coverage(self, definitions)?;
        array_set::coverage(self, definitions)?;
        formal_return::coverage(self, definitions)?;
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
        mul::coverage(self, definitions)?;
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
            return Err(format!("{} expected={:?} actual={:?}",
                fault("borrowed-use/set-coverage"), self.set_admissions, definitions.set_uses));
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

#[cfg(test)]
#[path = "borrowed_call_uses_mul_tests.rs"]
mod mul_tests;

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
    let original = function.clone();
    (function, move |rows| {
        state.verify_published(&tracked, &original, rows)
    })
}
