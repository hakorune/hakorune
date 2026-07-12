//! One-shot private draft builder for a structurally verified flat snapshot.

use super::{
    AtomValueKindV0, AtomValueV0, BoundedBodyAnalysisSnapshotV0, BoundedBodyBudgetV0,
    BudgetLimitV0, ChildCardinalityV0, ChildRoleV0, DepthConventionV0, PathV0, SnapshotNodeV0,
    TextClassV0, WireNodeKindV0,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SnapshotNodeIndexV0(usize);

impl SnapshotNodeIndexV0 {
    pub fn get(self) -> usize {
        self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SnapshotBuildErrorV0 {
    Budget(BudgetLimitV0),
    Poisoned,
    InvalidSourceVersion,
    InvalidNodeIndex,
    AlreadySealed,
    AtomSchema,
    ChildSchema,
    IncompleteDraft,
    RootPath,
    ChildTarget,
    ChildPath,
    Preorder,
    Depth,
    Disconnected,
    NodeCount,
}

#[derive(Debug)]
struct NodeDraftV0 {
    path: PathV0,
    kind: WireNodeKindV0,
    depth: usize,
    sealed: Option<(
        Vec<(super::AtomKeyV0, AtomValueV0)>,
        Vec<(ChildRoleV0, SnapshotNodeIndexV0)>,
    )>,
}

#[derive(Debug)]
pub struct SnapshotBuilderV0 {
    source_program_version: i32,
    drafts: Vec<NodeDraftV0>,
    roots: Vec<SnapshotNodeIndexV0>,
    budget: BoundedBodyBudgetV0,
    poisoned: Option<SnapshotBuildErrorV0>,
}

impl SnapshotBuilderV0 {
    pub fn new(source_program_version: i32) -> Self {
        Self {
            source_program_version,
            drafts: Vec::new(),
            roots: Vec::new(),
            budget: BoundedBodyBudgetV0::default(),
            poisoned: None,
        }
    }

    pub fn reserve_node(
        &mut self,
        path: PathV0,
        kind: WireNodeKindV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, SnapshotBuildErrorV0> {
        self.ensure_live()?;
        if let Err(limit) = self.budget.observe_node(depth) {
            return self.poison(SnapshotBuildErrorV0::Budget(limit));
        }
        let index = SnapshotNodeIndexV0(self.drafts.len());
        self.drafts.push(NodeDraftV0 {
            path,
            kind,
            depth,
            sealed: None,
        });
        Ok(index)
    }

    pub fn add_root(&mut self, index: SnapshotNodeIndexV0) -> Result<(), SnapshotBuildErrorV0> {
        self.ensure_live()?;
        if index.0 >= self.drafts.len() {
            return self.poison(SnapshotBuildErrorV0::InvalidNodeIndex);
        }
        self.roots.push(index);
        Ok(())
    }

    pub fn seal_node(
        &mut self,
        index: SnapshotNodeIndexV0,
        atoms: Vec<(super::AtomKeyV0, AtomValueV0)>,
        children: Vec<(ChildRoleV0, SnapshotNodeIndexV0)>,
    ) -> Result<(), SnapshotBuildErrorV0> {
        self.ensure_live()?;
        let Some(draft) = self.drafts.get(index.0) else {
            return self.poison(SnapshotBuildErrorV0::InvalidNodeIndex);
        };
        if draft.sealed.is_some() {
            return self.poison(SnapshotBuildErrorV0::AlreadySealed);
        }
        let kind = draft.kind;
        if let Err(error) = validate_atoms(kind, &atoms, &mut self.budget) {
            return self.poison(error);
        }
        if let Err(error) = validate_child_roles(kind, &children, &self.budget) {
            return self.poison(error);
        }
        self.drafts[index.0].sealed = Some((atoms, children));
        Ok(())
    }

    pub fn finish(self) -> Result<BoundedBodyAnalysisSnapshotV0, SnapshotBuildErrorV0> {
        if let Some(error) = self.poisoned.as_ref() {
            return Err(error.clone());
        }
        if self.source_program_version != 0 {
            return Err(SnapshotBuildErrorV0::InvalidSourceVersion);
        }
        self.budget
            .observe_body_children(self.roots.len())
            .map_err(SnapshotBuildErrorV0::Budget)?;
        if self.budget.node_count() != self.drafts.len() {
            return Err(SnapshotBuildErrorV0::NodeCount);
        }
        if self.drafts.iter().any(|draft| draft.sealed.is_none()) {
            return Err(SnapshotBuildErrorV0::IncompleteDraft);
        }
        validate_roots(&self.drafts, &self.roots)?;
        validate_edges_and_paths(&self.drafts)?;
        validate_preorder_and_depth(&self.drafts, &self.roots)?;

        let nodes = self
            .drafts
            .into_iter()
            .map(|draft| {
                let (atoms, children) = draft.sealed.expect("complete drafts checked above");
                SnapshotNodeV0::from_verified_parts(
                    draft.path,
                    draft.kind,
                    atoms,
                    children
                        .into_iter()
                        .map(|(role, target)| (role, target.0))
                        .collect(),
                )
            })
            .collect();
        Ok(BoundedBodyAnalysisSnapshotV0::from_verified_parts(
            self.source_program_version,
            nodes,
            self.budget.max_depth_observed(),
        ))
    }

    fn ensure_live(&self) -> Result<(), SnapshotBuildErrorV0> {
        match &self.poisoned {
            Some(_) => Err(SnapshotBuildErrorV0::Poisoned),
            None => Ok(()),
        }
    }

    fn poison<T>(&mut self, error: SnapshotBuildErrorV0) -> Result<T, SnapshotBuildErrorV0> {
        self.poisoned = Some(error.clone());
        Err(error)
    }
}

fn validate_atoms(
    kind: WireNodeKindV0,
    atoms: &[(super::AtomKeyV0, AtomValueV0)],
    budget: &mut BoundedBodyBudgetV0,
) -> Result<(), SnapshotBuildErrorV0> {
    let schema = kind.atom_schema();
    if atoms.len() != schema.len() {
        return Err(SnapshotBuildErrorV0::AtomSchema);
    }
    for ((key, value), spec) in atoms.iter().zip(schema) {
        if *key != spec.key || !atom_value_matches(spec.value_kind, value) {
            return Err(SnapshotBuildErrorV0::AtomSchema);
        }
        if let AtomValueV0::Text(text) = value {
            match spec.text_class {
                Some(TextClassV0::Literal) => budget.observe_literal(text),
                Some(TextClassV0::Atom) => budget.observe_atom(text),
                None => return Err(SnapshotBuildErrorV0::AtomSchema),
            }
            .map_err(SnapshotBuildErrorV0::Budget)?;
        } else if spec.text_class.is_some() {
            return Err(SnapshotBuildErrorV0::AtomSchema);
        }
    }
    Ok(())
}

fn atom_value_matches(kind: AtomValueKindV0, value: &AtomValueV0) -> bool {
    matches!(
        (kind, value),
        (AtomValueKindV0::I64, AtomValueV0::I64(_))
            | (AtomValueKindV0::Bool, AtomValueV0::Bool(_))
            | (AtomValueKindV0::Text, AtomValueV0::Text(_))
            | (AtomValueKindV0::Null, AtomValueV0::Null)
    )
}

fn validate_child_roles(
    kind: WireNodeKindV0,
    children: &[(ChildRoleV0, SnapshotNodeIndexV0)],
    budget: &BoundedBodyBudgetV0,
) -> Result<(), SnapshotBuildErrorV0> {
    let mut position = 0;
    for spec in kind.child_schema() {
        let start = position;
        while position < children.len() && children[position].0 == spec.role {
            position += 1;
        }
        let count = position - start;
        if spec.cardinality == ChildCardinalityV0::One && count != 1 {
            return Err(SnapshotBuildErrorV0::ChildSchema);
        }
        match spec.role {
            ChildRoleV0::Then | ChildRoleV0::Else | ChildRoleV0::Body => budget
                .observe_body_children(count)
                .map_err(SnapshotBuildErrorV0::Budget)?,
            ChildRoleV0::Args => budget
                .observe_arguments(count)
                .map_err(SnapshotBuildErrorV0::Budget)?,
            _ => {}
        }
    }
    (position == children.len())
        .then_some(())
        .ok_or(SnapshotBuildErrorV0::ChildSchema)
}

fn validate_roots(
    drafts: &[NodeDraftV0],
    roots: &[SnapshotNodeIndexV0],
) -> Result<(), SnapshotBuildErrorV0> {
    for (ordinal, root) in roots.iter().enumerate() {
        let Some(draft) = drafts.get(root.0) else {
            return Err(SnapshotBuildErrorV0::InvalidNodeIndex);
        };
        if draft.path != PathV0::root_body().index(ordinal) {
            return Err(SnapshotBuildErrorV0::RootPath);
        }
    }
    Ok(())
}

fn validate_edges_and_paths(drafts: &[NodeDraftV0]) -> Result<(), SnapshotBuildErrorV0> {
    for (parent_index, draft) in drafts.iter().enumerate() {
        let (_, children) = draft.sealed.as_ref().expect("complete drafts checked");
        for (edge_position, (role, target)) in children.iter().enumerate() {
            if target.0 <= parent_index || target.0 >= drafts.len() {
                return Err(SnapshotBuildErrorV0::ChildTarget);
            }
            let spec = draft
                .kind
                .child_schema()
                .iter()
                .find(|spec| spec.role == *role)
                .ok_or(SnapshotBuildErrorV0::ChildSchema)?;
            let ordinal = children[..edge_position]
                .iter()
                .filter(|(seen_role, _)| seen_role == role)
                .count();
            let base = draft.path.field(role.path_field());
            let expected = match spec.cardinality {
                ChildCardinalityV0::One => base,
                ChildCardinalityV0::List | ChildCardinalityV0::OptionalList => base.index(ordinal),
            };
            if drafts[target.0].path != expected {
                return Err(SnapshotBuildErrorV0::ChildPath);
            }
        }
    }
    Ok(())
}

fn validate_preorder_and_depth(
    drafts: &[NodeDraftV0],
    roots: &[SnapshotNodeIndexV0],
) -> Result<(), SnapshotBuildErrorV0> {
    let mut stack: Vec<(usize, usize)> = roots
        .iter()
        .rev()
        .map(|root| (root.0, DepthConventionV0::TOP_LEVEL_NODE))
        .collect();
    let mut visited = vec![false; drafts.len()];
    let mut order = Vec::with_capacity(drafts.len());
    while let Some((index, depth)) = stack.pop() {
        if index >= drafts.len() || visited[index] {
            return Err(SnapshotBuildErrorV0::Disconnected);
        }
        visited[index] = true;
        order.push(index);
        if drafts[index].depth != depth {
            return Err(SnapshotBuildErrorV0::Depth);
        }
        let (_, children) = drafts[index]
            .sealed
            .as_ref()
            .expect("complete drafts checked");
        for (_, child) in children.iter().rev() {
            stack.push((child.0, depth + 1));
        }
    }
    if visited.iter().any(|seen| !seen) {
        return Err(SnapshotBuildErrorV0::Disconnected);
    }
    if order.iter().copied().ne(0..drafts.len()) {
        return Err(SnapshotBuildErrorV0::Preorder);
    }
    Ok(())
}
