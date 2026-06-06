/*!
 * FastMemory table length facts.
 *
 * This module owns the semantic-memory metadata seam for TableIndex length
 * facts. It does not choose a page-map strategy and it does not infer table
 * lengths from layout contracts. Current refresh only normalizes explicit
 * facts already attached to the function.
 */

use crate::mir::{MirFunction, ValueId};
use std::collections::HashSet;

pub fn refresh_function_fastmem_table_length_facts(function: &mut MirFunction) {
    let mut seen = HashSet::new();
    let mut facts = Vec::new();

    for mut fact in function.metadata.fastmem_table_length_facts.drain(..) {
        if fact.table_id.is_empty()
            || fact.table_value == ValueId::INVALID
            || fact.length_value == ValueId::INVALID
            || fact.resolved_length == Some(0)
        {
            continue;
        }
        let key = (
            fact.region,
            fact.table_id.clone(),
            fact.table_value,
            fact.length_value,
            fact.resolved_length,
            fact.policy,
        );
        if !seen.insert(key) {
            continue;
        }
        fact.fact_id = facts.len() as u32;
        facts.push(fact);
    }

    function.metadata.fastmem_table_length_facts = facts;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::function::{FastMemTableLengthFact, FastMemTableLengthPolicyKind};
    use crate::mir::instruction::FastMemRegionId;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirType};

    fn make_function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "Main.fastmem/0".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        )
    }

    fn length_fact(fact_id: u32, table_value: ValueId) -> FastMemTableLengthFact {
        FastMemTableLengthFact {
            fact_id,
            region: FastMemRegionId::new(0),
            table_id: "page_table".to_string(),
            table_value,
            length_value: ValueId::new(50),
            resolved_length: Some(64),
            policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
        }
    }

    #[test]
    fn refresh_preserves_explicit_length_facts_and_reassigns_ids() {
        let mut function = make_function();
        function
            .metadata
            .fastmem_table_length_facts
            .push(length_fact(42, ValueId::new(1)));

        refresh_function_fastmem_table_length_facts(&mut function);

        assert_eq!(function.metadata.fastmem_table_length_facts.len(), 1);
        let fact = &function.metadata.fastmem_table_length_facts[0];
        assert_eq!(fact.fact_id, 0);
        assert_eq!(fact.table_id, "page_table");
        assert_eq!(fact.table_value, ValueId::new(1));
        assert_eq!(fact.length_value, ValueId::new(50));
        assert_eq!(fact.resolved_length, Some(64));
        assert_eq!(fact.policy, FastMemTableLengthPolicyKind::ExplicitConstLen);
    }

    #[test]
    fn refresh_drops_duplicate_and_malformed_length_facts() {
        let mut function = make_function();
        function
            .metadata
            .fastmem_table_length_facts
            .push(length_fact(10, ValueId::new(1)));
        function
            .metadata
            .fastmem_table_length_facts
            .push(length_fact(11, ValueId::new(1)));
        let mut malformed = length_fact(12, ValueId::new(2));
        malformed.length_value = ValueId::INVALID;
        function.metadata.fastmem_table_length_facts.push(malformed);

        refresh_function_fastmem_table_length_facts(&mut function);

        assert_eq!(function.metadata.fastmem_table_length_facts.len(), 1);
        assert_eq!(function.metadata.fastmem_table_length_facts[0].fact_id, 0);
        assert_eq!(
            function.metadata.fastmem_table_length_facts[0].table_value,
            ValueId::new(1)
        );
    }
}
