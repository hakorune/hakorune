//! Exit argument resolver for JoinIR loop lowering.
//!
//! 役割:
//! - ExitLiveness から得た live 集合と、名前→ValueId マップを突き合わせて exit_args を決定する。
//! - live 集合に未解決名があれば None で返し、フォールバックを促す。

use std::collections::BTreeSet;

use crate::mir::ValueId;

/// live 集合と名前マップから exit_args を決定する。
pub(crate) fn resolve_exit_args(
    live_names: &BTreeSet<String>,
    name_to_id: &std::collections::BTreeMap<String, ValueId>,
    fallback_carriers: &[String],
) -> Option<Vec<ValueId>> {
    let mut args = Vec::new();
    for name in live_names {
        if let Some(v) = name_to_id.get(name) {
            args.push(*v);
        } else {
            return None;
        }
    }

    if args.is_empty() {
        for name in fallback_carriers {
            if let Some(v) = name_to_id.get(name) {
                args.push(*v);
            }
        }
    }

    Some(args)
}
