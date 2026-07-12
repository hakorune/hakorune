//! Verification-only validated ProgramV0 view to snapshot witness.

use super::{
    AtomValueV0, ChildCardinalityV0, ChildRoleV0, DepthConventionV0, PathV0, SnapshotBuildErrorV0,
    SnapshotBuilderV0, SnapshotNodeIndexV0, ValidatedAtomValueV0, ValidatedNodeV0,
    ValidatedProgramV0BodyView, WireNodeKindV0,
};

pub(crate) fn build_snapshot_from_validated_view_v0(
    view: &ValidatedProgramV0BodyView,
) -> Result<super::BoundedBodyAnalysisSnapshotV0, SnapshotBuildErrorV0> {
    let mut builder = SnapshotBuilderV0::new(view.source_program_version());
    for ordinal in 0..view.body_len() {
        let node = view
            .body_node(ordinal)
            .expect("validated body length and node access must agree");
        let index = visit_node(
            node,
            PathV0::root_body().index(ordinal),
            DepthConventionV0::TOP_LEVEL_NODE,
            &mut builder,
        )?;
        builder.add_root(index)?;
    }
    builder.finish()
}

fn visit_node(
    node: ValidatedNodeV0<'_>,
    path: PathV0,
    depth: usize,
    builder: &mut SnapshotBuilderV0,
) -> Result<SnapshotNodeIndexV0, SnapshotBuildErrorV0> {
    let kind = node.kind();
    let index = builder.reserve_node(path.clone(), kind, depth)?;
    let children = node.children();
    let mut edges = Vec::with_capacity(children.len());
    for (position, (role, child)) in children.into_iter().enumerate() {
        let ordinal = edges[..position]
            .iter()
            .filter(|(seen_role, _)| *seen_role == role)
            .count();
        let child_path = child_path(&path, kind, role, ordinal);
        let child_index = visit_node(child, child_path, depth + 1, builder)?;
        edges.push((role, child_index));
    }
    let atoms = node
        .atoms()
        .into_iter()
        .map(|(key, value)| (key, owned_atom(value)))
        .collect();
    builder.seal_node(index, atoms, edges)?;
    Ok(index)
}

fn child_path(parent: &PathV0, kind: WireNodeKindV0, role: ChildRoleV0, ordinal: usize) -> PathV0 {
    let spec = kind
        .child_schema()
        .iter()
        .find(|spec| spec.role == role)
        .expect("validated child role must belong to node schema");
    let base = parent.field(role.path_field());
    match spec.cardinality {
        ChildCardinalityV0::One => base,
        ChildCardinalityV0::List | ChildCardinalityV0::OptionalList => base.index(ordinal),
    }
}

fn owned_atom(value: ValidatedAtomValueV0<'_>) -> AtomValueV0 {
    match value {
        ValidatedAtomValueV0::I64(value) => AtomValueV0::I64(value),
        ValidatedAtomValueV0::Bool(value) => AtomValueV0::Bool(value),
        ValidatedAtomValueV0::Text(value) => AtomValueV0::Text(value.value().to_owned()),
        ValidatedAtomValueV0::Null => AtomValueV0::Null,
    }
}
