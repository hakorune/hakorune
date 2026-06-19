use std::collections::HashMap;

use crate::mir::string_corridor_recognizer::{
    match_len_call, match_method_set_call, match_substring_call,
};
use crate::mir::value_origin::ValueDefMap;
use crate::mir::{MirFunction, MirInstruction, ValueId};

use super::{StringKernelPlanTextConsumer, ValueOriginCache};

#[derive(Clone, Copy, Default)]
pub(super) struct ReadAliasConsumerScan {
    pub(super) direct_set_uses: usize,
    pub(super) substring_uses: usize,
    pub(super) len_observer_uses: usize,
    pub(super) other_uses: usize,
}

#[derive(Clone, Copy, Default)]
struct TextConsumerScan {
    slot_text_uses: usize,
    non_slot_uses: usize,
}

pub(super) struct StringKernelConsumerAnalysis {
    text_consumers: HashMap<ValueId, TextConsumerScan>,
    read_alias_consumers: HashMap<ValueId, ReadAliasConsumerScan>,
}

impl StringKernelConsumerAnalysis {
    pub(super) fn new(function: &MirFunction, def_map: &ValueDefMap) -> Self {
        let mut origins = ValueOriginCache::new(function, def_map);
        let mut analysis = Self {
            text_consumers: HashMap::new(),
            read_alias_consumers: HashMap::new(),
        };

        for block in function.blocks.values() {
            for inst in &block.instructions {
                analysis.record_instruction(&mut origins, inst);
            }
            if let Some(term) = &block.terminator {
                analysis.record_instruction(&mut origins, term);
            }
        }

        analysis
    }

    pub(super) fn text_consumer(&self, plan_root: ValueId) -> Option<StringKernelPlanTextConsumer> {
        text_consumer_from_scan(
            self.text_consumers
                .get(&plan_root)
                .copied()
                .unwrap_or_default(),
        )
    }

    pub(super) fn read_alias_scan(&self, plan_root: ValueId) -> ReadAliasConsumerScan {
        self.read_alias_consumers
            .get(&plan_root)
            .copied()
            .unwrap_or_default()
    }

    fn record_instruction(&mut self, origins: &mut ValueOriginCache<'_>, inst: &MirInstruction) {
        self.record_text_consumer_use(origins, inst);
        self.record_read_alias_consumer_use(origins, inst);
    }

    fn text_scan_mut(&mut self, root: ValueId) -> &mut TextConsumerScan {
        self.text_consumers.entry(root).or_default()
    }

    fn read_alias_scan_mut(&mut self, root: ValueId) -> &mut ReadAliasConsumerScan {
        self.read_alias_consumers.entry(root).or_default()
    }

    fn record_text_consumer_use(
        &mut self,
        origins: &mut ValueOriginCache<'_>,
        inst: &MirInstruction,
    ) {
        if let Some((_, receiver, _, _, _)) = match_substring_call(inst) {
            let receiver_root = origins.origin(receiver);
            self.text_scan_mut(receiver_root).slot_text_uses += 1;
            self.record_text_fallback_uses_except(origins, inst, Some(receiver_root));
            return;
        }

        if let Some(store) = match_method_set_call(inst) {
            let value_root = origins.origin(store.value);
            self.text_scan_mut(value_root).non_slot_uses += 1;
            self.record_text_fallback_uses_except(origins, inst, Some(value_root));
            return;
        }

        match inst {
            MirInstruction::Return {
                value: Some(value), ..
            }
            | MirInstruction::Store { value, .. }
            | MirInstruction::FieldSet { value, .. } => {
                let value_root = origins.origin(*value);
                self.text_scan_mut(value_root).non_slot_uses += 1;
                return;
            }
            MirInstruction::Call {
                callee:
                    Some(crate::mir::Callee::Method {
                        method,
                        receiver: Some(receiver),
                        ..
                    }),
                ..
            } => {
                let receiver_root = origins.origin(*receiver);
                if matches!(method.as_str(), "length" | "size") {
                    self.record_text_fallback_uses_except(origins, inst, Some(receiver_root));
                } else {
                    self.text_scan_mut(receiver_root).non_slot_uses += 1;
                    self.record_text_fallback_uses_except(origins, inst, Some(receiver_root));
                }
                return;
            }
            MirInstruction::Phi { .. } => return,
            _ => {}
        }

        self.record_text_fallback_uses_except(origins, inst, None);
    }

    fn record_text_fallback_uses_except(
        &mut self,
        origins: &mut ValueOriginCache<'_>,
        inst: &MirInstruction,
        except: Option<ValueId>,
    ) {
        for value in inst.used_values() {
            let root = origins.origin(value);
            if Some(root) == except {
                continue;
            }
            self.text_scan_mut(root).non_slot_uses += 1;
        }
    }

    fn record_read_alias_consumer_use(
        &mut self,
        origins: &mut ValueOriginCache<'_>,
        inst: &MirInstruction,
    ) {
        if let MirInstruction::Copy { src, .. } = inst {
            let src_root = origins.origin(*src);
            self.read_alias_consumers.entry(src_root).or_default();
            return;
        }

        if let Some((_, receiver, _, _, _)) = match_substring_call(inst) {
            let receiver_root = origins.origin(receiver);
            self.read_alias_scan_mut(receiver_root).substring_uses += 1;
            self.record_read_alias_fallback_uses_except(origins, inst, Some(receiver_root));
            return;
        }

        if let Some((_, receiver, _)) = match_len_call(inst) {
            let receiver_root = origins.origin(receiver);
            self.read_alias_scan_mut(receiver_root).len_observer_uses += 1;
            self.record_read_alias_fallback_uses_except(origins, inst, Some(receiver_root));
            return;
        }

        if let Some(store) = match_method_set_call(inst) {
            let value_root = origins.origin(store.value);
            self.read_alias_scan_mut(value_root).direct_set_uses += 1;
            self.record_read_alias_fallback_uses_except(origins, inst, Some(value_root));
            return;
        }

        self.record_read_alias_fallback_uses_except(origins, inst, None);
    }

    fn record_read_alias_fallback_uses_except(
        &mut self,
        origins: &mut ValueOriginCache<'_>,
        inst: &MirInstruction,
        except: Option<ValueId>,
    ) {
        for value in inst.used_values() {
            let root = origins.origin(value);
            if Some(root) == except {
                continue;
            }
            self.read_alias_scan_mut(root).other_uses += 1;
        }
    }
}

fn text_consumer_from_scan(scan: TextConsumerScan) -> Option<StringKernelPlanTextConsumer> {
    if scan.non_slot_uses > 0 || scan.slot_text_uses > 1 {
        Some(StringKernelPlanTextConsumer::ExplicitColdPublish)
    } else if scan.slot_text_uses == 1 {
        Some(StringKernelPlanTextConsumer::SlotText)
    } else {
        None
    }
}
