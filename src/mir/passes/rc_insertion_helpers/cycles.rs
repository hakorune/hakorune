#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::BasicBlockId;
#[cfg(feature = "rc-insertion-minimal")]
use std::collections::{HashMap, HashSet};

#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn detect_jump_chain_cycles(
    jump_chain_next: &HashMap<BasicBlockId, BasicBlockId>,
) -> HashSet<BasicBlockId> {
    let mut cycles: HashSet<BasicBlockId> = HashSet::new();
    let mut done: HashSet<BasicBlockId> = HashSet::new();

    for start in jump_chain_next.keys() {
        if done.contains(start) {
            continue;
        }
        let mut path: Vec<BasicBlockId> = Vec::new();
        let mut index: HashMap<BasicBlockId, usize> = HashMap::new();
        let mut current = *start;

        loop {
            if done.contains(&current) {
                break;
            }
            if let Some(&pos) = index.get(&current) {
                for node in &path[pos..] {
                    cycles.insert(*node);
                }
                break;
            }

            index.insert(current, path.len());
            path.push(current);

            let Some(next) = jump_chain_next.get(&current) else {
                break;
            };
            current = *next;
        }

        for node in path {
            done.insert(node);
        }
    }

    cycles
}
