pub(super) fn ordered_harness_functions<'a>(
    module: &'a crate::mir::MirModule,
) -> Vec<(&'a String, &'a crate::mir::MirFunction)> {
    let mut functions: Vec<_> = module.functions.iter().collect();
    functions.sort_by(|(lhs_name, lhs_func), (rhs_name, rhs_func)| {
        harness_entry_priority(lhs_name, lhs_func)
            .cmp(&harness_entry_priority(rhs_name, rhs_func))
            .then_with(|| lhs_name.cmp(rhs_name))
    });
    functions
}

pub(super) fn harness_entry_priority(name: &str, func: &crate::mir::MirFunction) -> (u8, u8) {
    if func.metadata.is_entry_point {
        return (0, 0);
    }
    if name == "main" {
        return (0, 1);
    }
    if name == "ny_main" {
        return (0, 2);
    }
    if name.ends_with(".main/0") || name.ends_with(".main/1") {
        return (1, 0);
    }
    (2, 0)
}
