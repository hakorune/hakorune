//! One readonly binding relation verifier for both portable Loop wires.
//! Class lookup stays with each wire; this owner never chooses a source role.
use super::error::LoopRecipeRejectReasonV1 as Reject;
use super::ids::{LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::schema::LoopRecipeReadOnlyInputV1;
use std::collections::BTreeSet;

pub(super) fn check<Class: Copy + Eq>(
    root: LoopNodeKeyV1,
    rows: &[LoopRecipeReadOnlyInputV1<Class>],
    inputs: &[LoopValueKeyV1],
    carriers: impl Iterator<Item = (LoopBindingKeyV1, LoopValueKeyV1)>,
    binding_class: impl Fn(LoopBindingKeyV1) -> Result<Class, Reject>,
    expect_value_class: impl Fn(LoopValueKeyV1, Class) -> Result<(), Reject>,
    accesses: impl Iterator<Item = (LoopBindingKeyV1, bool)>,
) -> Result<(), Reject> {
    if rows.is_empty() { return Ok(()); }
    let carriers: Vec<_> = carriers.collect();
    let accesses: Vec<_> = accesses.collect();
    let mut bindings = BTreeSet::new();
    let mut values = BTreeSet::new();
    for row in rows {
        if row.owner_loop != root {
            return Err(Reject::ReadOnlyInputNotRoot { binding: row.binding });
        }
        if !bindings.insert(row.binding) {
            return Err(Reject::DuplicateReadOnlyInput { binding: row.binding });
        }
        if !values.insert(row.entry_value) {
            return Err(Reject::DuplicateReadOnlyValue { value: row.entry_value });
        }
        if !inputs.contains(&row.entry_value) {
            return Err(Reject::ReadOnlyInputNotDeclared { value: row.entry_value });
        }
        if carriers.iter().any(|(binding, value)| *binding == row.binding || *value == row.entry_value) {
            return Err(Reject::ReadOnlyInputCarrierOverlap { binding: row.binding });
        }
        if binding_class(row.binding)? != row.class {
            return Err(Reject::ValueClassMismatch { key: row.entry_value });
        }
        expect_value_class(row.entry_value, row.class)?;
        let mut read = false;
        for (binding, written) in &accesses {
            if *binding != row.binding { continue; }
            if *written { return Err(Reject::ReadOnlyInputWritten { binding: row.binding }); }
            read = true;
        }
        if !read { return Err(Reject::ReadOnlyInputUnused { binding: row.binding }); }
    }
    Ok(())
}
