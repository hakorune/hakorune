//! Header predecessor policy (SSOT)
//!
//! Entry preds:
//! - carrier entry_incoming blocks
//! - host entry block (if any)
//! Latch preds:
//! - header predecessors not in entry preds

use super::loop_header_phi_info::LoopHeaderPhiInfo;
use crate::mir::BasicBlockId;
use std::collections::BTreeSet;

pub(super) struct HeaderPredGroups {
    pub entry_preds: Vec<BasicBlockId>,
    pub latch_preds: Vec<BasicBlockId>,
    pub host_entry_added: bool,
}

pub(super) fn split_header_preds(
    info: &LoopHeaderPhiInfo,
    header_preds: &[BasicBlockId],
    host_entry_block_opt: Option<BasicBlockId>,
    latch_block: BasicBlockId,
) -> HeaderPredGroups {
    let mut entry_pred_set: BTreeSet<BasicBlockId> = BTreeSet::new();
    for entry in info.carrier_phis.values() {
        entry_pred_set.insert(entry.entry_incoming.0);
    }
    if let Some(host_entry_block) = host_entry_block_opt {
        entry_pred_set.insert(host_entry_block);
    }
    entry_pred_set.remove(&latch_block);

    let mut entry_preds: Vec<BasicBlockId> = header_preds
        .iter()
        .filter(|&&pred| entry_pred_set.contains(&pred))
        .copied()
        .collect();

    let mut host_entry_added = false;
    if let Some(host_entry_block) = host_entry_block_opt {
        if !entry_preds.contains(&host_entry_block) && host_entry_block != latch_block {
            entry_preds.push(host_entry_block);
            host_entry_added = true;
        }
    }

    let latch_preds: Vec<BasicBlockId> = header_preds
        .iter()
        .filter(|&&pred| !entry_pred_set.contains(&pred))
        .copied()
        .collect();

    HeaderPredGroups {
        entry_preds,
        latch_preds,
        host_entry_added,
    }
}
