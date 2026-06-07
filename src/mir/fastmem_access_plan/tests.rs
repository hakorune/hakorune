use super::*;
mod free_list;
mod remote;
mod support;
mod table;

use support::*;

#[test]
fn refresh_ignores_layout_table_memops_without_symbolic_ids() {
    let mut function = make_function(vec![memop(
        MemOpKind::FieldLoad,
        Some(ValueId::new(1)),
        vec![ValueId::new(0)],
        None,
    )]);

    refresh_function_fastmem_access_plans(&mut function);

    assert!(function.metadata.fastmem_access_plans.is_empty());
}
