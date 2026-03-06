use crate::config::env;
use crate::mir::join_ir::lowering::carrier_info::{CarrierInfo, ExitMeta, JoinFragmentMeta};
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

/// Build ExitMeta and JoinFragmentMeta for `loop_break`.
pub(crate) fn build_fragment_meta(
    carrier_info: &CarrierInfo,
    loop_var_name: &str,
    i_exit: ValueId,
    carrier_exit_ids: &[ValueId],
) -> JoinFragmentMeta {
    let dev_on = env::joinir_dev_enabled();

    if carrier_exit_ids.len() != carrier_info.carriers.len() {
        let msg = format!(
            "[joinir/boundary] exit value length mismatch: carriers={} exit_ids={}",
            carrier_info.carriers.len(),
            carrier_exit_ids.len()
        );
        debug_assert!(
            carrier_exit_ids.len() == carrier_info.carriers.len(),
            "{}",
            msg
        );
        if dev_on {
            panic!("{}", msg);
        }
    }

    let mut exit_values = Vec::new();
    exit_values.push((loop_var_name.to_string(), i_exit));

    for (idx, carrier) in carrier_info.carriers.iter().enumerate() {
        exit_values.push((carrier.name.clone(), carrier_exit_ids[idx]));
    }

    if env::joinir_debug_level() > 0 || dev_on {
        get_global_ring0().log.debug(&format!(
            "[joinir/boundary] Exit values (loop_var='{}', carriers={}): {:?}",
            loop_var_name,
            carrier_info.carriers.len(),
            exit_values
        ));
    }

    let exit_meta = ExitMeta::multiple(exit_values);
    JoinFragmentMeta::with_expr_result(i_exit, exit_meta)
}
