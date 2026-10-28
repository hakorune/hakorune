//! Original recursive interval census, checked before immutable RHS publication.
use super::*;

impl EmittedFieldWriteRhsV1 {
    pub(super) fn validate_original_scope(
        &self,
        function: &MirFunction,
        post_rhs_prefix: &[Binding],
    ) -> Result<(), String> {
        let root = self
            .recursive_nodes()
            .iter()
            .find(|node| node.node == 0)
            .ok_or_else(|| freeze("field-write-rhs/root-scope"))?;
        let whole = interval(function, root.entry, root.exit, post_rhs_prefix)?;
        let inventory = self.bindings()?;
        if whole.iter().any(|row| !inventory.contains(row)) {
            return Err(freeze("field-write-rhs/unowned-effect"));
        }
        for node in self.recursive_nodes() {
            let rows = interval(function, node.entry, node.exit, post_rhs_prefix)?;
            match &node.proof {
                FieldWriteRhsProofV1::Reference { .. }
                | FieldWriteRhsProofV1::ArrayHomeRead { .. } => {
                    if node.entry != node.exit {
                        return Err(freeze("field-write-rhs/reference-effect"));
                    }
                }
                FieldWriteRhsProofV1::Literal(row) => {
                    if rows.as_slice() != [row.clone()] {
                        return Err(freeze("field-write-rhs/literal-scope"));
                    }
                }
                FieldWriteRhsProofV1::Field(field) => {
                    if field.iter().any(|row| !rows.contains(row)) {
                        return Err(freeze("field-write-rhs/field-scope"));
                    }
                }
                FieldWriteRhsProofV1::Binary { append, .. } => {
                    let original = self.recipe.source().nodes()[node.node]
                        .binary()
                        .ok_or_else(|| freeze("field-write-rhs/binary-scope"))?;
                    let child = |site: &SourceExprSiteV1| {
                        self.recursive_nodes()
                            .iter()
                            .find(|node| self.recipe.source().nodes()[node.node].site() == site)
                            .ok_or_else(|| freeze("field-write-rhs/child-scope"))
                    };
                    let left = child(original.lhs())?;
                    let right = child(original.rhs())?;
                    if node.entry != left.entry
                        || left.exit != right.entry
                        || !interval(function, right.exit, node.exit, post_rhs_prefix)?
                            .contains(append)
                    {
                        return Err(freeze("field-write-rhs/child-order"));
                    }
                }
            }
            if rows.iter().any(|row| !whole.contains(row)) {
                return Err(freeze("field-write-rhs/escaped-node-scope"));
            }
        }
        Ok(())
    }
}
fn interval(
    function: &MirFunction,
    from: Cursor,
    to: Cursor,
    post_rhs_prefix: &[Binding],
) -> Result<Vec<Binding>, String> {
    let mut at = from;
    let mut seen = std::collections::BTreeSet::new();
    let mut rows = Vec::new();
    loop {
        if !seen.insert(at.block) {
            return Err(freeze("field-write-rhs/interval-cycle"));
        }
        let body = function
            .blocks
            .get(&at.block)
            .ok_or_else(|| freeze("field-write-rhs/interval-block"))?;
        let logical: Vec<_> = body
            .instructions
            .iter()
            .filter(|row| {
                !post_rhs_prefix
                    .iter()
                    .any(|(block, added)| *block == at.block && added == *row)
            })
            .collect();
        let end = if at.block == to.block {
            to.instruction
        } else {
            logical.len()
        };
        if at.instruction > end || end > logical.len() {
            return Err(freeze("field-write-rhs/interval-cursor"));
        }
        rows.extend(
            logical[at.instruction..end]
                .iter()
                .map(|row| (at.block, (*row).clone())),
        );
        if at.block == to.block {
            return Ok(rows);
        }
        let terminal = body
            .terminator
            .as_ref()
            .ok_or_else(|| freeze("field-write-rhs/interval-terminal"))?;
        let next = match terminal {
            MirInstruction::Jump {
                target,
                edge_args: None,
            } => *target,
            MirInstruction::Invoke { normal_landing, .. } => *normal_landing,
            _ => return Err(freeze("field-write-rhs/interval-normal-path")),
        };
        rows.push((at.block, terminal.clone()));
        at = Cursor {
            block: next,
            instruction: 0,
        };
    }
}
