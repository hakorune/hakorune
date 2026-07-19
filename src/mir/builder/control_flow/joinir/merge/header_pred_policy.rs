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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::control_flow::joinir::merge::loop_header_phi_info::CarrierPhiEntry;
    use crate::mir::join_ir::lowering::carrier_info::CarrierRole;
    use crate::mir::ValueId;

    fn block(id: u32) -> BasicBlockId {
        BasicBlockId::new(id)
    }

    fn header_info(entry: BasicBlockId) -> LoopHeaderPhiInfo {
        let mut info = LoopHeaderPhiInfo::empty(block(9));
        info.carrier_phis.insert(
            "index".to_string(),
            CarrierPhiEntry {
                phi_dst: ValueId::new(30),
                entry_incoming: (entry, ValueId::new(1)),
                latch_incoming: Some((block(8), ValueId::new(2))),
                role: CarrierRole::LoopState,
            },
        );
        info
    }

    #[test]
    fn host_entry_is_a_future_entry_predecessor_when_its_terminator_is_unpublished() {
        let entry = block(1);
        let latch = block(8);
        let host = block(0);
        let groups = split_header_preds(&header_info(entry), &[entry, latch], Some(host), latch);

        assert_eq!(groups.entry_preds, vec![entry, host]);
        assert_eq!(groups.latch_preds, vec![latch]);
        assert!(groups.host_entry_added);
    }
}
