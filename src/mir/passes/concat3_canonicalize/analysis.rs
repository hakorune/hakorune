mod def_use;
mod stringish;

use std::collections::{HashMap, HashSet};

use crate::mir::{BasicBlockId, MirFunction, ValueId};

pub(super) fn infer_stringish_values(function: &MirFunction) -> HashSet<ValueId> {
    stringish::infer_stringish_values(function)
}

pub(super) fn build_def_map(function: &MirFunction) -> HashMap<ValueId, (BasicBlockId, usize)> {
    def_use::build_def_map(function)
}

pub(super) fn build_use_counts(function: &MirFunction) -> HashMap<ValueId, usize> {
    def_use::build_use_counts(function)
}
