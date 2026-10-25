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
    ) -> Result<BTreeMap<u32, ClosedBorrowedEntryV1>, String> {
        let mut object_views = BTreeMap::new();
        for (name, mut state) in self.functions {
            state.completed_object_views_v1(module)?;
            state.prepare_finite_source_copies_v1()?;
            guarded_arguments::prepare(&mut state)?;
            let function = module
                .functions
                .get(&name)
                .ok_or_else(|| fault("borrowed-use/function-missing"))?;
            formal_return::prepare(&mut state, function, program.functions())?;
            if let Some(source) = program.handoff().root_source() {
                state.finite_birth_arguments_v1(source, program.handoff().birth_actuals().unwrap_or(&[]), function)?;
            }
            let tracked = state.tracked()?;
            state.verify_module(&tracked, function)?;
            state.verify_published(&tracked, function, program.functions())?;
            let views = object_field::object_view_values(&state)?;
            // Keep the admitted entry even when none of its formals has an
            // object class. A zero-call declaration has no Call actual row.
            if !state.roots.is_empty() {
                let index = program
                    .functions()
                    .iter()
                    .position(|function| function.name() == name)
                    .ok_or_else(|| fault("borrowed-use/object-view-function"))?;
                object_views.insert(index as u32, ClosedBorrowedEntryV1 {
                    finite_input: state.finite_input,
                    object_views: views, domains: state.domains.iter().map(|(value, domain)| (value.0, domain.clone())).collect(),
                });
            }
        }
        Ok(object_views)
    }
}

impl FunctionUses {
    /// Same final module scan for production and source-issued acceptance.
    /// Original coordinates and entry are never repaired by this consumer.
    pub(super) fn verify_module(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        function: &MirFunction,
    ) -> Result<(), String> {
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
        self.verify_text_occurrences_v1(&indexed)?;
        let views = self.scan_views(&tracked, &indexed)?;
        field_write::guards(self, tracked, &views, &indexed)?;
        array_set::guards(self, tracked, &views, &indexed)?;
        let dominates = |from: BasicBlockId, to: BasicBlockId| {
            path_dominates(function.entry_block, &successors, from, to)
        };
        for (coordinate, instruction) in &indexed {
            self.instruction(
                &tracked,
                &views,
                *coordinate,
                instruction,
                &mut definitions,
                &dominates,
            ).map_err(|issue| format!(
                "{issue} function={} scan=module coordinate={coordinate:?} instruction={instruction:?} {}",
                function.signature.name, definition_context(function, instruction),
            ))?;
        }
        self.definitions(tracked, &function.params, &definitions)
            .map_err(|issue| format!("{issue} function={} scan=module", function.signature.name))
    }

    pub(super) fn verify_published(
        &self,
        tracked: &BTreeMap<ValueId, BindingRefV1>,
        function: &MirFunction,
        rows: &[PublishedLifecyclePhysicalFunctionV1<'_>],
    ) -> Result<(), String> {
        let mut published_rows = rows.iter().filter(|row| row.name() == function.signature.name);
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
        for (coordinate, instruction) in &indexed {
            if self.has_source_integer_compare(*coordinate)
                && function.blocks.get(&coordinate.0)
                    .and_then(|block| block.all_instructions().nth(coordinate.1)) != Some(*instruction) {
                return Err(fault("borrowed-use/published-compare-instruction"));
            }
        }
        self.verify_text_occurrences_v1(&indexed)?;
        let views = self.scan_views(tracked, &indexed)?;
        field_write::guards(self, tracked, &views, &indexed)?;
        array_set::guards(self, tracked, &views, &indexed)?;
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
            ).map_err(|issue| format!(
                "{issue} function={} scan=published coordinate={coordinate:?} instruction={instruction:?} {}",
                function.signature.name, definition_context(function, instruction),
            ))?;
        }
        self.definitions(tracked, published.params(), &definitions)
            .map_err(|issue| format!("{issue} function={} scan=published", function.signature.name))
    }
}

// Failure-only context for the original definition; no permission or rewrite.
fn definition_context(function: &MirFunction, instruction: &MirInstruction) -> String {
    let Some(dst) = instruction.dst_value() else { return String::new() };
    let uses: Vec<_> = function.blocks.iter().flat_map(|(block, body)|
        body.all_instructions().enumerate().filter_map(move |(index, row)|
            row.used_values().contains(&dst).then_some((*block, index, row))))
        .collect();
    let return_env: Vec<_> = function.blocks.iter().filter_map(|(block, body)|
        body.return_env.as_ref().is_some_and(|values| values.contains(&dst)).then_some(*block))
        .collect();
    format!("definition_uses={uses:?} return_env={return_env:?} origin={:?}",
        function.metadata.value_origin_callers.get(&dst))
}
