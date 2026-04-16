pub(in crate::mir::builder) fn planner_required_for_loop_cond() -> bool {
    let strict_or_dev = crate::config::env::joinir_dev::strict_enabled()
        || crate::config::env::joinir_dev_enabled();
    strict_or_dev && crate::config::env::joinir_dev::planner_required_enabled()
}
