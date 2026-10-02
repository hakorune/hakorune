//! Final operand closure over original entry/Copy/Forwarded loans.
//! These temporary sets corroborate one publication; they issue no authority.
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
}

#[derive(Default)]
struct Scan {
    definitions: BTreeMap<ValueId, usize>,
    coordinates: BTreeSet<Coordinate>,
    arguments: BTreeSet<(BasicBlockId, usize, usize)>,
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
        })
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

    pub(super) fn finish(
        self,
        program: &PublishedLifecyclePhysicalProgramV1<'_>,
        module: &crate::mir::MirModule,
    ) -> Result<(), String> {
        for (name, state) in self.functions {
            let function = module
                .functions
                .get(&name)
                .ok_or_else(|| fault("borrowed-use/function-missing"))?;
            let tracked = state.tracked()?;
            let mut definitions = Scan::default();
            for (block, row) in &function.blocks {
                if *block != row.id {
                    return Err(fault("borrowed-use/block-identity"));
                }
                if row
                    .return_env
                    .as_ref()
                    .is_some_and(|values| values.iter().any(|value| tracked.contains_key(value)))
                {
                    return Err(fault("borrowed-use/return-env"));
                }
                for (index, instruction) in row.all_instructions().enumerate() {
                    state.instruction(&tracked, (*block, index), instruction, &mut definitions)?;
                }
            }
            state.definitions(&tracked, &function.params, &definitions)?;
            state.verify_published(&tracked, &name, program.functions())?;
        }
        Ok(())
    }
}

impl FunctionUses {
    fn verify_published(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        name: &str,
        rows: &[PublishedLifecyclePhysicalFunctionV1<'_>],
    ) -> Result<(), String> {
        let mut published_rows = rows.iter().filter(|row| row.name() == name);
        let published = published_rows
            .next()
            .ok_or_else(|| fault("borrowed-use/published-missing"))?;
        if published_rows.next().is_some() {
            return Err(fault("borrowed-use/published-duplicate"));
        }
        // Publication is checked independently: projection-only operand or
        // definition drift must not inherit the module's proof.
        let mut definitions = Scan::default();
        let mut blocks = BTreeSet::new();
        for block in published.blocks() {
            if !blocks.insert(block.id()) {
                return Err(fault("borrowed-use/published-block-duplicate"));
            }
            for edge in block.edges() {
                if edge
                    .args()
                    .is_some_and(|args| args.values.iter().any(|value| tracked.contains_key(value)))
                {
                    return Err(fault("borrowed-use/published-edge"));
                }
            }
            for instruction in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                self.instruction(
                    tracked,
                    (block.id(), instruction.index() as usize),
                    instruction.instruction(),
                    &mut definitions,
                )?;
            }
        }
        self.definitions(tracked, published.params(), &definitions)?;
        Ok(())
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

    fn instruction(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        coordinate: Coordinate,
        instruction: &MirInstruction,
        definitions: &mut Scan,
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
                if !self.copies.get(dst).is_some_and(|(original, expected)| {
                    original.1 == *instruction && *expected == Some(coordinate)
                }) {
                    return Err(fault("borrowed-use/unproved-copy"));
                }
            }
            MirInstruction::Invoke {
                operation: InvokeOperation::Call { call, .. },
                fault_frame,
                ..
            } => {
                let mut forbidden = tracked.contains_key(fault_frame);
                call.callee
                    .for_each_value_operand(|value| forbidden |= tracked.contains_key(&value));
                if forbidden {
                    return Err(fault("borrowed-use/callee-or-fault-frame"));
                }
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
            _ if instruction
                .used_values()
                .iter()
                .any(|value| tracked.contains_key(value)) =>
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
