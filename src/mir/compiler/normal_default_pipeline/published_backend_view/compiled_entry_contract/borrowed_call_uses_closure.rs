//! Final closure stage: per-function module scan, published corroboration,
//! coverage counts, and the sealed object-view map. Admission and operand
//! classification stay with the parent; this child only finishes.
use super::*;

impl BorrowedCallUses {
    /// Closes every recorded function and returns each callee's sealed
    /// formal object views as `function index -> param value -> canonical
    /// object` — the sealed view, not the physical row, is the authority.
    pub(in crate::mir::compiler::normal_default_pipeline::published_backend_view) fn finish(
        self,
        program: &PublishedLifecyclePhysicalProgramV1<'_>,
        module: &crate::mir::MirModule,
    ) -> Result<BTreeMap<u32, BTreeMap<u32, u32>>, String> {
        let mut object_views = BTreeMap::new();
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
            }
            let mut indexed = Vec::new();
            let mut successors: BTreeMap<BasicBlockId, Vec<BasicBlockId>> = BTreeMap::new();
            for (block, row) in &function.blocks {
                successors.entry(*block).or_default().extend(row.successors.iter().copied());
                for (index, instruction) in row.all_instructions().enumerate() {
                    indexed.push(((*block, index), instruction));
                }
            }
            let views = state.scan_views(&tracked, &indexed)?;
            let dominates = |from: BasicBlockId, to: BasicBlockId| {
                path_dominates(function.entry_block, &successors, from, to)
            };
            for (coordinate, instruction) in &indexed {
                state.instruction(
                    &tracked,
                    &views,
                    *coordinate,
                    instruction,
                    &mut definitions,
                    &dominates,
                )?;
            }
            state.definitions(&tracked, &function.params, &definitions)?;
            state.verify_published(&tracked, &name, program.functions())?;
            let views = object_field::object_view_values(&state)?;
            if !views.is_empty() {
                let index = program
                    .functions()
                    .iter()
                    .position(|function| function.name() == name)
                    .ok_or_else(|| fault("borrowed-use/object-view-function"))?;
                object_views.insert(index as u32, views);
            }
        }
        Ok(object_views)
    }
}

impl FunctionUses {
    pub(super) fn verify_published(
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
        let mut successors: BTreeMap<BasicBlockId, Vec<BasicBlockId>> = BTreeMap::new();
        let mut indexed = Vec::new();
        for block in published.blocks() {
            if !blocks.insert(block.id()) {
                return Err(fault("borrowed-use/published-block-duplicate"));
            }
            successors
                .entry(block.id())
                .or_default()
                .extend(block.edges().iter().map(|edge| edge.target()));
            for instruction in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                indexed.push((
                    (block.id(), instruction.index() as usize),
                    instruction.instruction(),
                ));
            }
        }
        let views = self.scan_views(tracked, &indexed)?;
        for block in published.blocks() {
            for edge in block.edges() {
                if edge.args().is_some_and(|args| {
                    args.values
                        .iter()
                        .any(|value| tracked.contains_key(value) || views.views.contains_key(value))
                }) {
                    return Err(fault("borrowed-use/published-edge"));
                }
            }
        }
        let dominates = |from: BasicBlockId, to: BasicBlockId| {
            path_dominates(published.entry(), &successors, from, to)
        };
        for (coordinate, instruction) in &indexed {
            self.instruction(
                tracked,
                &views,
                *coordinate,
                instruction,
                &mut definitions,
                &dominates,
            )?;
        }
        self.definitions(tracked, published.params(), &definitions)
    }
}
